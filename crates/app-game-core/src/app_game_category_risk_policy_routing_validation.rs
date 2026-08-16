use super::terminal_route;
use super::types::{
    AppGameCategoryProofState, AppGameCategoryRiskCandidateSource, AppGameCategoryRiskRoute,
    AppGameCategoryRiskRouteReason, AppGameCategoryRiskRouteRequest, AppGameCategoryRiskRouteState,
};

pub(super) fn route_failure(
    request: &AppGameCategoryRiskRouteRequest,
) -> Option<AppGameCategoryRiskRoute> {
    if request.candidate.confidence_permille > 1_000 {
        return Some(rejected(
            request,
            AppGameCategoryRiskRouteReason::InvalidConfidence,
        ));
    }
    if request.target_ref.is_none() {
        return Some(rejected(
            request,
            AppGameCategoryRiskRouteReason::MissingTargetReference,
        ));
    }
    if let Some(route) = category_proof_failure(request) {
        return Some(route);
    }
    if request.candidate.supporting_evidence_refs.is_empty() {
        return Some(rejected(
            request,
            AppGameCategoryRiskRouteReason::MissingSupportingEvidence,
        ));
    }
    ai_digest_failure(request)
}

fn category_proof_failure(
    request: &AppGameCategoryRiskRouteRequest,
) -> Option<AppGameCategoryRiskRoute> {
    match request.candidate.category_proof_state {
        AppGameCategoryProofState::Active if request.candidate.category_proof_ref.is_some() => None,
        AppGameCategoryProofState::Stale => Some(rejected(
            request,
            AppGameCategoryRiskRouteReason::StaleCategoryProof,
        )),
        AppGameCategoryProofState::ManualRequired => Some(terminal_route(
            request,
            AppGameCategoryRiskRouteState::ManualRequired,
            AppGameCategoryRiskRouteReason::CandidateRequiresManualReview,
        )),
        AppGameCategoryProofState::Active | AppGameCategoryProofState::Missing => Some(rejected(
            request,
            AppGameCategoryRiskRouteReason::MissingCategoryProof,
        )),
    }
}

fn ai_digest_failure(
    request: &AppGameCategoryRiskRouteRequest,
) -> Option<AppGameCategoryRiskRoute> {
    if request.candidate.candidate_source != AppGameCategoryRiskCandidateSource::LocalAi {
        return None;
    }
    let Some(digest_ref) = request.candidate.ai_digest_ref.as_ref() else {
        return Some(rejected(
            request,
            AppGameCategoryRiskRouteReason::MissingAiDigest,
        ));
    };
    (!request
        .candidate
        .supporting_evidence_refs
        .contains(digest_ref))
    .then(|| rejected(request, AppGameCategoryRiskRouteReason::UnboundAiDigest))
}

fn rejected(
    request: &AppGameCategoryRiskRouteRequest,
    reason: AppGameCategoryRiskRouteReason,
) -> AppGameCategoryRiskRoute {
    terminal_route(request, AppGameCategoryRiskRouteState::Rejected, reason)
}
