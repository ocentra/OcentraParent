use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::app_game_session::*;
use ocentra_network_evidence::dns::types::*;

#[test]
fn app_game_session_correlation_confirms_foreground_session_from_stored_refs() {
    let correlation = correlate_app_game_foreground_session(input(session(
        NetworkAppGameEvidenceKind::KnownGame,
        NetworkAppGameForegroundState::KnownForeground,
    )))
    .expect_value("stored foreground session should confirm app/game correlation");

    assert_eq!(
        correlation.state,
        NetworkAppGameSessionCorrelationState::ForegroundSessionConfirmed
    );
    assert_eq!(
        correlation.basis,
        NetworkAppGameSessionCorrelationBasis::StoredForegroundEvidence
    );
    assert_eq!(correlation.evidence_grade, NetworkEvidenceGrade::C);
    assert_eq!(correlation.foreground_time_seconds, 900);
    assert_eq!(
        correlation.evidence_refs,
        vec![
            "network-flow-1",
            "app-game-evidence-1",
            "session-summary-1",
            "foreground-evidence-1",
            "process-correlation-1",
            "launcher-ref-1",
        ]
    );
    assert_no_authority_or_content_claims(&correlation);
}

#[test]
fn app_game_session_correlation_confirms_running_session_without_foreground() {
    let correlation = correlate_app_game_foreground_session(input(session(
        NetworkAppGameEvidenceKind::KnownApp,
        NetworkAppGameForegroundState::KnownBackground,
    )))
    .expect_value("stored running session should confirm app/game correlation");

    assert_eq!(
        correlation.state,
        NetworkAppGameSessionCorrelationState::RunningSessionConfirmed
    );
    assert_eq!(
        correlation.basis,
        NetworkAppGameSessionCorrelationBasis::StoredSessionSummary
    );
    assert_eq!(correlation.evidence_grade, NetworkEvidenceGrade::C);
    assert!(!correlation.launcher_only_guarded);
    assert_no_authority_or_content_claims(&correlation);
}

#[test]
fn app_game_session_correlation_guards_launcher_only_evidence() {
    let mut launcher = session(
        NetworkAppGameEvidenceKind::LauncherOnly,
        NetworkAppGameForegroundState::KnownForeground,
    );
    launcher.display_name = Some("Steam".to_owned());

    let correlation = correlate_app_game_foreground_session(input(launcher))
        .expect_value("launcher-only rows should be guarded");

    assert_eq!(
        correlation.state,
        NetworkAppGameSessionCorrelationState::LauncherOnlyGuarded
    );
    assert_eq!(
        correlation.basis,
        NetworkAppGameSessionCorrelationBasis::LauncherOnlyEvidence
    );
    assert!(correlation.launcher_only_guarded);
    assert_eq!(correlation.evidence_grade, NetworkEvidenceGrade::D);
    assert_no_authority_or_content_claims(&correlation);
}

#[test]
fn app_game_session_correlation_keeps_candidates_and_missing_evidence_non_authoritative() {
    let candidate = correlate_app_game_foreground_session(input(session(
        NetworkAppGameEvidenceKind::AppGameCandidate,
        NetworkAppGameForegroundState::Unknown,
    )))
    .expect_value("candidate should remain review-only");
    assert_eq!(
        candidate.state,
        NetworkAppGameSessionCorrelationState::CandidateNeedsReview
    );
    assert_eq!(
        candidate.basis,
        NetworkAppGameSessionCorrelationBasis::CandidateStoredEvidence
    );
    assert_no_authority_or_content_claims(&candidate);

    let missing = correlate_app_game_foreground_session(NetworkAppGameSessionCorrelationInput {
        network_flow_ref: "network-flow-1".to_owned(),
        stored_session: None,
    })
    .expect_value("missing session should be explicit");
    assert_eq!(
        missing.state,
        NetworkAppGameSessionCorrelationState::NoSessionEvidence
    );
    assert_eq!(missing.evidence_refs, vec!["network-flow-1"]);
    assert_no_authority_or_content_claims(&missing);
}

#[test]
fn app_game_session_correlation_marks_adapter_unavailable() {
    let mut unavailable = session(
        NetworkAppGameEvidenceKind::KnownGame,
        NetworkAppGameForegroundState::KnownForeground,
    );
    unavailable.adapter_available = false;

    let correlation = correlate_app_game_foreground_session(input(unavailable))
        .expect_value("unavailable adapter should be explicit");

    assert_eq!(
        correlation.state,
        NetworkAppGameSessionCorrelationState::AdapterUnavailable
    );
    assert_eq!(
        correlation.basis,
        NetworkAppGameSessionCorrelationBasis::AdapterUnavailable
    );
    assert_eq!(correlation.evidence_grade, NetworkEvidenceGrade::D);
    assert_no_authority_or_content_claims(&correlation);
}

