//! Durable current public-key record carried with each producer wire.

use std::fmt;

use ed25519_dalek::VerifyingKey;
use sha2::{Digest, Sha256};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;

use super::key_registry::RegisteredIssuerKey;
use super::service_binding::{AccountIdentityIssuerService, AccountIdentityIssuerServiceBinding};
use super::AccountIdentityIssuerError;

/// An integrity-bound copy of the Account registry row that accompanies an
/// outer producer wire.  It is never an authority source; acknowledgement
/// re-reads the durable registry and verifies the wire against that row.
#[derive(Clone)]
pub(crate) struct AccountIdentityIssuerCurrentPublicKeyRecord {
    key_id: String,
    key_version: u64,
    public_key: [u8; 32],
    service_binding_id: String,
    service_label: String,
    account_id: String,
    household_id: String,
    authority_generation: u64,
    record_digest: String,
}

impl AccountIdentityIssuerCurrentPublicKeyRecord {
    pub(crate) fn from_registered(
        authority: &VerifiedAccountIdentityAuthority,
        binding: &AccountIdentityIssuerServiceBinding,
        registered: &RegisteredIssuerKey,
    ) -> Result<Self, AccountIdentityIssuerError> {
        let key_id = registered.handle.key_id().to_owned();
        let key_version = registered.handle.key_version();
        let public_key = registered.verifying_key.to_bytes();
        if crate::account_identity_authority_producer::expected_key_id(&registered.verifying_key)
            != key_id
        {
            return Err(AccountIdentityIssuerError::InvalidKeyRecord);
        }
        let record = Self {
            key_id,
            key_version,
            public_key,
            service_binding_id: binding.binding_id().to_owned(),
            service_label: binding.service().label().to_owned(),
            account_id: authority.account_id().to_string(),
            household_id: authority.household_id().to_string(),
            authority_generation: authority.authority_generation(),
            record_digest: String::new(),
        };
        record.with_digest()
    }

    fn with_digest(mut self) -> Result<Self, AccountIdentityIssuerError> {
        self.record_digest = current_key_record_digest(
            &self.key_id,
            self.key_version,
            &self.public_key,
            &self.service_binding_id,
            &self.service_label,
            &self.account_id,
            &self.household_id,
            self.authority_generation,
        );
        self.validate()?;
        Ok(self)
    }

    pub(crate) fn validate(&self) -> Result<(), AccountIdentityIssuerError> {
        let verifying_key = VerifyingKey::from_bytes(&self.public_key)
            .map_err(|_| AccountIdentityIssuerError::InvalidPublicKey)?;
        if self.key_id.trim().is_empty()
            || self.key_version == 0
            || self.service_binding_id.trim().is_empty()
            || self.service_label.trim().is_empty()
            || self.account_id.trim().is_empty()
            || self.household_id.trim().is_empty()
            || self.authority_generation == 0
            || crate::account_identity_authority_producer::expected_key_id(&verifying_key)
                != self.key_id
        {
            return Err(AccountIdentityIssuerError::InvalidKeyRecord);
        }
        let expected = current_key_record_digest(
            &self.key_id,
            self.key_version,
            &self.public_key,
            &self.service_binding_id,
            &self.service_label,
            &self.account_id,
            &self.household_id,
            self.authority_generation,
        );
        (self.record_digest == expected)
            .then_some(())
            .ok_or(AccountIdentityIssuerError::InvalidKeyRecord)
    }

    pub(crate) fn matches_context(
        &self,
        service: AccountIdentityIssuerService,
        binding_id: &str,
        account_id: &str,
        household_id: &str,
        authority_generation: u64,
    ) -> bool {
        self.service_label == service.label()
            && self.service_binding_id == binding_id
            && self.account_id == account_id
            && self.household_id == household_id
            && self.authority_generation == authority_generation
    }

    pub(crate) fn record_digest(&self) -> &str {
        &self.record_digest
    }
}

impl fmt::Debug for AccountIdentityIssuerCurrentPublicKeyRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AccountIdentityIssuerCurrentPublicKeyRecord")
            .field("key_id", &"redacted")
            .field("key_version", &self.key_version)
            .field("public_key", &"redacted")
            .field("service_binding_id", &"redacted")
            .field("service_label", &self.service_label)
            .field("account_id", &"redacted")
            .field("household_id", &"redacted")
            .field("authority_generation", &self.authority_generation)
            .field("record_digest", &"redacted")
            .finish()
    }
}

fn current_key_record_digest(
    key_id: &str,
    key_version: u64,
    public_key: &[u8; 32],
    service_binding_id: &str,
    service_label: &str,
    account_id: &str,
    household_id: &str,
    authority_generation: u64,
) -> String {
    let mut digest = Sha256::new();
    digest.update(b"ocentra.account-issuer.current-key-record.v1\0");
    let values: [&[u8]; 6] = [
        key_id.as_bytes(),
        service_binding_id.as_bytes(),
        service_label.as_bytes(),
        account_id.as_bytes(),
        household_id.as_bytes(),
        public_key,
    ];
    for value in values {
        digest.update((value.len() as u64).to_be_bytes());
        digest.update(value);
    }
    digest.update(key_version.to_be_bytes());
    digest.update(authority_generation.to_be_bytes());
    format!("sha256:{:x}", digest.finalize())
}
