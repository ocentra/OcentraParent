use std::path::Path;

use rusqlite::Connection;

use crate::account_identity_authority_producer_error::AccountIdentityAuthorityProducerError;

#[path = "account_identity_authority_issuer_delivery.rs"]
mod delivery;
#[path = "account_identity_authority_issuer_key_registry.rs"]
mod key_registry;
#[path = "account_identity_authority_issuer_outbox.rs"]
mod outbox;
#[path = "account_identity_authority_issuer_service_binding.rs"]
mod service_binding;
#[path = "account_identity_authority_issuer_startup.rs"]
pub mod startup;
#[path = "account_identity_authority_issuer_store.rs"]
mod store;
#[path = "account_identity_authority_issuer_transport.rs"]
mod transport;

use startup::AccountIdentityIssuerStartupState;

#[derive(Debug)]
pub enum AccountIdentityIssuerError {
    Unavailable,
    InvalidServiceBinding,
    BindingMismatch,
    ServiceBindingUnavailable,
    ServiceBindingRejected,
    InvalidPublicKey,
    InvalidKeyRecord,
    InvalidKeyVersion,
    KeyAlreadyRegistered,
    KeyUnavailable,
    VerificationKeyUnavailable,
    SignerCustodyUnavailable,
    CurrentAuthorityUnavailable,
    CurrentAuthorityRejected,
    InvalidDurableSchema,
    DurableIntegrityFailure,
    NonDurableStorage,
    ProtectedStoreUnavailable,
    InvalidClock,
    ClockRollback,
    ReplayDetected,
    InvalidTransport,
    TransportExpired,
    TransportContextMismatch,
    DeliveryUnavailable,
    DeliveryAcknowledgementRejected,
    Producer(AccountIdentityAuthorityProducerError),
}

impl std::fmt::Display for AccountIdentityIssuerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("account_identity_issuer_error")
    }
}

impl std::error::Error for AccountIdentityIssuerError {}

/// Preserve the Account repository's strict unknown-object boundary. Issuer
/// objects may be absent, but if any exist then the complete canonical schema,
/// indexes, integrity, and durable rows must validate together.
pub(crate) fn validate_optional_repository_schema(connection: &Connection) -> Result<(), ()> {
    let object_count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master
             WHERE name LIKE 'account_identity_issuer_%'",
            [],
            |row| row.get(0),
        )
        .map_err(|_error| ())?;
    if object_count == 0 {
        return Ok(());
    }
    key_registry::schema::validate(connection).map_err(|_error| ())?;
    key_registry::validate_durable_state(connection)
        .map(|_| ())
        .map_err(|_error| ())
}

/// Account-owned durable issuer boundary.
///
/// Account current authority and issuer state share one protected SQLite file
/// and one locking domain. The file stores only public key/registry, receipt,
/// clock, and delivery metadata; private signing material is never accepted.
/// Issuance and delivery remain unavailable until real sealed owner adapters
/// are installed.
pub struct AccountIdentityIssuer {
    store: store::AccountIdentityIssuerStore,
    startup_state: AccountIdentityIssuerStartupState,
}

impl AccountIdentityIssuer {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, AccountIdentityIssuerError> {
        let store = store::AccountIdentityIssuerStore::open(path.as_ref())?;
        Self::from_store(store)
    }

    fn from_store(
        store: store::AccountIdentityIssuerStore,
    ) -> Result<Self, AccountIdentityIssuerError> {
        let startup_state = startup::recover(store.repository().account_issuer_connection())?;
        Ok(Self {
            store,
            startup_state,
        })
    }

    pub fn startup_state(&self) -> AccountIdentityIssuerStartupState {
        self.startup_state
    }

    /// Re-run durable validation after an external restart/recovery boundary.
    /// No in-memory key or outbox cache is treated as authoritative.
    pub fn recover_startup(
        &mut self,
    ) -> Result<AccountIdentityIssuerStartupState, AccountIdentityIssuerError> {
        self.store.validate_identity()?;
        self.startup_state = startup::recover(self.store.repository().account_issuer_connection())?;
        self.store.validate_identity()?;
        Ok(self.startup_state)
    }
}
