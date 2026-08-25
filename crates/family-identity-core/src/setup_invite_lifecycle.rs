#![forbid(unsafe_code)]

//! Durable-adapter-neutral invite lifecycle state.

use crate::family_identity::{HouseholdRole, SetupInvite};
use crate::family_identity_contract_text::required_contract_text;
use crate::setup_lifecycle::{
    authorize_setup_invite, SetupInviteDecision, SetupInviteDecisionState, SetupInviteInput,
    SetupInvitePurpose, SetupInviteReplayState, SetupInviteState, SetupRecoveryAbuseState,
    SetupRecoveryResponseTimingState,
};
use chrono::{DateTime, FixedOffset, Utc};
use ocentra_eventing::error::EventingError;

/// Invite redemption is modeled as a state transition rather than a boolean
/// helper. The durable adapter owns persistence and token custody; this record
/// owns expiry, single-use, and revocation semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupInviteLifecycleRecord {
    pub(crate) invite: SetupInvite,
    pub(crate) purpose: SetupInvitePurpose,
    pub(crate) inviter_role: HouseholdRole,
    pub(crate) issued_at: String,
    pub(crate) accepted_at: Option<String>,
    pub(crate) revoked_at: Option<String>,
    pub(crate) use_count: u32,
}

impl SetupInviteLifecycleRecord {
    pub(crate) fn new(
        invite: SetupInvite,
        purpose: SetupInvitePurpose,
        inviter_role: HouseholdRole,
        issued_at: impl Into<String>,
    ) -> Result<Self, EventingError> {
        let issued_at =
            required_contract_text("family_identity.setup_invite.issued_at", issued_at)?;
        if !expires_after(&issued_at, &invite.expires_at) {
            return Err(EventingError::InvalidValue {
                field: "family_identity.setup_invite.expires_at",
                value: invite.expires_at,
            });
        }
        Ok(Self {
            invite,
            purpose,
            inviter_role,
            issued_at,
            accepted_at: None,
            revoked_at: None,
            use_count: 0,
        })
    }

    pub(crate) fn authorize_at(
        &self,
        same_family: bool,
        abuse_state: SetupRecoveryAbuseState,
        response_timing_state: SetupRecoveryResponseTimingState,
        observed_at: &str,
    ) -> SetupInviteDecision {
        authorize_setup_invite(SetupInviteInput {
            inviter_role: self.inviter_role,
            same_family,
            purpose: self.purpose,
            target_role: self.invite.role,
            invite_state: self.state_at(observed_at),
            single_use: true,
            replay_state: replay_state(self.use_count),
            abuse_state,
            response_timing_state,
        })
    }

    pub(crate) fn accept_at(
        &mut self,
        same_family: bool,
        abuse_state: SetupRecoveryAbuseState,
        response_timing_state: SetupRecoveryResponseTimingState,
        observed_at: impl Into<String>,
    ) -> Result<SetupInviteDecision, EventingError> {
        let observed_at =
            required_contract_text("family_identity.setup_invite.accepted_at", observed_at)?;
        let decision = self.authorize_at(
            same_family,
            abuse_state,
            response_timing_state,
            &observed_at,
        );
        if decision.decision_state == SetupInviteDecisionState::Acceptable {
            self.accepted_at = Some(observed_at);
            self.use_count = self.use_count.saturating_add(1);
        }
        Ok(decision)
    }

    pub(crate) fn revoke_at(&mut self, revoked_at: impl Into<String>) -> Result<(), EventingError> {
        self.revoked_at = Some(required_contract_text(
            "family_identity.setup_invite.revoked_at",
            revoked_at,
        )?);
        Ok(())
    }

    fn state_at(&self, observed_at: &str) -> SetupInviteState {
        if self.revoked_at.is_some() {
            return SetupInviteState::Revoked;
        }
        if self.accepted_at.is_some() || self.use_count > 0 {
            return SetupInviteState::Accepted;
        }
        if !expires_after(&self.issued_at, observed_at)
            || !expires_after(observed_at, &self.invite.expires_at)
        {
            SetupInviteState::Expired
        } else {
            SetupInviteState::Pending
        }
    }
}

fn replay_state(use_count: u32) -> SetupInviteReplayState {
    if use_count == 0 {
        SetupInviteReplayState::Fresh
    } else {
        SetupInviteReplayState::ReplayDetected
    }
}

fn expires_after(start: &str, end: &str) -> bool {
    match (parse_utc(start), parse_utc(end)) {
        (Some(start), Some(end)) => end > start,
        _ => false,
    }
}

fn parse_utc(value: &str) -> Option<DateTime<Utc>> {
    DateTime::<FixedOffset>::parse_from_rfc3339(value)
        .ok()
        .map(|timestamp| timestamp.with_timezone(&Utc))
}
