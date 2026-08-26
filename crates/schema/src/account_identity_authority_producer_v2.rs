//! Canonical v2 Account current-authority producer contract.
//!
//! The v2 producer is deliberately separate from the historical Ed25519 v1
//! contract.  Its payload is produced from an Account-owned opaque capability
//! by `ocentra-family-identity-core`; callers cannot construct an authority by
//! filling in this module's wire metadata.

use serde::{Deserialize, Serialize};

pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SCHEMA_VERSION: &str =
    "ocentra.account-authority-producer.v2";
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_INNER_DOMAIN: &[u8] =
    b"ocentra.account-authority-producer.signing.v2\0";
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_AUDIENCE: &str = "ocentra.account.authority.v2";
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ENVIRONMENT: &str = "account-owned";
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_ALGORITHM: &str =
    "ecdsa-p256-sha256-p1363";
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_OUTER_DOMAIN: &[u8] =
    b"ocentra.account-issuer.transport.v2\0";
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE: &str =
    "ocentra.account-authority-producer.cloudflare.v2";
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_DOMAIN: &[u8] =
    b"ocentra.account-authority-producer.key-id.v2\0";
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNER_CAPABILITY_DOMAIN: &[u8] =
    b"ocentra.account-issuer.signer-capability.v2\0";
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_PREFIX: &str = "sha256:ecdsa-p256:";
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_RECEIPT_ID_PREFIX: &str = "sha256:receipt:";
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES: usize = 64;
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES: usize = 65;
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_FIELD_BYTES: usize = 1_024;
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_PAYLOAD_BYTES: usize = 16 * 1_024;
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_LIFETIME_SECONDS: i64 = 5 * 60;
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_FUTURE_ISSUED_SKEW_SECONDS: i64 = 30;
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_WIRE_BYTES: usize =
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_PAYLOAD_BYTES
        + (ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_FIELD_BYTES * 13)
        + 256;

pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ISSUE_MESSAGE_KIND: u8 = 6;
pub const ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ACKNOWLEDGE_MESSAGE_KIND: u8 = 7;

pub const ACCOUNT_ISSUER_COMMAND_INVALID: &str =
    "AccountIssuer command correlation/idempotency is invalid";
pub const ACCOUNT_ISSUER_DELIVERY_ERROR: &str = "account_issuer_delivery_error";
pub const ACCOUNT_ISSUER_REPOSITORY_ERROR: &str = "account_issuer_repository_error";
pub const ACCOUNT_ISSUER_RPC_ERROR: &str = "account_issuer_rpc_error";
pub const ACCOUNT_ISSUER_SIGNING_ERROR: &str = "account_issuer_signing_error";

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct AccountIdentityAuthorityProducerV2CorrelationId(String);

