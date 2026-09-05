#![forbid(unsafe_code)]

use sha2::{Digest, Sha256};

pub const AUTHENTICATED_MANAGED_PROCESS_TARGET_SCHEMA_VERSION: u16 = 1;
pub const AUTHENTICATED_MANAGED_PROCESS_TARGET_SIGNATURE_BYTES: usize = 64;
pub const AUTHENTICATED_MANAGED_PROCESS_TARGET_MAX_FIELD_BYTES: usize = 512;
pub const AUTHENTICATED_MANAGED_PROCESS_TARGET_MAX_SIGNED_WIRE_BYTES: usize = 4_096;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AuthenticatedManagedProcessTargetBinding {
    pub schema_version: u16,
    pub issuer_key_id: String,
    pub grant_fingerprint: String,
    pub nonce: String,
    pub issuer_actor_id: String,
    pub household_id: String,
    pub parent_device_id: String,
    pub child_profile_id: String,
    pub target_device_id: String,
    pub policy_decision_id: String,
    pub policy_version: String,
    pub action_id: String,
    pub capability_id: String,
    /// Stable identity issued by the trusted local launcher/ownership
    /// registry. This is not a PID, process name, or caller-selected path.
    pub managed_process_identity: String,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthenticatedManagedProcessTargetValidationError {
    UnsupportedSchemaVersion,
    MissingBinding,
    InvalidProcessId,
    InvalidSignature,
    OversizedBinding,
    OversizedSignedWire,
}

impl AuthenticatedManagedProcessTargetBinding {
    pub fn validate_shape(&self) -> Result<(), AuthenticatedManagedProcessTargetValidationError> {
        if self.schema_version != AUTHENTICATED_MANAGED_PROCESS_TARGET_SCHEMA_VERSION {
            return Err(AuthenticatedManagedProcessTargetValidationError::UnsupportedSchemaVersion);
        }
        let bindings = [
            &self.issuer_key_id,
            &self.grant_fingerprint,
            &self.nonce,
            &self.issuer_actor_id,
            &self.household_id,
            &self.parent_device_id,
            &self.child_profile_id,
            &self.target_device_id,
            &self.policy_decision_id,
            &self.policy_version,
            &self.action_id,
            &self.capability_id,
            &self.managed_process_identity,
        ];
        if !bindings.iter().all(|value| !value.trim().is_empty()) {
            return Err(AuthenticatedManagedProcessTargetValidationError::MissingBinding);
        }
        if bindings
            .iter()
            .any(|value| value.len() > AUTHENTICATED_MANAGED_PROCESS_TARGET_MAX_FIELD_BYTES)
        {
            return Err(AuthenticatedManagedProcessTargetValidationError::OversizedBinding);
        }
        if self.signature.len() != AUTHENTICATED_MANAGED_PROCESS_TARGET_SIGNATURE_BYTES {
            return Err(AuthenticatedManagedProcessTargetValidationError::InvalidSignature);
        }
        if self.signing_bytes().len() > AUTHENTICATED_MANAGED_PROCESS_TARGET_MAX_SIGNED_WIRE_BYTES {
            return Err(AuthenticatedManagedProcessTargetValidationError::OversizedSignedWire);
        }
        Ok(())
    }

    pub fn signing_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"ocentra.authenticated-managed-process-target.v1\0");
        for value in [
            self.schema_version.to_string(),
            self.issuer_key_id.clone(),
            self.grant_fingerprint.clone(),
            self.nonce.clone(),
            self.issuer_actor_id.clone(),
            self.household_id.clone(),
            self.parent_device_id.clone(),
            self.child_profile_id.clone(),
            self.target_device_id.clone(),
            self.policy_decision_id.clone(),
            self.policy_version.clone(),
            self.action_id.clone(),
            self.capability_id.clone(),
            self.managed_process_identity.clone(),
        ] {
            bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
            bytes.extend_from_slice(value.as_bytes());
        }
        bytes
    }

    pub fn binding_fingerprint(&self) -> String {
        let mut digest = Sha256::new();
        digest.update(b"ocentra.authenticated-managed-process-target.fingerprint.v1\0");
        digest.update(self.signing_bytes());
        digest.update(&self.signature);
        format!("{:x}", digest.finalize())
    }
}