#[test]
fn app_game_session_correlation_rejects_content_ai_authority_and_bad_duration() {
    let mut exact = session(
        NetworkAppGameEvidenceKind::KnownGame,
        NetworkAppGameForegroundState::KnownForeground,
    );
    exact.exact_url_claimed = true;
    assert_eq!(
        correlate_app_game_foreground_session(input(exact)),
        Err(NetworkAppGameSessionCorrelationError::UnsupportedExactUrlClaim)
    );

    let mut screen = session(
        NetworkAppGameEvidenceKind::KnownGame,
        NetworkAppGameForegroundState::KnownForeground,
    );
    screen.screen_content_claimed = true;
    assert_eq!(
        correlate_app_game_foreground_session(input(screen)),
        Err(NetworkAppGameSessionCorrelationError::UnsupportedScreenContentClaim)
    );

    let mut ai = session(
        NetworkAppGameEvidenceKind::KnownGame,
        NetworkAppGameForegroundState::KnownForeground,
    );
    ai.ai_scanned_device = true;
    assert_eq!(
        correlate_app_game_foreground_session(input(ai)),
        Err(NetworkAppGameSessionCorrelationError::UnsupportedAiScannerClaim)
    );

    let mut authority = session(
        NetworkAppGameEvidenceKind::KnownGame,
        NetworkAppGameForegroundState::KnownForeground,
    );
    authority.enforcement_command_published = true;
    assert_eq!(
        correlate_app_game_foreground_session(input(authority)),
        Err(NetworkAppGameSessionCorrelationError::UnsupportedEnforcementCommandClaim)
    );

    let mut duration = session(
        NetworkAppGameEvidenceKind::KnownGame,
        NetworkAppGameForegroundState::KnownForeground,
    );
    duration.foreground_time_seconds = duration.running_time_seconds + 1;
    assert_eq!(
        correlate_app_game_foreground_session(input(duration)),
        Err(NetworkAppGameSessionCorrelationError::ForegroundTimeExceedsRunningTime)
    );
}

#[test]
fn app_game_session_correlation_rejects_empty_refs_and_invalid_confidence() {
    let result = correlate_app_game_foreground_session(NetworkAppGameSessionCorrelationInput {
        network_flow_ref: " ".to_owned(),
        stored_session: None,
    });
    assert_eq!(
        result,
        Err(NetworkAppGameSessionCorrelationError::EmptyNetworkFlowRef)
    );

    let mut evidence = session(
        NetworkAppGameEvidenceKind::KnownGame,
        NetworkAppGameForegroundState::KnownForeground,
    );
    evidence.session_summary_ref = Some(" ".to_owned());
    assert_eq!(
        correlate_app_game_foreground_session(input(evidence)),
        Err(NetworkAppGameSessionCorrelationError::EmptySessionSummaryRef)
    );

    let mut confidence = session(
        NetworkAppGameEvidenceKind::KnownGame,
        NetworkAppGameForegroundState::KnownForeground,
    );
    confidence.confidence = Some(101);
    assert_eq!(
        correlate_app_game_foreground_session(input(confidence)),
        Err(NetworkAppGameSessionCorrelationError::InvalidConfidence(
            101
        ))
    );
}

fn input(
    stored_session: NetworkAppGameStoredSessionEvidence,
) -> NetworkAppGameSessionCorrelationInput {
    NetworkAppGameSessionCorrelationInput {
        network_flow_ref: "network-flow-1".to_owned(),
        stored_session: Some(stored_session),
    }
}

fn session(
    evidence_kind: NetworkAppGameEvidenceKind,
    foreground_state: NetworkAppGameForegroundState,
) -> NetworkAppGameStoredSessionEvidence {
    NetworkAppGameStoredSessionEvidence {
        evidence_ref: "app-game-evidence-1".to_owned(),
        session_summary_ref: Some("session-summary-1".to_owned()),
        foreground_evidence_ref: Some("foreground-evidence-1".to_owned()),
        process_correlation_ref: Some("process-correlation-1".to_owned()),
        app_id: Some("game.client".to_owned()),
        display_name: Some("Game Client".to_owned()),
        process_name: Some("gameclient.exe".to_owned()),
        launcher_ref: Some("launcher-ref-1".to_owned()),
        evidence_kind,
        foreground_state,
        running_time_seconds: 1_200,
        foreground_time_seconds: 900,
        run_count: 2,
        adapter_available: true,
        confidence: Some(91),
        exact_url_claimed: false,
        screen_content_claimed: false,
        ai_scanned_device: false,
        policy_action_authority: false,
        adapter_action_authorized: false,
        enforcement_command_published: false,
    }
}

fn assert_no_authority_or_content_claims(correlation: &NetworkAppGameSessionCorrelation) {
    assert!(!correlation.exact_url_available);
    assert!(!correlation.screen_content_available);
    assert!(!correlation.ai_scanned_device);
    assert!(!correlation.policy_action_authority);
    assert!(!correlation.adapter_action_authorized);
    assert!(!correlation.enforcement_command_published);
}
