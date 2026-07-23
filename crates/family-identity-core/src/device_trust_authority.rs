use std::fmt;

use serde::Serialize;

use crate::household_authority::{
    validate_parent_step_up_assertion, AcceptedDeviceTrustAuthorization, HouseholdAuthorityAction,
    ParentStepUpValidationInput,
};
use crate::parent_presence::ParentPresenceVerificationAccepted;
use ocentra_eventing::ids::CorrelationId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceTrustAuthorityVerificationFailure {
    ParentStepUpRejected,
    HouseholdAuthorizationBindingMismatch,
    ActionNotAuthorized,
    TargetDeviceMissing,
    TargetDeviceMismatch,
    AuthorityActionMismatch,
}

/// The household command owns raw authorization evaluation and supplies only
/// its opaque accepted grant to this boundary.
pub struct DeviceTrustAuthorityInput {
    pub parent_presence: ParentPresenceVerificationAccepted,
    pub household_authorization: AcceptedDeviceTrustAuthorization,
}

#[derive(PartialEq, Eq)]
pub struct VerifiedParentDeviceTrustAuthority {
    family_id: String,
    parent_account_id: String,
    target_child_device_id: String,
    correlation_id: CorrelationId,
    receipt_ref: String,
    action: HouseholdAuthorityAction,
}

impl fmt::Debug for VerifiedParentDeviceTrustAuthority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VerifiedParentDeviceTrustAuthority")
            .field("family_id", &"[redacted]")
            .field("parent_account_id", &"[redacted]")
            .field("target_child_device_id", &"[redacted]")
            .field("action", &"[redacted]")
            .finish()
    }
}

pub fn verify_parent_device_trust_authority(
    input: DeviceTrustAuthorityInput,
) -> Result<VerifiedParentDeviceTrustAuthority, DeviceTrustAuthorityVerificationFailure> {
    let (receipt_ref, correlation_id, challenge, assertion, observed_at) =
        input.parent_presence.into_trust_bootstrap_parts();
    let validation_input = ParentStepUpValidationInput {
        assertion: Some(assertion),
        family_id: challenge.family_id.clone(),
        parent_account_id: challenge.parent_account_id.clone(),
        action_device_id: challenge.action_device_id.clone(),
        action_device_child_profile_id: challenge.action_device_child_profile_id,
        target_child_profile_id: challenge.target_child_profile_id,
        target_child_device_id: challenge.target_child_device_id.clone(),
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
    let target_child_device_id = challenge.target_child_device_id.as_deref();
    let Some(target_child_device_id) = target_child_device_id else {
        return Err(DeviceTrustAuthorityVerificationFailure::TargetDeviceMissing);
    };
    if !input.household_authorization.matches_device_trust_request(
        &challenge.family_id,
        &challenge.parent_account_id,
        target_child_device_id,
        challenge.privileged_action,
    ) {
        return Err(DeviceTrustAuthorityVerificationFailure::HouseholdAuthorizationBindingMismatch);
    }
    Ok(VerifiedParentDeviceTrustAuthority {
        family_id: challenge.family_id,
        parent_account_id: challenge.parent_account_id,
        target_child_device_id: target_child_device_id.to_owned(),
        correlation_id,
        receipt_ref: receipt_ref.as_str().to_owned(),
        action: challenge.privileged_action,
    })
}

impl VerifiedParentDeviceTrustAuthority {
    pub(crate) fn into_registry_parts(
        self,
    ) -> (
        String,
        String,
        String,
        HouseholdAuthorityAction,
        String,
        String,
    ) {
        (
            self.family_id,
            self.parent_account_id,
            self.target_child_device_id,
            self.action,
            self.correlation_id.as_str().to_owned(),
            self.receipt_ref,
        )
    }
}

fn is_device_trust_action(action: HouseholdAuthorityAction) -> bool {
    matches!(
        action,
        HouseholdAuthorityAction::PairChildDevice | HouseholdAuthorityAction::RevokeChildDevice
    )
}
