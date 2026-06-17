use serde::{Deserialize, Serialize};

use crate::NetworkEvidenceGrade;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAppGameEvidenceKind {
    KnownGame,
    KnownApp,
    LauncherOnly,
    AppGameCandidate,
    UnknownProcess,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAppGameForegroundState {
    KnownForeground,
    KnownBackground,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAppGameSessionCorrelationState {
    ForegroundSessionConfirmed,
    RunningSessionConfirmed,
    LauncherOnlyGuarded,
    CandidateNeedsReview,
    NoSessionEvidence,
    AdapterUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAppGameSessionCorrelationBasis {
    StoredForegroundEvidence,
    StoredSessionSummary,
    LauncherOnlyEvidence,
    CandidateStoredEvidence,
    MissingStoredEvidence,
    AdapterUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAppGameStoredSessionEvidence {
    pub evidence_ref: String,
    pub session_summary_ref: Option<String>,
    pub foreground_evidence_ref: Option<String>,
    pub process_correlation_ref: Option<String>,
    pub app_id: Option<String>,
    pub display_name: Option<String>,
    pub process_name: Option<String>,
    pub launcher_ref: Option<String>,
    pub evidence_kind: NetworkAppGameEvidenceKind,
    pub foreground_state: NetworkAppGameForegroundState,
    pub running_time_seconds: u64,
    pub foreground_time_seconds: u64,
    pub run_count: u32,
    pub adapter_available: bool,
    pub confidence: Option<u8>,
    pub exact_url_claimed: bool,
    pub screen_content_claimed: bool,
    pub ai_scanned_device: bool,
    pub policy_action_authority: bool,
    pub adapter_action_authorized: bool,
    pub enforcement_command_published: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAppGameSessionCorrelationInput {
    pub network_flow_ref: String,
    pub stored_session: Option<NetworkAppGameStoredSessionEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAppGameSessionCorrelation {
    pub state: NetworkAppGameSessionCorrelationState,
    pub basis: NetworkAppGameSessionCorrelationBasis,
    pub app_id: Option<String>,
    pub display_name: Option<String>,
    pub process_name: Option<String>,
    pub launcher_ref: Option<String>,
    pub session_summary_ref: Option<String>,
    pub foreground_evidence_ref: Option<String>,
    pub running_time_seconds: u64,
    pub foreground_time_seconds: u64,
    pub run_count: u32,
    pub confidence: Option<u8>,
    pub launcher_only_guarded: bool,
    pub exact_url_available: bool,
    pub screen_content_available: bool,
    pub ai_scanned_device: bool,
    pub policy_action_authority: bool,
    pub adapter_action_authorized: bool,
    pub enforcement_command_published: bool,
    pub evidence_refs: Vec<String>,
    pub evidence_grade: NetworkEvidenceGrade,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkAppGameSessionCorrelationError {
    EmptyNetworkFlowRef,
    EmptyEvidenceRef,
    EmptySessionSummaryRef,
    EmptyForegroundEvidenceRef,
    EmptyProcessCorrelationRef,
    EmptyAppId,
    EmptyDisplayName,
    EmptyProcessName,
    EmptyLauncherRef,
    InvalidConfidence(u8),
    ForegroundTimeExceedsRunningTime,
    UnsupportedExactUrlClaim,
    UnsupportedScreenContentClaim,
    UnsupportedAiScannerClaim,
    UnsupportedPolicyAuthorityClaim,
    UnsupportedAdapterAuthorityClaim,
    UnsupportedEnforcementCommandClaim,
}

pub fn correlate_app_game_foreground_session(
    input: NetworkAppGameSessionCorrelationInput,
) -> Result<NetworkAppGameSessionCorrelation, NetworkAppGameSessionCorrelationError> {
    validate_app_game_session_input(&input)?;

    let Some(session) = input.stored_session else {
        return Ok(missing_session_correlation(&input.network_flow_ref));
    };

    if !session.adapter_available {
        return Ok(app_game_session_correlation(
            input.network_flow_ref,
            session,
            NetworkAppGameSessionCorrelationState::AdapterUnavailable,
            NetworkAppGameSessionCorrelationBasis::AdapterUnavailable,
            false,
            NetworkEvidenceGrade::D,
        ));
    }

    if session.evidence_kind == NetworkAppGameEvidenceKind::LauncherOnly {
        return Ok(app_game_session_correlation(
            input.network_flow_ref,
            session,
            NetworkAppGameSessionCorrelationState::LauncherOnlyGuarded,
            NetworkAppGameSessionCorrelationBasis::LauncherOnlyEvidence,
            true,
            NetworkEvidenceGrade::D,
        ));
    }

    match session.evidence_kind {
        NetworkAppGameEvidenceKind::KnownGame | NetworkAppGameEvidenceKind::KnownApp
            if session.foreground_state == NetworkAppGameForegroundState::KnownForeground =>
        {
            Ok(app_game_session_correlation(
                input.network_flow_ref,
                session,
                NetworkAppGameSessionCorrelationState::ForegroundSessionConfirmed,
                NetworkAppGameSessionCorrelationBasis::StoredForegroundEvidence,
                false,
                NetworkEvidenceGrade::C,
            ))
        }
        NetworkAppGameEvidenceKind::KnownGame | NetworkAppGameEvidenceKind::KnownApp => {
            Ok(app_game_session_correlation(
                input.network_flow_ref,
                session,
                NetworkAppGameSessionCorrelationState::RunningSessionConfirmed,
                NetworkAppGameSessionCorrelationBasis::StoredSessionSummary,
                false,
                NetworkEvidenceGrade::C,
            ))
        }
        NetworkAppGameEvidenceKind::AppGameCandidate => Ok(app_game_session_correlation(
            input.network_flow_ref,
            session,
            NetworkAppGameSessionCorrelationState::CandidateNeedsReview,
            NetworkAppGameSessionCorrelationBasis::CandidateStoredEvidence,
            false,
            NetworkEvidenceGrade::D,
        )),
        NetworkAppGameEvidenceKind::UnknownProcess => Ok(app_game_session_correlation(
            input.network_flow_ref,
            session,
            NetworkAppGameSessionCorrelationState::NoSessionEvidence,
            NetworkAppGameSessionCorrelationBasis::MissingStoredEvidence,
            false,
            NetworkEvidenceGrade::D,
        )),
        NetworkAppGameEvidenceKind::LauncherOnly => unreachable!(),
    }
}

fn app_game_session_correlation(
    network_flow_ref: String,
    session: NetworkAppGameStoredSessionEvidence,
    state: NetworkAppGameSessionCorrelationState,
    basis: NetworkAppGameSessionCorrelationBasis,
    launcher_only_guarded: bool,
    evidence_grade: NetworkEvidenceGrade,
) -> NetworkAppGameSessionCorrelation {
    let evidence_refs = evidence_refs(&network_flow_ref, &session);
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

fn missing_session_correlation(network_flow_ref: &str) -> NetworkAppGameSessionCorrelation {
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

fn validate_app_game_session_input(
    input: &NetworkAppGameSessionCorrelationInput,
) -> Result<(), NetworkAppGameSessionCorrelationError> {
    if input.network_flow_ref.trim().is_empty() {
        return Err(NetworkAppGameSessionCorrelationError::EmptyNetworkFlowRef);
    }
    if let Some(session) = &input.stored_session {
        validate_session(session)?;
    }
    Ok(())
}

fn validate_session(
    session: &NetworkAppGameStoredSessionEvidence,
) -> Result<(), NetworkAppGameSessionCorrelationError> {
    if session.evidence_ref.trim().is_empty() {
        return Err(NetworkAppGameSessionCorrelationError::EmptyEvidenceRef);
    }
    validate_optional_ref(
        session.session_summary_ref.as_ref(),
        NetworkAppGameSessionCorrelationError::EmptySessionSummaryRef,
    )?;
    validate_optional_ref(
        session.foreground_evidence_ref.as_ref(),
        NetworkAppGameSessionCorrelationError::EmptyForegroundEvidenceRef,
    )?;
    validate_optional_ref(
        session.process_correlation_ref.as_ref(),
        NetworkAppGameSessionCorrelationError::EmptyProcessCorrelationRef,
    )?;
    validate_optional_ref(
        session.app_id.as_ref(),
        NetworkAppGameSessionCorrelationError::EmptyAppId,
    )?;
    validate_optional_ref(
        session.display_name.as_ref(),
        NetworkAppGameSessionCorrelationError::EmptyDisplayName,
    )?;
    validate_optional_ref(
        session.process_name.as_ref(),
        NetworkAppGameSessionCorrelationError::EmptyProcessName,
    )?;
    validate_optional_ref(
        session.launcher_ref.as_ref(),
        NetworkAppGameSessionCorrelationError::EmptyLauncherRef,
    )?;
    if let Some(confidence) = session.confidence {
        if confidence > 100 {
            return Err(NetworkAppGameSessionCorrelationError::InvalidConfidence(
                confidence,
            ));
        }
    }
    if session.foreground_time_seconds > session.running_time_seconds {
        return Err(NetworkAppGameSessionCorrelationError::ForegroundTimeExceedsRunningTime);
    }
    validate_session_non_claims(session)?;
    Ok(())
}

fn validate_session_non_claims(
    session: &NetworkAppGameStoredSessionEvidence,
) -> Result<(), NetworkAppGameSessionCorrelationError> {
    if session.exact_url_claimed {
        return Err(NetworkAppGameSessionCorrelationError::UnsupportedExactUrlClaim);
    }
    if session.screen_content_claimed {
        return Err(NetworkAppGameSessionCorrelationError::UnsupportedScreenContentClaim);
    }
    if session.ai_scanned_device {
        return Err(NetworkAppGameSessionCorrelationError::UnsupportedAiScannerClaim);
    }
    if session.policy_action_authority {
        return Err(NetworkAppGameSessionCorrelationError::UnsupportedPolicyAuthorityClaim);
    }
    if session.adapter_action_authorized {
        return Err(NetworkAppGameSessionCorrelationError::UnsupportedAdapterAuthorityClaim);
    }
    if session.enforcement_command_published {
        return Err(NetworkAppGameSessionCorrelationError::UnsupportedEnforcementCommandClaim);
    }
    Ok(())
}

fn validate_optional_ref(
    value: Option<&String>,
    error: NetworkAppGameSessionCorrelationError,
) -> Result<(), NetworkAppGameSessionCorrelationError> {
    if value.is_some_and(|value| value.trim().is_empty()) {
        return Err(error);
    }
    Ok(())
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
