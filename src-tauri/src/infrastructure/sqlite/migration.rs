use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use rusqlite::{Connection, TransactionBehavior};

pub const CURRENT_SCHEMA_VERSION: u32 = 1;
pub const MIN_READER_SCHEMA_VERSION: u32 = 1;
pub const MIN_WRITER_SCHEMA_VERSION: u32 = 1;
pub const BUSY_TIMEOUT_MS: u64 = 2_500;

pub const MIGRATION_V0_TO_V1_ID: &str = "desktop-0001-initial-planner";
pub const MIGRATION_V0_TO_V1_SHA256: &str = "c88155c8d7093f5fe15d076043aebe0359746fadf509fc0acb3883188b103594";
const SCHEMA_V1: &str = include_str!("../../../migrations/0001_initial.sql");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfileSchemaInfo {
    pub schema_version: u32,
    pub min_reader: u32,
    pub min_writer: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileOpenError {
    Io,
    Database,
    UnsupportedSchema { found: u32, max_writer: u32 },
    InvalidSchemaMetadata,
    IntegrityCheckFailed,
    MigrationRecovered,
    WalUnavailable,
}

impl From<rusqlite::Error> for ProfileOpenError {
    fn from(_: rusqlite::Error) -> Self {
        Self::Database
    }
}

fn epoch_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis().min(i64::MAX as u128) as i64)
        .unwrap_or(0)
}

fn sidecar_path(profile: &Path, suffix: &str) -> PathBuf {
    let name = profile
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("profile.db");
    profile.with_file_name(format!("{name}.{suffix}"))
}

fn backup_path(profile: &Path, from_version: u32) -> PathBuf {
    sidecar_path(profile, &format!("pre-migration-v{from_version}.sqlite"))
}

fn journal_path(profile: &Path) -> PathBuf {
    sidecar_path(profile, "migration-journal.json")
}

fn write_journal(path: &Path, from: u32, to: u32, state: &str) -> Result<(), ProfileOpenError> {
    let body = format!(
        "{{\n  \"formatVersion\": 1,\n  \"fromSchema\": {from},\n  \"toSchema\": {to},\n  \"state\": \"{state}\"\n}}\n"
    );
    fs::write(path, body).map_err(|_| ProfileOpenError::Io)
}

fn configure_connection(conn: &Connection, file_backed: bool) -> Result<(), ProfileOpenError> {
    conn.busy_timeout(std::time::Duration::from_millis(BUSY_TIMEOUT_MS))?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.pragma_update(None, "synchronous", "FULL")?;

    if file_backed {
        let mode: String = conn.query_row("PRAGMA journal_mode=WAL", [], |row| row.get(0))?;
        if !mode.eq_ignore_ascii_case("wal") {
            return Err(ProfileOpenError::WalUnavailable);
        }
    }
    Ok(())
}

fn pragma_user_version(conn: &Connection) -> Result<u32, ProfileOpenError> {
    let version: i64 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;
    u32::try_from(version).map_err(|_| ProfileOpenError::InvalidSchemaMetadata)
}

