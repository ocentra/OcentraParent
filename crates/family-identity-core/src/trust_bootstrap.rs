use serde::Serialize;

use crate::household_authority::{
    validate_parent_step_up_assertion, ParentStepUpValidationFailureReason,
    ParentStepUpValidationInput,
};
use crate::parent_presence::ParentPresenceVerificationAccepted;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Default)]
struct TrustBootstrapSealingMarker;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TrustBootstrapLifecycleIntent {
    #[serde(rename = "seal-parent-device-trust")]
    SealParentDeviceTrust,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TrustBootstrapInput {
    pub trust_bootstrap_ref: String,
    pub device_trust_ref: String,
    pub lifecycle_intent: TrustBootstrapLifecycleIntent,
    pub parent_presence: ParentPresenceVerificationAccepted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum TrustBootstrapDecision {
    AwaitingPlatformKeySealing(AwaitingPlatformKeySealingRequest),
    Rejected(TrustBootstrapRejection),
}

pub fn evaluate_trust_bootstrap(input: TrustBootstrapInput) -> TrustBootstrapDecision {
    let TrustBootstrapInput {
        trust_bootstrap_ref,
        device_trust_ref,
        lifecycle_intent,
        parent_presence,
    } = input;

    let (parent_presence_challenge, parent_step_up_assertion, observed_at) =
        parent_presence.into_trust_bootstrap_parts();

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
        trust_bootstrap_ref,
        device_trust_ref,
        lifecycle_intent,
        sealing_marker: TrustBootstrapSealingMarker,
    })
}
