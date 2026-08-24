use std::path::Path;
use std::time::Duration;

use ed25519_dalek::VerifyingKey;
use ocentra_schema::account_identity_authority::AccountIdentityCurrentMemberDeviceAuthorityHandoff;
use rusqlite::Connection;

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_authority_producer::AccountIdentityAuthorityProducerCustody;
use crate::account_identity_authority_producer_error::AccountIdentityAuthorityProducerError;

#[path = "account_identity_authority_issuer_currentness.rs"]
pub(crate) mod currentness;
#[path = "account_identity_authority_issuer_key_custody.rs"]
pub(crate) mod key_custody;
#[path = "account_identity_authority_issuer_key_registry.rs"]
pub(crate) mod key_registry;
#[path = "account_identity_authority_issuer_service_binding.rs"]
pub(crate) mod service_binding;
#[path = "account_identity_authority_issuer_startup.rs"]
pub mod startup;
#[path = "account_identity_authority_issuer_transport.rs"]
pub(crate) mod transport;

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
    InvalidClock,
    ClockRollback,
    ReplayDetected,
    InvalidTransport,
    TransportExpired,
    TransportContextMismatch,
    Producer(AccountIdentityAuthorityProducerError),
}

impl std::fmt::Display for AccountIdentityIssuerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("account_identity_issuer_error")
    }
}

impl std::error::Error for AccountIdentityIssuerError {}

/// Account-owned durable issuer boundary.
///
/// The SQLite file stores only public keys, versions, revocation state, and
/// identity/service-binding metadata. Private signing material is never
/// accepted by this type. Issuance remains unavailable until a real protected
/// signer and authenticated service-binding adapter are installed by the
/// owning runtime.
pub struct AccountIdentityIssuer {
    connection: Connection,
    signer: Option<key_custody::AccountIdentityIssuerKeyCustody>,
    binding_authenticator:
        Option<Box<dyn service_binding::AccountIdentityIssuerServiceBindingAuthenticator>>,
    authority_resolver: Option<Box<dyn currentness::AccountIdentityIssuerAuthorityResolver>>,
    startup_state: AccountIdentityIssuerStartupState,
}

impl AccountIdentityIssuer {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, AccountIdentityIssuerError> {
        currentness::validate_durable_path(path.as_ref())?;
        let connection =
            Connection::open(path).map_err(|_| AccountIdentityIssuerError::Unavailable)?;
        connection
            .busy_timeout(Duration::from_secs(5))
            .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
        let startup_state = startup::initialize(&connection)?;
        Ok(Self {
            connection,
            signer: None,
            binding_authenticator: None,
            authority_resolver: None,
            startup_state,
        })
    }

    pub fn startup_state(&self) -> AccountIdentityIssuerStartupState {
        self.startup_state
    }

    /// Re-run durable validation after an external restart/recovery boundary.
    /// No in-memory key cache is treated as authoritative.
    pub fn recover_startup(
        &mut self,
    ) -> Result<AccountIdentityIssuerStartupState, AccountIdentityIssuerError> {
        self.startup_state = startup::recover(&self.connection)?;
        Ok(self.startup_state)
    }

    pub(crate) fn service_binding(
        &self,
        authority: &VerifiedAccountIdentityAuthority,
        service: service_binding::AccountIdentityIssuerService,
    ) -> Result<service_binding::AccountIdentityIssuerServiceBinding, AccountIdentityIssuerError>
    {
        service_binding::AccountIdentityIssuerServiceBinding::from_authority(authority, service)
    }

    /// Install a real platform signer. The adapter must keep private key
    /// bytes outside this process and reject handles it did not receive from
    /// the durable issuer registry.
    pub(crate) fn install_signer(
        &mut self,
        signer: Box<dyn key_custody::AccountIdentityIssuerSignerAdapter>,
    ) {
        self.signer = Some(key_custody::AccountIdentityIssuerKeyCustody::from_signer(
            signer,
        ));
    }

    pub(crate) fn install_binding_authenticator(
        &mut self,
        authenticator: Box<dyn service_binding::AccountIdentityIssuerServiceBindingAuthenticator>,
    ) {
        self.binding_authenticator = Some(authenticator);
    }

    pub(crate) fn install_authority_resolver(
        &mut self,
        resolver: Box<dyn currentness::AccountIdentityIssuerAuthorityResolver>,
    ) {
        self.authority_resolver = Some(resolver);
    }

