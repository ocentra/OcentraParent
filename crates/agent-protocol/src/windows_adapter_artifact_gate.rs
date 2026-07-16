use serde::{Deserialize, Serialize};

use crate::{
    constants::windows_adapter_artifact_gate as artifact_gate_constants,
    WindowsAdapterCapabilitySurface,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum WindowsAdapterArtifactKind {
    #[serde(rename = "same-identity-app-package-evidence")]
    SameIdentityAppPackageEvidence,
    #[serde(rename = "adapter-apply-result")]
    AdapterApplyResult,
    #[serde(rename = "adapter-rollback-result")]
    AdapterRollbackResult,
    #[serde(rename = "audit-custody-event")]
    AuditCustodyEvent,
    #[serde(rename = "managed-browser-exact-url-evidence")]
    ManagedBrowserExactUrlEvidence,
    #[serde(rename = "network-domain-filter-apply")]
    NetworkDomainFilterApply,
    #[serde(rename = "network-domain-filter-rollback")]
    NetworkDomainFilterRollback,
}

impl WindowsAdapterArtifactKind {
    const PROTOCOL_STRINGS: [&'static str; 7] = [
        artifact_gate_constants::ARTIFACT_KIND_SAME_IDENTITY_APP,
        artifact_gate_constants::ARTIFACT_KIND_APPLY_RESULT,
        artifact_gate_constants::ARTIFACT_KIND_ROLLBACK_RESULT,
        artifact_gate_constants::ARTIFACT_KIND_AUDIT_CUSTODY_EVENT,
        artifact_gate_constants::ARTIFACT_KIND_MANAGED_BROWSER_EXACT_URL,
        artifact_gate_constants::ARTIFACT_KIND_NETWORK_FILTER_APPLY,
        artifact_gate_constants::ARTIFACT_KIND_NETWORK_FILTER_ROLLBACK,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum WindowsAdapterArtifactGateDecision {
    #[serde(rename = "refused-missing-artifacts")]
    RefusedMissingArtifacts,
    #[serde(rename = "refused-unsupported-surface")]
    RefusedUnsupportedSurface,
    #[serde(rename = "ready-for-manual-review")]
    ReadyForManualReview,
}

impl WindowsAdapterArtifactGateDecision {
    const PROTOCOL_STRINGS: [&'static str; 3] = [
        artifact_gate_constants::DECISION_REFUSED_MISSING_ARTIFACTS,
        artifact_gate_constants::DECISION_REFUSED_UNSUPPORTED_SURFACE,
        artifact_gate_constants::DECISION_READY_FOR_MANUAL_REVIEW,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsAdapterArtifactEvidence {
    pub schema_version: String,
    pub artifact_id: String,
    pub artifact_kind: WindowsAdapterArtifactKind,
    pub surface: WindowsAdapterCapabilitySurface,
    pub subject_ref: String,
    pub custody_event_id: Option<String>,
    pub verified_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsAdapterArtifactGateEntry {
    pub schema_version: String,
    pub gate_entry_id: String,
    pub capability_entry_id: String,
    pub surface: WindowsAdapterCapabilitySurface,
    pub required_artifact_kinds: Vec<WindowsAdapterArtifactKind>,
    pub present_artifact_ids: Vec<String>,
    pub missing_artifact_kinds: Vec<WindowsAdapterArtifactKind>,
    pub refusal_reasons: Vec<String>,
    pub decision: WindowsAdapterArtifactGateDecision,
    pub ready_for_manual_review: bool,
    pub claim_upgrade_allowed: bool,
    pub product_claim_boundary: String,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsAdapterArtifactGateProof {
    pub schema_version: String,
    pub read_model_id: String,
    pub generated_at: String,
    pub capability_read_model_id: String,
    pub entries: Vec<WindowsAdapterArtifactGateEntry>,
}
