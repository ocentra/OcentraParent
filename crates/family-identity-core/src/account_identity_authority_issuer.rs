use std::path::Path;

use ed25519_dalek::VerifyingKey;
use ocentra_schema::account_identity_authority::AccountIdentityCurrentMemberDeviceAuthorityHandoff;
use rusqlite::Connection;

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_authority_producer::AccountIdentityAuthorityProducerCustody;
use crate::account_identity_authority_producer_error::AccountIdentityAuthorityProducerError;

#[path = "account_identity_authority_issuer_cloudflare_delivery.rs"]
mod cloudflare_delivery;
#[path = "account_identity_authority_issuer_current_key_record.rs"]
mod current_key_record;
#[path = "account_identity_authority_issuer_currentness.rs"]
mod currentness;
#[path = "account_identity_authority_issuer_delivery.rs"]
mod delivery;
#[path = "account_identity_authority_issuer_key_custody.rs"]
mod key_custody;
#[path = "account_identity_authority_issuer_key_registry.rs"]
mod key_registry;
#[path = "account_identity_authority_issuer_outbox.rs"]
mod outbox;
#[path = "account_identity_authority_issuer_protected_signer.rs"]
mod protected_signer;
#[path = "account_identity_authority_issuer_runtime.rs"]
pub(crate) mod runtime;
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
        .map_err(|_| ())?;
    if object_count == 0 {
        return Ok(());
    }
    key_registry::schema::validate(connection).map_err(|_| ())?;
    key_registry::validate_durable_state(connection)
        .map(|_| ())
        .map_err(|_| ())
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
    signer: Option<key_custody::AccountIdentityIssuerKeyCustody>,
    binding_authenticator:
        Option<Box<dyn service_binding::AccountIdentityIssuerServiceBindingAuthenticator>>,
    delivery_owner: Option<Box<dyn outbox::AccountIdentityIssuerDeliveryOwnerAdapter>>,
    startup_state: AccountIdentityIssuerStartupState,
}

impl AccountIdentityIssuer {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, AccountIdentityIssuerError> {
        let store = store::AccountIdentityIssuerStore::open(path.as_ref())?;
        Self::from_store(store)
    }

    pub(crate) fn open_with_protected_store_owner(
        path: impl AsRef<Path>,
        owner: &dyn store::AccountIdentityIssuerProtectedStoreOwner,
    ) -> Result<Self, AccountIdentityIssuerError> {
        let store = owner.open_protected_store(path.as_ref())?;
        Self::from_store(store)
    }

