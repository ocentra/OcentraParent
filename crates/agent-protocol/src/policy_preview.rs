use serde::{Deserialize, Serialize};

use crate::{
    constants::policy_control, LocalAiParentRuleContextRef, ParentEvidenceReference,
    PolicyDecision, PolicyTarget,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyPreviewNetworkEvidenceMapping {
    pub evidence_grade: String,
    pub requested_action: String,
    pub mapped_action: String,
    pub mode: String,
    pub adapter_action_authorized: bool,
    pub enforcement_command_authorized: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyPreviewSaveState {
    #[serde(rename = "preview-required")]
    PreviewRequired,
    #[serde(rename = "ready-to-save")]
    ReadyToSave,
    #[serde(rename = "blocked")]
    Blocked,
}

impl PolicyPreviewSaveState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::PreviewRequired => "preview-required",
            Self::ReadyToSave => "ready-to-save",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyPreviewManualReviewState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

impl PolicyPreviewManualReviewState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::NotRequired => "not-required",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyPreviewTargetState {
    #[serde(rename = "supported")]
    Supported,
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "stale")]
    Stale,
}

impl PolicyPreviewTargetState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Unsupported => "unsupported",
            Self::ManualRequired => "manual-required",
            Self::Offline => "offline",
            Self::Stale => "stale",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyPreviewFindingKind {
    #[serde(rename = "overlapping-schedule")]
    OverlappingSchedule,
    #[serde(rename = "timezone-boundary")]
    TimezoneBoundary,
    #[serde(rename = "ambiguous-local-time")]
    AmbiguousLocalTime,
    #[serde(rename = "nonexistent-local-time")]
    NonexistentLocalTime,
    #[serde(rename = "clock-skew")]
    ClockSkew,
    #[serde(rename = "unsupported-target")]
    UnsupportedTarget,
    #[serde(rename = "manual-required-target")]
    ManualRequiredTarget,
    #[serde(rename = "offline-target")]
    OfflineTarget,
    #[serde(rename = "stale-target")]
    StaleTarget,
    #[serde(rename = "stale-source-document")]
    StaleSourceDocument,
}

impl PolicyPreviewFindingKind {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::OverlappingSchedule => "overlapping-schedule",
            Self::TimezoneBoundary => "timezone-boundary",
            Self::AmbiguousLocalTime => "ambiguous-local-time",
            Self::NonexistentLocalTime => "nonexistent-local-time",
            Self::ClockSkew => "clock-skew",
            Self::UnsupportedTarget => "unsupported-target",
            Self::ManualRequiredTarget => "manual-required-target",
            Self::OfflineTarget => "offline-target",
            Self::StaleTarget => "stale-target",
            Self::StaleSourceDocument => "stale-source-document",
        }
    }
}

pub fn policy_preview_finding_kinds_csv(kinds: &[PolicyPreviewFindingKind]) -> Option<String> {
    if kinds.is_empty() {
        return None;
    }

    Some(
        kinds
            .iter()
            .map(PolicyPreviewFindingKind::as_protocol_str)
            .collect::<Vec<_>>()
            .join(&crate::constants::delimiter::LIST.to_string()),
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicySourceStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "preview")]
    Preview,
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "acknowledged")]
    Acknowledged,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "partially-active")]
    PartiallyActive,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "superseded")]
    Superseded,
    #[serde(rename = "rolled-back")]
    RolledBack,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

