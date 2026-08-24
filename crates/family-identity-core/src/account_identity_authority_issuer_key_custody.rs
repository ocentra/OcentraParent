use std::fmt;

use ed25519_dalek::VerifyingKey;

use crate::account_identity_authority_issuer::AccountIdentityIssuerError;

/// An opaque reference to a key held by an external protected signer.
///
/// The handle deliberately contains no secret material and cannot be cloned
/// or constructed by a caller.  A platform adapter receives only this opaque
/// reference when the issuer asks it to sign.
pub(crate) struct AccountIdentityIssuerSigningHandle {
    key_id: String,
    key_version: u64,
    account_id: String,
    household_id: String,
    service_binding_id: String,
}

impl AccountIdentityIssuerSigningHandle {
    pub(crate) fn new(
        key_id: String,
        key_version: u64,
        account_id: String,
        household_id: String,
        service_binding_id: String,
    ) -> Result<Self, AccountIdentityIssuerError> {
        if key_id.trim().is_empty()
            || account_id.trim().is_empty()
            || household_id.trim().is_empty()
            || service_binding_id.trim().is_empty()
            || key_version == 0
        {
            return Err(AccountIdentityIssuerError::InvalidKeyRecord);
        }
        Ok(Self {
            key_id,
            key_version,
            account_id,
            household_id,
            service_binding_id,
        })
    }

    pub(crate) fn key_id(&self) -> &str {
        &self.key_id
    }

    pub(crate) fn key_version(&self) -> u64 {
        self.key_version
    }

    pub(crate) fn account_id(&self) -> &str {
        &self.account_id
    }

    pub(crate) fn household_id(&self) -> &str {
        &self.household_id
    }

    pub(crate) fn service_binding_id(&self) -> &str {
        &self.service_binding_id
    }
}

impl fmt::Debug for AccountIdentityIssuerSigningHandle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AccountIdentityIssuerSigningHandle")
            .field("key_id", &"redacted")
            .field("key_version", &self.key_version)
            .field("account_id", &"redacted")
            .field("household_id", &"redacted")
            .field("service_binding_id", &"redacted")
            .finish()
    }
}

/// Platform-owned signer boundary.  Implementations must keep private key
/// bytes inside an OS/HSM/isolated signer and must reject an unknown handle.
/// This crate intentionally provides no process-local or SQLite-backed
/// implementation.
pub(crate) trait AccountIdentityIssuerSignerAdapter: Send + Sync {
    fn sign(
        &self,
        handle: &AccountIdentityIssuerSigningHandle,
        signing_bytes: &[u8],
    ) -> Result<[u8; 64], AccountIdentityIssuerError>;
}

pub(crate) struct AccountIdentityIssuerKeyCustody {
    signer: Box<dyn AccountIdentityIssuerSignerAdapter>,
}

impl AccountIdentityIssuerKeyCustody {
    pub(crate) fn from_signer(signer: Box<dyn AccountIdentityIssuerSignerAdapter>) -> Self {
        Self { signer }
    }

    pub(crate) fn sign(
        &self,
        handle: &AccountIdentityIssuerSigningHandle,
        signing_bytes: &[u8],
    ) -> Result<[u8; 64], AccountIdentityIssuerError> {
        self.signer.sign(handle, signing_bytes)
    }
}

pub(crate) struct RegisteredProducerCustody<'a> {
    handle: &'a AccountIdentityIssuerSigningHandle,
    public_key: VerifyingKey,
    custody: &'a AccountIdentityIssuerKeyCustody,
}

impl<'a> RegisteredProducerCustody<'a> {
    pub(crate) fn new(
        handle: &'a AccountIdentityIssuerSigningHandle,
        public_key: VerifyingKey,
        custody: &'a AccountIdentityIssuerKeyCustody,
    ) -> Self {
        Self {
            handle,
            public_key,
            custody,
        }
    }

    pub(crate) fn key_id(&self) -> &str {
        self.handle.key_id()
    }

    pub(crate) fn public_key(&self) -> &VerifyingKey {
        &self.public_key
    }

    pub(crate) fn sign(
        &self,
        signing_bytes: &[u8],
    ) -> Result<[u8; 64], AccountIdentityIssuerError> {
        self.custody.sign(self.handle, signing_bytes)
    }
}
