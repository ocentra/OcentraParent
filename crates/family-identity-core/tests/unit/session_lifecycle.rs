use ocentra_family_identity_core::family_identity::SessionFreshnessState;
use ocentra_family_identity_core::household_authority::AuditRequirementState;
use ocentra_family_identity_core::session_lifecycle::{
    authorize_session_credential_issuance, authorize_session_token_action, SessionActivityState,
    SessionCredentialIssuanceAction, SessionCredentialIssuanceDecision,
    SessionCredentialIssuanceInput, SessionCredentialIssuanceState, SessionCredentialKind,
    SessionLifecycleAction, SessionTokenAuthorizationState, SessionTokenFailureReason,
    SessionTokenInput, TokenAuditRedactionState, TokenReplayState, TokenValidityWindowState,
};

fn active_browser_session(action: SessionLifecycleAction) -> SessionTokenInput {
    SessionTokenInput {
        credential_kind: SessionCredentialKind::BrowserUserSession,
        action,
        activity_state: SessionActivityState::Active,
        replay_state: TokenReplayState::Fresh,
        validity_window_state: TokenValidityWindowState::Valid,
        session_freshness_state: SessionFreshnessState::Fresh,
    }
}

fn issuance_decision(
    issuance_state: SessionCredentialIssuanceState,
    failure_reason: Option<SessionTokenFailureReason>,
) -> SessionCredentialIssuanceDecision {
    SessionCredentialIssuanceDecision {
        issuance_state,
        audit_requirement_state: AuditRequirementState::Required,
        audit_redaction_state: TokenAuditRedactionState::Redacted,
        failure_reason,
    }
}

#[test]
fn expiry_boundary_allows_clock_skew_tolerance_but_rejects_expired_sessions() {
    let within_tolerance = authorize_session_token_action(SessionTokenInput {
        validity_window_state: TokenValidityWindowState::ValidWithinClockSkewTolerance,
        action: SessionLifecycleAction::PerformPrivilegedUserAction,
        ..active_browser_session(SessionLifecycleAction::PerformPrivilegedUserAction)
    });

    assert_eq!(
        within_tolerance.authorization_state,
        SessionTokenAuthorizationState::Authorized
    );
    assert_eq!(
        within_tolerance.audit_requirement_state,
        AuditRequirementState::Required
    );
    assert_eq!(
        within_tolerance.audit_redaction_state,
        TokenAuditRedactionState::Redacted
    );

    let expired = authorize_session_token_action(SessionTokenInput {
        validity_window_state: TokenValidityWindowState::Expired,
        action: SessionLifecycleAction::PerformPrivilegedUserAction,
        ..active_browser_session(SessionLifecycleAction::PerformPrivilegedUserAction)
    });

    assert_eq!(
        expired.authorization_state,
        SessionTokenAuthorizationState::Rejected
    );
    assert_eq!(
        expired.failure_reason,
        Some(SessionTokenFailureReason::TokenExpired)
    );
}

#[test]
fn refresh_rejects_revoked_session_state() {
    let revoked = authorize_session_token_action(SessionTokenInput {
        activity_state: SessionActivityState::Revoked,
        action: SessionLifecycleAction::RefreshBrowserSession,
        ..active_browser_session(SessionLifecycleAction::RefreshBrowserSession)
    });

    assert_eq!(
        revoked.authorization_state,
        SessionTokenAuthorizationState::Rejected
    );
    assert_eq!(
        revoked.failure_reason,
        Some(SessionTokenFailureReason::SessionRevoked)
    );
}

#[test]
fn logout_invalidates_future_privileged_actions() {
    let logout = authorize_session_token_action(SessionTokenInput {
        action: SessionLifecycleAction::LogoutBrowserSession,
        ..active_browser_session(SessionLifecycleAction::LogoutBrowserSession)
    });
    assert_eq!(
        logout.authorization_state,
        SessionTokenAuthorizationState::Authorized
    );

    let after_logout = authorize_session_token_action(SessionTokenInput {
        activity_state: SessionActivityState::LoggedOut,
        action: SessionLifecycleAction::PerformPrivilegedUserAction,
        ..active_browser_session(SessionLifecycleAction::PerformPrivilegedUserAction)
    });
    assert_eq!(
        after_logout.authorization_state,
        SessionTokenAuthorizationState::Rejected
    );
    assert_eq!(
        after_logout.failure_reason,
        Some(SessionTokenFailureReason::SessionLoggedOut)
    );
}

#[test]
fn replayed_pairing_token_is_rejected() {
    let replayed = authorize_session_token_action(SessionTokenInput {
        credential_kind: SessionCredentialKind::PairingToken,
        action: SessionLifecycleAction::AcceptPairingToken,
        replay_state: TokenReplayState::ReplayDetected,
        ..active_browser_session(SessionLifecycleAction::AcceptPairingToken)
    });

    assert_eq!(
        replayed.authorization_state,
        SessionTokenAuthorizationState::Rejected
    );
    assert_eq!(
        replayed.failure_reason,
        Some(SessionTokenFailureReason::TokenReplayRejected)
    );
}

#[test]
fn not_yet_valid_session_rejects_clock_skew_violation() {
    let skewed = authorize_session_token_action(SessionTokenInput {
        validity_window_state: TokenValidityWindowState::NotYetValid,
        action: SessionLifecycleAction::PerformPrivilegedUserAction,
        ..active_browser_session(SessionLifecycleAction::PerformPrivilegedUserAction)
    });

    assert_eq!(
        skewed.authorization_state,
        SessionTokenAuthorizationState::Rejected
    );
    assert_eq!(
        skewed.failure_reason,
        Some(SessionTokenFailureReason::TokenNotYetValid)
    );
}

