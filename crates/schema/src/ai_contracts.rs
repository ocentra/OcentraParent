//! Rust-owned canonical contracts for the AI family.
use serde::{Deserialize, Serialize};

pub const AI_CONTRACT_SCHEMA_VERSION: &str = "ai-contracts-v1";
pub const AI_INITIAL_LIFECYCLE_SEQUENCE: u64 = 0;

pub mod context;
pub mod explanation;
pub mod identity;
pub mod journal;
pub mod memory;
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

/// A redaction receipt is intentionally crate-private. The parent/redaction
/// owner is the only code allowed to issue one; callers cannot relabel raw
/// text by choosing a public redaction constructor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct AiRedactionReceipt {
    redaction: AiRedactionState,
}

impl AiRedactionReceipt {
    pub(crate) fn issue(redaction: AiRedactionState) -> Option<Self> {
        redaction.is_safe().then_some(Self { redaction })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiSafeText {
    text: AiText,
    redaction: AiRedactionState,
}

impl AiSafeText {
    pub(crate) fn from_redaction_receipt(
        value: impl Into<String>,
        receipt: AiRedactionReceipt,
    ) -> Option<Self> {
        Self::from_parts(value.into(), receipt.redaction).ok()
    }

    pub fn as_str(&self) -> &str {
        self.text.as_str()
    }

    pub fn redaction(&self) -> AiRedactionState {
        self.redaction
    }

    fn from_parts(value: String, redaction: AiRedactionState) -> Result<Self, &'static str> {
        if !redaction.is_safe() {
            return Err("safe AI text requires redacted or fully-redacted state");
        }
        Ok(Self {
            text: AiText::parse(value).ok_or("safe AI text is empty, oversized, or invalid")?,
            redaction,
        })
    }
}
