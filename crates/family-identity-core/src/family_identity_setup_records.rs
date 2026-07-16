#![forbid(unsafe_code)]

use crate::family_identity::{
    DeviceId, HouseholdId, ParentMemberId, RecoveryId, RecoveryState, SetupAuditActionId,
    SetupAuditEvent, SetupAuditEventId, SetupAuditEvidenceRef, SetupAuditTargetId, SetupInvite,
    SetupInviteId,
};
use crate::family_identity_contract_text::required_contract_text;
use crate::setup_lifecycle::{RecoveryKind, SetupInviteTargetRole};
use ocentra_eventing::error::EventingError;

impl SetupInvite {
    pub fn new(
        invite_id: SetupInviteId,
        household_id: HouseholdId,
        invitee_email: impl Into<String>,
        role: SetupInviteTargetRole,
        expires_at: impl Into<String>,
    ) -> Result<Self, EventingError> {
        let invitee_email =
            required_contract_text("family_identity.setup_invite.invitee_email", invitee_email)?;
        if !invitee_email.contains('@') {
            return Err(EventingError::InvalidValue {
                field: "family_identity.setup_invite.invitee_email",
                value: invitee_email,
            });
        }

        Ok(Self {
            invite_id,
            household_id,
            invitee_email,
            role,
            expires_at: required_contract_text(
                "family_identity.setup_invite.expires_at",
                expires_at,
            )?,
        })
    }
}

impl RecoveryState {
    pub fn new(
        recovery_id: RecoveryId,
        device_id: DeviceId,
        reason: RecoveryKind,
        parent_action_required: bool,
    ) -> Result<Self, EventingError> {
        if recovery_kind_requires_parent_action(reason) && !parent_action_required {
            return Err(EventingError::InvalidValue {
                field: "family_identity.recovery_record.parent_action_required",
                value: String::from("false"),
            });
        }

        Ok(Self {
            recovery_id,
            device_id,
            reason,
            parent_action_required,
        })
    }
}

impl SetupAuditEvent {
    pub fn new(
        event_id: SetupAuditEventId,
        household_id: HouseholdId,
        actor_member_id: ParentMemberId,
        target_id: SetupAuditTargetId,
        action: SetupAuditActionId,
        timestamp: impl Into<String>,
        evidence_ref: Option<SetupAuditEvidenceRef>,
    ) -> Result<Self, EventingError> {
        Ok(Self {
            event_id,
            household_id,
            actor_member_id,
            target_id,
            action,
            timestamp: required_contract_text(
                "family_identity.setup_audit_event.timestamp",
                timestamp,
            )?,
            evidence_ref,
        })
    }
}

fn recovery_kind_requires_parent_action(reason: RecoveryKind) -> bool {
    matches!(
        reason,
        RecoveryKind::LostParentDevice
            | RecoveryKind::CompromisedAccount
            | RecoveryKind::HouseholdTransfer
    )
}
