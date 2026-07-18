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
    #[serde(rename = "expired")]
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    pub observed_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentPresenceReceiptRef(String);

impl ParentPresenceReceiptRef {
    pub fn from_static(bytes: &'static [u8]) -> Self {
        Self(String::from_utf8_lossy(bytes).into_owned())
    }
}

impl fmt::Display for ParentPresenceReceiptRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentPresenceObservedAt(String);

impl ParentPresenceObservedAt {
    pub fn from_static(bytes: &'static [u8]) -> Self {
        Self(String::from_utf8_lossy(bytes).into_owned())
    }
}

impl fmt::Display for ParentPresenceObservedAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParentPresenceVerificationReceipt {
    receipt_ref: ParentPresenceReceiptRef,
}

impl ParentPresenceVerificationReceipt {
    pub fn receipt_ref(&self) -> ParentPresenceReceiptRef {
        self.receipt_ref.clone()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParentPresenceVerificationAccepted {
    receipt: ParentPresenceVerificationReceipt,
    challenge: ParentPresenceChallenge,
    assertion_snapshot: ParentStepUpAssertionSnapshot,
    observed_at: ParentPresenceObservedAt,
}

impl ParentPresenceVerificationAccepted {
    pub fn receipt_ref(&self) -> ParentPresenceReceiptRef {
        self.receipt.receipt_ref()
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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParentPresenceVerificationPort {
    issued_challenges: BTreeMap<String, ParentPresenceChallenge>,
    consumed_challenge_refs: BTreeSet<String>,
}

impl ParentPresenceVerificationPort {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn issue_challenge(&mut self, challenge: ParentPresenceChallenge) {
        self.issued_challenges
            .insert(challenge.challenge_ref.clone(), challenge);
    }

    pub fn verify_and_consume(
        &mut self,
        input: ParentPresenceVerificationInput,
    ) -> Result<ParentPresenceVerificationAccepted, ParentPresenceVerificationFailureReason> {
        if self.consumed_challenge_refs.contains(&input.challenge_ref) {
            return Err(ParentPresenceVerificationFailureReason::ReplayRejected);
        }

        let Some(challenge) = self.issued_challenges.get(&input.challenge_ref) else {
            return Err(ParentPresenceVerificationFailureReason::ChallengeNotIssued);
        };

        if challenge.family_id != input.assertion.family_id {
            return Err(ParentPresenceVerificationFailureReason::HouseholdMismatch);
        }

        if challenge.parent_account_id != input.assertion.parent_account_id {
            return Err(ParentPresenceVerificationFailureReason::ParentAccountMismatch);
        }

        if challenge.privileged_action != input.assertion.action {
            return Err(ParentPresenceVerificationFailureReason::ActionMismatch);
        }

        if challenge.action_device_id != input.assertion.action_device_id {
            return Err(ParentPresenceVerificationFailureReason::ActionDeviceMismatch);
        }

        if challenge.action_device_child_profile_id
            != input.assertion.action_device_child_profile_id
        {
            return Err(ParentPresenceVerificationFailureReason::ActionDeviceChildProfileMismatch);
        }

        if challenge.target_child_profile_id != input.assertion.target_child_profile_id {
            return Err(ParentPresenceVerificationFailureReason::TargetChildProfileMismatch);
        }

        if challenge.nonce_ref != input.assertion.nonce {
            return Err(ParentPresenceVerificationFailureReason::NonceMismatch);
        }

        if challenge.expires_at != input.assertion.expires_at {
            return Err(ParentPresenceVerificationFailureReason::Expired);
        }

        if input.observed_at > challenge.expires_at {
            return Err(ParentPresenceVerificationFailureReason::Expired);
        }

        let challenge = self
            .issued_challenges
            .remove(&input.challenge_ref)
            .expect("challenge must still exist after a successful lookup");
        self.consumed_challenge_refs
            .insert(challenge.challenge_ref.clone());

        Ok(ParentPresenceVerificationAccepted {
            receipt: ParentPresenceVerificationReceipt {
                receipt_ref: ParentPresenceReceiptRef(format!(
                    "parent-presence-receipt:{}",
                    challenge.challenge_ref
                )),
            },
            challenge,
            assertion_snapshot: input.assertion,
            observed_at: ParentPresenceObservedAt(input.observed_at),
        })
    }
}
