#[path = "app_game_risk_candidate_detection_evidence.rs"]
mod evidence;

use ocentra_parent_agent_protocol::app_game::{
    AppGameInventoryCategoryCandidate, AppGameInventoryEvidenceRow, AppGameRiskCategoryKind,
    APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS,
};

use crate::app_game_category_risk_policy_routing::compile_app_game_category_risk_candidate;
use crate::app_game_category_risk_policy_routing::types::{
    AppGameCategoryProofState, AppGameCategoryRiskCandidate, AppGameCategoryRiskCandidateKind,
    AppGameCategoryRiskCandidateSource, AppGameCategoryRiskRouteRequest,
};
use crate::app_game_policy_target_compiler::references::AppGamePolicyEvidenceRef;
use crate::app_game_policy_target_compiler::types::AppGamePolicyCompilerRequestedAction;

use super::{
    AppGameRiskCandidateDetection, AppGameRiskCandidateDetectionState,
    AppGameRiskCandidateRouteContext, AppGameRiskEvidenceState,
};

pub(super) fn compile_candidate(
    candidate: Option<&AppGameCategoryRiskCandidate>,
    context: AppGameRiskCandidateRouteContext,
) -> Option<super::AppGameCategoryRiskCompilation> {
    candidate.cloned().map(|candidate| {
        compile_app_game_category_risk_candidate(
            &AppGameCategoryRiskRouteRequest {
                compile_request_id: context.compile_request_id,
                rule_ref: context.rule_ref,
                device_id: context.device_id,
                local_user_ref: context.local_user_ref,
                target_ref: context.target_ref,
                schedule_ref: context.schedule_ref,
                candidate,
                capability_refs: context.capability_refs,
                authority_refs: context.authority_refs,
            },
            context.compiler_context,
        )
    })
}

pub(super) fn detect_candidate(row: &AppGameInventoryEvidenceRow) -> AppGameRiskCandidateDetection {
    if let Some((category, signal)) = row.category_candidates.iter().find_map(|category| {
        risk_signal(category.category_kind.as_str()).map(|signal| (category, signal))
    }) {
        let (category_proof_refs, mut invalid_evidence_refs) =
            evidence::parse_evidence(&category.evidence);
        let (supporting_evidence_refs, supporting_invalid_refs) =
            evidence::parse_evidence(&row.evidence);
        invalid_evidence_refs.extend(supporting_invalid_refs);
        let category_proof_ref = category_proof_refs.into_iter().next();
        let evidence_state = evidence::evidence_state(
            category_proof_ref.is_some(),
            !supporting_evidence_refs.is_empty(),
            !invalid_evidence_refs.is_empty(),
        );
        return catalog_candidate(
            category,
            signal,
            evidence_state,
            category_proof_ref,
            supporting_evidence_refs,
            invalid_evidence_refs,
        );
    }
    if row.classification_state == APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS {
        return unknown_candidate(row);
    }
    not_risk_candidate()
}

fn catalog_candidate(
    category: &AppGameInventoryCategoryCandidate,
    signal: AppGameRiskCategoryKind,
    evidence_state: AppGameRiskEvidenceState,
    category_proof_ref: Option<AppGamePolicyEvidenceRef>,
    supporting_evidence_refs: Vec<AppGamePolicyEvidenceRef>,
    invalid_evidence_refs: Vec<String>,
) -> AppGameRiskCandidateDetection {
    AppGameRiskCandidateDetection {
        state: catalog_candidate_state(category.catalog_ref.is_some()),
        signal: Some(signal),
        evidence_state,
        invalid_evidence_refs,
        candidate: Some(AppGameCategoryRiskCandidate {
            candidate_kind: AppGameCategoryRiskCandidateKind::AppRisk,
            candidate_source: AppGameCategoryRiskCandidateSource::NativeInventory,
            confidence_permille: confidence_permille(category.confidence),
            category_proof_state: category_proof_state(evidence_state),
            category_proof_ref,
            supporting_evidence_refs,
            ai_digest_ref: None,
            requested_action: AppGamePolicyCompilerRequestedAction::AskParent,
        }),
    }
}

fn unknown_candidate(row: &AppGameInventoryEvidenceRow) -> AppGameRiskCandidateDetection {
    AppGameRiskCandidateDetection {
        state: AppGameRiskCandidateDetectionState::UnknownCandidate,
        signal: Some(AppGameRiskCategoryKind::UnknownRisk),
        evidence_state: evidence::unknown_evidence_state(row),
        invalid_evidence_refs: evidence::invalid_evidence_refs(&row.evidence),
        candidate: Some(AppGameCategoryRiskCandidate {
            candidate_kind: AppGameCategoryRiskCandidateKind::AppRisk,
            candidate_source: AppGameCategoryRiskCandidateSource::NativeInventory,
            confidence_permille: confidence_permille(row.confidence),
            category_proof_state: AppGameCategoryProofState::Missing,
            category_proof_ref: None,
            supporting_evidence_refs: evidence::parse_evidence(&row.evidence).0,
            ai_digest_ref: None,
            requested_action: AppGamePolicyCompilerRequestedAction::AskParent,
        }),
    }
}

fn not_risk_candidate() -> AppGameRiskCandidateDetection {
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

fn catalog_candidate_state(has_catalog_ref: bool) -> AppGameRiskCandidateDetectionState {
    match has_catalog_ref {
        true => AppGameRiskCandidateDetectionState::CatalogCandidate,
        false => AppGameRiskCandidateDetectionState::InventoryCandidate,
    }
}

fn category_proof_state(evidence_state: AppGameRiskEvidenceState) -> AppGameCategoryProofState {
    match evidence_state {
        AppGameRiskEvidenceState::Complete => AppGameCategoryProofState::Active,
        AppGameRiskEvidenceState::Invalid => AppGameCategoryProofState::ManualRequired,
        AppGameRiskEvidenceState::Missing => AppGameCategoryProofState::Missing,
    }
}
