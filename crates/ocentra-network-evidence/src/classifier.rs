use serde::{Deserialize, Serialize};

use crate::{CategoryMatchKind, DomainCategoryLookup, NetworkCategory, NetworkEvidenceGrade};

mod matching;
mod validation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkClassifierBasis {
    DomainCategory,
    BrowserConfirmedCdn,
    BrowserConfirmedProcess,
    CdnCandidateNeedsConfirmation,
    ProcessCandidateNeedsConfirmation,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CdnClassifierHint {
    pub provider_domain: String,
    pub category_hint: NetworkCategory,
    pub confidence_percent: u8,
    pub source_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessClassifierHint {
    pub process_name: String,
    pub category_hint: NetworkCategory,
    pub confidence_percent: u8,
    pub source_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserClassifierConfirmation {
    pub confirmed_domain: String,
    pub category: NetworkCategory,
    pub source_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkActivityClassifierInput {
    pub domain_lookup: DomainCategoryLookup,
    pub cdn_hint: Option<CdnClassifierHint>,
    pub process_hint: Option<ProcessClassifierHint>,
    pub browser_confirmation: Option<BrowserClassifierConfirmation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkActivityClassification {
    pub category: NetworkCategory,
    pub basis: NetworkClassifierBasis,
    pub confidence_percent: u8,
    pub evidence_refs: Vec<String>,
    pub browser_confirmation_required: bool,
    pub evidence_grade: NetworkEvidenceGrade,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkClassifierError {
    InvalidCdnConfidence(u8),
    InvalidProcessConfidence(u8),
    EmptyCdnSourceRef,
    EmptyProcessSourceRef,
    EmptyBrowserConfirmationRef,
}

pub fn classify_social_video_game_activity(
    input: NetworkActivityClassifierInput,
) -> Result<NetworkActivityClassification, NetworkClassifierError> {
    validation::validate_input(&input)?;

    if matching::target_category(input.domain_lookup.category)
        && input.domain_lookup.match_kind != CategoryMatchKind::NoMatch
        && matching::fresh_enough(input.domain_lookup.freshness)
    {
        return Ok(NetworkActivityClassification {
            category: input.domain_lookup.category,
            basis: NetworkClassifierBasis::DomainCategory,
            confidence_percent: input.domain_lookup.confidence_percent.unwrap_or(80),
            evidence_refs: input.domain_lookup.source_id.into_iter().collect(),
            browser_confirmation_required: false,
            evidence_grade: NetworkEvidenceGrade::C,
            exact_url_available: false,
            decrypted_payload_available: false,
        });
    }

    if let Some(classification) = matching::browser_confirmed_cdn(&input) {
        return Ok(classification);
    }

    if let Some(classification) = matching::browser_confirmed_process(&input) {
        return Ok(classification);
    }

    if let Some(hint) = input.cdn_hint {
        if matching::target_category(hint.category_hint) {
            return Ok(NetworkActivityClassification {
                category: hint.category_hint,
                basis: NetworkClassifierBasis::CdnCandidateNeedsConfirmation,
                confidence_percent: hint.confidence_percent.min(60),
                evidence_refs: vec![hint.source_ref],
                browser_confirmation_required: true,
                evidence_grade: NetworkEvidenceGrade::D,
                exact_url_available: false,
                decrypted_payload_available: false,
            });
        }
    }

    if let Some(hint) = input.process_hint {
        if matching::target_category(hint.category_hint) {
            return Ok(NetworkActivityClassification {
                category: hint.category_hint,
                basis: NetworkClassifierBasis::ProcessCandidateNeedsConfirmation,
                confidence_percent: hint.confidence_percent.min(70),
                evidence_refs: vec![hint.source_ref],
                browser_confirmation_required: true,
                evidence_grade: NetworkEvidenceGrade::D,
                exact_url_available: false,
                decrypted_payload_available: false,
            });
        }
    }

    Ok(unknown_classification())
}

fn unknown_classification() -> NetworkActivityClassification {
    NetworkActivityClassification {
        category: NetworkCategory::Unknown,
        basis: NetworkClassifierBasis::Unknown,
        confidence_percent: 0,
        evidence_refs: Vec::new(),
        browser_confirmation_required: true,
        evidence_grade: NetworkEvidenceGrade::D,
        exact_url_available: false,
        decrypted_payload_available: false,
    }
}
