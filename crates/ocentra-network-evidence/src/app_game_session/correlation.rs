use crate::dns::types::NetworkEvidenceGrade;

use super::{
    NetworkAppGameEvidenceKind, NetworkAppGameForegroundState, NetworkAppGameSessionCorrelation,
    NetworkAppGameSessionCorrelationBasis, NetworkAppGameSessionCorrelationState,
    NetworkAppGameStoredSessionEvidence,
};

pub(super) fn adapter_unavailable_correlation(
    network_flow_ref: &str,
    session: NetworkAppGameStoredSessionEvidence,
) -> NetworkAppGameSessionCorrelation {
    session_correlation(
        network_flow_ref,
        session,
        NetworkAppGameSessionCorrelationState::AdapterUnavailable,
        NetworkAppGameSessionCorrelationBasis::AdapterUnavailable,
        false,
        NetworkEvidenceGrade::D,
    )
}

pub(super) fn missing_session_correlation(
    network_flow_ref: &str,
) -> NetworkAppGameSessionCorrelation {
    NetworkAppGameSessionCorrelation {
        state: NetworkAppGameSessionCorrelationState::NoSessionEvidence,
        basis: NetworkAppGameSessionCorrelationBasis::MissingStoredEvidence,
        app_id: None,
        display_name: None,
        process_name: None,
        launcher_ref: None,
        session_summary_ref: None,
        foreground_evidence_ref: None,
        running_time_seconds: 0,
        foreground_time_seconds: 0,
        run_count: 0,
        confidence: None,
        launcher_only_guarded: false,
        exact_url_available: false,
        screen_content_available: false,
        ai_scanned_device: false,
        policy_action_authority: false,
        adapter_action_authorized: false,
        enforcement_command_published: false,
        evidence_refs: vec![network_flow_ref.to_owned()],
        evidence_grade: NetworkEvidenceGrade::D,
    }
}

pub(super) fn correlate_session(
    network_flow_ref: &str,
    session: NetworkAppGameStoredSessionEvidence,
) -> NetworkAppGameSessionCorrelation {
    match session.evidence_kind {
        NetworkAppGameEvidenceKind::KnownGame | NetworkAppGameEvidenceKind::KnownApp
            if session.foreground_state == NetworkAppGameForegroundState::KnownForeground =>
        {
            session_correlation(
                network_flow_ref,
                session,
                NetworkAppGameSessionCorrelationState::ForegroundSessionConfirmed,
                NetworkAppGameSessionCorrelationBasis::StoredForegroundEvidence,
                false,
                NetworkEvidenceGrade::C,
            )
        }
        NetworkAppGameEvidenceKind::KnownGame | NetworkAppGameEvidenceKind::KnownApp => {
            session_correlation(
                network_flow_ref,
                session,
                NetworkAppGameSessionCorrelationState::RunningSessionConfirmed,
                NetworkAppGameSessionCorrelationBasis::StoredSessionSummary,
                false,
                NetworkEvidenceGrade::C,
            )
        }
        NetworkAppGameEvidenceKind::AppGameCandidate => session_correlation(
            network_flow_ref,
            session,
            NetworkAppGameSessionCorrelationState::CandidateNeedsReview,
            NetworkAppGameSessionCorrelationBasis::CandidateStoredEvidence,
            false,
            NetworkEvidenceGrade::D,
        ),
        NetworkAppGameEvidenceKind::UnknownProcess => session_correlation(
            network_flow_ref,
            session,
            NetworkAppGameSessionCorrelationState::NoSessionEvidence,
            NetworkAppGameSessionCorrelationBasis::MissingStoredEvidence,
            false,
            NetworkEvidenceGrade::D,
        ),
        NetworkAppGameEvidenceKind::LauncherOnly => session_correlation(
            network_flow_ref,
            session,
            NetworkAppGameSessionCorrelationState::LauncherOnlyGuarded,
            NetworkAppGameSessionCorrelationBasis::LauncherOnlyEvidence,
            true,
            NetworkEvidenceGrade::D,
        ),
    }
}

fn session_correlation(
    network_flow_ref: &str,
    session: NetworkAppGameStoredSessionEvidence,
    state: NetworkAppGameSessionCorrelationState,
    basis: NetworkAppGameSessionCorrelationBasis,
    launcher_only_guarded: bool,
    evidence_grade: NetworkEvidenceGrade,
) -> NetworkAppGameSessionCorrelation {
    let evidence_refs = evidence_refs(network_flow_ref, &session);
    NetworkAppGameSessionCorrelation {
        state,
        basis,
        app_id: trim_owned(session.app_id),
        display_name: trim_owned(session.display_name),
        process_name: trim_owned(session.process_name),
        launcher_ref: trim_owned(session.launcher_ref),
        session_summary_ref: trim_owned(session.session_summary_ref),
        foreground_evidence_ref: trim_owned(session.foreground_evidence_ref),
        running_time_seconds: session.running_time_seconds,
        foreground_time_seconds: session.foreground_time_seconds,
        run_count: session.run_count,
        confidence: session.confidence,
        launcher_only_guarded,
        exact_url_available: false,
        screen_content_available: false,
        ai_scanned_device: false,
        policy_action_authority: false,
        adapter_action_authorized: false,
        enforcement_command_published: false,
        evidence_refs,
        evidence_grade,
    }
}

fn evidence_refs(
    network_flow_ref: &str,
    session: &NetworkAppGameStoredSessionEvidence,
) -> Vec<String> {
    let mut refs = vec![network_flow_ref.to_owned(), session.evidence_ref.clone()];
    push_optional_ref(&mut refs, session.session_summary_ref.as_ref());
    push_optional_ref(&mut refs, session.foreground_evidence_ref.as_ref());
    push_optional_ref(&mut refs, session.process_correlation_ref.as_ref());
    push_optional_ref(&mut refs, session.launcher_ref.as_ref());
    refs
}

fn push_optional_ref(refs: &mut Vec<String>, value: Option<&String>) {
    if let Some(value) = value.map(String::as_str).and_then(trim_ref) {
        refs.push(value.to_owned());
    }
}

fn trim_ref(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then_some(trimmed)
}

fn trim_owned(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}
