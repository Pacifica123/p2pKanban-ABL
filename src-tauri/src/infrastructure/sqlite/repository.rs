use rusqlite::{params, Connection, OptionalExtension, Transaction, TransactionBehavior};

use crate::{
    application::repository::{
        PlannerMutation, PlannerRepository, PlannerTransaction, RepositoryError,
    },
    domain::planner::{
        compare_card_order, AccessEpoch, BoardId, CardId, CardLifecycle, CardRecord, CardTombstone,
        ColumnId, OrderKey, VersionStamp, WorkspaceId,
    },
};

use crate::infrastructure::profile::ProfileStoragePaths;

use super::migration::{open_profile, ProfileOpenError, ProfileSchemaInfo};

pub struct SqlitePlannerRepository {
    connection: Connection,
    schema: ProfileSchemaInfo,
}

impl SqlitePlannerRepository {
    pub fn open(layout: &ProfileStoragePaths) -> Result<Self, ProfileOpenError> {
        let (connection, schema) = open_profile(layout)?;
        Ok(Self { connection, schema })
    }

    pub fn schema_info(&self) -> ProfileSchemaInfo {
        self.schema
    }

    #[cfg(test)]
    fn seed_contract_scope(&mut self) {
        self.connection
            .execute(
                "INSERT INTO workspaces(id, access_epoch) VALUES (?1, '3')",
                ["018f0000-0000-7000-8000-000000000001"],
            )
            .unwrap();
        self.connection
            .execute(
                "INSERT INTO boards(id, workspace_id) VALUES (?1, ?2)",
                params![
                    "018f0000-0000-7000-8000-000000000002",
                    "018f0000-0000-7000-8000-000000000001"
                ],
            )
            .unwrap();
        for column in [
            "018f0000-0000-7000-8000-000000000005",
            "018f0000-0000-7000-8000-000000000006",
        ] {
            self.connection
                .execute(
                    "INSERT INTO columns(id, board_id) VALUES (?1, ?2)",
                    params![column, "018f0000-0000-7000-8000-000000000002"],
                )
                .unwrap();
        }
    }
}

fn storage_failure<T>(_: T) -> RepositoryError {
    RepositoryError::StorageFailure
}

fn current_access_epoch_on(
    conn: &Connection,
    workspace_id: &WorkspaceId,
) -> Result<AccessEpoch, RepositoryError> {
    let value: Option<String> = conn
        .query_row(
            "SELECT access_epoch FROM workspaces WHERE id = ?1",
            [workspace_id.as_str()],
            |row| row.get(0),
        )
        .optional()
        .map_err(storage_failure)?;
    let value = value.ok_or(RepositoryError::WorkspaceNotFound)?;
    AccessEpoch::new(value.parse::<u64>().map_err(storage_failure)?).map_err(storage_failure)
}

fn parse_card(
    id: String,
    workspace_id: String,
    board_id: String,
    column_id: String,
    title: String,
    position: f64,
    lifecycle: String,
) -> Result<CardRecord, RepositoryError> {
    let lifecycle = match lifecycle.as_str() {
        "active" => CardLifecycle::Active,
        "archived" => CardLifecycle::Archived,
        _ => return Err(RepositoryError::StorageFailure),
    };
    Ok(CardRecord {
        id: CardId::new(id).map_err(storage_failure)?,
        workspace_id: WorkspaceId::new(workspace_id).map_err(storage_failure)?,
        board_id: BoardId::new(board_id).map_err(storage_failure)?,
        column_id: ColumnId::new(column_id).map_err(storage_failure)?,
        title,
        position: OrderKey::new(position).map_err(storage_failure)?,
        lifecycle,
    })
}

fn card_by_id(conn: &Connection, card_id: &CardId) -> Result<Option<CardRecord>, RepositoryError> {
    let raw = conn
        .query_row(
            "SELECT id, workspace_id, board_id, column_id, title, position, lifecycle FROM cards WHERE id = ?1",
            [card_id.as_str()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, f64>(5)?,
                    row.get::<_, String>(6)?,
                ))
            },
        )
        .optional()
        .map_err(storage_failure)?;
    raw.map(|(id, ws, board, column, title, position, lifecycle)| {
        parse_card(id, ws, board, column, title, position, lifecycle)
    })
    .transpose()
}