fn schema_info(conn: &Connection) -> Result<ProfileSchemaInfo, ProfileOpenError> {
    let (schema, reader, writer): (i64, i64, i64) = conn.query_row(
        "SELECT schema_version, min_reader, min_writer FROM profile_meta WHERE singleton = 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )?;
    let info = ProfileSchemaInfo {
        schema_version: u32::try_from(schema).map_err(|_| ProfileOpenError::InvalidSchemaMetadata)?,
        min_reader: u32::try_from(reader).map_err(|_| ProfileOpenError::InvalidSchemaMetadata)?,
        min_writer: u32::try_from(writer).map_err(|_| ProfileOpenError::InvalidSchemaMetadata)?,
    };
    if info.schema_version == 0 || info.min_reader == 0 || info.min_writer == 0 {
        return Err(ProfileOpenError::InvalidSchemaMetadata);
    }
    Ok(info)
}

fn check_integrity(conn: &Connection) -> Result<(), ProfileOpenError> {
    let integrity: String = conn.query_row("PRAGMA integrity_check", [], |row| row.get(0))?;
    if integrity != "ok" {
        return Err(ProfileOpenError::IntegrityCheckFailed);
    }
    let fk_rows: i64 = conn.query_row(
        "SELECT COUNT(*) FROM pragma_foreign_key_check",
        [],
        |row| row.get(0),
    )?;
    if fk_rows != 0 {
        return Err(ProfileOpenError::IntegrityCheckFailed);
    }
    Ok(())
}

fn backup_is_valid(path: &Path) -> Result<(), ProfileOpenError> {
    let conn = Connection::open(path)?;
    check_integrity(&conn)
}

fn apply_v0_to_v1(conn: &mut Connection, force_failure: bool) -> Result<(), ProfileOpenError> {
    let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
    tx.execute_batch(SCHEMA_V1)?;
    let applied_at = epoch_millis();
    tx.execute(
        "UPDATE profile_meta SET created_at_unix_ms = ?1 WHERE singleton = 1",
        [applied_at],
    )?;
    tx.execute(
        "INSERT INTO schema_migrations(id, checksum, applied_at_unix_ms) VALUES (?1, ?2, ?3)",
        rusqlite::params![MIGRATION_V0_TO_V1_ID, MIGRATION_V0_TO_V1_SHA256, applied_at],
    )?;
    if force_failure {
        return Err(ProfileOpenError::MigrationRecovered);
    }
    tx.commit()?;
    Ok(())
}

fn migrate_if_needed(
    conn: &mut Connection,
    profile_path: &Path,
    existed_before_open: bool,
    force_failure: bool,
) -> Result<(), ProfileOpenError> {
    let from = pragma_user_version(conn)?;
    if from > CURRENT_SCHEMA_VERSION {
        return Err(ProfileOpenError::UnsupportedSchema {
            found: from,
            max_writer: CURRENT_SCHEMA_VERSION,
        });
    }
    if from == CURRENT_SCHEMA_VERSION {
        return Ok(());
    }
    if from != 0 {
        return Err(ProfileOpenError::UnsupportedSchema {
            found: from,
            max_writer: CURRENT_SCHEMA_VERSION,
        });
    }

    let journal = journal_path(profile_path);
    let backup = backup_path(profile_path, from);
    let should_backup = existed_before_open;
    if should_backup {
        let _ = fs::remove_file(&backup);
        conn.backup("main", &backup, None)?;
        backup_is_valid(&backup)?;
    }
    write_journal(&journal, from, CURRENT_SCHEMA_VERSION, "prepared")?;

    let migration_result = apply_v0_to_v1(conn, force_failure).and_then(|_| check_integrity(conn));
    if let Err(err) = migration_result {
        if should_backup {
            conn.restore("main", &backup, None::<fn(rusqlite::backup::Progress)>)?;
            backup_is_valid(&backup)?;
        }
        write_journal(&journal, from, CURRENT_SCHEMA_VERSION, "restored-after-failure")?;
        return Err(err);
    }

    write_journal(&journal, from, CURRENT_SCHEMA_VERSION, "completed")?;
    let _ = fs::remove_file(&journal);
    Ok(())
}

fn verify_migration_line(conn: &Connection) -> Result<(), ProfileOpenError> {
    let row: (String, String) = conn.query_row(
        "SELECT id, checksum FROM schema_migrations ORDER BY applied_at_unix_ms, id LIMIT 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    if row.0 != MIGRATION_V0_TO_V1_ID || row.1 != MIGRATION_V0_TO_V1_SHA256 {
        return Err(ProfileOpenError::InvalidSchemaMetadata);
    }
    Ok(())
}

pub(crate) fn open_profile(path: &Path) -> Result<(Connection, ProfileSchemaInfo), ProfileOpenError> {
    let existed_before_open = path.exists();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|_| ProfileOpenError::Io)?;
    }
    let mut conn = Connection::open(path)?;
    configure_connection(&conn, true)?;
    migrate_if_needed(&mut conn, path, existed_before_open, false)?;
    let info = schema_info(&conn)?;
    verify_migration_line(&conn)?;
    if info.schema_version != CURRENT_SCHEMA_VERSION
        || info.min_reader > CURRENT_SCHEMA_VERSION
        || info.min_writer > CURRENT_SCHEMA_VERSION
    {
        return Err(ProfileOpenError::UnsupportedSchema {
            found: info.schema_version,
            max_writer: CURRENT_SCHEMA_VERSION,
        });
    }
    check_integrity(&conn)?;
    Ok((conn, info))
}

