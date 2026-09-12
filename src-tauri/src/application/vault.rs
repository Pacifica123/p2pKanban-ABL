use std::{
    collections::BTreeMap,
    sync::{Mutex, MutexGuard},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecretKind {
    RefreshToken,
    BoardCapability,
    DevicePrivateKey,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecretRecordKey {
    pub kind: SecretKind,
    pub scope_id: String,
}

impl SecretRecordKey {
    pub fn new(kind: SecretKind, scope_id: impl Into<String>) -> Result<Self, VaultError> {
        let scope_id = scope_id.into();
        if scope_id.trim().is_empty() {
            return Err(VaultError::InvalidKey);
        }
        Ok(Self { kind, scope_id })
    }
}

/// Secret bytes intentionally do not implement Debug or Display.
///
/// Session-only material is zeroed when the owning value is dropped. A09 may
/// replace this provider with Secret Service/passphrase-backed implementations
/// without changing application callers.
pub struct SecretValue(Vec<u8>);

impl SecretValue {
    pub fn new(bytes: impl Into<Vec<u8>>) -> Result<Self, VaultError> {
        let bytes = bytes.into();
        if bytes.is_empty() {
            return Err(VaultError::EmptySecret);
        }
        Ok(Self(bytes))
    }

    pub fn expose(&self) -> &[u8] {
        &self.0
    }

    fn duplicate(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Drop for SecretValue {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultMode {
    SessionOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VaultStatus {
    pub mode: VaultMode,
    pub durable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaultError {
    InvalidKey,
    EmptySecret,
    Unavailable,
    Poisoned,
}

pub trait SecretVault: Send + Sync {
    fn status(&self) -> VaultStatus;
    fn put(&self, key: SecretRecordKey, value: SecretValue) -> Result<(), VaultError>;
    fn get(&self, key: &SecretRecordKey) -> Result<Option<SecretValue>, VaultError>;
    fn delete(&self, key: &SecretRecordKey) -> Result<(), VaultError>;
}

#[derive(Default)]
pub struct SessionVault {
    values: Mutex<BTreeMap<SecretRecordKey, SecretValue>>,
}

impl SessionVault {
    fn lock(&self) -> Result<MutexGuard<'_, BTreeMap<SecretRecordKey, SecretValue>>, VaultError> {
        self.values.lock().map_err(|_| VaultError::Poisoned)
    }
}

impl SecretVault for SessionVault {
    fn status(&self) -> VaultStatus {
        VaultStatus {
            mode: VaultMode::SessionOnly,
            durable: false,
        }
    }

    fn put(&self, key: SecretRecordKey, value: SecretValue) -> Result<(), VaultError> {
        self.lock()?.insert(key, value);
        Ok(())
    }

    fn get(&self, key: &SecretRecordKey) -> Result<Option<SecretValue>, VaultError> {
        Ok(self.lock()?.get(key).map(SecretValue::duplicate))
    }

    fn delete(&self, key: &SecretRecordKey) -> Result<(), VaultError> {
        self.lock()?.remove(key);
        Ok(())
    }
}

#[derive(Default)]
pub struct VaultService {
    provider: SessionVault,
}

impl VaultService {
    pub fn session_only() -> Self {
        Self::default()
    }

    pub fn status(&self) -> VaultStatus {
        self.provider.status()
    }

    pub fn provider(&self) -> &dyn SecretVault {
        &self.provider
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_vault_round_trip_is_memory_only_and_typed() {
        let vault = SessionVault::default();
        let key = SecretRecordKey::new(SecretKind::BoardCapability, "board-a").unwrap();
        vault.put(key.clone(), SecretValue::new(b"canary-secret".to_vec()).unwrap()).unwrap();
        let value = vault.get(&key).unwrap().unwrap();
        assert_eq!(value.expose(), b"canary-secret");
        vault.delete(&key).unwrap();
        assert!(vault.get(&key).unwrap().is_none());
        assert_eq!(
            vault.status(),
            VaultStatus {
                mode: VaultMode::SessionOnly,
                durable: false,
            }
        );
    }

    #[test]
    fn empty_secret_or_scope_is_rejected() {
        assert_eq!(
            SecretRecordKey::new(SecretKind::RefreshToken, " "),
            Err(VaultError::InvalidKey)
        );
        assert!(matches!(SecretValue::new(Vec::<u8>::new()), Err(VaultError::EmptySecret)));
    }
}
