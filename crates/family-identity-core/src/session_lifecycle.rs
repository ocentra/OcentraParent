use crate::family_identity::SessionFreshnessState;
use crate::household_authority::AuditRequirementState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionCredentialKind {
    #[serde(rename = "browser-user-session")]
    BrowserUserSession,
    #[serde(rename = "device-credential")]
    DeviceCredential,
    #[serde(rename = "invite-token")]
    InviteToken,
    #[serde(rename = "pairing-token")]
    PairingToken,
    #[serde(rename = "recovery-token")]
    RecoveryToken,
    #[serde(rename = "remote-session-grant")]
    RemoteSessionGrant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionLifecycleAction {
    #[serde(rename = "refresh-browser-session")]
    RefreshBrowserSession,
    #[serde(rename = "perform-privileged-user-action")]
    PerformPrivilegedUserAction,
    #[serde(rename = "logout-browser-session")]
    LogoutBrowserSession,
    #[serde(rename = "use-device-credential")]
    UseDeviceCredential,
    #[serde(rename = "accept-invite-token")]
    AcceptInviteToken,
    #[serde(rename = "accept-pairing-token")]
    AcceptPairingToken,
    #[serde(rename = "redeem-recovery-token")]
    RedeemRecoveryToken,
    #[serde(rename = "use-remote-session-grant")]
    UseRemoteSessionGrant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionCredentialIssuanceAction {
    #[serde(rename = "create-browser-session")]
    CreateBrowserSession,
    #[serde(rename = "rotate-browser-session")]
    RotateBrowserSession,
    #[serde(rename = "issue-device-credential")]
    IssueDeviceCredential,
    #[serde(rename = "issue-invite-token")]
    IssueInviteToken,
    #[serde(rename = "issue-pairing-token")]
    IssuePairingToken,
    #[serde(rename = "issue-recovery-token")]
    IssueRecoveryToken,
    #[serde(rename = "issue-remote-session-grant")]
    IssueRemoteSessionGrant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionActivityState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "logged-out")]
    LoggedOut,
    #[serde(rename = "revoked")]
    Revoked,
    #[serde(rename = "globally-revoked")]
    GloballyRevoked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenReplayState {
    #[serde(rename = "fresh")]
    Fresh,
    #[serde(rename = "replay-detected")]
    ReplayDetected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenValidityWindowState {
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "valid-within-clock-skew-tolerance")]
    ValidWithinClockSkewTolerance,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "not-yet-valid")]
    NotYetValid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionTokenAuthorizationState {
    #[serde(rename = "authorized")]
    Authorized,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionCredentialIssuanceState {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "rotated")]
    Rotated,
    #[serde(rename = "issued")]
    Issued,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenAuditRedactionState {
    #[serde(rename = "redacted")]
    Redacted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionTokenFailureReason {
    #[serde(rename = "token-expired")]
    TokenExpired,
    #[serde(rename = "token-not-yet-valid")]
    TokenNotYetValid,
    #[serde(rename = "token-replay-rejected")]
    TokenReplayRejected,
    #[serde(rename = "session-logged-out")]
    SessionLoggedOut,
    #[serde(rename = "session-revoked")]
    SessionRevoked,
    #[serde(rename = "session-globally-revoked")]
    SessionGloballyRevoked,
    #[serde(rename = "session-not-fresh")]
    SessionNotFresh,
    #[serde(rename = "wrong-credential-kind")]
    WrongCredentialKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionTokenInput {
    pub credential_kind: SessionCredentialKind,
    pub action: SessionLifecycleAction,
    pub activity_state: SessionActivityState,
    pub replay_state: TokenReplayState,
    pub validity_window_state: TokenValidityWindowState,
    pub session_freshness_state: SessionFreshnessState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionCredentialIssuanceInput {
    pub issuance_action: SessionCredentialIssuanceAction,
    pub issued_credential_kind: SessionCredentialKind,
    pub source_session: Option<SessionTokenInput>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionTokenDecision {
    pub authorization_state: SessionTokenAuthorizationState,
    pub audit_requirement_state: AuditRequirementState,
    pub audit_redaction_state: TokenAuditRedactionState,
    pub failure_reason: Option<SessionTokenFailureReason>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionCredentialIssuanceDecision {
    pub issuance_state: SessionCredentialIssuanceState,
    pub audit_requirement_state: AuditRequirementState,
    pub audit_redaction_state: TokenAuditRedactionState,
    pub failure_reason: Option<SessionTokenFailureReason>,
}

pub(crate) fn authorize_session_token_action(input: SessionTokenInput) -> SessionTokenDecision {
    if let Some(failure_reason) = session_token_failure_reason(&input) {
        return rejected(input.action, failure_reason);
    }

    SessionTokenDecision {
        authorization_state: SessionTokenAuthorizationState::Authorized,
        audit_requirement_state: audit_requirement_state(input.action),
        audit_redaction_state: TokenAuditRedactionState::Redacted,
        failure_reason: None,
    }
}

/// Projects untrusted session facts into a failure hint for non-authorizing read models.
///
/// `None` is not proof of an authorized session. Runtime authorization must come
/// from repository-owned credential custody through `SqliteAccountIdentityAuthorityRepository`.
pub fn session_token_failure_reason_for_read_model(
    input: &SessionTokenInput,
) -> Option<SessionTokenFailureReason> {
    session_token_failure_reason(input)
}

fn rejected(
    action: SessionLifecycleAction,
    failure_reason: SessionTokenFailureReason,
) -> SessionTokenDecision {
    SessionTokenDecision {
        authorization_state: SessionTokenAuthorizationState::Rejected,
        audit_requirement_state: audit_requirement_state(action),
        audit_redaction_state: TokenAuditRedactionState::Redacted,
        failure_reason: Some(failure_reason),
    }
}

fn session_token_failure_reason(input: &SessionTokenInput) -> Option<SessionTokenFailureReason> {
    [
        (
            input.replay_state != TokenReplayState::Fresh,
            SessionTokenFailureReason::TokenReplayRejected,
        ),
        (
            matches!(
                input.validity_window_state,
                TokenValidityWindowState::Expired
            ),
            SessionTokenFailureReason::TokenExpired,
        ),
        (
            matches!(
                input.validity_window_state,
                TokenValidityWindowState::NotYetValid
            ),
            SessionTokenFailureReason::TokenNotYetValid,
        ),
        (
            matches!(input.activity_state, SessionActivityState::LoggedOut),
            SessionTokenFailureReason::SessionLoggedOut,
        ),
        (
            matches!(input.activity_state, SessionActivityState::Revoked),
            SessionTokenFailureReason::SessionRevoked,
        ),
        (
            matches!(input.activity_state, SessionActivityState::GloballyRevoked),
            SessionTokenFailureReason::SessionGloballyRevoked,
        ),
        (
            requires_fresh_session(input.action)
                && input.session_freshness_state != SessionFreshnessState::Fresh,
            SessionTokenFailureReason::SessionNotFresh,
        ),
        (
            !credential_kind_matches_action(input.credential_kind, input.action),
            SessionTokenFailureReason::WrongCredentialKind,
        ),
    ]
    .into_iter()
    .find_map(|(failed, reason)| failed.then_some(reason))
}

fn credential_kind_matches_action(
    credential_kind: SessionCredentialKind,
    action: SessionLifecycleAction,
) -> bool {
    matches!(
        (credential_kind, action),
        (
            SessionCredentialKind::BrowserUserSession,
            SessionLifecycleAction::RefreshBrowserSession
                | SessionLifecycleAction::PerformPrivilegedUserAction
                | SessionLifecycleAction::LogoutBrowserSession
        ) | (
            SessionCredentialKind::DeviceCredential,
            SessionLifecycleAction::UseDeviceCredential
        ) | (
            SessionCredentialKind::InviteToken,
            SessionLifecycleAction::AcceptInviteToken
        ) | (
            SessionCredentialKind::PairingToken,
            SessionLifecycleAction::AcceptPairingToken
        ) | (
            SessionCredentialKind::RecoveryToken,
            SessionLifecycleAction::RedeemRecoveryToken
        ) | (
            SessionCredentialKind::RemoteSessionGrant,
            SessionLifecycleAction::UseRemoteSessionGrant
        )
    )
}

fn requires_fresh_session(action: SessionLifecycleAction) -> bool {
    matches!(
        action,
        SessionLifecycleAction::PerformPrivilegedUserAction
            | SessionLifecycleAction::UseRemoteSessionGrant
    )
}

fn audit_requirement_state(action: SessionLifecycleAction) -> AuditRequirementState {
    match action {
        SessionLifecycleAction::RefreshBrowserSession => AuditRequirementState::NotRequired,
        SessionLifecycleAction::PerformPrivilegedUserAction
        | SessionLifecycleAction::LogoutBrowserSession
        | SessionLifecycleAction::UseDeviceCredential
        | SessionLifecycleAction::AcceptInviteToken
        | SessionLifecycleAction::AcceptPairingToken
        | SessionLifecycleAction::RedeemRecoveryToken
        | SessionLifecycleAction::UseRemoteSessionGrant => AuditRequirementState::Required,
    }
}
