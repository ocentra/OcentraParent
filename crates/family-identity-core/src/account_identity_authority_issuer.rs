use std::path::Path;
use std::time::Duration;

use ed25519_dalek::VerifyingKey;
use ocentra_schema::account_identity_authority::AccountIdentityCurrentMemberDeviceAuthorityHandoff;
use rusqlite::Connection;

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_authority_producer::{
    AccountIdentityAuthorityProducerCustody, AccountIdentityAuthorityProducerTransport,
};
use crate::account_identity_authority_producer_error::AccountIdentityAuthorityProducerError;

#[path = "account_identity_authority_issuer_key_custody.rs"]
mod key_custody;
#[path = "account_identity_authority_issuer_key_registry.rs"]
mod key_registry;
#[path = "account_identity_authority_issuer_service_binding.rs"]
mod service_binding;
#[path = "account_identity_authority_issuer_startup.rs"]
pub mod startup;

use startup::AccountIdentityIssuerStartupState;

pub(crate) use key_custody::AccountIdentityIssuerSignerAdapter;
pub(crate) use service_binding::{
    AccountIdentityIssuerAuthenticatedBinding, AccountIdentityIssuerService,
    AccountIdentityIssuerServiceBinding, AccountIdentityIssuerServiceBindingAuthenticator,
};

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
    binding_authenticator: Option<Box<dyn AccountIdentityIssuerServiceBindingAuthenticator>>,
    startup_state: AccountIdentityIssuerStartupState,
}

impl AccountIdentityIssuer {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, AccountIdentityIssuerError> {
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
        service: AccountIdentityIssuerService,
    ) -> Result<AccountIdentityIssuerServiceBinding, AccountIdentityIssuerError> {
        service_binding::AccountIdentityIssuerServiceBinding::from_authority(authority, service)
    }

    /// Install a real platform signer. The adapter must keep private key
    /// bytes outside this process and reject handles it did not receive from
    /// the durable issuer registry.
    pub(crate) fn install_signer(&mut self, signer: Box<dyn AccountIdentityIssuerSignerAdapter>) {
        self.signer = Some(key_custody::AccountIdentityIssuerKeyCustody::from_signer(
            signer,
        ));
    }

    pub(crate) fn install_binding_authenticator(
        &mut self,
        authenticator: Box<dyn AccountIdentityIssuerServiceBindingAuthenticator>,
    ) {
        self.binding_authenticator = Some(authenticator);
    }

    /// Register a public verification key for the exact current Account and
    /// household binding. Registration rotates the previous active version in
    /// one SQLite transaction; callers cannot select the issuer key id.
    pub(crate) fn register_public_key(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        binding: &AccountIdentityIssuerServiceBinding,
        public_key: [u8; 32],
    ) -> Result<key_custody::AccountIdentityIssuerSigningHandle, AccountIdentityIssuerError> {
        self.authenticate_binding(authority, binding)?;
        Ok(key_registry::register(&mut self.connection, authority, binding, public_key)?.handle)
    }

    pub(crate) fn revoke_public_key(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        binding: &AccountIdentityIssuerServiceBinding,
        handle: &key_custody::AccountIdentityIssuerSigningHandle,
    ) -> Result<(), AccountIdentityIssuerError> {
        self.authenticate_binding(authority, binding)?;
        key_registry::revoke(&mut self.connection, authority, binding, handle)
    }

    /// Sign only with the durable current key selected by Account. The
    /// binding and authority are checked before the producer envelope is
    /// constructed; there is no key selector in this API.
    pub(crate) fn issue_current_authority(
        &self,
        authority: &VerifiedAccountIdentityAuthority,
        binding: &AccountIdentityIssuerServiceBinding,
    ) -> Result<AccountIdentityAuthorityProducerTransport, AccountIdentityIssuerError> {
        self.authenticate_binding(authority, binding)?;
        let signer = self
            .signer
            .as_ref()
            .ok_or(AccountIdentityIssuerError::SignerCustodyUnavailable)?;
        let registered = key_registry::current(&self.connection, authority, binding)?;
        let custody = key_custody::RegisteredProducerCustody::new(
            &registered.handle,
            registered.verifying_key,
            signer,
        );
        crate::account_identity_authority_producer::issue(authority, &custody)
            .map_err(AccountIdentityIssuerError::Producer)
    }

    /// Verify a producer wire against the active durable key for this exact
    /// service binding. Consumers must still re-resolve current Account
    /// authority before accepting the returned handoff.
    pub(crate) fn verify_current_authority(
        &self,
        authority: &VerifiedAccountIdentityAuthority,
        binding: &AccountIdentityIssuerServiceBinding,
        wire: &[u8],
    ) -> Result<AccountIdentityCurrentMemberDeviceAuthorityHandoff, AccountIdentityIssuerError>
    {
        self.authenticate_binding(authority, binding)?;
        let registered = key_registry::current(&self.connection, authority, binding)?;
        let custody = RegisteredVerificationCustody {
            key_id: registered.handle.key_id().to_owned(),
            public_key: registered.verifying_key,
        };
        crate::account_identity_authority_producer::verify(wire, &custody)
            .map_err(AccountIdentityIssuerError::Producer)
    }

    fn authenticate_binding(
        &self,
        authority: &VerifiedAccountIdentityAuthority,
        binding: &AccountIdentityIssuerServiceBinding,
    ) -> Result<AccountIdentityIssuerAuthenticatedBinding, AccountIdentityIssuerError> {
        if !binding.matches_authority(authority) {
            return Err(AccountIdentityIssuerError::BindingMismatch);
        }
        let authenticator = self
            .binding_authenticator
            .as_deref()
            .ok_or(AccountIdentityIssuerError::ServiceBindingUnavailable)?;
        let authenticated = authenticator.authenticate(binding)?;
        (authenticated.binding_id() == binding.binding_id())
            .then_some(authenticated)
            .ok_or(AccountIdentityIssuerError::ServiceBindingRejected)
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

struct RegisteredVerificationCustody {
    key_id: String,
    public_key: VerifyingKey,
}

impl AccountIdentityAuthorityProducerCustody for RegisteredVerificationCustody {
    fn signing_key_id(&self) -> &str {
        "verification-only"
    }

    fn verification_key(
        &self,
        key_id: &str,
    ) -> Result<VerifyingKey, AccountIdentityAuthorityProducerError> {
        (key_id == self.key_id)
            .then_some(self.public_key)
            .ok_or(AccountIdentityAuthorityProducerError::VerificationKeyUnavailable)
    }

    fn sign(&self, _payload: &[u8]) -> Result<[u8; 64], AccountIdentityAuthorityProducerError> {
        Err(AccountIdentityAuthorityProducerError::SignerCustodyUnavailable)
    }
}