fn tombstone_by_id(
    conn: &Connection,
    card_id: &CardId,
) -> Result<Option<CardTombstone>, RepositoryError> {
    let raw = conn
        .query_row(
            "SELECT workspace_id, board_id, card_id, logical_clock, replica_id, event_id FROM card_tombstones WHERE card_id = ?1",
            [card_id.as_str()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                ))
            },
        )
        .optional()
        .map_err(storage_failure)?;
    raw.map(|(workspace, board, card, clock, replica, event)| {
        Ok(CardTombstone {
            workspace_id: WorkspaceId::new(workspace).map_err(storage_failure)?,
            board_id: BoardId::new(board).map_err(storage_failure)?,
            card_id: CardId::new(card).map_err(storage_failure)?,
            version: VersionStamp::new(
                clock.parse::<u64>().map_err(storage_failure)?,
                replica,
                event,
            )
            .map_err(storage_failure)?,
        })
    })
    .transpose()
}

fn board_workspace(conn: &Connection, board_id: &BoardId) -> Result<WorkspaceId, RepositoryError> {
    let value: Option<String> = conn
        .query_row(
            "SELECT workspace_id FROM boards WHERE id = ?1",
            [board_id.as_str()],
            |row| row.get(0),
        )
        .optional()
        .map_err(storage_failure)?;
    value
        .ok_or(RepositoryError::BoardNotFound)
        .and_then(|value| WorkspaceId::new(value).map_err(storage_failure))
}

fn column_board(conn: &Connection, column_id: &ColumnId) -> Result<BoardId, RepositoryError> {
    let value: Option<String> = conn
        .query_row(
            "SELECT board_id FROM columns WHERE id = ?1",
            [column_id.as_str()],
            |row| row.get(0),
        )
        .optional()
        .map_err(storage_failure)?;
    value
        .ok_or(RepositoryError::ColumnNotFound)
        .and_then(|value| BoardId::new(value).map_err(storage_failure))
}

