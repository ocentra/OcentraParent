#![forbid(unsafe_code)]

use ocentra_schema::account_identity_authority::AccountIdentityProvider;

use super::{SessionAuditAction, SessionLifecycleRepositoryError};
use crate::session_lifecycle::SessionActivityState;
use crate::session_lifecycle_custody::record::SessionCredentialClass;

#[derive(Clone, Copy)]
pub(crate) struct StoredLabel(pub(crate) &'static str);

struct CredentialClassLabel {
    value: SessionCredentialClass,
    bytes: &'static [u8],
    label: StoredLabel,
}

struct ProviderLabel {
    value: AccountIdentityProvider,
    bytes: &'static [u8],
    label: StoredLabel,
}

struct ActivityLabel {
    value: SessionActivityState,
    bytes: &'static [u8],
    label: StoredLabel,
}

struct AuditLabel {
    value: SessionAuditAction,
    bytes: &'static [u8],
    label: StoredLabel,
}

struct ActivityAuditAction {
    activity: SessionActivityState,
    action: SessionAuditAction,
}

const CREDENTIAL_CLASS_LABELS: &[CredentialClassLabel] = &[CredentialClassLabel {
    value: SessionCredentialClass::BrowserUserSession,
    bytes: b"browser-user-session",
    label: StoredLabel("browser-user-session"),
}];

const PROVIDER_LABELS: &[ProviderLabel] = &[
    ProviderLabel {
        value: AccountIdentityProvider::Authjs,
        bytes: b"authjs",
        label: StoredLabel("authjs"),
    },
    ProviderLabel {
        value: AccountIdentityProvider::Firebase,
        bytes: b"firebase",
        label: StoredLabel("firebase"),
    },
];

const ACTIVITY_LABELS: &[ActivityLabel] = &[
    ActivityLabel {
        value: SessionActivityState::Active,
        bytes: b"active",
        label: StoredLabel("active"),
    },
    ActivityLabel {
        value: SessionActivityState::LoggedOut,
        bytes: b"logged-out",
        label: StoredLabel("logged-out"),
    },
    ActivityLabel {
        value: SessionActivityState::Revoked,
        bytes: b"revoked",
        label: StoredLabel("revoked"),
    },
    ActivityLabel {
        value: SessionActivityState::GloballyRevoked,
        bytes: b"globally-revoked",
        label: StoredLabel("globally-revoked"),
    },
];

const AUDIT_LABELS: &[AuditLabel] = &[
    AuditLabel {
        value: SessionAuditAction::Created,
        bytes: b"created",
        label: StoredLabel("created"),
    },
    AuditLabel {
        value: SessionAuditAction::Rotated,
        bytes: b"rotated",
        label: StoredLabel("rotated"),
    },
    AuditLabel {
        value: SessionAuditAction::LoggedOut,
        bytes: b"logged-out",
        label: StoredLabel("logged-out"),
    },
    AuditLabel {
        value: SessionAuditAction::Revoked,
        bytes: b"revoked",
        label: StoredLabel("revoked"),
    },
    AuditLabel {
        value: SessionAuditAction::GloballyRevoked,
        bytes: b"globally-revoked",
        label: StoredLabel("globally-revoked"),
    },
];

const ACTIVITY_AUDIT_ACTIONS: &[ActivityAuditAction] = &[
    ActivityAuditAction {
        activity: SessionActivityState::Active,
        action: SessionAuditAction::Created,
    },
    ActivityAuditAction {
        activity: SessionActivityState::LoggedOut,
        action: SessionAuditAction::LoggedOut,
    },
    ActivityAuditAction {
        activity: SessionActivityState::Revoked,
        action: SessionAuditAction::Revoked,
    },
    ActivityAuditAction {
        activity: SessionActivityState::GloballyRevoked,
        action: SessionAuditAction::GloballyRevoked,
    },
];

pub(crate) fn credential_class_label(value: SessionCredentialClass) -> StoredLabel {
    CREDENTIAL_CLASS_LABELS
        .iter()
        .find(|entry| entry.value == value)
        .map(|entry| entry.label)
        .expect("all session credential classes have labels")
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
    PROVIDER_LABELS
        .iter()
        .find(|entry| &entry.value == value)
        .map(|entry| entry.label)
        .expect("all account identity providers have labels")
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
    ACTIVITY_LABELS
        .iter()
        .find(|entry| entry.value == value)
        .map(|entry| entry.label)
        .expect("all session activity states have labels")
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
    ACTIVITY_AUDIT_ACTIONS
        .iter()
        .find(|entry| entry.activity == value)
        .map(|entry| entry.action)
        .expect("all session activity states have audit actions")
}

pub(crate) fn audit_label(value: SessionAuditAction) -> StoredLabel {
    AUDIT_LABELS
        .iter()
        .find(|entry| entry.value == value)
        .map(|entry| entry.label)
        .expect("all session audit actions have labels")
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
