use serde::{Deserialize, Serialize};

use crate::constants::policy_control;

use super::{
    policy::{ParentEvidenceReference, PolicyDecision, PolicyTarget},
    policy_context::LocalAiParentRuleContextRef,
};

fn protocol_lookup<T: Copy, const N: usize>(
    value: impl AsRef<str>,
    variants: [(&'static str, T); N],
) -> Option<T> {
    let value = value.as_ref();
    variants
        .into_iter()
        .find_map(|(protocol, variant)| (value == protocol).then_some(variant))
}

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
    const PROTOCOL_STRINGS: [&'static str; 3] = ["preview-required", "ready-to-save", "blocked"];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
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
    const PROTOCOL_STRINGS: [&'static str; 2] = ["required", "not-required"];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
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
    const PROTOCOL_STRINGS: [&'static str; 5] = [
        "supported",
        "unsupported",
        "manual-required",
        "offline",
        "stale",
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
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
    const PROTOCOL_STRINGS: [&'static str; 10] = [
        "overlapping-schedule",
        "timezone-boundary",
        "ambiguous-local-time",
        "nonexistent-local-time",
        "clock-skew",
        "unsupported-target",
        "manual-required-target",
        "offline-target",
        "stale-target",
        "stale-source-document",
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
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
    const PROTOCOL_STRINGS: [&'static str; 14] = [
        policy_control::source::STATUS_DRAFT,
        policy_control::source::STATUS_PREVIEW,
        policy_control::source::STATUS_CONFIRMED,
        policy_control::source::STATUS_QUEUED,
        policy_control::source::STATUS_DELIVERED,
        policy_control::source::STATUS_ACKNOWLEDGED,
        policy_control::source::STATUS_ACTIVE,
        policy_control::source::STATUS_PARTIALLY_ACTIVE,
        policy_control::source::STATUS_REJECTED,
        policy_control::source::STATUS_SUPERSEDED,
        policy_control::source::STATUS_ROLLED_BACK,
        policy_control::source::STATUS_STALE,
        policy_control::source::STATUS_EXPIRED,
        policy_control::source::STATUS_MANUAL_REQUIRED,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }

    pub fn from_protocol_str(value: impl AsRef<str>) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (policy_control::source::STATUS_DRAFT, Self::Draft),
                (policy_control::source::STATUS_PREVIEW, Self::Preview),
                (policy_control::source::STATUS_CONFIRMED, Self::Confirmed),
                (policy_control::source::STATUS_QUEUED, Self::Queued),
                (policy_control::source::STATUS_DELIVERED, Self::Delivered),
                (
                    policy_control::source::STATUS_ACKNOWLEDGED,
                    Self::Acknowledged,
                ),
                (policy_control::source::STATUS_ACTIVE, Self::Active),
                (
                    policy_control::source::STATUS_PARTIALLY_ACTIVE,
                    Self::PartiallyActive,
                ),
                (policy_control::source::STATUS_REJECTED, Self::Rejected),
                (policy_control::source::STATUS_SUPERSEDED, Self::Superseded),
                (policy_control::source::STATUS_ROLLED_BACK, Self::RolledBack),
                (policy_control::source::STATUS_STALE, Self::Stale),
                (policy_control::source::STATUS_EXPIRED, Self::Expired),
                (
                    policy_control::source::STATUS_MANUAL_REQUIRED,
                    Self::ManualRequired,
                ),
            ],
        )
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
    const PROTOCOL_STRINGS: [&'static str; 4] = [
        policy_control::source::SURFACE_PARENT_PORTAL,
        policy_control::source::SURFACE_PARENT_COMPANION,
        policy_control::source::SURFACE_AI_PREVIEW,
        policy_control::source::SURFACE_DOMAIN_CACHE,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }

    pub fn from_protocol_str(value: impl AsRef<str>) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    policy_control::source::SURFACE_PARENT_PORTAL,
                    Self::ParentPortal,
                ),
                (
                    policy_control::source::SURFACE_PARENT_COMPANION,
                    Self::ParentCompanion,
                ),
                (policy_control::source::SURFACE_AI_PREVIEW, Self::AiPreview),
                (
                    policy_control::source::SURFACE_DOMAIN_CACHE,
                    Self::DomainCache,
                ),
            ],
        )
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
    const PROTOCOL_STRINGS: [&'static str; 2] = ["child", "assistant-draft"];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }

    pub fn from_protocol_str(value: impl AsRef<str>) -> Option<Self> {
        protocol_lookup(
            value,
            [
                ("child", Self::Child),
                ("assistant-draft", Self::AssistantDraft),
            ],
        )
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
    const PROTOCOL_STRINGS: [&'static str; 3] = [
        "not-required",
        "parent-confirmation-required",
        "parent-confirmed",
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }

    pub fn from_protocol_str(value: impl AsRef<str>) -> Option<Self> {
        protocol_lookup(
            value,
            [
                ("not-required", Self::NotRequired),
                (
                    "parent-confirmation-required",
                    Self::ParentConfirmationRequired,
                ),
                ("parent-confirmed", Self::ParentConfirmed),
            ],
        )
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
    #[serde(rename = "replay-rejected")]
    ReplayRejected,
}

impl PolicyRequestStatus {
    const PROTOCOL_STRINGS: [&'static str; 7] = [
        policy_control::request::STATUS_PREVIEW_ONLY,
        policy_control::request::STATUS_PENDING_PARENT_REVIEW,
        policy_control::request::STATUS_APPROVED,
        policy_control::request::STATUS_DENIED,
        policy_control::request::STATUS_MODIFIED,
        policy_control::request::STATUS_EXPIRED,
        policy_control::request::STATUS_REPLAY_REJECTED,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }

    pub fn from_protocol_str(value: impl AsRef<str>) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    policy_control::request::STATUS_PREVIEW_ONLY,
                    Self::PreviewOnly,
                ),
                (
                    policy_control::request::STATUS_PENDING_PARENT_REVIEW,
                    Self::PendingParentReview,
                ),
                (policy_control::request::STATUS_APPROVED, Self::Approved),
                (policy_control::request::STATUS_DENIED, Self::Denied),
                (policy_control::request::STATUS_MODIFIED, Self::Modified),
                (policy_control::request::STATUS_EXPIRED, Self::Expired),
                (
                    policy_control::request::STATUS_REPLAY_REJECTED,
                    Self::ReplayRejected,
                ),
            ],
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyPreviewConfirmationContext {
    pub request_id: Option<String>,
    pub submission_key: Option<String>,
    pub household_id: Option<String>,
    pub child_profile_id: Option<String>,
    pub device_id: Option<String>,
    pub source_document_id: Option<String>,
    pub policy_version: Option<u64>,
    pub target_reference_id: Option<String>,
    pub rule_id: Option<String>,
    pub requested_at: Option<String>,
    pub expires_at: Option<String>,
    pub assistant_preview_id: Option<String>,
    pub audit_reference_ids: Option<String>,
    pub actor_id: Option<String>,
    pub actor_role: Option<String>,
    pub actor_state: Option<String>,
    pub confirmation_audit_reference_id: Option<String>,
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
    pub policy_approval_id: Option<String>,
    pub policy_override_id: Option<String>,
    pub policy_replay_of_approval_id: Option<String>,
    pub policy_reviewed_by_actor_id: Option<String>,
    pub policy_reviewed_by_actor_role: Option<String>,
    pub policy_reviewed_at: Option<String>,
    pub policy_audit_reference_id: Option<String>,
    pub network_evidence_mapping: Option<PolicyPreviewNetworkEvidenceMapping>,
    pub confirmation_context: Option<PolicyPreviewConfirmationContext>,
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