fn apply_mutation(
    tx: &Transaction<'_>,
    workspace_id: &WorkspaceId,
    mutation: PlannerMutation,
) -> Result<(), RepositoryError> {
    match mutation {
        PlannerMutation::CreateCard(card) => {
            if board_workspace(tx, &card.board_id)? != *workspace_id
                || card.workspace_id != *workspace_id
                || column_board(tx, &card.column_id)? != card.board_id
            {
                return Err(RepositoryError::ScopeMismatch);
            }
            if tombstone_by_id(tx, &card.id)?.is_some() {
                return Err(RepositoryError::Tombstoned);
            }
            if card_by_id(tx, &card.id)?.is_some() {
                return Err(RepositoryError::DuplicateCard);
            }
            let lifecycle = match card.lifecycle {
                CardLifecycle::Active => "active",
                CardLifecycle::Archived => "archived",
            };
            tx.execute(
                "INSERT INTO cards(id, workspace_id, board_id, column_id, title, position, lifecycle) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    card.id.as_str(),
                    card.workspace_id.as_str(),
                    card.board_id.as_str(),
                    card.column_id.as_str(),
                    card.title,
                    card.position.get(),
                    lifecycle,
                ],
            )
            .map_err(storage_failure)?;
        }
        PlannerMutation::MoveCard {
            card_id,
            target_column_id,
            position,
        } => {
            let card = card_by_id(tx, &card_id)?.ok_or(RepositoryError::CardNotFound)?;
            if card.workspace_id != *workspace_id {
                return Err(RepositoryError::ScopeMismatch);
            }
            if column_board(tx, &target_column_id)? != card.board_id {
                return Err(RepositoryError::ScopeMismatch);
            }
            tx.execute(
                "UPDATE cards SET column_id = ?1, position = ?2 WHERE id = ?3",
                params![target_column_id.as_str(), position.get(), card_id.as_str()],
            )
            .map_err(storage_failure)?;
        }
        PlannerMutation::SetCardArchived { card_id, archived } => {
            let card = card_by_id(tx, &card_id)?.ok_or(RepositoryError::CardNotFound)?;
            if card.workspace_id != *workspace_id {
                return Err(RepositoryError::ScopeMismatch);
            }
            tx.execute(
                "UPDATE cards SET lifecycle = ?1 WHERE id = ?2",
                params![if archived { "archived" } else { "active" }, card_id.as_str()],
            )
            .map_err(storage_failure)?;
        }
        PlannerMutation::DeleteCard { card_id, version } => {
            if let Some(existing) = tombstone_by_id(tx, &card_id)? {
                if existing.workspace_id != *workspace_id {
                    return Err(RepositoryError::ScopeMismatch);
                }
                if version > existing.version {
                    tx.execute(
                        "UPDATE card_tombstones SET logical_clock = ?1, replica_id = ?2, event_id = ?3 WHERE card_id = ?4",
                        params![
                            version.logical_clock.to_string(),
                            version.replica_id,
                            version.event_id,
                            card_id.as_str(),
                        ],
                    )
                    .map_err(storage_failure)?;
                }
                return Ok(());
            }
            let card = card_by_id(tx, &card_id)?.ok_or(RepositoryError::CardNotFound)?;
            if card.workspace_id != *workspace_id {
                return Err(RepositoryError::ScopeMismatch);
            }
            tx.execute("DELETE FROM cards WHERE id = ?1", [card_id.as_str()])
                .map_err(storage_failure)?;
            tx.execute(
                "INSERT INTO card_tombstones(card_id, workspace_id, board_id, logical_clock, replica_id, event_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    card_id.as_str(),
                    card.workspace_id.as_str(),
                    card.board_id.as_str(),
                    version.logical_clock.to_string(),
                    version.replica_id,
                    version.event_id,
                ],
            )
            .map_err(storage_failure)?;
        }
        PlannerMutation::ReorderColumn { column_id, positions } => {
            let target_board = column_board(tx, &column_id)?;
            if board_workspace(tx, &target_board)? != *workspace_id {
                return Err(RepositoryError::ScopeMismatch);
            }
            let mut seen = std::collections::BTreeSet::new();
            for (card_id, _) in &positions {
                if !seen.insert(card_id.clone()) {
                    return Err(RepositoryError::DuplicateReorderItem);
                }
                let card = card_by_id(tx, card_id)?.ok_or(RepositoryError::CardNotFound)?;
                if card.board_id != target_board || card.column_id != column_id {
                    return Err(RepositoryError::ScopeMismatch);
                }
            }
            for (card_id, position) in positions {
                tx.execute(
                    "UPDATE cards SET position = ?1 WHERE id = ?2",
                    params![position.get(), card_id.as_str()],
                )
                .map_err(storage_failure)?;
            }
        }
    }
    Ok(())
}

impl PlannerRepository for SqlitePlannerRepository {
    fn current_access_epoch(
        &self,
        workspace_id: &WorkspaceId,
    ) -> Result<AccessEpoch, RepositoryError> {
        current_access_epoch_on(&self.connection, workspace_id)
    }

    fn get_card(&self, card_id: &CardId) -> Result<Option<CardRecord>, RepositoryError> {
        card_by_id(&self.connection, card_id)
    }

    fn get_card_tombstone(
        &self,
        card_id: &CardId,
    ) -> Result<Option<CardTombstone>, RepositoryError> {
        tombstone_by_id(&self.connection, card_id)
    }

