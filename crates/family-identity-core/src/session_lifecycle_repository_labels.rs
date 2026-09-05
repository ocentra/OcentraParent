#![forbid(unsafe_code)]

use ocentra_schema::account_identity_authority::AccountIdentityProvider;

use super::{SessionAuditAction, SessionLifecycleRepositoryError};
use crate::session_lifecycle::SessionActivityState;
use crate::session_lifecycle_custody::record::SessionCredentialClass;

#[path = "session_lifecycle_repository_labels_activity.rs"]
mod activity_labels;
#[path = "session_lifecycle_repository_labels_audit.rs"]
mod audit_labels;

#[derive(Clone, Copy)]
pub(crate) struct StoredLabel(pub(crate) &'static str);

struct CredentialClassLabel {
    value: SessionCredentialClass,
    bytes: &'static [u8],
}

struct ProviderLabel {
    value: AccountIdentityProvider,
    bytes: &'static [u8],
}

struct ActivityLabel {
    value: SessionActivityState,
    bytes: &'static [u8],
}

struct AuditLabel {
    value: SessionAuditAction,
    bytes: &'static [u8],
}

const CREDENTIAL_CLASS_LABELS: &[CredentialClassLabel] = &[CredentialClassLabel {
    value: SessionCredentialClass::BrowserUserSession,
    bytes: b"browser-user-session",
}];

const PROVIDER_LABELS: &[ProviderLabel] = &[
    ProviderLabel {
        value: AccountIdentityProvider::Authjs,
        bytes: b"authjs",
    },
    ProviderLabel {
        value: AccountIdentityProvider::Firebase,
        bytes: b"firebase",
    },
];

const ACTIVITY_LABELS: &[ActivityLabel] = &[
    ActivityLabel {
        value: SessionActivityState::Active,
        bytes: b"active",
    },
    ActivityLabel {
        value: SessionActivityState::LoggedOut,
        bytes: b"logged-out",
    },
    ActivityLabel {
        value: SessionActivityState::Revoked,
        bytes: b"revoked",
    },
    ActivityLabel {
        value: SessionActivityState::GloballyRevoked,
        bytes: b"globally-revoked",
    },
];

const AUDIT_LABELS: &[AuditLabel] = &[
    AuditLabel {
        value: SessionAuditAction::Created,
        bytes: b"created",
    },
    AuditLabel {
        value: SessionAuditAction::Rotated,
        bytes: b"rotated",
    },
    AuditLabel {
        value: SessionAuditAction::LoggedOut,
        bytes: b"logged-out",
    },
    AuditLabel {
        value: SessionAuditAction::Revoked,
        bytes: b"revoked",
    },
    AuditLabel {
        value: SessionAuditAction::GloballyRevoked,
        bytes: b"globally-revoked",
    },
];

pub(crate) fn credential_class_label(value: SessionCredentialClass) -> StoredLabel {
    activity_labels::credential_class(value)
}

pub(crate) fn parse_credential_class(
    value: &[u8],
) -> Result<SessionCredentialClass, SessionLifecycleRepositoryError> {
    CREDENTIAL_CLASS_LABELS
        .iter()
        .find(|entry| entry.bytes == value)
        .map(|entry| entry.value)
        .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)
}

pub(crate) fn provider_label(value: &AccountIdentityProvider) -> StoredLabel {
    activity_labels::provider(value)
}

pub(crate) fn parse_provider(
    value: &[u8],
) -> Result<AccountIdentityProvider, SessionLifecycleRepositoryError> {
    PROVIDER_LABELS
        .iter()
        .find(|entry| entry.bytes == value)
        .map(|entry| entry.value.clone())
        .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)
}

pub(crate) fn activity_label(value: SessionActivityState) -> StoredLabel {
    activity_labels::activity(value)
}

pub(crate) fn parse_activity_state(
    value: &[u8],
) -> Result<SessionActivityState, SessionLifecycleRepositoryError> {
    ACTIVITY_LABELS
        .iter()
        .find(|entry| entry.bytes == value)
        .map(|entry| entry.value)
        .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)
}

pub(crate) fn audit_action_for_state(value: SessionActivityState) -> SessionAuditAction {
    audit_labels::action_for_state(value)
}

pub(crate) fn audit_label(value: SessionAuditAction) -> StoredLabel {
    audit_labels::audit(value)
}

pub(crate) fn parse_audit_action(
    value: &[u8],
) -> Result<SessionAuditAction, SessionLifecycleRepositoryError> {
    AUDIT_LABELS
        .iter()
        .find(|entry| entry.bytes == value)
        .map(|entry| entry.value)
        .ok_or(SessionLifecycleRepositoryError::InvalidAuditRecord)
}
