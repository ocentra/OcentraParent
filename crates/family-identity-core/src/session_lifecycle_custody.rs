#![forbid(unsafe_code)]

//! Opaque session custody records used by the Account-owned repository.

use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::family_identity::SessionFreshnessState;
use crate::family_identity_account::AccountUserId;
use crate::session_lifecycle::{
    authorize_session_token_action, SessionActivityState, SessionCredentialKind,
    SessionLifecycleAction, SessionTokenDecision, SessionTokenInput, TokenReplayState,
    TokenValidityWindowState,
};
use ocentra_eventing::error::EventingError;

family_identity_text_id!(
    SessionRefreshFamilyId,
    "family_identity.session_refresh_family_id"
);
family_identity_text_id!(SessionTokenDigest, "family_identity.session_token_digest");
family_identity_text_id!(SessionTimestamp, "family_identity.session_timestamp");
family_identity_text_id!(
    SessionAuditEventId,
    "family_identity.session_audit_event_id"
);

impl SessionTokenDigest {
    /// Hash a bearer value at the custody boundary. The raw value is never
    /// stored in a record, repository row, audit event, or proof artifact.
    pub fn from_bearer(bearer: &[u8]) -> Result<Self, EventingError> {
        if bearer.is_empty() {
            return Err(EventingError::EmptyValue {
                field: "family_identity.session.bearer",
            });
        }
        let mut hasher = Sha256::new();
        hasher.update(b"ocentra-account-session-token-v1\0");
        hasher.update(bearer);
        let digest = hasher.finalize();
        Ok(Self::parse(String::from(hex_lower(&digest))).expect("sha256 digest is non-empty"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionCredentialRecord {
    pub session_id: super::session_lifecycle_record::SessionId,
    pub account_user_id: AccountUserId,
    pub token_digest: SessionTokenDigest,
    pub refresh_family_id: SessionRefreshFamilyId,
    pub refresh_generation: u64,
    pub issued_at: SessionTimestamp,
    pub expires_at: SessionTimestamp,
    pub activity_state: SessionActivityState,
    pub freshness_state: SessionFreshnessState,
    pub global_revoke_epoch: u64,
    pub last_transition_at: SessionTimestamp,
}

impl SessionCredentialRecord {
    pub fn new(
        session_id: super::session_lifecycle_record::SessionId,
        account_user_id: AccountUserId,
        token_digest: SessionTokenDigest,
        refresh_family_id: SessionRefreshFamilyId,
        issued_at: SessionTimestamp,
        expires_at: SessionTimestamp,
        global_revoke_epoch: u64,
    ) -> Result<Self, EventingError> {
        if global_revoke_epoch == 0 || !expires_after(&issued_at, &expires_at) {
            return Err(EventingError::InvalidValue {
                field: "family_identity.session.window",
                value: expires_at.to_string(),
            });
        }
        Ok(Self {
            session_id,
            account_user_id,
            token_digest,
            refresh_family_id,
            refresh_generation: 1,
            last_transition_at: issued_at.clone(),
            issued_at,
            expires_at,
            activity_state: SessionActivityState::Active,
            freshness_state: SessionFreshnessState::Fresh,
            global_revoke_epoch,
        })
    }

    pub fn authorize(
        &self,
        credential_kind: SessionCredentialKind,
        action: SessionLifecycleAction,
        replay_state: TokenReplayState,
        observed_at: &SessionTimestamp,
        current_global_revoke_epoch: u64,
    ) -> SessionTokenDecision {
        let activity_state = if current_global_revoke_epoch > self.global_revoke_epoch {
            SessionActivityState::GloballyRevoked
        } else {
            self.activity_state
        };
        authorize_session_token_action(SessionTokenInput {
            credential_kind,
            action,
            activity_state,
            replay_state,
            validity_window_state: self.validity_window_state_at(observed_at),
            session_freshness_state: self.freshness_state,
        })
    }

    pub fn validity_window_state_at(
        &self,
        observed_at: &SessionTimestamp,
    ) -> TokenValidityWindowState {
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

    pub fn rotated(
        &self,
        session_id: super::session_lifecycle_record::SessionId,
        token_digest: SessionTokenDigest,
        issued_at: SessionTimestamp,
        expires_at: SessionTimestamp,
        transitioned_at: SessionTimestamp,
    ) -> Result<Self, EventingError> {
        let mut next = Self::new(
            session_id,
            self.account_user_id.clone(),
            token_digest,
            self.refresh_family_id.clone(),
            issued_at,
            expires_at,
            self.global_revoke_epoch,
        )?;
        next.refresh_generation =
            self.refresh_generation
                .checked_add(1)
                .ok_or(EventingError::InvalidValue {
                    field: "family_identity.session.refresh_generation",
                    value: String::from("overflow"),
                })?;
        next.last_transition_at = transitioned_at;
        Ok(next)
    }
}

fn expires_after(issued_at: &SessionTimestamp, expires_at: &SessionTimestamp) -> bool {
    match (parse_utc(issued_at), parse_utc(expires_at)) {
        (Some(issued_at), Some(expires_at)) => expires_at > issued_at,
        _ => false,
    }
}

fn parse_utc(value: &SessionTimestamp) -> Option<DateTime<Utc>> {
    DateTime::<FixedOffset>::parse_from_rfc3339(value.as_str())
        .ok()
        .map(|timestamp| timestamp.with_timezone(&Utc))
}

struct HexDigest(String);

impl From<HexDigest> for String {
    fn from(value: HexDigest) -> Self {
        value.0
    }
}

fn hex_lower(bytes: &[u8]) -> HexDigest {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    HexDigest(output)
}