    fn list_column_cards(
        &self,
        board_id: &BoardId,
        column_id: &ColumnId,
        include_archived: bool,
    ) -> Result<Vec<CardRecord>, RepositoryError> {
        let mut stmt = self
            .connection
            .prepare(
                "SELECT id, workspace_id, board_id, column_id, title, position, lifecycle FROM cards WHERE board_id = ?1 AND column_id = ?2 AND (?3 = 1 OR lifecycle = 'active')",
            )
            .map_err(storage_failure)?;
        let rows = stmt
            .query_map(
                params![board_id.as_str(), column_id.as_str(), if include_archived { 1 } else { 0 }],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, String>(4)?,
                        row.get::<_, f64>(5)?,
                        row.get::<_, String>(6)?,
                    ))
                },
            )
            .map_err(storage_failure)?;
        let mut cards = Vec::new();
        for row in rows {
            let (id, ws, board, column, title, position, lifecycle) = row.map_err(storage_failure)?;
            cards.push(parse_card(id, ws, board, column, title, position, lifecycle)?);
        }
        cards.sort_by(compare_card_order);
        Ok(cards)
    }

    fn commit(&mut self, transaction: PlannerTransaction) -> Result<(), RepositoryError> {
        let tx = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(storage_failure)?;
        let current = current_access_epoch_on(&tx, &transaction.workspace_id)?;
        if current != transaction.access_epoch {
            return Err(RepositoryError::StaleAccessEpoch {
                provided: transaction.access_epoch,
                current,
            });
        }
        for mutation in transaction.mutations {
            apply_mutation(&tx, &transaction.workspace_id, mutation)?;
        }
        tx.commit().map_err(storage_failure)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        application::repository::{
            contract::{
                assert_atomic_rollback_contract, assert_reorder_contract,
                assert_repository_contract, assert_stale_epoch_contract,
            },
            PlannerMutation, PlannerTransaction,
        },
        domain::planner::{
            AccessEpoch, BoardId, CardId, CardLifecycle, CardRecord, ColumnId, OrderKey,
            WorkspaceId,
        },
    };
    use std::{fs, sync::atomic::{AtomicU64, Ordering}};

    static NEXT: AtomicU64 = AtomicU64::new(1);

    fn temp_profile(name: &str) -> ProfileStoragePaths {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        ProfileStoragePaths::new(std::env::temp_dir().join(format!(
            "p2pkanban-a06-repo-{name}-{}-{n}/profiles/default",
            std::process::id()
        )))
    }

    fn cleanup(layout: &ProfileStoragePaths) {
        let root = layout
            .root()
            .ancestors()
            .nth(2)
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| layout.root().to_path_buf());
        let _ = fs::remove_dir_all(root);
    }

    fn fixture_repo(layout: &ProfileStoragePaths) -> SqlitePlannerRepository {
        cleanup(layout);
        let mut repo = SqlitePlannerRepository::open(layout).unwrap();
        repo.seed_contract_scope();
        repo
    }

    #[test]
    fn sqlite_adapter_passes_the_same_a04_repository_contract() {
        let layout = temp_profile("contract");
        let repo = fixture_repo(&layout);
        assert_repository_contract(repo);
        cleanup(&layout);
    }

    #[test]
    fn sqlite_adapter_preserves_stale_epoch_rejection() {
        let layout = temp_profile("stale-epoch");
        let repo = fixture_repo(&layout);
        assert_stale_epoch_contract(repo);
        cleanup(&layout);
    }

    #[test]
    fn sqlite_adapter_preserves_atomic_batch_rollback() {
        let layout = temp_profile("atomic-rollback");
        let repo = fixture_repo(&layout);
        assert_atomic_rollback_contract(repo);
        cleanup(&layout);
    }

    #[test]
    fn sqlite_adapter_preserves_reorder_semantics() {
        let layout = temp_profile("reorder");
        let repo = fixture_repo(&layout);
        assert_reorder_contract(repo);
        cleanup(&layout);
    }

    #[test]
    fn card_survives_close_and_reopen() {
        let layout = temp_profile("reopen");
        let mut repo = fixture_repo(&layout);
        let card_id = CardId::new("018f0000-0000-7000-8000-000000000777").unwrap();
        let card = CardRecord {
            id: card_id.clone(),
            workspace_id: WorkspaceId::new("018f0000-0000-7000-8000-000000000001").unwrap(),
            board_id: BoardId::new("018f0000-0000-7000-8000-000000000002").unwrap(),
            column_id: ColumnId::new("018f0000-0000-7000-8000-000000000005").unwrap(),
            title: "durable card".into(),
            position: OrderKey::new(1000.0).unwrap(),
            lifecycle: CardLifecycle::Active,
        };
        repo.commit(PlannerTransaction {
            workspace_id: card.workspace_id.clone(),
            access_epoch: AccessEpoch::new(3).unwrap(),
            mutations: vec![PlannerMutation::CreateCard(card.clone())],
        })
        .unwrap();
        drop(repo);

        let reopened = SqlitePlannerRepository::open(&layout).unwrap();
        assert_eq!(reopened.get_card(&card_id).unwrap(), Some(card));
        drop(reopened);
        cleanup(&layout);
    }
}
