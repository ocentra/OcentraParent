use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::household_authority::{HouseholdAuthorityAction, ParentStepUpAssertionSnapshot};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentPresenceVerificationFailureReason {
    #[serde(rename = "challenge-not-issued")]
    ChallengeNotIssued,
    #[serde(rename = "replay-rejected")]
    ReplayRejected,
    #[serde(rename = "duplicate-challenge-ref")]
    DuplicateChallengeRef,
    #[serde(rename = "household-mismatch")]
    HouseholdMismatch,
    #[serde(rename = "parent-account-mismatch")]
    ParentAccountMismatch,
    #[serde(rename = "action-mismatch")]
    ActionMismatch,
    #[serde(rename = "action-device-mismatch")]
    ActionDeviceMismatch,
    #[serde(rename = "action-device-child-profile-mismatch")]
    ActionDeviceChildProfileMismatch,
    #[serde(rename = "target-child-profile-mismatch")]
    TargetChildProfileMismatch,
    #[serde(rename = "nonce-mismatch")]
    NonceMismatch,
    #[serde(rename = "timestamp-invalid")]
    TimestampInvalid,
    #[serde(rename = "expired")]
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentPresenceChallengeIssuanceFailureReason {
    #[serde(rename = "duplicate-challenge-ref")]
    DuplicateChallengeRef,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentPresenceTimestampParseFailureReason {
    Malformed,
    NonCanonical,
    OffsetNotAllowed,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentPresenceChallenge {
    pub challenge_ref: String,
    pub nonce_ref: String,
    pub family_id: String,
    pub parent_account_id: String,
    pub privileged_action: HouseholdAuthorityAction,
    pub action_device_id: String,
    pub action_device_child_profile_id: Option<String>,
    pub target_child_profile_id: Option<String>,
    pub expires_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentPresenceVerificationInput {
    pub challenge_ref: String,
    pub assertion: ParentStepUpAssertionSnapshot,
}

#[derive(PartialEq, Eq)]
pub struct ParentPresenceReceiptRef(String);

impl ParentPresenceReceiptRef {
    pub(crate) fn from_string(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ParentPresenceReceiptRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ParentPresenceReceiptRef")
            .field(&"[redacted]")
            .finish()
    }
}

impl fmt::Display for ParentPresenceReceiptRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct ParentPresenceObservedAt {
    pub(crate) epoch_millis: i128,
    pub(crate) canonical: String,
}

#[derive(PartialEq, Eq)]
pub struct ParentPresenceVerificationAccepted {
    receipt_ref: ParentPresenceReceiptRef,
    challenge: ParentPresenceChallenge,
    assertion_snapshot: ParentStepUpAssertionSnapshot,
    observed_at: ParentPresenceObservedAt,
}

pub struct ParentPresenceVerificationPort {
    pub(crate) clock: Box<dyn Fn() -> ParentPresenceObservedAt>,
    pub(crate) issued_challenges: BTreeMap<String, ParentPresenceChallenge>,
    pub(crate) consumed_challenge_refs: BTreeSet<String>,
}

impl fmt::Debug for ParentPresenceChallenge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParentPresenceChallenge")
            .field("challenge_ref", &self.challenge_ref)
            .field("expires_at", &self.expires_at)
            .field("family_id", &"[redacted]")
            .field("parent_account_id", &"[redacted]")
            .field("privileged_action", &"[redacted]")
            .field("action_device_id", &"[redacted]")
            .field("action_device_child_profile_id", &"[redacted]")
            .field("target_child_profile_id", &"[redacted]")
            .field("nonce_ref", &"[redacted]")
            .finish()
    }
}

impl ParentPresenceVerificationAccepted {
    pub(crate) fn new(
        receipt_ref: ParentPresenceReceiptRef,
        challenge: ParentPresenceChallenge,
        assertion_snapshot: ParentStepUpAssertionSnapshot,
        observed_at: ParentPresenceObservedAt,
    ) -> Self {
        Self {
            receipt_ref,
            challenge,
            assertion_snapshot,
            observed_at,
        }
    }

    pub fn receipt_ref(&self) -> &ParentPresenceReceiptRef {
        &self.receipt_ref
    }

    pub fn assertion_snapshot(&self) -> &ParentStepUpAssertionSnapshot {
        &self.assertion_snapshot
    }

    pub fn observed_at(&self) -> ParentPresenceObservedAt {
        self.observed_at.clone()
    }

    pub(crate) fn into_trust_bootstrap_parts(
        self,
    ) -> (
        ParentPresenceChallenge,
        ParentStepUpAssertionSnapshot,
        ParentPresenceObservedAt,
    ) {
        (self.challenge, self.assertion_snapshot, self.observed_at)
    }
}

impl fmt::Debug for ParentPresenceVerificationAccepted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParentPresenceVerificationAccepted")
            .field("receipt_ref", &self.receipt_ref)
            .field("observed_at", &self.observed_at)
            .field("challenge", &"[redacted]")
            .field("assertion_snapshot", &"[redacted]")
            .finish()
    }
}
