use serde::{Deserialize, Serialize};

use crate::NetworkEvidenceGrade;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkCascadeSourceKind {
    ManagedBrowserExactUrl,
    ProcessAppCorrelation,
    DomainCategory,
    TunnelIndicator,
    TransferCandidate,
    ScreenSummary,
    LocalAiSuggestion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkCascadeSignalStrength {
    Confirmed,
    Candidate,
    WeakHint,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkCascadeNextCheck {
    ManagedBrowserCorrelation,
    ProcessAppCorrelation,
    ScreenSummary,
    LocalAiReview,
    ParentReview,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkCascadeSource {
    pub source_kind: NetworkCascadeSourceKind,
    pub signal_strength: NetworkCascadeSignalStrength,
    pub evidence_grade: NetworkEvidenceGrade,
    pub source_ref: String,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub policy_action_authority: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkEvidenceCascadeInput {
    pub sources: Vec<NetworkCascadeSource>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkEvidenceCascadeDecision {
    pub primary_source: Option<NetworkCascadeSourceKind>,
    pub next_checks: Vec<NetworkCascadeNextCheck>,
    pub parent_review_required: bool,
    pub adapter_action_authorized: bool,
    pub policy_action_authority: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub evidence_refs: Vec<String>,
    pub evidence_grade: NetworkEvidenceGrade,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkEvidenceCascadeError {
    EmptySourceRef,
    UnsupportedNetworkExactUrlClaim(NetworkCascadeSourceKind),
    UnsupportedDecryptedPayloadClaim,
}

pub fn route_network_evidence_cascade(
    input: NetworkEvidenceCascadeInput,
) -> Result<NetworkEvidenceCascadeDecision, NetworkEvidenceCascadeError> {
    validate_cascade_input(&input.sources)?;

    let Some(primary) = strongest_source(&input.sources) else {
        return Ok(no_source_decision());
    };

    Ok(NetworkEvidenceCascadeDecision {
        primary_source: Some(primary.source_kind),
        next_checks: next_checks_for(primary),
        parent_review_required: primary.signal_strength == NetworkCascadeSignalStrength::Candidate,
        adapter_action_authorized: false,
        policy_action_authority: false,
        exact_url_available: managed_browser_exact_url(primary),
        decrypted_payload_available: false,
        evidence_refs: vec![primary.source_ref.clone()],
        evidence_grade: primary.evidence_grade,
    })
}

fn validate_cascade_input(
    sources: &[NetworkCascadeSource],
) -> Result<(), NetworkEvidenceCascadeError> {
    for source in sources {
        if source.source_ref.trim().is_empty() {
            return Err(NetworkEvidenceCascadeError::EmptySourceRef);
        }
        if source.decrypted_payload_available {
            return Err(NetworkEvidenceCascadeError::UnsupportedDecryptedPayloadClaim);
        }
        if source.exact_url_available
            && source.source_kind != NetworkCascadeSourceKind::ManagedBrowserExactUrl
        {
            return Err(
                NetworkEvidenceCascadeError::UnsupportedNetworkExactUrlClaim(source.source_kind),
            );
        }
    }
    Ok(())
}

fn strongest_source(sources: &[NetworkCascadeSource]) -> Option<&NetworkCascadeSource> {
    sources
        .iter()
        .filter(|source| source.signal_strength != NetworkCascadeSignalStrength::Unavailable)
        .max_by_key(|source| source_score(source))
}

fn source_score(source: &NetworkCascadeSource) -> u16 {
    strength_score(source.signal_strength)
        + kind_score(source.source_kind)
        + grade_score(source.evidence_grade)
}

fn strength_score(strength: NetworkCascadeSignalStrength) -> u16 {
    match strength {
        NetworkCascadeSignalStrength::Confirmed => 600,
        NetworkCascadeSignalStrength::Candidate => 300,
        NetworkCascadeSignalStrength::WeakHint => 100,
        NetworkCascadeSignalStrength::Unavailable => 0,
    }
}

fn kind_score(kind: NetworkCascadeSourceKind) -> u16 {
    match kind {
        NetworkCascadeSourceKind::ManagedBrowserExactUrl => 70,
        NetworkCascadeSourceKind::ProcessAppCorrelation => 60,
        NetworkCascadeSourceKind::DomainCategory => 50,
        NetworkCascadeSourceKind::TunnelIndicator => 40,
        NetworkCascadeSourceKind::TransferCandidate => 30,
        NetworkCascadeSourceKind::ScreenSummary => 20,
        NetworkCascadeSourceKind::LocalAiSuggestion => 10,
    }
}

fn grade_score(grade: NetworkEvidenceGrade) -> u16 {
    match grade {
        NetworkEvidenceGrade::B => 3,
        NetworkEvidenceGrade::C => 2,
        NetworkEvidenceGrade::D => 1,
    }
}

fn next_checks_for(source: &NetworkCascadeSource) -> Vec<NetworkCascadeNextCheck> {
    match source.signal_strength {
        NetworkCascadeSignalStrength::Confirmed => Vec::new(),
        NetworkCascadeSignalStrength::Candidate => vec![NetworkCascadeNextCheck::ParentReview],
        NetworkCascadeSignalStrength::WeakHint => vec![
            NetworkCascadeNextCheck::ManagedBrowserCorrelation,
            NetworkCascadeNextCheck::ProcessAppCorrelation,
            NetworkCascadeNextCheck::ScreenSummary,
            NetworkCascadeNextCheck::LocalAiReview,
        ],
        NetworkCascadeSignalStrength::Unavailable => vec![
            NetworkCascadeNextCheck::ManagedBrowserCorrelation,
            NetworkCascadeNextCheck::ProcessAppCorrelation,
        ],
    }
}

fn no_source_decision() -> NetworkEvidenceCascadeDecision {
    NetworkEvidenceCascadeDecision {
        primary_source: None,
        next_checks: vec![
            NetworkCascadeNextCheck::ManagedBrowserCorrelation,
            NetworkCascadeNextCheck::ProcessAppCorrelation,
            NetworkCascadeNextCheck::ScreenSummary,
        ],
        parent_review_required: false,
        adapter_action_authorized: false,
        policy_action_authority: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        evidence_refs: Vec::new(),
        evidence_grade: NetworkEvidenceGrade::D,
    }
}

fn managed_browser_exact_url(source: &NetworkCascadeSource) -> bool {
    source.source_kind == NetworkCascadeSourceKind::ManagedBrowserExactUrl
        && source.exact_url_available
}
