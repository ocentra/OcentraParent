#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use super::{
    action::PolicyContractAction,
    app_game::{
        AppGameCategoryRiskPolicyCandidateAction, AppGameCategoryRiskPolicyRouteFamily,
        AppGameCategoryRiskPolicyRouteSourceKind, AppGameCategoryRiskPolicyRoutingState,
    },
    preview::PolicyContractDecision,
    schedule::{
        assert_resolution_has_no_review_or_override_artifacts,
        assert_resolution_has_no_review_override_or_replay_artifacts, assert_utc_timestamp,
        validate_policy_schedule_boundary, PolicyContractScheduleBoundary,
    },
    PolicyContractValidationResult,
};

mod approval_resolution;
mod override_grant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyContractAuthoritySource {
    #[serde(rename = "parent-policy")]
    ParentPolicy,
    #[serde(rename = "local-ai-result")]
    LocalAiResult,
    #[serde(rename = "tracking-signal")]
    TrackingSignal,
    #[serde(rename = "activity-evidence")]
    ActivityEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyContractAuthorityState {
    #[serde(rename = "authorized")]
    Authorized,
    #[serde(rename = "evidence-only")]
    EvidenceOnly,
    #[serde(rename = "dry-run")]
    DryRun,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyContractApprovalOrigin {
    #[serde(rename = "child-request")]
    ChildRequest,
    #[serde(rename = "assistant-draft")]
    AssistantDraft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyContractApprovalKind {
    #[serde(rename = "ask-parent")]
    AskParent,
    #[serde(rename = "temporary-override")]
    TemporaryOverride,
    #[serde(rename = "bonus-time")]
    BonusTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyContractApprovalState {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "denied")]
    Denied,
    #[serde(rename = "modified")]
    Modified,
    #[serde(rename = "expired-request")]
    ExpiredRequest,
    #[serde(rename = "replay-rejected")]
    ReplayRejected,
    #[serde(rename = "preview-only")]
    PreviewOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyContractOverrideType {
    #[serde(rename = "temporary-allow")]
    TemporaryAllow,
    #[serde(rename = "temporary-block")]
    TemporaryBlock,
    #[serde(rename = "bonus-time")]
    BonusTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyContractOverrideState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "revoked")]
    Revoked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContractApprovalRequest {
    pub origin: PolicyContractApprovalOrigin,
    pub kind: PolicyContractApprovalKind,
    pub child_profile_id: String,
    pub requested_at: String,
    pub expires_at: String,
    pub requested_bonus_time_minutes: Option<u16>,
    pub schedule_boundary: Option<PolicyContractScheduleBoundary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContractOverrideGrant {
    pub override_type: PolicyContractOverrideType,
    pub state: PolicyContractOverrideState,
    pub action: PolicyContractAction,
    pub effective_from: String,
    pub effective_until: String,
    pub bonus_time_minutes: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContractApprovalResolution {
    pub approval: PolicyContractApprovalRequest,
    pub state: PolicyContractApprovalState,
    pub evaluated_at: String,
    pub reviewed_by_actor_id: Option<String>,
    pub reviewed_at: Option<String>,
    pub audit_reference_id: Option<String>,
    pub override_grant: Option<PolicyContractOverrideGrant>,
    pub replay_of_approval_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContractAuthorityRequest {
    pub source: PolicyContractAuthoritySource,
    pub decision: PolicyContractDecision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContractAuthorityDecision {
    pub source: PolicyContractAuthoritySource,
    pub state: PolicyContractAuthorityState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGameCategoryRiskPolicyRoute {
    pub route_family: AppGameCategoryRiskPolicyRouteFamily,
    pub source_kind: AppGameCategoryRiskPolicyRouteSourceKind,
    pub target_kind: String,
    pub candidate_action: AppGameCategoryRiskPolicyCandidateAction,
    pub requested_action: String,
    pub policy_action: PolicyContractAction,
    pub routing_state: AppGameCategoryRiskPolicyRoutingState,
    pub category_proof_kind: String,
    pub category_proof_evidence_state: String,
    pub supporting_evidence_count: usize,
    pub has_ai_digest_ref: bool,
}

pub fn resolve_policy_authority(
    request: &PolicyContractAuthorityRequest,
) -> PolicyContractAuthorityDecision {
    PolicyContractAuthorityDecision {
        source: request.source,
        state: resolve_policy_authority_state(request.source, request.decision.dry_run),
    }
}

pub fn resolve_policy_authority_state(
    source: PolicyContractAuthoritySource,
    dry_run: bool,
) -> PolicyContractAuthorityState {
    if dry_run {
        PolicyContractAuthorityState::DryRun
    } else if source == PolicyContractAuthoritySource::ParentPolicy {
        PolicyContractAuthorityState::Authorized
    } else {
        PolicyContractAuthorityState::EvidenceOnly
    }
}

pub fn validate_policy_approval_resolution(
    resolution: &PolicyContractApprovalResolution,
) -> PolicyContractValidationResult {
    approval_resolution::validate_policy_approval_resolution(resolution)
}
