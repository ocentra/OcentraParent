use ocentra_parent_agent_protocol::app_game::{
    AppGameInventoryEvidenceRow, AppGameRiskCategoryKind, APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS,
};
use ocentra_parent_agent_protocol::ActivityEvidenceRef;
use serde::{Deserialize, Serialize};

use crate::app_game_category_risk_policy_routing::types::{
    AppGameCategoryProofState, AppGameCategoryRiskCandidate, AppGameCategoryRiskCandidateKind,
    AppGameCategoryRiskCandidateSource,
};
use crate::app_game_policy_target_compiler::references::AppGamePolicyEvidenceRef;
use crate::app_game_policy_target_compiler::types::AppGamePolicyCompilerRequestedAction;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameRiskCandidateDetectionState {
    CatalogCandidate,
    InventoryCandidate,
    UnknownCandidate,
    NotRiskCandidate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameRiskEvidenceState {
    Complete,
    Missing,
    Invalid,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameRiskCandidateDetection {
    pub state: AppGameRiskCandidateDetectionState,
    pub signal: Option<AppGameRiskCategoryKind>,
    pub evidence_state: AppGameRiskEvidenceState,
    pub invalid_evidence_refs: Vec<String>,
    pub candidate: Option<AppGameCategoryRiskCandidate>,
}

pub fn detect_app_game_risk_candidate(
    row: &AppGameInventoryEvidenceRow,
) -> AppGameRiskCandidateDetection {
    if let Some((category, signal)) = row.category_candidates.iter().find_map(|category| {
        risk_signal(category.category_kind.as_str()).map(|signal| (category, signal))
    }) {
        let (category_proof_refs, mut invalid_evidence_refs) = parse_evidence(&category.evidence);
        let (supporting_evidence_refs, supporting_invalid_refs) = parse_evidence(&row.evidence);
        invalid_evidence_refs.extend(supporting_invalid_refs);
        let category_proof_ref = category_proof_refs.into_iter().next();
        let evidence_state = evidence_state(
            category_proof_ref.is_some(),
            !supporting_evidence_refs.is_empty(),
            !invalid_evidence_refs.is_empty(),
        );

        return AppGameRiskCandidateDetection {
            state: if category.catalog_ref.is_some() {
                AppGameRiskCandidateDetectionState::CatalogCandidate
            } else {
                AppGameRiskCandidateDetectionState::InventoryCandidate
            },
            signal: Some(signal),
            evidence_state,
            invalid_evidence_refs,
            candidate: Some(AppGameCategoryRiskCandidate {
                candidate_kind: AppGameCategoryRiskCandidateKind::AppRisk,
                candidate_source: AppGameCategoryRiskCandidateSource::NativeInventory,
                confidence_permille: confidence_permille(category.confidence),
                category_proof_state: match evidence_state {
                    AppGameRiskEvidenceState::Complete => AppGameCategoryProofState::Active,
                    AppGameRiskEvidenceState::Invalid => AppGameCategoryProofState::ManualRequired,
                    AppGameRiskEvidenceState::Missing => AppGameCategoryProofState::Missing,
                },
                category_proof_ref,
                supporting_evidence_refs,
                ai_digest_ref: None,
                requested_action: AppGamePolicyCompilerRequestedAction::AskParent,
            }),
        };
    }

    if row.classification_state == APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS {
        return AppGameRiskCandidateDetection {
            state: AppGameRiskCandidateDetectionState::UnknownCandidate,
            signal: Some(AppGameRiskCategoryKind::UnknownRisk),
            evidence_state: unknown_evidence_state(row),
            invalid_evidence_refs: invalid_evidence_refs(&row.evidence),
            candidate: Some(AppGameCategoryRiskCandidate {
                candidate_kind: AppGameCategoryRiskCandidateKind::AppRisk,
                candidate_source: AppGameCategoryRiskCandidateSource::NativeInventory,
                confidence_permille: confidence_permille(row.confidence),
                category_proof_state: AppGameCategoryProofState::Missing,
                category_proof_ref: None,
                supporting_evidence_refs: parse_evidence(&row.evidence).0,
                ai_digest_ref: None,
                requested_action: AppGamePolicyCompilerRequestedAction::AskParent,
            }),
        };
    }

    AppGameRiskCandidateDetection {
        state: AppGameRiskCandidateDetectionState::NotRiskCandidate,
        signal: None,
        evidence_state: AppGameRiskEvidenceState::Missing,
        invalid_evidence_refs: Vec::new(),
        candidate: None,
    }
}

fn risk_signal(category_kind: &str) -> Option<AppGameRiskCategoryKind> {
    AppGameRiskCategoryKind::parse(category_kind)
}

fn confidence_permille(confidence: f64) -> u16 {
    if !confidence.is_finite() {
        return 0;
    }
    (confidence.clamp(0.0, 1.0) * 1_000.0).round() as u16
}

fn parse_evidence(values: &[ActivityEvidenceRef]) -> (Vec<AppGamePolicyEvidenceRef>, Vec<String>) {
    let mut parsed = Vec::new();
    let mut invalid = Vec::new();
    for evidence in values {
        match AppGamePolicyEvidenceRef::parse(evidence.evidence_id.clone()) {
            Ok(reference) => parsed.push(reference),
            Err(_) => invalid.push(evidence.evidence_id.clone()),
        }
    }
    (parsed, invalid)
}

fn invalid_evidence_refs(values: &[ActivityEvidenceRef]) -> Vec<String> {
    parse_evidence(values).1
}

fn evidence_state(
    has_category_proof: bool,
    has_supporting_evidence: bool,
    has_invalid_evidence: bool,
) -> AppGameRiskEvidenceState {
    if has_invalid_evidence {
        AppGameRiskEvidenceState::Invalid
    } else if has_category_proof && has_supporting_evidence {
        AppGameRiskEvidenceState::Complete
    } else {
        AppGameRiskEvidenceState::Missing
    }
}

fn unknown_evidence_state(row: &AppGameInventoryEvidenceRow) -> AppGameRiskEvidenceState {
    let (_, invalid) = parse_evidence(&row.evidence);
    if !invalid.is_empty() {
        AppGameRiskEvidenceState::Invalid
    } else {
        AppGameRiskEvidenceState::Missing
    }
}
