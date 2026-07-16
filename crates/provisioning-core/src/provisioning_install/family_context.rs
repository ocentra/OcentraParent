use super::*;

pub(super) fn derive_provisioning_readiness_input_from_family_context(
    input: ProvisioningFamilyContextInput,
) -> ProvisioningReadinessInput {
    let authority_decision = authorize_household_action(input.household_authority_input);
    let session_decision = authorize_session_token_action(input.pairing_session_input);
    let recovery_decision = input.recovery_operation.map(evaluate_recovery_operation);

    ProvisioningReadinessInput {
        membership_state: input.household_authority_input.membership_state,
        account_readiness_state: family_account::provisioning_account_state_from_family_context(
            input,
            authority_decision,
            recovery_decision,
            session_decision.failure_reason,
        ),
        parent_app_readiness_state: input.parent_app_readiness_state,
        parent_device_registration_state: input.parent_device_registration_state,
        child_install_state: input.child_install_state,
        child_service_state: input.child_service_state,
        child_app_readiness_state: family_account::provisioning_child_app_readiness_state(
            input.child_install_state,
            input.child_service_state,
        ),
        child_device_ownership_scope: input.household_authority_input.device_ownership_scope,
        device_trust_state: input
            .recovery_operation
            .map(device_trust_state_for_recovery_operation)
            .unwrap_or(input.household_authority_input.device_trust_state),
        permission_readiness_state: input.permission_readiness_state,
        pairing_lifecycle_state: family_pairing::provisioning_pairing_state_from_family_context(
            input,
            authority_decision,
            session_decision.failure_reason,
        ),
        policy_baseline_state: input.policy_baseline_state,
        data_custody_sync_state: family_recovery::provisioning_custody_state_from_family_context(
            input,
            recovery_decision,
        ),
        network_reachability_state: input.network_reachability_state,
        recovery_state: family_recovery::provisioning_recovery_state_from_family_context(
            input,
            session_decision.failure_reason,
        ),
    }
}

pub(super) fn first_some<T, const N: usize>(values: [Option<T>; N]) -> Option<T> {
    values.into_iter().flatten().next()
}

pub(super) fn mapped_projection<T: Copy + PartialEq, U: Copy, const N: usize>(
    state: T,
    mappings: [(T, U); N],
) -> Option<U> {
    mappings
        .into_iter()
        .find_map(|(candidate, projection)| (state == candidate).then_some(projection))
}