    fn from_store(
        store: store::AccountIdentityIssuerStore,
    ) -> Result<Self, AccountIdentityIssuerError> {
        let startup_state = startup::recover(store.repository().account_issuer_connection())?;
        Ok(Self {
            store,
            signer: None,
            binding_authenticator: None,
            delivery_owner: None,
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

    /// Install a real platform signer. The adapter must keep private key bytes
    /// outside this process and reject handles absent from its protected store.
    pub(crate) fn install_signer(
        &mut self,
        signer: Box<dyn key_custody::AccountIdentityIssuerSignerAdapter>,
    ) {
        self.signer = Some(
            protected_signer::AccountIdentityIssuerProtectedSigner::from_platform_owner(signer)
                .into_custody(),
        );
    }

    pub(crate) fn install_binding_authenticator(
        &mut self,
        authenticator: Box<dyn service_binding::AccountIdentityIssuerServiceBindingAuthenticator>,
    ) {
        self.binding_authenticator = Some(authenticator);
    }

    pub(crate) fn install_delivery_owner(
        &mut self,
        delivery_owner: Box<dyn outbox::AccountIdentityIssuerDeliveryOwnerAdapter>,
    ) {
        self.delivery_owner = Some(delivery_owner);
    }

    /// Register a public verification key for exact current Account authority.
    /// The current row, authenticated service binding, version rotation, and
    /// new row commit are linearized by one SQLite `BEGIN IMMEDIATE`.
    pub(crate) fn register_public_key(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        service: service_binding::AccountIdentityIssuerService,
    ) -> Result<(), AccountIdentityIssuerError> {
        self.store.validate_identity()?;
        let authenticator = self.binding_authenticator.as_deref();
        let signer = self
            .signer
            .as_ref()
            .ok_or(AccountIdentityIssuerError::SignerCustodyUnavailable)?;
        let transaction = self
            .store
            .repository_mut()
            .begin_account_issuer_transaction()
            .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
        let now = key_registry::receipts::trusted_now(&transaction)?;
        currentness::ensure_exact_current(&transaction, authority, now)?;
        let binding = currentness::binding_for_current(authority, service)?;
        let _authenticated = currentness::authenticate_binding(authenticator, authority, &binding)?;
        let provisioned = signer.provision_public_key(authority, &binding)?;
        outbox::reconcile::supersede_for_rotation(&transaction, authority, &binding, now)?;
        key_registry::register(&transaction, authority, &binding, provisioned.bytes())?;
        transaction
            .commit()
            .map_err(|_| AccountIdentityIssuerError::Unavailable)
    }

    pub(crate) fn revoke_public_key(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        service: service_binding::AccountIdentityIssuerService,
    ) -> Result<(), AccountIdentityIssuerError> {
        self.store.validate_identity()?;
        let authenticator = self.binding_authenticator.as_deref();
        let transaction = self
            .store
            .repository_mut()
            .begin_account_issuer_transaction()
            .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
        let now = key_registry::receipts::trusted_now(&transaction)?;
        currentness::ensure_exact_current(&transaction, authority, now)?;
        let binding = currentness::binding_for_current(authority, service)?;
        let _authenticated = currentness::authenticate_binding(authenticator, authority, &binding)?;
        outbox::reconcile::supersede_for_rotation(&transaction, authority, &binding, now)?;
        key_registry::revoke(&transaction, authority, &binding)?;
        transaction
            .commit()
            .map_err(|_| AccountIdentityIssuerError::Unavailable)
    }

    /// Sign only with the durable current key selected by Account. Issuance,
    /// replay receipt creation, and durable outbox enqueue commit together.
    pub(crate) fn issue_current_authority(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        service: service_binding::AccountIdentityIssuerService,
    ) -> Result<transport::AccountIdentityIssuerTransport, AccountIdentityIssuerError> {
        self.store.validate_identity()?;
        let authenticator = self.binding_authenticator.as_deref();
        let signer = self
            .signer
            .as_ref()
            .ok_or(AccountIdentityIssuerError::SignerCustodyUnavailable)?;
        let transaction = self
            .store
            .repository_mut()
            .begin_account_issuer_transaction()
            .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
        let now = key_registry::receipts::trusted_now(&transaction)?;
        currentness::ensure_exact_current(&transaction, authority, now)?;
        let binding = currentness::binding_for_current(authority, service)?;
        let _authenticated = currentness::authenticate_binding(authenticator, authority, &binding)?;
        let registered = key_registry::current(&transaction, authority, &binding)?;
        let custody = key_custody::RegisteredProducerCustody::new(
            &registered.handle,
            registered.verifying_key,
            signer,
        );
        let inner = crate::account_identity_authority_producer::issue_at(authority, &custody, now)
            .map_err(AccountIdentityIssuerError::Producer)?;
        let transport = transport::issue(authority, &binding, &registered, &custody, inner, now)?;
        key_registry::receipts::record_transport_receipt(
            &transaction,
            transport.receipt_id(),
            authority,
            &binding,
            transport.key_id(),
            transport.key_version(),
            transport.issued_at(),
            transport.expires_at(),
        )?;
        outbox::enqueue(&transaction, authority, &binding, &transport)?;
        transaction
            .commit()
            .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
        Ok(transport)
    }

    /// Verify and consume one producer transport against exact current Account
    /// authority and the current durable issuer key in the same transaction.
    pub(crate) fn verify_current_authority(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        service: service_binding::AccountIdentityIssuerService,
        wire: &[u8],
    ) -> Result<AccountIdentityCurrentMemberDeviceAuthorityHandoff, AccountIdentityIssuerError>
    {
        self.store.validate_identity()?;
        let authenticator = self.binding_authenticator.as_deref();
        let transaction = self
            .store
            .repository_mut()
            .begin_account_issuer_transaction()
            .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
        let now = key_registry::receipts::trusted_now(&transaction)?;
        currentness::ensure_exact_current(&transaction, authority, now)?;
        let binding = currentness::binding_for_current(authority, service)?;
        let _authenticated = currentness::authenticate_binding(authenticator, authority, &binding)?;
        let registered = key_registry::current(&transaction, authority, &binding)?;
        let verified = transport::verify(wire, authority, &binding, &registered, now)?;
        key_registry::receipts::consume_transport_receipt(
            &transaction,
            verified.receipt_id(),
            authority,
            &binding,
            verified.key_id(),
            verified.key_version(),
            now,
        )?;
        let handoff = verified.into_handoff();
        transaction
            .commit()
            .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
        Ok(handoff)
    }
}

impl AccountIdentityAuthorityProducerCustody for key_custody::RegisteredProducerCustody<'_> {
    fn signing_key_id(&self) -> &str {
        self.key_id()
    }

    fn verification_key(
        &self,
        key_id: &str,
    ) -> Result<VerifyingKey, AccountIdentityAuthorityProducerError> {
        (key_id == self.key_id())
            .then_some(*self.public_key())
            .ok_or(AccountIdentityAuthorityProducerError::VerificationKeyUnavailable)
    }

    fn sign(&self, payload: &[u8]) -> Result<[u8; 64], AccountIdentityAuthorityProducerError> {
        self.sign(payload).map_err(|error| match error {
            AccountIdentityIssuerError::SignerCustodyUnavailable
            | AccountIdentityIssuerError::Unavailable => {
                AccountIdentityAuthorityProducerError::SignerCustodyUnavailable
            }
            _ => AccountIdentityAuthorityProducerError::SignatureInvalid,
        })
    }
}
