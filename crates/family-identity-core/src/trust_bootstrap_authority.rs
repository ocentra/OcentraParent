use crate::household_authority::{validate_parent_step_up_assertion, ParentStepUpValidationInput};
use crate::trust_bootstrap::{
    AwaitingPlatformKeySealingRequest, DeviceTrustRef, DeviceTrustRefGenerationFailure,
    SealParentDeviceTrustAuthorityReceipt, TrustBootstrapDecision, TrustBootstrapInput,
    TrustBootstrapManualRequirement, TrustBootstrapManualRequirementReason,
    TrustBootstrapRejection, TrustBootstrapSealingMarker,
};

pub(crate) fn evaluate(
    input: TrustBootstrapInput,
    authority_receipt: Option<SealParentDeviceTrustAuthorityReceipt>,
) -> TrustBootstrapDecision {
    let TrustBootstrapInput {
        trust_bootstrap_ref,
        lifecycle_intent,
        parent_presence,
    } = input;
    let (receipt_ref, challenge, assertion, observed_at) =
        parent_presence.into_trust_bootstrap_parts();
    let validation = validate_parent_step_up_assertion(&ParentStepUpValidationInput {
        assertion: Some(assertion),
        family_id: challenge.family_id.clone(),
        parent_account_id: challenge.parent_account_id.clone(),
        action_device_id: challenge.action_device_id.clone(),
        action_device_child_profile_id: challenge.action_device_child_profile_id.clone(),
        target_child_profile_id: challenge.target_child_profile_id.clone(),
        action: challenge.privileged_action,
        observed_at: observed_at.to_string(),
        expected_nonce: Some(challenge.nonce_ref.clone()),
    });
    if let Some(parent_step_up_failure_reason) = validation.failure_reason {
        return TrustBootstrapDecision::Rejected(TrustBootstrapRejection {
            parent_step_up_failure_reason,
        });
    }
    if challenge.action_device_child_profile_id.is_some()
        || challenge.target_child_profile_id.is_some()
    {
        return manual(TrustBootstrapManualRequirementReason::ChildScopedCeremonyRejected);
    }
    if !matches!(
        (lifecycle_intent, challenge.privileged_action),
        (
            crate::trust_bootstrap::TrustBootstrapLifecycleIntent::SealParentDeviceTrust,
            crate::household_authority::HouseholdAuthorityAction::SealParentDeviceTrust
        )
    ) {
        return manual(TrustBootstrapManualRequirementReason::AuthorizedChallengeActionUnavailable);
    }
    if !authority_receipt.is_some_and(|receipt| receipt.matches(&receipt_ref, &challenge)) {
        return manual(TrustBootstrapManualRequirementReason::AuthorityReceiptRequired);
    }
    let device_trust_ref = match DeviceTrustRef::generate() {
        Ok(reference) => reference,
        Err(DeviceTrustRefGenerationFailure::EntropyUnavailable) => {
            return manual(
                TrustBootstrapManualRequirementReason::DeviceTrustReferenceGenerationUnavailable,
            );
        }
    };
    TrustBootstrapDecision::AwaitingPlatformKeySealing(AwaitingPlatformKeySealingRequest {
        device_trust_ref,
        trust_bootstrap_ref,
        lifecycle_intent,
        approved_parent_device_ceremony: super::ApprovedParentDeviceCeremony {
            trust_subject: challenge.parent_account_id.clone(),
            device_ref: challenge.action_device_id.clone(),
            device_role: "trusted-parent".to_owned(),
        },
        family_id: challenge.family_id,
        parent_account_id: challenge.parent_account_id,
        device_ref: challenge.action_device_id,
        sealing_marker: TrustBootstrapSealingMarker,
    })
}

fn manual(reason: TrustBootstrapManualRequirementReason) -> TrustBootstrapDecision {
    TrustBootstrapDecision::ManualRequired(TrustBootstrapManualRequirement { reason })
}