#[test]
fn device_token_is_not_accepted_as_browser_user_session() {
    let wrong_kind = authorize_session_token_action(SessionTokenInput {
        credential_kind: SessionCredentialKind::DeviceCredential,
        action: SessionLifecycleAction::PerformPrivilegedUserAction,
        ..active_browser_session(SessionLifecycleAction::PerformPrivilegedUserAction)
    });

    assert_eq!(
        wrong_kind.authorization_state,
        SessionTokenAuthorizationState::Rejected
    );
    assert_eq!(
        wrong_kind.failure_reason,
        Some(SessionTokenFailureReason::WrongCredentialKind)
    );
}

#[test]
fn stale_remote_session_grant_requires_fresh_session() {
    let stale_remote_grant = authorize_session_token_action(SessionTokenInput {
        credential_kind: SessionCredentialKind::RemoteSessionGrant,
        action: SessionLifecycleAction::UseRemoteSessionGrant,
        session_freshness_state: SessionFreshnessState::Stale,
        ..active_browser_session(SessionLifecycleAction::UseRemoteSessionGrant)
    });

    assert_eq!(
        stale_remote_grant.authorization_state,
        SessionTokenAuthorizationState::Rejected
    );
    assert_eq!(
        stale_remote_grant.failure_reason,
        Some(SessionTokenFailureReason::SessionNotFresh)
    );
}

#[test]
fn browser_session_create_issuance_is_created_without_source_session() {
    let created = authorize_session_credential_issuance(SessionCredentialIssuanceInput {
        issuance_action: SessionCredentialIssuanceAction::CreateBrowserSession,
        issued_credential_kind: SessionCredentialKind::BrowserUserSession,
        source_session: None,
    });

    assert_eq!(
        created,
        issuance_decision(SessionCredentialIssuanceState::Created, None)
    );
}

#[test]
fn browser_session_rotate_issuance_is_rotated_from_refresh_source() {
    let rotated = authorize_session_credential_issuance(SessionCredentialIssuanceInput {
        issuance_action: SessionCredentialIssuanceAction::RotateBrowserSession,
        issued_credential_kind: SessionCredentialKind::BrowserUserSession,
        source_session: Some(active_browser_session(
            SessionLifecycleAction::RefreshBrowserSession,
        )),
    });

    assert_eq!(
        rotated,
        issuance_decision(SessionCredentialIssuanceState::Rotated, None)
    );
}

#[test]
fn revoked_refresh_source_denies_rotation_issuance() {
    let revoked = authorize_session_credential_issuance(SessionCredentialIssuanceInput {
        issuance_action: SessionCredentialIssuanceAction::RotateBrowserSession,
        issued_credential_kind: SessionCredentialKind::BrowserUserSession,
        source_session: Some(SessionTokenInput {
            activity_state: SessionActivityState::Revoked,
            ..active_browser_session(SessionLifecycleAction::RefreshBrowserSession)
        }),
    });

    assert_eq!(
        revoked,
        issuance_decision(
            SessionCredentialIssuanceState::Rejected,
            Some(SessionTokenFailureReason::SessionRevoked),
        )
    );
}

#[test]
fn privileged_browser_session_can_issue_scoped_credentials() {
    let privileged_source =
        active_browser_session(SessionLifecycleAction::PerformPrivilegedUserAction);

    for (issuance_action, issued_credential_kind) in [
        (
            SessionCredentialIssuanceAction::IssueDeviceCredential,
            SessionCredentialKind::DeviceCredential,
        ),
        (
            SessionCredentialIssuanceAction::IssueInviteToken,
            SessionCredentialKind::InviteToken,
        ),
        (
            SessionCredentialIssuanceAction::IssuePairingToken,
            SessionCredentialKind::PairingToken,
        ),
        (
            SessionCredentialIssuanceAction::IssueRecoveryToken,
            SessionCredentialKind::RecoveryToken,
        ),
        (
            SessionCredentialIssuanceAction::IssueRemoteSessionGrant,
            SessionCredentialKind::RemoteSessionGrant,
        ),
    ] {
        let issued = authorize_session_credential_issuance(SessionCredentialIssuanceInput {
            issuance_action,
            issued_credential_kind,
            source_session: Some(privileged_source),
        });

        assert_eq!(
            issued,
            issuance_decision(SessionCredentialIssuanceState::Issued, None)
        );
    }
}

#[test]
fn wrong_issuance_kind_is_rejected() {
    let wrong_kind = authorize_session_credential_issuance(SessionCredentialIssuanceInput {
        issuance_action: SessionCredentialIssuanceAction::IssueDeviceCredential,
        issued_credential_kind: SessionCredentialKind::InviteToken,
        source_session: Some(active_browser_session(
            SessionLifecycleAction::PerformPrivilegedUserAction,
        )),
    });

    assert_eq!(
        wrong_kind,
        issuance_decision(
            SessionCredentialIssuanceState::Rejected,
            Some(SessionTokenFailureReason::WrongCredentialKind),
        )
    );
}

#[test]
fn non_create_issuance_rejects_missing_source_session() {
    let missing_source = authorize_session_credential_issuance(SessionCredentialIssuanceInput {
        issuance_action: SessionCredentialIssuanceAction::IssueInviteToken,
        issued_credential_kind: SessionCredentialKind::InviteToken,
        source_session: None,
    });

    assert_eq!(
        missing_source,
        issuance_decision(
            SessionCredentialIssuanceState::Rejected,
            Some(SessionTokenFailureReason::SessionLoggedOut),
        )
    );
}
