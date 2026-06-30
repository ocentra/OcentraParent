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
        PolicyContractScheduleBoundaryState,
    },
    PolicyContractValidationResult,
};

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
    validate_policy_approval_request(&resolution.approval)?;
    assert_utc_timestamp(&resolution.evaluated_at, "evaluatedAt")?;
    validate_policy_approval_resolution_review_timestamp(resolution)?;
    validate_policy_approval_resolution_state_rules(resolution)?;
    Ok(())
}

fn validate_policy_approval_resolution_review_timestamp(
    resolution: &PolicyContractApprovalResolution,
) -> PolicyContractValidationResult {
    if let Some(reviewed_at) = &resolution.reviewed_at {
        assert_utc_timestamp(reviewed_at, "reviewedAt")?;
        if reviewed_at > &resolution.evaluated_at {
            return Err("reviewedAt cannot be after evaluatedAt");
        }
    }

    Ok(())
}

fn validate_policy_approval_resolution_state_rules(
    resolution: &PolicyContractApprovalResolution,
) -> PolicyContractValidationResult {
    match resolution.state {
        PolicyContractApprovalState::Pending => {
            validate_policy_approval_resolution_pending_state(resolution)
        }
        PolicyContractApprovalState::PreviewOnly => {
            validate_policy_approval_resolution_preview_only_state(resolution)
        }
        PolicyContractApprovalState::ExpiredRequest => {
            validate_policy_approval_resolution_expired_request_state(resolution)
        }
        PolicyContractApprovalState::ReplayRejected => {
            validate_policy_approval_resolution_replay_rejected_state(resolution)
        }
        PolicyContractApprovalState::Denied => {
            validate_policy_approval_resolution_denied_state(resolution)
        }
        PolicyContractApprovalState::Approved | PolicyContractApprovalState::Modified => {
            validate_policy_approval_resolution_reviewed_state(resolution)
        }
    }
}

fn validate_policy_approval_resolution_pending_state(
    resolution: &PolicyContractApprovalResolution,
) -> PolicyContractValidationResult {
    if resolution.reviewed_by_actor_id.is_some()
        || resolution.reviewed_at.is_some()
        || resolution.audit_reference_id.is_some()
        || resolution.override_grant.is_some()
        || resolution.replay_of_approval_id.is_some()
    {
        return Err("pending approvals cannot include review, replay, or override artifacts");
    }

    Ok(())
}

fn validate_policy_approval_resolution_preview_only_state(
    resolution: &PolicyContractApprovalResolution,
) -> PolicyContractValidationResult {
    if resolution.approval.origin != PolicyContractApprovalOrigin::AssistantDraft {
        return Err("preview-only approvals require assistant-draft origin");
    }
    assert_resolution_has_no_review_override_or_replay_artifacts(
        resolution,
        "preview-only approvals must remain unconfirmed and override-free",
    )
}

fn validate_policy_approval_resolution_expired_request_state(
    resolution: &PolicyContractApprovalResolution,
) -> PolicyContractValidationResult {
    if resolution.evaluated_at < resolution.approval.expires_at {
        return Err("expired-request state requires evaluatedAt on or after approval.expiresAt");
    }
    assert_resolution_has_no_review_override_or_replay_artifacts(
        resolution,
        "expired-request state cannot include review or override artifacts",
    )
}

fn validate_policy_approval_resolution_replay_rejected_state(
    resolution: &PolicyContractApprovalResolution,
) -> PolicyContractValidationResult {
    if resolution.replay_of_approval_id.is_none() {
        return Err("replay-rejected state requires replayOfApprovalId");
    }
    assert_resolution_has_no_review_or_override_artifacts(
        resolution,
        "replay-rejected state cannot include review or override artifacts",
    )
}

fn validate_policy_approval_resolution_denied_state(
    resolution: &PolicyContractApprovalResolution,
) -> PolicyContractValidationResult {
    if resolution.reviewed_by_actor_id.is_none()
        || resolution.reviewed_at.is_none()
        || resolution.audit_reference_id.is_none()
    {
        return Err("denied approvals require review and audit artifacts");
    }
    if resolution.override_grant.is_some() || resolution.replay_of_approval_id.is_some() {
        return Err("denied approvals cannot include overrides or replay pointers");
    }

    Ok(())
}

fn validate_policy_approval_resolution_reviewed_state(
    resolution: &PolicyContractApprovalResolution,
) -> PolicyContractValidationResult {
    if resolution.reviewed_by_actor_id.is_none()
        || resolution.reviewed_at.is_none()
        || resolution.audit_reference_id.is_none()
        || resolution.override_grant.is_none()
    {
        return Err(
            "approved and modified approvals require review, audit, and override artifacts",
        );
    }
    if resolution.replay_of_approval_id.is_some() {
        return Err("approved and modified approvals cannot point at replayOfApprovalId");
    }
    if resolution.reviewed_by_actor_id.as_deref()
        == Some(resolution.approval.child_profile_id.as_str())
    {
        return Err("child requests cannot self-approve or self-modify");
    }
    let Some(override_grant) = resolution.override_grant.as_ref() else {
        return Err(
            "approved and modified approvals require review, audit, and override artifacts",
        );
    };
    validate_policy_override_grant(
        override_grant,
        &resolution.approval,
        &resolution.evaluated_at,
    )
}

