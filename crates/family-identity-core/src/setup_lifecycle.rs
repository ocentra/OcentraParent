use serde::{Deserialize, Serialize};

use crate::{AuditRequirementState, FamilyActorRole, HouseholdMembership};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SetupInvitePurpose {
    #[serde(rename = "co-parent-invite")]
    CoParentInvite,
    #[serde(rename = "observer-invite")]
    ObserverInvite,
    #[serde(rename = "child-device-pairing")]
    ChildDevicePairing,
    #[serde(rename = "household-transfer")]
    HouseholdTransfer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SetupInviteState {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "revoked")]
    Revoked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SetupInviteReplayState {
    #[serde(rename = "fresh")]
    Fresh,
    #[serde(rename = "replay-detected")]
    ReplayDetected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SetupInviteTargetRole {
    #[serde(rename = "co-parent-guardian")]
    CoParentGuardian,
    #[serde(rename = "observer")]
    Observer,
    #[serde(rename = "child-device-agent")]
    ChildDeviceAgent,
    #[serde(rename = "parent-owner")]
    ParentOwner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SetupInviteDecisionState {
    #[serde(rename = "acceptable")]
    Acceptable,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SetupInviteFailureReason {
    #[serde(rename = "invite-not-active")]
    InviteNotActive,
    #[serde(rename = "invite-replay-rejected")]
    InviteReplayRejected,
    #[serde(rename = "invite-not-single-use")]
    InviteNotSingleUse,
    #[serde(rename = "wrong-household")]
    WrongHousehold,
    #[serde(rename = "wrong-target-role")]
    WrongTargetRole,
    #[serde(rename = "inviter-not-authorized")]
    InviterNotAuthorized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetupInviteInput {
    pub inviter_role: FamilyActorRole,
    pub household_membership: HouseholdMembership,
    pub purpose: SetupInvitePurpose,
    pub target_role: SetupInviteTargetRole,
    pub invite_state: SetupInviteState,
    pub single_use: bool,
    pub replay_state: SetupInviteReplayState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetupInviteDecision {
    pub decision_state: SetupInviteDecisionState,
    pub audit_requirement_state: AuditRequirementState,
    pub failure_reason: Option<SetupInviteFailureReason>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryKind {
    #[serde(rename = "forgot-login")]
    ForgotLogin,
    #[serde(rename = "lost-parent-device")]
    LostParentDevice,
    #[serde(rename = "compromised-account")]
    CompromisedAccount,
    #[serde(rename = "child-reinstall")]
    ChildReinstall,
    #[serde(rename = "household-transfer")]
    HouseholdTransfer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryState {
    #[serde(rename = "pending-identity-proof")]
    PendingIdentityProof,
    #[serde(rename = "owner-approval-required")]
    OwnerApprovalRequired,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "revoked")]
    Revoked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryIdentityProofState {
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoverySupportChannel {
    #[serde(rename = "self-serve")]
    SelfServe,
    #[serde(rename = "household-owner-assisted")]
    HouseholdOwnerAssisted,
    #[serde(rename = "support-assisted")]
    SupportAssisted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryDecisionState {
    #[serde(rename = "authorized")]
    Authorized,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryChildEvidenceAccessState {
    #[serde(rename = "allowed")]
    Allowed,
    #[serde(rename = "blocked")]
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryDataCustodyHandoffState {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "export-delete-handoff-required")]
    ExportDeleteHandoffRequired,
    #[serde(rename = "household-transfer-handoff-required")]
    HouseholdTransferHandoffRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryFailureReason {
    #[serde(rename = "recovery-not-active")]
    RecoveryNotActive,
    #[serde(rename = "wrong-household")]
    WrongHousehold,
    #[serde(rename = "identity-proof-required")]
    IdentityProofRequired,
    #[serde(rename = "role-not-authorized")]
    RoleNotAuthorized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryOperation {
    pub requester_role: FamilyActorRole,
    pub household_membership: HouseholdMembership,
    pub kind: RecoveryKind,
    pub state: RecoveryState,
    pub owner_approval_required: bool,
    pub identity_proof_state: RecoveryIdentityProofState,
    pub support_channel: RecoverySupportChannel,
    pub delete_export_handoff_required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryDecision {
    pub decision_state: RecoveryDecisionState,
    pub owner_approval_required: bool,
    pub audit_requirement_state: AuditRequirementState,
    pub child_evidence_access_state: RecoveryChildEvidenceAccessState,
    pub data_custody_handoff_state: RecoveryDataCustodyHandoffState,
    pub failure_reason: Option<RecoveryFailureReason>,
}

pub fn authorize_setup_invite(input: SetupInviteInput) -> SetupInviteDecision {
    if input.invite_state != SetupInviteState::Pending {
        return rejected_setup_invite(SetupInviteFailureReason::InviteNotActive);
    }

    if !input.single_use {
        return rejected_setup_invite(SetupInviteFailureReason::InviteNotSingleUse);
    }

    if input.replay_state != SetupInviteReplayState::Fresh {
        return rejected_setup_invite(SetupInviteFailureReason::InviteReplayRejected);
    }

    if input.household_membership != HouseholdMembership::Member {
        return rejected_setup_invite(SetupInviteFailureReason::WrongHousehold);
    }

    if !purpose_matches_target_role(input.purpose, input.target_role) {
        return rejected_setup_invite(SetupInviteFailureReason::WrongTargetRole);
    }

    if !inviter_can_issue(input.inviter_role, input.purpose) {
        return rejected_setup_invite(SetupInviteFailureReason::InviterNotAuthorized);
    }

    SetupInviteDecision {
        decision_state: SetupInviteDecisionState::Acceptable,
        audit_requirement_state: AuditRequirementState::Required,
        failure_reason: None,
    }
}

pub fn evaluate_recovery_operation(input: RecoveryOperation) -> RecoveryDecision {
    let owner_approval_required = input.owner_approval_required
        || matches!(
            input.kind,
            RecoveryKind::LostParentDevice
                | RecoveryKind::CompromisedAccount
                | RecoveryKind::HouseholdTransfer
        );
    let child_evidence_access_state = child_evidence_access_state(input);
    let data_custody_handoff_state = data_custody_handoff_state(input);

    if input.state == RecoveryState::Revoked {
        return rejected_recovery(
            RecoveryFailureReason::RecoveryNotActive,
            owner_approval_required,
            child_evidence_access_state,
            data_custody_handoff_state,
        );
    }

    if input.requester_role != FamilyActorRole::SupportAdmin
        && input.household_membership != HouseholdMembership::Member
    {
        return rejected_recovery(
            RecoveryFailureReason::WrongHousehold,
            owner_approval_required,
            child_evidence_access_state,
            data_custody_handoff_state,
        );
    }

    if input.identity_proof_state != RecoveryIdentityProofState::Verified {
        return rejected_recovery(
            RecoveryFailureReason::IdentityProofRequired,
            owner_approval_required,
            child_evidence_access_state,
            data_custody_handoff_state,
        );
    }

    if !requester_can_recover(input.requester_role, input.kind, input.support_channel) {
        return rejected_recovery(
            RecoveryFailureReason::RoleNotAuthorized,
            owner_approval_required,
            child_evidence_access_state,
            data_custody_handoff_state,
        );
    }

    RecoveryDecision {
        decision_state: RecoveryDecisionState::Authorized,
        owner_approval_required,
        audit_requirement_state: AuditRequirementState::Required,
        child_evidence_access_state,
        data_custody_handoff_state,
        failure_reason: None,
    }
}

fn rejected_setup_invite(failure_reason: SetupInviteFailureReason) -> SetupInviteDecision {
    SetupInviteDecision {
        decision_state: SetupInviteDecisionState::Rejected,
        audit_requirement_state: AuditRequirementState::Required,
        failure_reason: Some(failure_reason),
    }
}

fn rejected_recovery(
    failure_reason: RecoveryFailureReason,
    owner_approval_required: bool,
    child_evidence_access_state: RecoveryChildEvidenceAccessState,
    data_custody_handoff_state: RecoveryDataCustodyHandoffState,
) -> RecoveryDecision {
    RecoveryDecision {
        decision_state: RecoveryDecisionState::Rejected,
        owner_approval_required,
        audit_requirement_state: AuditRequirementState::Required,
        child_evidence_access_state,
        data_custody_handoff_state,
        failure_reason: Some(failure_reason),
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

fn inviter_can_issue(role: FamilyActorRole, purpose: SetupInvitePurpose) -> bool {
    match purpose {
        SetupInvitePurpose::CoParentInvite
        | SetupInvitePurpose::ObserverInvite
        | SetupInvitePurpose::ChildDevicePairing => {
            matches!(role, FamilyActorRole::Parent | FamilyActorRole::Guardian)
        }
        SetupInvitePurpose::HouseholdTransfer => matches!(role, FamilyActorRole::Parent),
    }
}

fn requester_can_recover(
    role: FamilyActorRole,
    kind: RecoveryKind,
    support_channel: RecoverySupportChannel,
) -> bool {
    if role == FamilyActorRole::SupportAdmin {
        return support_channel == RecoverySupportChannel::SupportAssisted;
    }

    match kind {
        RecoveryKind::HouseholdTransfer => role == FamilyActorRole::Parent,
        RecoveryKind::ForgotLogin
        | RecoveryKind::LostParentDevice
        | RecoveryKind::CompromisedAccount
        | RecoveryKind::ChildReinstall => {
            matches!(role, FamilyActorRole::Parent | FamilyActorRole::Guardian)
        }
    }
}

fn child_evidence_access_state(input: RecoveryOperation) -> RecoveryChildEvidenceAccessState {
    let has_household_authority = input.household_membership == HouseholdMembership::Member
        && matches!(
            input.requester_role,
            FamilyActorRole::Parent | FamilyActorRole::Guardian
        );

    if has_household_authority && input.support_channel != RecoverySupportChannel::SupportAssisted {
        RecoveryChildEvidenceAccessState::Allowed
    } else {
        RecoveryChildEvidenceAccessState::Blocked
    }
}

fn data_custody_handoff_state(input: RecoveryOperation) -> RecoveryDataCustodyHandoffState {
    if input.kind == RecoveryKind::HouseholdTransfer {
        return RecoveryDataCustodyHandoffState::HouseholdTransferHandoffRequired;
    }

    if input.delete_export_handoff_required {
        return RecoveryDataCustodyHandoffState::ExportDeleteHandoffRequired;
    }

    RecoveryDataCustodyHandoffState::None
}
