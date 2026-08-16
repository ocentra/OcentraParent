use serde::{Deserialize, Serialize};

use crate::dns::types::NetworkEvidenceGrade;

mod correlation;
mod validation;

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
    validation::validate_app_game_session_input(&input)?;

    let Some(session) = input.stored_session else {
        return Ok(correlation::missing_session_correlation(
            &input.network_flow_ref,
        ));
    };

    if !session.adapter_available {
        return Ok(correlation::adapter_unavailable_correlation(
            &input.network_flow_ref,
            session,
        ));
    }

    Ok(correlation::correlate_session(
        &input.network_flow_ref,
        session,
    ))
}
