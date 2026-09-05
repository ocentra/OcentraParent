use ocentra_family_identity_core::family_identity::SessionFreshnessState;
use ocentra_family_identity_core::session_lifecycle::{
    session_token_failure_reason_for_read_model, SessionActivityState, SessionCredentialKind,
    SessionLifecycleAction, SessionTokenFailureReason, SessionTokenInput, TokenReplayState,
    TokenValidityWindowState,
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

#[test]
fn a_fresh_valid_browser_session_has_no_failure_hint() {
    assert_eq!(
        session_token_failure_reason_for_read_model(&active_browser_session(
            SessionLifecycleAction::PerformPrivilegedUserAction,
        )),
        None
    );
}

#[test]
fn expiry_and_not_yet_valid_windows_are_reported() {
    let expired = session_token_failure_reason_for_read_model(&SessionTokenInput {
        validity_window_state: TokenValidityWindowState::Expired,
        ..active_browser_session(SessionLifecycleAction::PerformPrivilegedUserAction)
    });
    assert_eq!(expired, Some(SessionTokenFailureReason::TokenExpired));

    let not_yet_valid = session_token_failure_reason_for_read_model(&SessionTokenInput {
        validity_window_state: TokenValidityWindowState::NotYetValid,
        ..active_browser_session(SessionLifecycleAction::PerformPrivilegedUserAction)
    });
    assert_eq!(
        not_yet_valid,
        Some(SessionTokenFailureReason::TokenNotYetValid)
    );
}

#[test]
fn replay_is_reported_before_other_token_failures() {
    let replayed = session_token_failure_reason_for_read_model(&SessionTokenInput {
        replay_state: TokenReplayState::ReplayDetected,
        validity_window_state: TokenValidityWindowState::Expired,
        ..active_browser_session(SessionLifecycleAction::PerformPrivilegedUserAction)
    });
    assert_eq!(
        replayed,
        Some(SessionTokenFailureReason::TokenReplayRejected)
    );
}

#[test]
fn logged_out_and_revoked_sessions_are_not_current() {
    let logged_out = session_token_failure_reason_for_read_model(&SessionTokenInput {
        activity_state: SessionActivityState::LoggedOut,
        ..active_browser_session(SessionLifecycleAction::PerformPrivilegedUserAction)
    });
    assert_eq!(
        logged_out,
        Some(SessionTokenFailureReason::SessionLoggedOut)
    );

    let revoked = session_token_failure_reason_for_read_model(&SessionTokenInput {
        activity_state: SessionActivityState::Revoked,
        ..active_browser_session(SessionLifecycleAction::PerformPrivilegedUserAction)
    });
    assert_eq!(revoked, Some(SessionTokenFailureReason::SessionRevoked));

    let globally_revoked = session_token_failure_reason_for_read_model(&SessionTokenInput {
        activity_state: SessionActivityState::GloballyRevoked,
        ..active_browser_session(SessionLifecycleAction::PerformPrivilegedUserAction)
    });
    assert_eq!(
        globally_revoked,
        Some(SessionTokenFailureReason::SessionGloballyRevoked)
    );
}

#[test]
fn privileged_and_remote_grant_actions_require_a_fresh_session() {
    let stale_privileged = session_token_failure_reason_for_read_model(&SessionTokenInput {
        session_freshness_state: SessionFreshnessState::Stale,
        ..active_browser_session(SessionLifecycleAction::PerformPrivilegedUserAction)
    });
    assert_eq!(
        stale_privileged,
        Some(SessionTokenFailureReason::SessionNotFresh)
    );

    let stale_remote_grant = session_token_failure_reason_for_read_model(&SessionTokenInput {
        credential_kind: SessionCredentialKind::RemoteSessionGrant,
        action: SessionLifecycleAction::UseRemoteSessionGrant,
        session_freshness_state: SessionFreshnessState::Stale,
        ..active_browser_session(SessionLifecycleAction::UseRemoteSessionGrant)
    });
    assert_eq!(
        stale_remote_grant,
        Some(SessionTokenFailureReason::SessionNotFresh)
    );
}

#[test]
fn credential_kind_must_match_the_action() {
    let wrong_kind = session_token_failure_reason_for_read_model(&SessionTokenInput {
        credential_kind: SessionCredentialKind::DeviceCredential,
        ..active_browser_session(SessionLifecycleAction::PerformPrivilegedUserAction)
    });
    assert_eq!(
        wrong_kind,
        Some(SessionTokenFailureReason::WrongCredentialKind)
    );
}
