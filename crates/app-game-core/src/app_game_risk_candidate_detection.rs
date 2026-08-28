use ocentra_parent_agent_protocol::app_game::{
    AppGameInventoryEvidenceRow, AppGameRiskCategoryKind,
};
use serde::{Deserialize, Serialize};

use crate::app_game_category_risk_policy_routing::types::AppGameCategoryRiskCandidate;
use crate::app_game_category_risk_policy_routing::AppGameCategoryRiskCompilation;
use crate::app_game_policy_target_compiler::references::{
    AppGamePolicyCompileRequestId, AppGamePolicyDeviceId, AppGamePolicyLocalUserRef,
    AppGamePolicyRuleRef, AppGamePolicyScheduleRef, AppGamePolicyTargetRef,
};
use crate::app_game_policy_target_compiler::types::{
    AppGamePolicyCompilerAuthorityEvidence, AppGamePolicyCompilerCapabilityEvidence,
    AppGamePolicyCompilerContext,
};

#[path = "app_game_risk_candidate_detection_candidates.rs"]
mod app_game_risk_candidate_detection_candidates;

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGameRiskCandidateRouteContext {
    pub compile_request_id: AppGamePolicyCompileRequestId,
    pub rule_ref: AppGamePolicyRuleRef,
    pub device_id: AppGamePolicyDeviceId,
    pub local_user_ref: AppGamePolicyLocalUserRef,
    pub target_ref: Option<AppGamePolicyTargetRef>,
    pub schedule_ref: Option<AppGamePolicyScheduleRef>,
    pub capability_refs: Vec<AppGamePolicyCompilerCapabilityEvidence>,
    pub authority_refs: Vec<AppGamePolicyCompilerAuthorityEvidence>,
    pub compiler_context: AppGamePolicyCompilerContext,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGameRiskPolicyCompilation {
    pub detection: AppGameRiskCandidateDetection,
    pub compilation: Option<AppGameCategoryRiskCompilation>,
}

pub fn compile_app_game_risk_candidate(
    row: &AppGameInventoryEvidenceRow,
    context: AppGameRiskCandidateRouteContext,
) -> AppGameRiskPolicyCompilation {
    let detection = detect_app_game_risk_candidate(row);
    let compilation = app_game_risk_candidate_detection_candidates::compile_candidate(
        detection.candidate.as_ref(),
        context,
    );

    AppGameRiskPolicyCompilation {
        detection,
        compilation,
    }
}

pub fn detect_app_game_risk_candidate(
    row: &AppGameInventoryEvidenceRow,
) -> AppGameRiskCandidateDetection {
    app_game_risk_candidate_detection_candidates::detect_candidate(row)
}
