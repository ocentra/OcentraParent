use std::fmt;

use serde::Serialize;

use crate::household_authority::{
    validate_parent_step_up_assertion, HouseholdAuthorityAction, ParentStepUpValidationInput,
};
use crate::parent_presence::ParentPresenceVerificationAccepted;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceTrustAuthorityVerificationFailure {
    ParentStepUpRejected,
    ActionNotAuthorized,
}

#[derive(PartialEq, Eq)]
pub struct VerifiedParentDeviceTrustAuthority {
    family_id: String,
    parent_account_id: String,
    device_id: String,
    action: HouseholdAuthorityAction,
}

impl fmt::Debug for VerifiedParentDeviceTrustAuthority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VerifiedParentDeviceTrustAuthority")
            .field("family_id", &"[redacted]")
            .field("parent_account_id", &"[redacted]")
            .field("device_id", &"[redacted]")
            .field("action", &"[redacted]")
            .finish()
    }
}

pub fn verify_parent_device_trust_authority(
    parent_presence: ParentPresenceVerificationAccepted,
) -> Result<VerifiedParentDeviceTrustAuthority, DeviceTrustAuthorityVerificationFailure> {
    let (_receipt_ref, challenge, assertion, observed_at) =
        parent_presence.into_trust_bootstrap_parts();
    let validation_input = ParentStepUpValidationInput {
        assertion: Some(assertion),
        family_id: challenge.family_id.clone(),
        parent_account_id: challenge.parent_account_id.clone(),
        action_device_id: challenge.action_device_id.clone(),
        action_device_child_profile_id: challenge.action_device_child_profile_id,
        target_child_profile_id: challenge.target_child_profile_id,
        action: challenge.privileged_action,
        observed_at: observed_at.to_string(),
        expected_nonce: Some(challenge.nonce_ref),
    };
    if validate_parent_step_up_assertion(&validation_input)
        .failure_reason
        .is_some()
    {
        return Err(DeviceTrustAuthorityVerificationFailure::ParentStepUpRejected);
    }
    if !is_device_trust_action(challenge.privileged_action) {
        return Err(DeviceTrustAuthorityVerificationFailure::ActionNotAuthorized);
    }
    Ok(VerifiedParentDeviceTrustAuthority {
        family_id: challenge.family_id,
        parent_account_id: challenge.parent_account_id,
        device_id: challenge.action_device_id,
        action: challenge.privileged_action,
    })
}

impl VerifiedParentDeviceTrustAuthority {
    pub(crate) fn into_registry_parts(self) -> (String, String, String, HouseholdAuthorityAction) {
        (
            self.family_id,
            self.parent_account_id,
            self.device_id,
            self.action,
        )
    }
}

fn is_device_trust_action(action: HouseholdAuthorityAction) -> bool {
    matches!(
        action,
        HouseholdAuthorityAction::PairChildDevice | HouseholdAuthorityAction::RevokeChildDevice
    )
}
