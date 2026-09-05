use std::fmt;

use ed25519_dalek::VerifyingKey;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParentStepUpCeremonyError {
    InvalidIntent,
}

/// The exact high-risk registration action. Its digest is carried in the
/// durable nonce identity, so the challenge cannot be redirected to another
/// child, installation, route, or signer key after restart.
pub(crate) struct RegisterLanSignerAnchorIntent {
    lifecycle_generation: u64,
    installation_binding_generation: u64,
    authority_generation: u64,
    intent_digest: String,
}

#[derive(Clone, Copy)]
pub(crate) struct RegisterLanSignerAnchorIntentInput<'a> {
    pub(crate) family_id: &'a str,
    pub(crate) trust_subject: &'a str,
    pub(crate) parent_account_id: &'a str,
    pub(crate) parent_device_id: &'a str,
    pub(crate) child_device_id: &'a str,
    pub(crate) installation_id: &'a str,
    pub(crate) pairing_id: &'a str,
    pub(crate) route_id: &'a str,
    pub(crate) signer_public_key: &'a [u8; 32],
    pub(crate) lifecycle_generation: u64,
    pub(crate) installation_binding_generation: u64,
    pub(crate) authority_generation: u64,
    pub(crate) correlation_id: &'a str,
}

impl fmt::Debug for RegisterLanSignerAnchorIntent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RegisterLanSignerAnchorIntent")
            .field("redaction", &"sensitive-fields-omitted")
            .field("lifecycle_generation", &self.lifecycle_generation)
            .field(
                "installation_binding_generation",
                &self.installation_binding_generation,
            )
            .field("authority_generation", &self.authority_generation)
            .finish()
    }
}

impl RegisterLanSignerAnchorIntent {
    pub(crate) fn new(
        input: RegisterLanSignerAnchorIntentInput<'_>,
    ) -> Result<Self, ParentStepUpCeremonyError> {
        let values = [
            input.family_id,
            input.trust_subject,
            input.parent_account_id,
            input.parent_device_id,
            input.child_device_id,
            input.installation_id,
            input.pairing_id,
            input.route_id,
            input.correlation_id,
        ];
        if values
            .iter()
            .any(|value| value.trim().is_empty() || value.len() > 512)
            || input.lifecycle_generation == 0
            || input.installation_binding_generation == 0
            || input.authority_generation == 0
            || VerifyingKey::from_bytes(input.signer_public_key)
                .map(|key| key.is_weak())
                .unwrap_or(true)
        {
            return Err(ParentStepUpCeremonyError::InvalidIntent);
        }
        let intent_digest = intent_digest(
            &values,
            input.signer_public_key,
            input.lifecycle_generation,
            input.installation_binding_generation,
            input.authority_generation,
        );
        Ok(Self {
            lifecycle_generation: input.lifecycle_generation,
            installation_binding_generation: input.installation_binding_generation,
            authority_generation: input.authority_generation,
            intent_digest,
        })
    }

    pub(crate) fn intent_digest(&self) -> &str {
        &self.intent_digest
    }
}

fn intent_digest(
    values: &[&str; 9],
    signer_public_key: &[u8; 32],
    lifecycle_generation: u64,
    installation_binding_generation: u64,
    authority_generation: u64,
) -> String {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"ocentra-register-lan-signer-anchor-v1\0");
    for value in values {
        bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
        bytes.extend_from_slice(value.as_bytes());
    }
    bytes.extend_from_slice(signer_public_key);
    bytes.extend_from_slice(&lifecycle_generation.to_be_bytes());
    bytes.extend_from_slice(&installation_binding_generation.to_be_bytes());
    bytes.extend_from_slice(&authority_generation.to_be_bytes());
    hex(&Sha256::digest(bytes))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
