use serde::{Deserialize, Serialize};

use crate::family_identity::{DeviceTrustState, HouseholdRole};
use crate::household_authority::AuditRequirementState;

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
pub enum SetupRecoveryAbuseState {
    #[serde(rename = "within-limit")]
    WithinLimit,
    #[serde(rename = "throttled")]
    Throttled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SetupRecoveryResponseTimingState {
    #[serde(rename = "uniform")]
    Uniform,
    #[serde(rename = "variable")]
    Variable,
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
    pub inviter_role: HouseholdRole,
    pub same_family: bool,
    pub purpose: SetupInvitePurpose,
    pub target_role: SetupInviteTargetRole,
    pub invite_state: SetupInviteState,
    pub single_use: bool,
    pub replay_state: SetupInviteReplayState,
    pub abuse_state: SetupRecoveryAbuseState,
    pub response_timing_state: SetupRecoveryResponseTimingState,
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
    pub requester_role: HouseholdRole,
    pub same_family: bool,
    pub kind: RecoveryKind,
    pub state: RecoveryState,
    pub owner_approval_required: bool,
    pub identity_proof_state: RecoveryIdentityProofState,
    pub support_channel: RecoverySupportChannel,
    pub delete_export_handoff_required: bool,
    pub abuse_state: SetupRecoveryAbuseState,
    pub response_timing_state: SetupRecoveryResponseTimingState,
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
    crate::setup_lifecycle_validation::setup_invite_failure_reason(&input)
        .map_or_else(acceptable_setup_invite, rejected_setup_invite)
}

pub fn evaluate_recovery_operation(input: RecoveryOperation) -> RecoveryDecision {
    let owner_approval_required = input.owner_approval_required
        || matches!(
            input.kind,
            RecoveryKind::LostParentDevice
                | RecoveryKind::CompromisedAccount
                | RecoveryKind::HouseholdTransfer
        );
    let child_evidence_access_state =
        crate::setup_lifecycle_validation::child_evidence_access_state(input);
    let data_custody_handoff_state =
        crate::setup_lifecycle_validation::data_custody_handoff_state(input);
    crate::setup_lifecycle_validation::recovery_failure_reason(&input).map_or_else(
        || RecoveryDecision {
            decision_state: RecoveryDecisionState::Authorized,
            owner_approval_required,
            audit_requirement_state: AuditRequirementState::Required,
            child_evidence_access_state,
            data_custody_handoff_state,
            failure_reason: None,
        },
        |failure_reason| RecoveryDecision {
            decision_state: RecoveryDecisionState::Rejected,
            owner_approval_required,
            audit_requirement_state: AuditRequirementState::Required,
            child_evidence_access_state,
            data_custody_handoff_state,
            failure_reason: Some(failure_reason),
        },
    )
}

pub fn device_trust_state_for_recovery_state(state: RecoveryState) -> DeviceTrustState {
    match state {
        RecoveryState::PendingIdentityProof
        | RecoveryState::OwnerApprovalRequired
        | RecoveryState::Approved => DeviceTrustState::ResetRequired,
        RecoveryState::Completed => DeviceTrustState::Pending,
        RecoveryState::Revoked => DeviceTrustState::Revoked,
    }
}

pub fn device_trust_state_for_recovery_operation(input: RecoveryOperation) -> DeviceTrustState {
    match (input.state, input.delete_export_handoff_required) {
        (RecoveryState::Revoked, _) => DeviceTrustState::Revoked,
        (RecoveryState::Completed, true) => DeviceTrustState::ResetRequired,
        _ => device_trust_state_for_recovery_state(input.state),
    }
}

fn acceptable_setup_invite() -> SetupInviteDecision {
    SetupInviteDecision {
        decision_state: SetupInviteDecisionState::Acceptable,
        audit_requirement_state: AuditRequirementState::Required,
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
