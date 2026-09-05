use super::{SessionAuditAction, StoredLabel};
use crate::session_lifecycle::SessionActivityState;

pub(super) fn action_for_state(value: SessionActivityState) -> SessionAuditAction {
    match value {
        SessionActivityState::Active => SessionAuditAction::Created,
        SessionActivityState::LoggedOut => SessionAuditAction::LoggedOut,
        SessionActivityState::Revoked => SessionAuditAction::Revoked,
        SessionActivityState::GloballyRevoked => SessionAuditAction::GloballyRevoked,
    }
}

pub(super) fn audit(value: SessionAuditAction) -> StoredLabel {
    match value {
        SessionAuditAction::Created => StoredLabel("created"),
        SessionAuditAction::Rotated => StoredLabel("rotated"),
        SessionAuditAction::LoggedOut => StoredLabel("logged-out"),
        SessionAuditAction::Revoked => StoredLabel("revoked"),
        SessionAuditAction::GloballyRevoked => StoredLabel("globally-revoked"),
    }
}
