#![forbid(unsafe_code)]

//! Durable-adapter-neutral session metadata and lifecycle transitions.

use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

use crate::family_identity::SessionFreshnessState;
use crate::family_identity_account::AccountUserId;
use crate::family_identity_contract_text::required_contract_text;
use crate::session_lifecycle::{
    authorize_session_token_action, SessionActivityState, SessionCredentialKind,
    SessionLifecycleAction, SessionTokenDecision, SessionTokenInput, TokenReplayState,
    TokenValidityWindowState,
};
use ocentra_eventing::error::EventingError;

family_identity_text_id!(SessionId, "family_identity.session_id");

/// The record deliberately contains no bearer secret.  An adapter owns token
/// material and replay storage; this crate owns lifecycle transitions and the
/// fail-closed decision made from the current record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionRecord {
    pub session_id: SessionId,
    pub account_user_id: AccountUserId,
    pub issued_at: String,
    pub expires_at: String,
    pub activity_state: SessionActivityState,
    pub freshness_state: SessionFreshnessState,
    pub last_transition_at: String,
}

impl SessionRecord {
    pub fn new(
        session_id: SessionId,
        account_user_id: AccountUserId,
        issued_at: impl Into<String>,
        expires_at: impl Into<String>,
    ) -> Result<Self, EventingError> {
        let issued_at = required_contract_text("family_identity.session.issued_at", issued_at)?;
        let expires_at = required_contract_text("family_identity.session.expires_at", expires_at)?;
        if !expires_after(&issued_at, &expires_at) {
            return Err(EventingError::InvalidValue {
                field: "family_identity.session.expires_at",
                value: expires_at,
            });
        }

        Ok(Self {
            session_id,
            account_user_id,
            last_transition_at: issued_at.clone(),
            issued_at,
            expires_at,
            activity_state: SessionActivityState::Active,
            freshness_state: SessionFreshnessState::Fresh,
        })
    }

    pub fn authorize(
        &self,
        credential_kind: SessionCredentialKind,
        action: SessionLifecycleAction,
        replay_state: TokenReplayState,
        observed_at: &str,
    ) -> SessionTokenDecision {
        authorize_session_token_action(SessionTokenInput {
            credential_kind,
            action,
            activity_state: self.activity_state,
            replay_state,
            validity_window_state: self.validity_window_state_at(observed_at),
            session_freshness_state: self.freshness_state,
        })
    }

    pub fn validity_window_state_at(&self, observed_at: &str) -> TokenValidityWindowState {
        let Some(observed_at) = parse_utc(observed_at) else {
            return TokenValidityWindowState::Expired;
        };
        let Some(issued_at) = parse_utc(&self.issued_at) else {
            return TokenValidityWindowState::Expired;
        };
        let Some(expires_at) = parse_utc(&self.expires_at) else {
            return TokenValidityWindowState::Expired;
        };
        if observed_at < issued_at {
            TokenValidityWindowState::NotYetValid
        } else if observed_at >= expires_at {
            TokenValidityWindowState::Expired
        } else {
            TokenValidityWindowState::Valid
        }
    }

    pub fn mark_stale(&mut self, transitioned_at: impl Into<String>) -> Result<(), EventingError> {
        self.last_transition_at = transition_time(transitioned_at)?;
        self.freshness_state = SessionFreshnessState::Stale;
        Ok(())
    }

    pub fn logout(&mut self, transitioned_at: impl Into<String>) -> Result<(), EventingError> {
        self.transition_activity(SessionActivityState::LoggedOut, transitioned_at)
    }

    pub fn revoke(&mut self, transitioned_at: impl Into<String>) -> Result<(), EventingError> {
        self.transition_activity(SessionActivityState::Revoked, transitioned_at)
    }

    pub fn globally_revoke(
        &mut self,
        transitioned_at: impl Into<String>,
    ) -> Result<(), EventingError> {
        self.transition_activity(SessionActivityState::GloballyRevoked, transitioned_at)
    }

    fn transition_activity(
        &mut self,
        activity_state: SessionActivityState,
        transitioned_at: impl Into<String>,
    ) -> Result<(), EventingError> {
        self.last_transition_at = transition_time(transitioned_at)?;
        self.activity_state = activity_state;
        self.freshness_state = SessionFreshnessState::Stale;
        Ok(())
    }
}

fn transition_time(value: impl Into<String>) -> Result<String, EventingError> {
    required_contract_text("family_identity.session.last_transition_at", value)
}

fn expires_after(issued_at: &str, expires_at: &str) -> bool {
    match (parse_utc(issued_at), parse_utc(expires_at)) {
        (Some(issued_at), Some(expires_at)) => expires_at > issued_at,
        _ => false,
    }
}

fn parse_utc(value: &str) -> Option<DateTime<Utc>> {
    DateTime::<FixedOffset>::parse_from_rfc3339(value)
        .ok()
        .map(|timestamp| timestamp.with_timezone(&Utc))
}