impl PolicySourceStatus {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Draft => policy_control::source::STATUS_DRAFT,
            Self::Preview => policy_control::source::STATUS_PREVIEW,
            Self::Confirmed => policy_control::source::STATUS_CONFIRMED,
            Self::Queued => policy_control::source::STATUS_QUEUED,
            Self::Delivered => policy_control::source::STATUS_DELIVERED,
            Self::Acknowledged => policy_control::source::STATUS_ACKNOWLEDGED,
            Self::Active => policy_control::source::STATUS_ACTIVE,
            Self::PartiallyActive => policy_control::source::STATUS_PARTIALLY_ACTIVE,
            Self::Rejected => policy_control::source::STATUS_REJECTED,
            Self::Superseded => policy_control::source::STATUS_SUPERSEDED,
            Self::RolledBack => policy_control::source::STATUS_ROLLED_BACK,
            Self::Stale => policy_control::source::STATUS_STALE,
            Self::Expired => policy_control::source::STATUS_EXPIRED,
            Self::ManualRequired => policy_control::source::STATUS_MANUAL_REQUIRED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicySourceSurface {
    #[serde(rename = "parent-portal")]
    ParentPortal,
    #[serde(rename = "parent-companion")]
    ParentCompanion,
    #[serde(rename = "ai-preview")]
    AiPreview,
    #[serde(rename = "domain-cache")]
    DomainCache,
}

impl PolicySourceSurface {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ParentPortal => policy_control::source::SURFACE_PARENT_PORTAL,
            Self::ParentCompanion => policy_control::source::SURFACE_PARENT_COMPANION,
            Self::AiPreview => policy_control::source::SURFACE_AI_PREVIEW,
            Self::DomainCache => policy_control::source::SURFACE_DOMAIN_CACHE,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyRequestOrigin {
    #[serde(rename = "child")]
    Child,
    #[serde(rename = "assistant-draft")]
    AssistantDraft,
}

impl PolicyRequestOrigin {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Child => "child",
            Self::AssistantDraft => "assistant-draft",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyAssistantConfirmationState {
    #[serde(rename = "not-required")]
    NotRequired,
    #[serde(rename = "parent-confirmation-required")]
    ParentConfirmationRequired,
    #[serde(rename = "parent-confirmed")]
    ParentConfirmed,
}

impl PolicyAssistantConfirmationState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::NotRequired => "not-required",
            Self::ParentConfirmationRequired => "parent-confirmation-required",
            Self::ParentConfirmed => "parent-confirmed",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyRequestStatus {
    #[serde(rename = "preview-only")]
    PreviewOnly,
    #[serde(rename = "pending-parent-review")]
    PendingParentReview,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "denied")]
    Denied,
    #[serde(rename = "modified")]
    Modified,
    #[serde(rename = "expired")]
    Expired,
}

impl PolicyRequestStatus {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::PreviewOnly => policy_control::request::STATUS_PREVIEW_ONLY,
            Self::PendingParentReview => policy_control::request::STATUS_PENDING_PARENT_REVIEW,
            Self::Approved => policy_control::request::STATUS_APPROVED,
            Self::Denied => policy_control::request::STATUS_DENIED,
            Self::Modified => policy_control::request::STATUS_MODIFIED,
            Self::Expired => policy_control::request::STATUS_EXPIRED,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyPreviewReadModelRow {
    pub preview_id: String,
    pub source_event_id: String,
    pub observed_at: String,
    pub target: PolicyTarget,
    pub evidence_references: Vec<ParentEvidenceReference>,
    pub parent_rule_context_references: Vec<LocalAiParentRuleContextRef>,
    pub decision: PolicyDecision,
    pub policy_preview_save_state: Option<PolicyPreviewSaveState>,
    pub policy_preview_manual_review_state: Option<PolicyPreviewManualReviewState>,
    pub policy_preview_target_state: Option<PolicyPreviewTargetState>,
    pub policy_preview_target_explanation_code: Option<String>,
    pub policy_preview_finding_kinds: Option<String>,
    pub policy_source_status: Option<PolicySourceStatus>,
    pub policy_source_surface: Option<PolicySourceSurface>,
    pub policy_request_origin: Option<PolicyRequestOrigin>,
    pub policy_assistant_confirmation_state: Option<PolicyAssistantConfirmationState>,
    pub policy_request_status: Option<PolicyRequestStatus>,
    pub network_evidence_mapping: Option<PolicyPreviewNetworkEvidenceMapping>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyPreviewReadModel {
    pub schema_version: String,
    pub generated_at: String,
    pub custody: String,
    pub limit: u64,
    pub returned: u64,
    pub capability_status: String,
    pub rows: Vec<PolicyPreviewReadModelRow>,
}
