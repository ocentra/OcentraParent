use serde::{Deserialize, Serialize};

use crate::{FamilyActorRole, HouseholdMembership};

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetupInviteDecision {
    pub decision_state: SetupInviteDecisionState,
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
pub struct RecoveryOperation {
    pub requester_role: FamilyActorRole,
    pub household_membership: HouseholdMembership,
    pub kind: RecoveryKind,
    pub state: RecoveryState,
    pub owner_approval_required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryDecision {
    pub owner_approval_required: bool,
}

pub fn authorize_setup_invite(input: SetupInviteInput) -> SetupInviteDecision {
    if input.invite_state != SetupInviteState::Pending {
        return rejected_setup_invite(SetupInviteFailureReason::InviteNotActive);
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
        failure_reason: None,
    }
}

pub fn evaluate_recovery_operation(input: RecoveryOperation) -> RecoveryDecision {
    RecoveryDecision {
        owner_approval_required: input.owner_approval_required
            || matches!(
                input.kind,
                RecoveryKind::LostParentDevice
                    | RecoveryKind::CompromisedAccount
                    | RecoveryKind::HouseholdTransfer
            ),
    }
}

fn rejected_setup_invite(failure_reason: SetupInviteFailureReason) -> SetupInviteDecision {
    SetupInviteDecision {
        decision_state: SetupInviteDecisionState::Rejected,
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
