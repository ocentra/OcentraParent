//! Rust-owned canonical contracts for the AI family.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const AI_CONTRACT_SCHEMA_VERSION: &str = "ai-contracts-v1";
pub const AI_INITIAL_LIFECYCLE_SEQUENCE: u64 = 0;

pub mod context;
pub mod explanation;
pub mod identity;
pub mod journal;
pub mod memory;
mod reference_inventory;
pub mod remote_assistant;
pub mod result;
pub mod work;

pub(crate) fn validate_contract_schema_version(
    value: &identity::AiSchemaVersion,
) -> Result<(), &'static str> {
    value
        .is_current()
        .then_some(())
        .ok_or("AI contract schema version is not current")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiAuthorityBoundary {
    EvidenceOnly,
    DeterministicPolicyRequired,
    ManualReviewRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiCustodyState {
    ChildLocalEncrypted,
    ParentLocalEncrypted,
    ParentAuthorizedRedacted,
    EphemeralLocal,
    Deleted,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiRetentionState {
    Active,
    Expired,
    Tombstoned,
    Deleted,
    ManualRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiRedactionState {
    NotApplicable,
    Redacted,
    FullyRedacted,
    RejectedPrivatePayload,
}

impl AiRedactionState {
    pub(crate) fn is_safe(self) -> bool {
        matches!(self, Self::Redacted | Self::FullyRedacted)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiDegradedState {
    None,
    MissingEvidence,
    InvalidOutput,
    Timeout,
    ModelUnavailable,
    ProviderUnavailable,
    CustodyUnavailable,
    ManualRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiDurabilityState {
    Durable,
    AppendPending,
    ReplayOnly,
    NotDurable,
    ManualRequired,
}

impl AiDurabilityState {
    pub(crate) fn is_durable(self) -> bool {
        matches!(self, Self::Durable | Self::ReplayOnly)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiValidationState {
    Accepted,
    Rejected,
    ManualRequired,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(transparent)]
pub struct AiConfidence(f64);

impl AiConfidence {
    pub fn parse(value: f64) -> Option<Self> {
        value
            .is_finite()
            .then_some(value)
            .filter(|value| (0.0..=1.0).contains(value))
            .map(Self)
    }

    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

impl<'de> Deserialize<'de> for AiConfidence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = f64::deserialize(deserializer)?;
        Self::parse(value).ok_or_else(|| {
            serde::de::Error::custom("AI confidence must be finite and between 0 and 1")
        })
    }
}

/// Raw text is restricted to local/request construction. Parent-visible and
/// remote-boundary fields use the AiSafeText type, which can only be issued
/// with an owner-held redaction receipt.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(transparent)]
pub(crate) struct AiText(String);

impl AiText {
    pub(crate) fn parse(value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        let valid = !value.trim().is_empty()
            && value.len() <= 64 * 1024
            && !value.chars().any(char::is_control);
        valid.then_some(Self(value))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

/// Text received from an untrusted wire boundary. It is deliberately not
/// serializable and exposes no conversion into `AiSafeText`; a trusted owner
/// must inspect it and issue an `AiRedactionReceipt` in-process.
#[derive(Clone, Eq, PartialEq)]
pub struct AiUntrustedText(String);

impl<'de> Deserialize<'de> for AiUntrustedText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        AiText::parse(value)
            .map(|text| Self(text.0))
            .ok_or_else(|| {
                serde::de::Error::custom(
                    "AI untrusted text is empty, oversized, or contains control characters",
                )
            })
    }
}

/// A redaction receipt is intentionally crate-private, non-cloneable, and
/// consumed on use. Its digest binds the exact untrusted source text to the
/// exact owner-produced safe output, so neither side can be substituted.
pub(crate) struct AiRedactionReceipt {
    binding_digest: [u8; 32],
    safe_output: AiText,
    redaction: AiRedactionState,
}

impl AiRedactionReceipt {
    pub(crate) fn issue(
        binding_domain: &[u8],
        binding_fields: &[&[u8]],
        source: &AiUntrustedText,
        safe_output: impl Into<String>,
        redaction: AiRedactionState,
    ) -> Option<Self> {
        if !redaction.is_safe() {
            return None;
        }
        let safe_output = AiText::parse(safe_output)?;
        Some(Self {
            binding_digest: ai_redaction_binding_digest(
                binding_domain,
                binding_fields,
                &source.0,
                safe_output.as_str(),
                redaction,
            ),
            safe_output,
            redaction,
        })
    }

    fn into_safe_text_for(
        self,
        binding_domain: &[u8],
        binding_fields: &[&[u8]],
        source: &AiUntrustedText,
    ) -> Option<AiSafeText> {
        let expected = ai_redaction_binding_digest(
            binding_domain,
            binding_fields,
            &source.0,
            self.safe_output.as_str(),
            self.redaction,
        );
        (expected == self.binding_digest).then_some(AiSafeText {
            text: self.safe_output,
            redaction: self.redaction,
        })
    }
}

fn ai_redaction_binding_digest(
    binding_domain: &[u8],
    binding_fields: &[&[u8]],
    source: &str,
    safe_output: &str,
    redaction: AiRedactionState,
) -> [u8; 32] {
    let redaction = match redaction {
        AiRedactionState::Redacted => b"redacted".as_slice(),
        AiRedactionState::FullyRedacted => b"fully-redacted".as_slice(),
        AiRedactionState::NotApplicable => b"not-applicable".as_slice(),
        AiRedactionState::RejectedPrivatePayload => b"rejected-private-payload".as_slice(),
    };
    let mut digest = Sha256::new();
    let terminal_fields = [source.as_bytes(), safe_output.as_bytes(), redaction];
    for value in std::iter::once(binding_domain)
        .chain(binding_fields.iter().copied())
        .chain(terminal_fields)
    {
        digest.update((value.len() as u64).to_be_bytes());
        digest.update(value);
    }
    digest.finalize().into()
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiSafeText {
    text: AiText,
    redaction: AiRedactionState,
}

impl AiSafeText {
    pub(crate) fn from_owner_redaction(
        binding_domain: &[u8],
        binding_fields: &[&[u8]],
        source: &AiUntrustedText,
        receipt: AiRedactionReceipt,
    ) -> Option<Self> {
        receipt.into_safe_text_for(binding_domain, binding_fields, source)
    }

    pub fn as_str(&self) -> &str {
        self.text.as_str()
    }

    pub fn redaction(&self) -> AiRedactionState {
        self.redaction
    }
}
