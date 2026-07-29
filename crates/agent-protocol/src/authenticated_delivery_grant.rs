#![forbid(unsafe_code)]

use serde::Serialize;

pub const AUTHENTICATED_DELIVERY_GRANT_SCHEMA_VERSION: u16 = 1;
pub const AUTHENTICATED_DELIVERY_GRANT_SIGNATURE_BYTES: usize = 64;
pub const AUTHENTICATED_DELIVERY_GRANT_PAYLOAD_DIGEST_HEX_BYTES: usize = 64;
pub const AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES: usize = 512;
pub const AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES: usize = 4_096;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthenticatedDeliveryGrantInstant(i64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatedDeliveryGrant {
    pub schema_version: u16,
    pub issuer_key_id: String,
    pub issuer_actor_id: String,
    pub household_id: String,
    pub parent_device_id: String,
    pub child_profile_id: String,
    pub target_device_id: String,
    pub policy_decision_id: String,
    pub policy_version: String,
    pub action_id: String,
    pub capability_id: String,
    pub evidence_digest: String,
    pub payload_digest: String,
    pub dry_run: bool,
    pub nonce: String,
    pub issued_at: String,
    pub expires_at: String,
    pub revocation_version: String,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthenticatedDeliveryGrantValidationError {
    UnsupportedSchemaVersion,
    MissingBinding,
    InvalidPayloadDigest,
    InvalidSignature,
    InvalidTimestamp,
    InvalidTimeWindow,
}

impl AuthenticatedDeliveryGrant {
    pub fn validate_shape(&self) -> Result<(), AuthenticatedDeliveryGrantValidationError> {
        if self.schema_version != AUTHENTICATED_DELIVERY_GRANT_SCHEMA_VERSION {
            return Err(AuthenticatedDeliveryGrantValidationError::UnsupportedSchemaVersion);
        }
        if ![
            &self.issuer_key_id,
            &self.issuer_actor_id,
            &self.household_id,
            &self.parent_device_id,
            &self.child_profile_id,
            &self.target_device_id,
            &self.policy_decision_id,
            &self.policy_version,
            &self.action_id,
            &self.capability_id,
            &self.evidence_digest,
            &self.payload_digest,
            &self.nonce,
            &self.issued_at,
            &self.expires_at,
            &self.revocation_version,
        ]
        .into_iter()
        .all(|value| !value.trim().is_empty())
        {
            return Err(AuthenticatedDeliveryGrantValidationError::MissingBinding);
        }
        if self.payload_digest.len() != AUTHENTICATED_DELIVERY_GRANT_PAYLOAD_DIGEST_HEX_BYTES
            || !self
                .payload_digest
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit())
        {
            return Err(AuthenticatedDeliveryGrantValidationError::InvalidPayloadDigest);
        }
        if self.signature.len() != AUTHENTICATED_DELIVERY_GRANT_SIGNATURE_BYTES {
            return Err(AuthenticatedDeliveryGrantValidationError::InvalidSignature);
        }
        if self.issued_at_instant()? > self.expires_at_instant()? {
            return Err(AuthenticatedDeliveryGrantValidationError::InvalidTimeWindow);
        }
        Ok(())
    }

    pub fn issued_at_instant(
        &self,
    ) -> Result<AuthenticatedDeliveryGrantInstant, AuthenticatedDeliveryGrantValidationError> {
        parse_authenticated_delivery_grant_instant(&self.issued_at)
    }

    pub fn expires_at_instant(
        &self,
    ) -> Result<AuthenticatedDeliveryGrantInstant, AuthenticatedDeliveryGrantValidationError> {
        parse_authenticated_delivery_grant_instant(&self.expires_at)
    }

    pub fn signing_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for value in [
            self.schema_version.to_string(),
            self.issuer_key_id.clone(),
            self.issuer_actor_id.clone(),
            self.household_id.clone(),
            self.parent_device_id.clone(),
            self.child_profile_id.clone(),
            self.target_device_id.clone(),
            self.policy_decision_id.clone(),
            self.policy_version.clone(),
            self.action_id.clone(),
            self.capability_id.clone(),
            self.evidence_digest.clone(),
            self.payload_digest.clone(),
            self.dry_run.to_string(),
            self.nonce.clone(),
            self.issued_at.clone(),
            self.expires_at.clone(),
            self.revocation_version.clone(),
        ] {
            let length = value.len() as u64;
            bytes.extend_from_slice(&length.to_be_bytes());
            bytes.extend_from_slice(value.as_bytes());
        }
        bytes
    }
}

pub fn parse_authenticated_delivery_grant_instant(
    value: &str,
) -> Result<AuthenticatedDeliveryGrantInstant, AuthenticatedDeliveryGrantValidationError> {
    timestamp::parse(value).ok_or(AuthenticatedDeliveryGrantValidationError::InvalidTimestamp)
}

mod timestamp;
mod wire;
