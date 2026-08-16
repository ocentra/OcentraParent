use serde::{Deserialize, Serialize};

use crate::dns::types::NetworkEvidenceGrade;

mod decision;
mod score;
mod selection;
mod validation;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkCascadeSourceKind {
    ManagedBrowserExactUrl = 0,
    ProcessAppCorrelation = 1,
    DomainCategory = 2,
    TunnelIndicator = 3,
    TransferCandidate = 4,
    ScreenSummary = 5,
    LocalAiSuggestion = 6,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkCascadeSignalStrength {
    Confirmed = 0,
    Candidate = 1,
    WeakHint = 2,
    Unavailable = 3,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkCascadeNextCheck {
    ManagedBrowserCorrelation = 0,
    ProcessAppCorrelation = 1,
    ScreenSummary = 2,
    LocalAiReview = 3,
    ParentReview = 4,
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
    let NetworkEvidenceCascadeInput { sources } = input;
    validation::validate_cascade_input(&sources)?;

    let Some(primary) = selection::strongest_source(&sources) else {
        return Ok(decision::no_source_decision());
    };

    Ok(decision::build_decision(primary))
}
