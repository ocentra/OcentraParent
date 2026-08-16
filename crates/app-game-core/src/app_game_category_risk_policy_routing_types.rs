use serde::{Deserialize, Serialize};

use crate::app_game_policy_target_compiler::references::{
    AppGamePolicyAuthorityRef, AppGamePolicyCapabilityRef, AppGamePolicyCompileRequestId,
    AppGamePolicyDeviceId, AppGamePolicyEvidenceRef, AppGamePolicyLocalUserRef,
    AppGamePolicyRuleRef, AppGamePolicyScheduleRef, AppGamePolicyTargetRef,
};
use crate::app_game_policy_target_compiler::types::{
    AppGamePolicyCompileRequest, AppGamePolicyCompilerAuthorityEvidence,
    AppGamePolicyCompilerCapabilityEvidence, AppGamePolicyCompilerRequestedAction,
    AppGamePolicyTargetKind,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameCategoryRiskCandidateKind {
    NativeAppCategory,
    AppRisk,
    NativeGameCategory,
    MultiplayerGameContext,
    UserGeneratedContentGameContext,
    PurchaseCapableGameContext,
    MatureGameContext,
}

impl AppGameCategoryRiskCandidateKind {
    pub(super) fn target_kind(self) -> AppGamePolicyTargetKind {
        match self {
            Self::NativeAppCategory => AppGamePolicyTargetKind::AppCategory,
            Self::AppRisk => AppGamePolicyTargetKind::RiskApp,
            Self::NativeGameCategory => AppGamePolicyTargetKind::GameCategory,
            Self::MultiplayerGameContext => AppGamePolicyTargetKind::MultiplayerGame,
            Self::UserGeneratedContentGameContext => AppGamePolicyTargetKind::UgcGame,
            Self::PurchaseCapableGameContext => AppGamePolicyTargetKind::PurchaseCapableGame,
            Self::MatureGameContext => AppGamePolicyTargetKind::MatureGame,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameCategoryRiskCandidateSource {
    NativeInventory,
    GameContext,
    LocalAi,
    ParentManualReview,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameCategoryProofState {
    Active,
    Stale,
    Missing,
    ManualRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameCategoryRiskRouteState {
    CompileReady,
    ManualRequired,
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameCategoryRiskRouteReason {
    None,
    MissingTargetReference,
    MissingCategoryProof,
    StaleCategoryProof,
    MissingSupportingEvidence,
    MissingAiDigest,
    UnboundAiDigest,
    InvalidConfidence,
    CandidateRequiresManualReview,
    HardActionRequiresManualReview,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameCategoryRiskAdapterDispatchState {
    NotDispatched,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGameCategoryRiskCandidate {
    pub candidate_kind: AppGameCategoryRiskCandidateKind,
    pub candidate_source: AppGameCategoryRiskCandidateSource,
    pub confidence_permille: u16,
    pub category_proof_state: AppGameCategoryProofState,
    pub category_proof_ref: Option<AppGamePolicyEvidenceRef>,
    pub supporting_evidence_refs: Vec<AppGamePolicyEvidenceRef>,
    pub ai_digest_ref: Option<AppGamePolicyEvidenceRef>,
    pub requested_action: AppGamePolicyCompilerRequestedAction,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGameCategoryRiskRouteRequest {
    pub compile_request_id: AppGamePolicyCompileRequestId,
    pub rule_ref: AppGamePolicyRuleRef,
    pub device_id: AppGamePolicyDeviceId,
    pub local_user_ref: AppGamePolicyLocalUserRef,
    pub target_ref: Option<AppGamePolicyTargetRef>,
    pub schedule_ref: Option<AppGamePolicyScheduleRef>,
    pub candidate: AppGameCategoryRiskCandidate,
    pub capability_refs: Vec<AppGamePolicyCompilerCapabilityEvidence>,
    pub authority_refs: Vec<AppGamePolicyCompilerAuthorityEvidence>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGameCategoryRiskRoute {
    pub route_state: AppGameCategoryRiskRouteState,
    pub route_reason: AppGameCategoryRiskRouteReason,
    pub target_kind: AppGamePolicyTargetKind,
    pub compiler_request: Option<AppGamePolicyCompileRequest>,
    pub adapter_dispatch_state: AppGameCategoryRiskAdapterDispatchState,
    pub supporting_evidence_refs: Vec<AppGamePolicyEvidenceRef>,
    pub capability_refs: Vec<AppGamePolicyCapabilityRef>,
    pub authority_refs: Vec<AppGamePolicyAuthorityRef>,
}
