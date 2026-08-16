use crate::family_identity::HouseholdRole;
use crate::setup_lifecycle::{
    RecoveryChildEvidenceAccessState, RecoveryDataCustodyHandoffState, RecoveryFailureReason,
    RecoveryIdentityProofState, RecoveryKind, RecoveryOperation, RecoveryState,
    SetupInviteFailureReason, SetupInviteInput, SetupInvitePurpose, SetupInviteReplayState,
    SetupInviteState, SetupInviteTargetRole, SetupRecoveryAbuseState,
    SetupRecoveryResponseTimingState,
};

pub(crate) fn setup_invite_failure_reason(
    input: &SetupInviteInput,
) -> Option<SetupInviteFailureReason> {
    [
        (
            input.invite_state != SetupInviteState::Pending,
            SetupInviteFailureReason::InviteNotActive,
        ),
        (
            !input.single_use,
            SetupInviteFailureReason::InviteNotSingleUse,
        ),
        (
            input.replay_state != SetupInviteReplayState::Fresh,
            SetupInviteFailureReason::InviteReplayRejected,
        ),
        (
            input.abuse_state == SetupRecoveryAbuseState::Throttled,
            SetupInviteFailureReason::InviteNotActive,
        ),
        (
            input.response_timing_state != SetupRecoveryResponseTimingState::Uniform,
            SetupInviteFailureReason::InviteNotActive,
        ),
        (!input.same_family, SetupInviteFailureReason::WrongHousehold),
        (
            !purpose_matches_target_role(input.purpose, input.target_role),
            SetupInviteFailureReason::WrongTargetRole,
        ),
        (
            !inviter_can_issue(input.inviter_role, input.purpose),
            SetupInviteFailureReason::InviterNotAuthorized,
        ),
    ]
    .into_iter()
    .find_map(|(failed, reason)| failed.then_some(reason))
}

pub(crate) fn recovery_failure_reason(input: &RecoveryOperation) -> Option<RecoveryFailureReason> {
    [
        (
            input.state == RecoveryState::Revoked,
            RecoveryFailureReason::RecoveryNotActive,
        ),
        (
            input.requester_role != HouseholdRole::SupportAdmin && !input.same_family,
            RecoveryFailureReason::WrongHousehold,
        ),
        (
            input.identity_proof_state != RecoveryIdentityProofState::Verified,
            RecoveryFailureReason::IdentityProofRequired,
        ),
        (
            input.abuse_state == SetupRecoveryAbuseState::Throttled,
            RecoveryFailureReason::IdentityProofRequired,
        ),
        (
            input.response_timing_state != SetupRecoveryResponseTimingState::Uniform,
            RecoveryFailureReason::IdentityProofRequired,
        ),
        (
            !requester_can_recover(input.requester_role, input.kind, input.support_channel),
            RecoveryFailureReason::RoleNotAuthorized,
        ),
    ]
    .into_iter()
    .find_map(|(failed, reason)| failed.then_some(reason))
}

pub(crate) fn child_evidence_access_state(
    input: RecoveryOperation,
) -> RecoveryChildEvidenceAccessState {
    match (
        input.same_family
            && matches!(
                input.requester_role,
                HouseholdRole::ParentOwner | HouseholdRole::CoParentGuardian
            ),
        input.support_channel,
    ) {
        (true, crate::setup_lifecycle::RecoverySupportChannel::SupportAssisted) => {
            RecoveryChildEvidenceAccessState::Blocked
        }
        (true, _) => RecoveryChildEvidenceAccessState::Allowed,
        (false, _) => RecoveryChildEvidenceAccessState::Blocked,
    }
}

pub(crate) fn data_custody_handoff_state(
    input: RecoveryOperation,
) -> RecoveryDataCustodyHandoffState {
    match (input.kind, input.delete_export_handoff_required) {
        (RecoveryKind::HouseholdTransfer, _) => {
            RecoveryDataCustodyHandoffState::HouseholdTransferHandoffRequired
        }
        (_, true) => RecoveryDataCustodyHandoffState::ExportDeleteHandoffRequired,
        (_, false) => RecoveryDataCustodyHandoffState::None,
    }
}

fn purpose_matches_target_role(
    purpose: SetupInvitePurpose,
    target_role: SetupInviteTargetRole,
) -> bool {
    matches!(
        (purpose, target_role),
        (
            SetupInvitePurpose::CoParentInvite,
            SetupInviteTargetRole::CoParentGuardian
        ) | (
            SetupInvitePurpose::ObserverInvite,
            SetupInviteTargetRole::Observer
        ) | (
            SetupInvitePurpose::ChildDevicePairing,
            SetupInviteTargetRole::ChildDeviceAgent
        ) | (
            SetupInvitePurpose::HouseholdTransfer,
            SetupInviteTargetRole::ParentOwner
        )
    )
}

fn inviter_can_issue(role: HouseholdRole, purpose: SetupInvitePurpose) -> bool {
    matches!(
        (role, purpose),
        (
            HouseholdRole::ParentOwner | HouseholdRole::CoParentGuardian,
            SetupInvitePurpose::CoParentInvite
                | SetupInvitePurpose::ObserverInvite
                | SetupInvitePurpose::ChildDevicePairing
        ) | (
            HouseholdRole::ParentOwner,
            SetupInvitePurpose::HouseholdTransfer
        )
    )
}

fn requester_can_recover(
    role: HouseholdRole,
    kind: RecoveryKind,
    support_channel: crate::setup_lifecycle::RecoverySupportChannel,
) -> bool {
    if role == HouseholdRole::SupportAdmin {
        return support_channel == crate::setup_lifecycle::RecoverySupportChannel::SupportAssisted;
    }

    matches!(
        (role, kind),
        (HouseholdRole::ParentOwner, RecoveryKind::HouseholdTransfer)
            | (
                HouseholdRole::ParentOwner | HouseholdRole::CoParentGuardian,
                RecoveryKind::ForgotLogin
                    | RecoveryKind::LostParentDevice
                    | RecoveryKind::CompromisedAccount
                    | RecoveryKind::ChildReinstall
            )
    )
}
