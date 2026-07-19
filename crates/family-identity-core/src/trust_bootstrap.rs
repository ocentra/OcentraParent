use std::fmt;

use serde::Serialize;

use crate::household_authority::{
    validate_parent_step_up_assertion, ParentStepUpValidationFailureReason,
    ParentStepUpValidationInput,
};
use crate::parent_presence::{ParentPresenceReceiptRef, ParentPresenceVerificationAccepted};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Default)]
struct TrustBootstrapSealingMarker;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TrustBootstrapLifecycleIntent {
    #[serde(rename = "seal-parent-device-trust")]
    SealParentDeviceTrust,
}

#[derive(PartialEq, Eq)]
pub struct TrustBootstrapInput {
    pub trust_bootstrap_ref: String,
    pub lifecycle_intent: TrustBootstrapLifecycleIntent,
    pub parent_presence: ParentPresenceVerificationAccepted,
}

#[derive(PartialEq, Eq, Serialize)]
pub struct AwaitingPlatformKeySealingRequest {
    pub trust_bootstrap_ref: String,
    pub device_trust_ref: String,
    pub lifecycle_intent: TrustBootstrapLifecycleIntent,
    #[serde(skip)]
    sealing_marker: TrustBootstrapSealingMarker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct TrustBootstrapRejection {
    pub parent_step_up_failure_reason: ParentStepUpValidationFailureReason,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum TrustBootstrapDecision {
    AwaitingPlatformKeySealing(AwaitingPlatformKeySealingRequest),
    Rejected(TrustBootstrapRejection),
}

impl fmt::Debug for TrustBootstrapInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TrustBootstrapInput")
            .field("trust_bootstrap_ref", &"[redacted]")
            .field("lifecycle_intent", &self.lifecycle_intent)
            .field("parent_presence", &"[redacted]")
            .finish()
    }
}

impl fmt::Debug for AwaitingPlatformKeySealingRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AwaitingPlatformKeySealingRequest")
            .field("trust_bootstrap_ref", &"[redacted]")
            .field("device_trust_ref", &"[redacted]")
            .field("lifecycle_intent", &self.lifecycle_intent)
            .field("sealing_marker", &self.sealing_marker)
            .finish()
    }
}

pub fn evaluate_trust_bootstrap(input: TrustBootstrapInput) -> TrustBootstrapDecision {
    let TrustBootstrapInput {
        trust_bootstrap_ref,
        lifecycle_intent,
        parent_presence,
    } = input;

    let (
        parent_presence_receipt_ref,
        parent_presence_challenge,
        parent_step_up_assertion,
        observed_at,
    ) = parent_presence.into_trust_bootstrap_parts();

    let device_trust_ref = derive_device_trust_ref(
        &trust_bootstrap_ref,
        &parent_presence_receipt_ref,
        &parent_presence_challenge,
        lifecycle_intent,
    );

    let validation_input = ParentStepUpValidationInput {
        assertion: Some(parent_step_up_assertion),
        family_id: parent_presence_challenge.family_id,
        parent_account_id: parent_presence_challenge.parent_account_id,
        action_device_id: parent_presence_challenge.action_device_id,
        action_device_child_profile_id: parent_presence_challenge.action_device_child_profile_id,
        target_child_profile_id: parent_presence_challenge.target_child_profile_id,
        action: parent_presence_challenge.privileged_action,
        observed_at: observed_at.to_string(),
        expected_nonce: Some(parent_presence_challenge.nonce_ref),
    };

    let validation = validate_parent_step_up_assertion(&validation_input);

    if let Some(parent_step_up_failure_reason) = validation.failure_reason {
        return TrustBootstrapDecision::Rejected(TrustBootstrapRejection {
            parent_step_up_failure_reason,
        });
    }

    TrustBootstrapDecision::AwaitingPlatformKeySealing(AwaitingPlatformKeySealingRequest {
        device_trust_ref,
        trust_bootstrap_ref,
        lifecycle_intent,
        sealing_marker: TrustBootstrapSealingMarker,
    })
}

fn derive_device_trust_ref(
    trust_bootstrap_ref: &str,
    receipt_ref: &ParentPresenceReceiptRef,
    challenge: &crate::parent_presence::ParentPresenceChallenge,
    lifecycle_intent: TrustBootstrapLifecycleIntent,
) -> String {
    format!(
        "device-trust:{trust_bootstrap_ref}:{}:{}:{}:{}:{}:{}:{}:{:?}:{}:{:?}",
        receipt_ref.as_str(),
        challenge.family_id,
        challenge.parent_account_id,
        challenge.action_device_id,
        challenge
            .action_device_child_profile_id
            .as_deref()
            .unwrap_or("-"),
        challenge.target_child_profile_id.as_deref().unwrap_or("-"),
        challenge.nonce_ref,
        challenge.privileged_action,
        challenge.expires_at,
        lifecycle_intent
    )
}
