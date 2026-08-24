//! Rust-owned cross-boundary contracts for the AI plan.
//!
//! This module owns encoded identity, work, evidence, result, memory, journal,
//! explanation, prompt/runtime, and remote-assistant shapes.  It deliberately
//! contains no model execution or policy authority.  Consumers must treat an
//! AI result as evidence and pass it through the deterministic policy owner
//! before any action is considered.

use serde::{Deserialize, Serialize};

pub const AI_CONTRACT_SCHEMA_VERSION: &str = "ai-contracts-v1";

pub mod context;
pub mod explanation;
pub mod identity;
pub mod journal;
pub mod memory;
pub mod remote_assistant;
pub mod result;
pub mod work;

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

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(transparent)]
pub struct AiText(String);

impl AiText {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        (!value.trim().is_empty()).then_some(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for AiText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).ok_or_else(|| serde::de::Error::custom("AI text must not be empty"))
    }
}