    /// Register a public verification key for the exact current Account and
    /// household binding. Registration rotates the previous active version in
    /// one SQLite transaction; callers cannot select the issuer key id.
    pub(crate) fn register_public_key(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        binding: &service_binding::AccountIdentityIssuerServiceBinding,
        public_key: [u8; 32],
    ) -> Result<key_custody::AccountIdentityIssuerSigningHandle, AccountIdentityIssuerError> {
        let context = currentness::acquire_current_context(
            self.authority_resolver.as_deref(),
            authority,
            binding,
        )?;
        let current_authority = context.authority();
        let current_binding = context.binding();
        currentness::authenticate_binding(
            self.binding_authenticator.as_deref(),
            current_authority,
            current_binding,
        )?;
        let handle = key_registry::register(
            &mut self.connection,
            current_authority,
            current_binding,
            public_key,
        )?
        .handle;
        context.assert_current()?;
        Ok(handle)
    }

    pub(crate) fn revoke_public_key(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        binding: &service_binding::AccountIdentityIssuerServiceBinding,
        handle: &key_custody::AccountIdentityIssuerSigningHandle,
    ) -> Result<(), AccountIdentityIssuerError> {
        let context = currentness::acquire_current_context(
            self.authority_resolver.as_deref(),
            authority,
            binding,
        )?;
        let current_authority = context.authority();
        let current_binding = context.binding();
        currentness::authenticate_binding(
            self.binding_authenticator.as_deref(),
            current_authority,
            current_binding,
        )?;
        key_registry::revoke(
            &mut self.connection,
            current_authority,
            current_binding,
            handle,
        )?;
        context.assert_current()
    }

    /// Sign only with the durable current key selected by Account. The
    /// binding and authority are checked before the producer envelope is
    /// constructed; there is no key selector in this API.
    pub(crate) fn issue_current_authority(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        binding: &service_binding::AccountIdentityIssuerServiceBinding,
    ) -> Result<transport::AccountIdentityIssuerTransport, AccountIdentityIssuerError> {
        let context = currentness::acquire_current_context(
            self.authority_resolver.as_deref(),
            authority,
            binding,
        )?;
        let current_authority = context.authority();
        let current_binding = context.binding();
        currentness::authenticate_binding(
            self.binding_authenticator.as_deref(),
            current_authority,
            current_binding,
        )?;
        let signer = self
            .signer
            .as_ref()
            .ok_or(AccountIdentityIssuerError::SignerCustodyUnavailable)?;
        let transaction = key_registry::begin_immediate(&mut self.connection)?;
        let now = key_registry::receipts::trusted_now(&transaction)?;
        let registered = key_registry::current(&transaction, current_authority, current_binding)?;
        context.assert_current()?;
        let custody = key_custody::RegisteredProducerCustody::new(
            &registered.handle,
            registered.verifying_key,
            signer,
        );
        let inner =
            crate::account_identity_authority_producer::issue_at(current_authority, &custody, now)
                .map_err(AccountIdentityIssuerError::Producer)?;
        let transport = transport::issue(
            current_authority,
            current_binding,
            &registered,
            &custody,
            inner,
            now,
        )?;
        context.assert_current()?;
        key_registry::receipts::record_transport_receipt(
            &transaction,
            transport.receipt_id(),
            current_authority,
            current_binding,
            transport.key_id(),
            transport.key_version(),
            transport.issued_at(),
            transport.expires_at(),
        )?;
        transaction
            .commit()
            .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
        context.assert_current()?;
        Ok(transport)
    }

    /// Verify a producer wire against the active durable key for this exact
    /// service binding. Consumers must still re-resolve current Account
    /// authority before accepting the returned handoff.
    pub(crate) fn verify_current_authority(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        binding: &service_binding::AccountIdentityIssuerServiceBinding,
        wire: &[u8],
    ) -> Result<AccountIdentityCurrentMemberDeviceAuthorityHandoff, AccountIdentityIssuerError>
    {
        let context = currentness::acquire_current_context(
            self.authority_resolver.as_deref(),
            authority,
            binding,
        )?;
        let current_authority = context.authority();
        let current_binding = context.binding();
        currentness::authenticate_binding(
            self.binding_authenticator.as_deref(),
            current_authority,
            current_binding,
        )?;
        let transaction = key_registry::begin_immediate(&mut self.connection)?;
        let now = key_registry::receipts::trusted_now(&transaction)?;
        let registered = key_registry::current(&transaction, current_authority, current_binding)?;
        context.assert_current()?;
        let verified =
            transport::verify(wire, current_authority, current_binding, &registered, now)?;
        context.assert_current()?;
        key_registry::receipts::consume_transport_receipt(
            &transaction,
            verified.receipt_id(),
            current_authority,
            current_binding,
            verified.key_id(),
            verified.key_version(),
            now,
        )?;
        transaction
            .commit()
            .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
        context.assert_current()?;
        Ok(verified.into_handoff())
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
