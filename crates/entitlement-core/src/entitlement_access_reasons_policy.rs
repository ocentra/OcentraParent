use crate::entitlement_access::{
    EntitlementCapabilityInput, EntitlementCapabilityRejectionReason, EntitlementCapabilityScope,
    EntitlementPolicyState, FamilySetupState, OfflineGraceState, SubscriptionState,
};
use crate::entitlement_snapshot_values::{
    EntitlementDeviceTrustRequirementState, EntitlementDeviceTrustState,
    EntitlementPackageBuildState, EntitlementSnapshotBindingState,
};

pub(crate) fn binding_or_trust_reason(
    input: &EntitlementCapabilityInput,
) -> Option<EntitlementCapabilityRejectionReason> {
    if input.snapshot_context.household_binding_state == EntitlementSnapshotBindingState::Mismatched
    {
        return Some(EntitlementCapabilityRejectionReason::WrongHousehold);
    }

    if input.snapshot_context.device_binding_state == EntitlementSnapshotBindingState::Mismatched {
        return Some(EntitlementCapabilityRejectionReason::WrongDevice);
    }

    if input.snapshot_context.device_trust_requirement_state
        == EntitlementDeviceTrustRequirementState::Required
        && input.snapshot_context.device_trust_state == EntitlementDeviceTrustState::Missing
    {
        return Some(EntitlementCapabilityRejectionReason::MissingDeviceTrust);
    }

    if input.snapshot_context.package_build_state == EntitlementPackageBuildState::Invalid {
        return Some(EntitlementCapabilityRejectionReason::InvalidPackageBuild);
    }

    None
}

pub(crate) fn package_family_policy_scope_subscription_reason(
    input: &EntitlementCapabilityInput,
) -> Option<EntitlementCapabilityRejectionReason> {
    if input.family_setup_state == FamilySetupState::Incomplete {
        return Some(EntitlementCapabilityRejectionReason::IncompleteFamilySetup);
    }

    if input.policy_state == EntitlementPolicyState::PaymentDispute {
        return Some(EntitlementCapabilityRejectionReason::PaymentDispute);
    }

    if input.capability_scope == EntitlementCapabilityScope::ParentPortalOnly {
        return Some(EntitlementCapabilityRejectionReason::ParentPortalOnlyScope);
    }

    if input.subscription_state == SubscriptionState::Inactive
        && input.offline_grace_state == OfflineGraceState::Inactive
    {
        return Some(EntitlementCapabilityRejectionReason::InactiveSubscription);
    }

    None
}