#[cfg(test)]
pub(crate) fn open_profile_with_forced_migration_failure(
    path: &Path,
) -> Result<(), ProfileOpenError> {
    let existed_before_open = path.exists();
    let mut conn = Connection::open(path)?;
    configure_connection(&conn, true)?;
    migrate_if_needed(&mut conn, path, existed_before_open, true)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT: AtomicU64 = AtomicU64::new(1);

    fn temp_profile(name: &str) -> PathBuf {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("p2pkanban-a05-{name}-{}-{n}.sqlite", std::process::id()))
    }

    fn cleanup(path: &Path) {
        for candidate in [
            path.to_path_buf(),
            sidecar_path(path, "pre-migration-v0.sqlite"),
            sidecar_path(path, "migration-journal.json"),
            PathBuf::from(format!("{}-wal", path.display())),
            PathBuf::from(format!("{}-shm", path.display())),
        ] {
            let _ = fs::remove_file(candidate);
        }
    }

    #[test]
    fn new_profile_uses_v1_metadata_and_durability_pragmas() {
        let path = temp_profile("pragmas");
        cleanup(&path);
        let (conn, info) = open_profile(&path).unwrap();
        assert_eq!(info.schema_version, 1);
        assert_eq!(info.min_reader, 1);
        assert_eq!(info.min_writer, 1);
        let fk: i64 = conn.query_row("PRAGMA foreign_keys", [], |row| row.get(0)).unwrap();
        let sync: i64 = conn.query_row("PRAGMA synchronous", [], |row| row.get(0)).unwrap();
        let mode: String = conn.query_row("PRAGMA journal_mode", [], |row| row.get(0)).unwrap();
        assert_eq!(fk, 1);
        assert_eq!(sync, 2);
        assert_eq!(mode.to_ascii_lowercase(), "wal");
        check_integrity(&conn).unwrap();
        drop(conn);
        cleanup(&path);
    }

    #[test]
    fn newer_writer_schema_is_refused() {
        let path = temp_profile("future");
        cleanup(&path);
        let (conn, _) = open_profile(&path).unwrap();
        conn.pragma_update(None, "user_version", 99).unwrap();
        drop(conn);
        let result = open_profile(&path);
        assert!(matches!(
            result,
            Err(ProfileOpenError::UnsupportedSchema { found: 99, max_writer: 1 })
        ));
        cleanup(&path);
    }

    #[test]
    fn forced_migration_failure_restores_existing_v0_profile() {
        let path = temp_profile("restore");
        cleanup(&path);
        {
            let conn = Connection::open(&path).unwrap();
            conn.execute_batch(
                "CREATE TABLE legacy_marker(value TEXT NOT NULL);\nINSERT INTO legacy_marker(value) VALUES ('keep-me');\nPRAGMA user_version=0;",
            )
            .unwrap();
        }
        assert_eq!(
            open_profile_with_forced_migration_failure(&path),
            Err(ProfileOpenError::MigrationRecovered)
        );
        let conn = Connection::open(&path).unwrap();
        let marker: String = conn
            .query_row("SELECT value FROM legacy_marker", [], |row| row.get(0))
            .unwrap();
        assert_eq!(marker, "keep-me");
        assert_eq!(pragma_user_version(&conn).unwrap(), 0);
        drop(conn);
        assert!(sidecar_path(&path, "pre-migration-v0.sqlite").is_file());
        let journal = fs::read_to_string(sidecar_path(&path, "migration-journal.json")).unwrap();
        assert!(journal.contains("restored-after-failure"));
        cleanup(&path);
    }
}
