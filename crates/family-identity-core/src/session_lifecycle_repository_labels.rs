#![forbid(unsafe_code)]

use super::{SessionAuditAction, SessionLifecycleRepositoryError};
use crate::family_identity::SessionFreshnessState;
use crate::session_lifecycle::SessionActivityState;

pub(crate) struct StoredLabel(pub(crate) &'static str);

pub(crate) fn audit_action_for_state(value: SessionActivityState) -> SessionAuditAction {
    [
        (
            SessionActivityState::LoggedOut,
            SessionAuditAction::LoggedOut,
        ),
        (SessionActivityState::Revoked, SessionAuditAction::Revoked),
        (
            SessionActivityState::GloballyRevoked,
            SessionAuditAction::GloballyRevoked,
        ),
        (SessionActivityState::Active, SessionAuditAction::Created),
    ]
    .into_iter()
    .find_map(|(candidate, action)| (candidate == value).then_some(action))
    .expect("all session activity states have an audit action")
}

pub(crate) fn activity_label(value: SessionActivityState) -> StoredLabel {
    [
        (SessionActivityState::Active, StoredLabel("active")),
        (SessionActivityState::LoggedOut, StoredLabel("logged-out")),
        (SessionActivityState::Revoked, StoredLabel("revoked")),
        (
            SessionActivityState::GloballyRevoked,
            StoredLabel("globally-revoked"),
        ),
    ]
    .into_iter()
    .find_map(|(candidate, label)| (candidate == value).then_some(label))
    .expect("all session activity states have a storage label")
}

pub(crate) fn freshness_label(value: SessionFreshnessState) -> StoredLabel {
    [
        (SessionFreshnessState::Fresh, StoredLabel("fresh")),
        (SessionFreshnessState::Stale, StoredLabel("stale")),
        (SessionFreshnessState::Expired, StoredLabel("expired")),
    ]
    .into_iter()
    .find_map(|(candidate, label)| (candidate == value).then_some(label))
    .expect("all session freshness states have a storage label")
}

pub(crate) fn parse_activity_state(
    value: &[u8],
) -> Result<SessionActivityState, SessionLifecycleRepositoryError> {
    [
        (b"active" as &[u8], SessionActivityState::Active),
        (b"logged-out", SessionActivityState::LoggedOut),
        (b"revoked", SessionActivityState::Revoked),
        (b"globally-revoked", SessionActivityState::GloballyRevoked),
    ]
    .into_iter()
    .find_map(|(label, state)| (value == label).then_some(state))
    .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)
}

pub(crate) fn parse_freshness_state(
    value: &[u8],
) -> Result<SessionFreshnessState, SessionLifecycleRepositoryError> {
    [
        (b"fresh" as &[u8], SessionFreshnessState::Fresh),
        (b"stale", SessionFreshnessState::Stale),
        (b"expired", SessionFreshnessState::Expired),
    ]
    .into_iter()
    .find_map(|(label, state)| (value == label).then_some(state))
    .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)
}

pub(crate) fn audit_label(value: &SessionAuditAction) -> StoredLabel {
    [
        (SessionAuditAction::Created, StoredLabel("created")),
        (SessionAuditAction::Rotated, StoredLabel("rotated")),
        (SessionAuditAction::LoggedOut, StoredLabel("logged-out")),
        (SessionAuditAction::Revoked, StoredLabel("revoked")),
        (
            SessionAuditAction::GloballyRevoked,
            StoredLabel("globally-revoked"),
        ),
    ]
    .into_iter()
    .find_map(|(candidate, label)| (candidate == value.clone()).then_some(label))
    .expect("all session audit actions have a storage label")
}

pub(crate) fn parse_audit_action(
    value: &[u8],
) -> Result<SessionAuditAction, SessionLifecycleRepositoryError> {
    [
        (b"created" as &[u8], SessionAuditAction::Created),
        (b"rotated", SessionAuditAction::Rotated),
        (b"logged-out", SessionAuditAction::LoggedOut),
        (b"revoked", SessionAuditAction::Revoked),
        (b"globally-revoked", SessionAuditAction::GloballyRevoked),
    ]
    .into_iter()
    .find_map(|(label, action)| (value == label).then_some(action))
    .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)
}
