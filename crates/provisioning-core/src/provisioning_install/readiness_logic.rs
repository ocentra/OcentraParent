use super::*;

pub(super) fn provisioning_blocker_reason(
    input: ProvisioningReadinessInput,
) -> Option<ProvisioningBlockerReason> {
    readiness_blockers::provisioning_blocker_reason(input)
}

pub(super) fn provisioning_overall_state(
    blocker_reason: Option<ProvisioningBlockerReason>,
) -> ProvisioningOverallState {
    readiness_actions::provisioning_overall_state(blocker_reason)
}

pub(super) fn provisioning_recovery_action(
    blocker_reason: Option<ProvisioningBlockerReason>,
) -> ProvisioningRecoveryAction {
    readiness_actions::provisioning_recovery_action(blocker_reason)
}
