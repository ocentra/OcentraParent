use super::{
    NetworkCascadeNextCheck, NetworkCascadeSignalStrength, NetworkCascadeSource,
    NetworkCascadeSourceKind, NetworkEvidenceCascadeDecision,
};

pub(super) fn build_decision(source: &NetworkCascadeSource) -> NetworkEvidenceCascadeDecision {
    NetworkEvidenceCascadeDecision {
        primary_source: Some(source.source_kind),
        next_checks: next_checks_for(source),
        parent_review_required: source.signal_strength == NetworkCascadeSignalStrength::Candidate,
        adapter_action_authorized: false,
        policy_action_authority: false,
        exact_url_available: managed_browser_exact_url(source),
        decrypted_payload_available: false,
        evidence_refs: vec![source.source_ref.clone()],
        evidence_grade: source.evidence_grade,
    }
}

pub(super) fn no_source_decision() -> NetworkEvidenceCascadeDecision {
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
        evidence_grade: crate::dns::types::NetworkEvidenceGrade::D,
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

fn managed_browser_exact_url(source: &NetworkCascadeSource) -> bool {
    source.source_kind == NetworkCascadeSourceKind::ManagedBrowserExactUrl
        && source.exact_url_available
}