fn validate_policy_schedule_boundary_optional_sections(
    boundary: &PolicyContractScheduleBoundary,
) -> PolicyContractValidationResult {
    if let Some(exception) = &boundary.exception {
        assert_utc_timestamp(&exception.starts_at, "exception.startsAt")?;
        assert_utc_timestamp(&exception.expires_at, "exception.expiresAt")?;
        if exception.expires_at <= exception.starts_at {
            return Err("schedule exceptions must expire after they start");
        }
    }
    if let Some(expiry) = &boundary.expiry {
        assert_utc_timestamp(&expiry.expires_at, "expiry.expiresAt")?;
        assert_utc_timestamp(&expiry.expired_at, "expiry.expiredAt")?;
        if expiry.expired_at < expiry.expires_at {
            return Err("expiry.expiredAt must be on or after expiry.expiresAt");
        }
        if boundary.state != PolicyContractScheduleBoundaryState::Expired
            && boundary.evaluated_at >= expiry.expires_at
        {
            return Err("non-expired schedule boundaries cannot be evaluated after expiry");
        }
    }
    Ok(())
}

fn validate_policy_approval_request(
    request: &PolicyContractApprovalRequest,
) -> PolicyContractValidationResult {
    assert_utc_timestamp(&request.requested_at, "approval.requestedAt")?;
    assert_utc_timestamp(&request.expires_at, "approval.expiresAt")?;
    if request.expires_at <= request.requested_at {
        return Err("approval.expiresAt must be after approval.requestedAt");
    }

    if let Some(schedule_boundary) = &request.schedule_boundary {
        validate_policy_schedule_boundary(schedule_boundary)?;
    }

    match request.kind {
        PolicyContractApprovalKind::BonusTime => {
            if request.requested_bonus_time_minutes.unwrap_or(0) == 0 {
                return Err(
                    "bonus-time requests must include a positive requestedBonusTimeMinutes value",
                );
            }
            let Some(schedule_boundary) = &request.schedule_boundary else {
                return Err("bonus-time requests must include scheduleBoundary details");
            };
            if schedule_boundary.time_budget.is_none() {
                return Err("bonus-time requests must include scheduleBoundary.timeBudget details");
            }
        }
        PolicyContractApprovalKind::AskParent | PolicyContractApprovalKind::TemporaryOverride => {
            if request.requested_bonus_time_minutes.is_some() {
                return Err("only bonus-time requests may include requestedBonusTimeMinutes");
            }
        }
    }

    Ok(())
}

fn validate_policy_override_grant(
    grant: &PolicyContractOverrideGrant,
    approval: &PolicyContractApprovalRequest,
    evaluated_at: &str,
) -> PolicyContractValidationResult {
    assert_utc_timestamp(&grant.effective_from, "override.effectiveFrom")?;
    assert_utc_timestamp(&grant.effective_until, "override.effectiveUntil")?;
    if grant.effective_until <= grant.effective_from {
        return Err("override.effectiveUntil must be after override.effectiveFrom");
    }

    match grant.override_type {
        PolicyContractOverrideType::TemporaryAllow => {
            if grant.action != PolicyContractAction::Allow || grant.bonus_time_minutes.is_some() {
                return Err("temporary-allow overrides must resolve to allow without bonus time");
            }
        }
        PolicyContractOverrideType::TemporaryBlock => {
            if grant.action != PolicyContractAction::Block || grant.bonus_time_minutes.is_some() {
                return Err("temporary-block overrides must resolve to block without bonus time");
            }
        }
        PolicyContractOverrideType::BonusTime => {
            if approval.kind != PolicyContractApprovalKind::BonusTime {
                return Err("bonus-time overrides require a bonus-time approval request");
            }
            if !matches!(
                grant.action,
                PolicyContractAction::Allow | PolicyContractAction::TimeLimit
            ) {
                return Err("bonus-time overrides must keep the action within allow or time-limit");
            }
        }
    }

    match grant.state {
        PolicyContractOverrideState::Active => {
            if evaluated_at < grant.effective_from.as_str()
                || evaluated_at >= grant.effective_until.as_str()
            {
                return Err("active overrides require evaluatedAt within the effective window");
            }
        }
        PolicyContractOverrideState::Expired => {
            if evaluated_at < grant.effective_until.as_str() {
                return Err("expired overrides require evaluatedAt on or after effectiveUntil");
            }
        }
        PolicyContractOverrideState::Revoked => {
            if evaluated_at < grant.effective_from.as_str() {
                return Err("revoked overrides require an effectiveFrom boundary");
            }
        }
    }

    Ok(())
}
