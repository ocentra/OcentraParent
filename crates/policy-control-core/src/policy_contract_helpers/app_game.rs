#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use super::action::PolicyContractAction;
use super::authority::AppGameCategoryRiskPolicyRoute;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppGameCategoryRiskPolicyRouteFamily {
    #[serde(rename = "nativeApp")]
    NativeApp,
    #[serde(rename = "nativeGame")]
    NativeGame,
    #[serde(rename = "riskCandidate")]
    RiskCandidate,
    #[serde(rename = "gameContext")]
    GameContext,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppGameCategoryRiskPolicyRouteSourceKind {
    #[serde(rename = "catalog")]
    Catalog,
    #[serde(rename = "storeMetadata")]
    StoreMetadata,
    #[serde(rename = "launcherManifest")]
    LauncherManifest,
    #[serde(rename = "parentLabel")]
    ParentLabel,
    #[serde(rename = "localAi")]
    LocalAi,
    #[serde(rename = "processMetadata")]
    ProcessMetadata,
    #[serde(rename = "executableName")]
    ExecutableName,
    #[serde(rename = "managedDevice")]
    ManagedDevice,
    #[serde(rename = "manualReview")]
    ManualReview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppGameCategoryRiskPolicyCandidateAction {
    #[serde(rename = "observe")]
    Observe,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "askParent")]
    AskParent,
    #[serde(rename = "manualReview")]
    ManualReview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppGameCategoryRiskPolicyRoutingState {
    #[serde(rename = "compile-ready")]
    CompileReady,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

pub fn app_game_category_risk_policy_route_target_matches_family(
    route: &AppGameCategoryRiskPolicyRoute,
) -> bool {
    match route.route_family {
        AppGameCategoryRiskPolicyRouteFamily::NativeApp => route.target_kind == "app-category",
        AppGameCategoryRiskPolicyRouteFamily::RiskCandidate => route.target_kind == "risk-app",
        AppGameCategoryRiskPolicyRouteFamily::NativeGame => route.target_kind == "game-category",
        AppGameCategoryRiskPolicyRouteFamily::GameContext => matches!(
            route.target_kind.as_str(),
            "multiplayer-game" | "ugc-game" | "purchase-capable-game" | "mature-game"
        ),
    }
}

pub fn app_game_category_risk_policy_route_uses_category_proof(
    route: &AppGameCategoryRiskPolicyRoute,
) -> bool {
    route.category_proof_kind == "category-proof"
        && route.category_proof_evidence_state == "active"
        && route.supporting_evidence_count > 0
}

pub fn app_game_category_risk_policy_route_action_matches_candidate(
    route: &AppGameCategoryRiskPolicyRoute,
) -> bool {
    match route.candidate_action {
        AppGameCategoryRiskPolicyCandidateAction::Observe => {
            route.requested_action == "observe"
                && route.policy_action == PolicyContractAction::Unknown
        }
        AppGameCategoryRiskPolicyCandidateAction::Warn => {
            route.requested_action == "warn" && route.policy_action == PolicyContractAction::Warn
        }
        AppGameCategoryRiskPolicyCandidateAction::AskParent => {
            route.requested_action == "ask-parent"
                && route.policy_action == PolicyContractAction::AskParent
        }
        AppGameCategoryRiskPolicyCandidateAction::ManualReview => {
            route.requested_action == "manual-required"
                && route.policy_action == PolicyContractAction::AskParent
        }
    }
}

pub fn app_game_category_risk_policy_route_keeps_soft_boundary(
    route: &AppGameCategoryRiskPolicyRoute,
) -> bool {
    matches!(
        route.requested_action.as_str(),
        "observe" | "warn" | "ask-parent" | "manual-required"
    )
}

pub fn app_game_category_risk_policy_route_manual_review_requires_manual_state(
    route: &AppGameCategoryRiskPolicyRoute,
) -> bool {
    route.candidate_action != AppGameCategoryRiskPolicyCandidateAction::ManualReview
        || route.routing_state == AppGameCategoryRiskPolicyRoutingState::ManualRequired
}

pub fn app_game_category_risk_policy_route_local_ai_requires_digest(
    route: &AppGameCategoryRiskPolicyRoute,
) -> bool {
    route.source_kind != AppGameCategoryRiskPolicyRouteSourceKind::LocalAi
        || route.has_ai_digest_ref
}