impl AccountIdentityAuthorityProducerV2CorrelationId {
    pub fn parse(value: impl Into<String>) -> Result<Self, &'static str> {
        let value = value.into();
        valid_identifier(&value)
            .then_some(Self(value))
            .ok_or("v2 correlation id is invalid")
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct AccountIdentityAuthorityProducerV2IdempotencyKey(String);

impl AccountIdentityAuthorityProducerV2IdempotencyKey {
    pub fn parse(value: impl Into<String>) -> Result<Self, &'static str> {
        let value = value.into();
        valid_identifier(&value)
            .then_some(Self(value))
            .ok_or("v2 idempotency key is invalid")
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

fn valid_identifier(value: &str) -> bool {
    !value.trim().is_empty()
        && value.len() <= ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_FIELD_BYTES
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename = "v2")]
pub enum AccountIdentityAuthorityProducerV2SchemaVersion {
    V2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AccountIdentityAuthorityProducerV2Operation {
    IssueCurrentAuthority,
    AcknowledgeReceipt,
}

impl AccountIdentityAuthorityProducerV2Operation {
    pub fn message_kind(self) -> u8 {
        if self == Self::IssueCurrentAuthority {
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ISSUE_MESSAGE_KIND
        } else {
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_ACKNOWLEDGE_MESSAGE_KIND
        }
    }
}

/// Self-contained v2 authority claims. This is intentionally not the
/// historical `AccountIdentityCurrentMemberDeviceAuthorityHandoff` DTO: v2
/// carries only the identity claims needed by the producer and keeps the
/// historical handoff as a family-local input boundary.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountIdentityAuthorityProducerV2Claims {
    pub account_id: String,
    pub household_id: String,
    pub provider: String,
    pub provider_subject: String,
    pub member_id: String,
    pub device_id: String,
    pub session_id: String,
}

impl AccountIdentityAuthorityProducerV2Claims {
    pub fn validate_shape(&self) -> Result<(), &'static str> {
        for value in [
            self.account_id.as_str(),
            self.household_id.as_str(),
            self.provider.as_str(),
            self.provider_subject.as_str(),
            self.member_id.as_str(),
            self.device_id.as_str(),
            self.session_id.as_str(),
        ] {
            if value.trim().is_empty()
                || value.len() > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_FIELD_BYTES
            {
                return Err("v2 producer authority claim is invalid");
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountIdentityAuthorityProducerV2Binding {
    pub account_id: String,
    pub household_id: String,
    pub receipt_id: String,
    pub service_binding_id: String,
    pub key_id: String,
    pub key_generation: u64,
    pub authority_generation: u64,
    pub session_generation: u64,
    pub correlation_id: String,
    pub idempotency_key: String,
}

impl AccountIdentityAuthorityProducerV2Binding {
    pub fn validate_shape(&self) -> Result<(), &'static str> {
        for value in [
            self.account_id.as_str(),
            self.household_id.as_str(),
            self.receipt_id.as_str(),
            self.service_binding_id.as_str(),
            self.key_id.as_str(),
            self.correlation_id.as_str(),
            self.idempotency_key.as_str(),
        ] {
            if value.trim().is_empty()
                || value.len() > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_FIELD_BYTES
            {
                return Err("v2 producer binding field is invalid");
            }
        }
        if self.key_generation == 0
            || self.authority_generation == 0
            || self.session_generation == 0
        {
            return Err("v2 producer binding generation is invalid");
        }
        if !self
            .key_id
            .starts_with(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_PREFIX)
        {
            return Err("v2 producer key id is invalid");
        }
        if !self
            .receipt_id
            .starts_with(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_RECEIPT_ID_PREFIX)
        {
            return Err("v2 producer receipt id is invalid");
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountIdentityAuthorityProducerV2Receipt {
    pub receipt_id: String,
    pub operation: AccountIdentityAuthorityProducerV2Operation,
    pub account_id: String,
    pub household_id: String,
    pub service_binding_id: String,
    pub correlation_id: String,
    pub idempotency_key: String,
    pub payload_digest: String,
    pub key_id: String,
    pub key_generation: u64,
    pub authority_generation: u64,
    pub session_generation: u64,
    pub issued_at: String,
    pub expires_at: String,
}

impl AccountIdentityAuthorityProducerV2Receipt {
    pub fn validate_shape(&self) -> Result<(), &'static str> {
        for value in [
            self.receipt_id.as_str(),
            self.account_id.as_str(),
            self.household_id.as_str(),
            self.service_binding_id.as_str(),
            self.correlation_id.as_str(),
            self.idempotency_key.as_str(),
            self.payload_digest.as_str(),
            self.key_id.as_str(),
            self.issued_at.as_str(),
            self.expires_at.as_str(),
        ] {
            if value.trim().is_empty()
                || value.len() > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_FIELD_BYTES
            {
                return Err("v2 producer receipt field is invalid");
            }
        }
        if self.key_generation == 0
            || self.authority_generation == 0
            || self.session_generation == 0
            || !self
                .key_id
                .starts_with(ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_KEY_ID_PREFIX)
        {
            return Err("v2 producer receipt generation or key id is invalid");
        }
        Ok(())
    }
}
