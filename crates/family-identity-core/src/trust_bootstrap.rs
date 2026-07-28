use std::fmt;

use serde::{Deserialize, Serialize};

use crate::household_authority::{
    validate_parent_step_up_assertion, HouseholdAuthorityAction,
    ParentStepUpValidationFailureReason, ParentStepUpValidationInput,
};
use crate::parent_presence::ParentPresenceVerificationAccepted;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Default)]
struct TrustBootstrapSealingMarker;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    pub device_trust_ref: DeviceTrustRef,
    pub lifecycle_intent: TrustBootstrapLifecycleIntent,
    #[serde(skip)]
    sealing_marker: TrustBootstrapSealingMarker,
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedPlatformKeyUnsealingCredential {
    trust_bootstrap_ref: String,
    device_trust_ref: DeviceTrustRef,
    lifecycle_intent: TrustBootstrapLifecycleIntent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CurrentParentDeviceTrustAuthorityError {
    NotTrusted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct TrustBootstrapRejection {
    pub parent_step_up_failure_reason: ParentStepUpValidationFailureReason,
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DeviceTrustRef(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceTrustRefGenerationFailure {
    EntropyUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrustBootstrapManualRequirementReason {
    AuthorizedChallengeActionUnavailable,
    DeviceTrustReferenceGenerationUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct TrustBootstrapManualRequirement {
    pub reason: TrustBootstrapManualRequirementReason,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum TrustBootstrapDecision {
    AwaitingPlatformKeySealing(AwaitingPlatformKeySealingRequest),
    Rejected(TrustBootstrapRejection),
    ManualRequired(TrustBootstrapManualRequirement),
}

impl DeviceTrustRef {
    pub fn generate() -> Result<Self, DeviceTrustRefGenerationFailure> {
        let mut random = [0_u8; 32];
        getrandom::fill(&mut random)
            .map_err(|_error| DeviceTrustRefGenerationFailure::EntropyUnavailable)?;
        Ok(Self(encode_hex(&random)))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for DeviceTrustRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DeviceTrustRef")
            .field(&"[redacted]")
            .finish()
    }
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

impl fmt::Debug for PersistedPlatformKeyUnsealingCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PersistedPlatformKeyUnsealingCredential")
            .field("trust_bootstrap_ref", &"[redacted]")
            .field("device_trust_ref", &"[redacted]")
            .field("lifecycle_intent", &self.lifecycle_intent)
            .finish()
    }
}

impl AwaitingPlatformKeySealingRequest {
    pub fn consume_for_platform_key_sealing(self) -> PersistedPlatformKeyUnsealingCredential {
        PersistedPlatformKeyUnsealingCredential {
            trust_bootstrap_ref: self.trust_bootstrap_ref,
            device_trust_ref: self.device_trust_ref,
            lifecycle_intent: self.lifecycle_intent,
        }
    }
}

impl PersistedPlatformKeyUnsealingCredential {
    pub fn lifecycle_intent(&self) -> TrustBootstrapLifecycleIntent {
        self.lifecycle_intent
    }

    pub fn device_trust_ref(&self) -> &DeviceTrustRef {
        &self.device_trust_ref
    }

    pub fn trust_bootstrap_ref(&self) -> &str {
        &self.trust_bootstrap_ref
    }
}

pub fn current_parent_device_trust_authority(
    input: crate::household_authority::HouseholdAuthorityInput,
) -> Result<(), CurrentParentDeviceTrustAuthorityError> {
    let is_sealing_action = input.action == HouseholdAuthorityAction::SealParentDeviceTrust;
    let decision = crate::household_authority::authorize_household_action(input);
    (is_sealing_action
        && decision.authorization_state
            == crate::household_authority::HouseholdAuthorizationState::Authorized)
        .then_some(())
        .ok_or(CurrentParentDeviceTrustAuthorityError::NotTrusted)
}

pub fn evaluate_trust_bootstrap(input: TrustBootstrapInput) -> TrustBootstrapDecision {
    let TrustBootstrapInput {
        trust_bootstrap_ref,
        lifecycle_intent,
        parent_presence,
    } = input;

    let (
        _parent_presence_receipt_ref,
        parent_presence_challenge,
        parent_step_up_assertion,
        observed_at,
    ) = parent_presence.into_trust_bootstrap_parts();

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

    if !challenge_action_is_authorized_for_lifecycle_intent(
        lifecycle_intent,
        parent_presence_challenge.privileged_action,
    ) {
        return TrustBootstrapDecision::ManualRequired(TrustBootstrapManualRequirement {
            reason: TrustBootstrapManualRequirementReason::AuthorizedChallengeActionUnavailable,
        });
    }

    let device_trust_ref = match DeviceTrustRef::generate() {
        Ok(reference) => reference,
        Err(DeviceTrustRefGenerationFailure::EntropyUnavailable) => {
            return TrustBootstrapDecision::ManualRequired(TrustBootstrapManualRequirement {
                reason:
                    TrustBootstrapManualRequirementReason::DeviceTrustReferenceGenerationUnavailable,
            });
        }
    };

    TrustBootstrapDecision::AwaitingPlatformKeySealing(AwaitingPlatformKeySealingRequest {
        device_trust_ref,
        trust_bootstrap_ref,
        lifecycle_intent,
        sealing_marker: TrustBootstrapSealingMarker,
    })
}

fn challenge_action_is_authorized_for_lifecycle_intent(
    lifecycle_intent: TrustBootstrapLifecycleIntent,
    challenge_action: HouseholdAuthorityAction,
) -> bool {
    const SEAL_PARENT_DEVICE_TRUST_ACTIONS: &[HouseholdAuthorityAction] =
        &[HouseholdAuthorityAction::SealParentDeviceTrust];
    match lifecycle_intent {
        TrustBootstrapLifecycleIntent::SealParentDeviceTrust => {
            SEAL_PARENT_DEVICE_TRUST_ACTIONS.contains(&challenge_action)
        }
    }
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}
