use serde::{Deserialize, Serialize};

use crate::schema_domain_mirrors::{ai::*, family::*, policy::*};

macro_rules! text_identifier {
    ($name:ident) => {
        #[derive(
            Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_owned())
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }
    };
}

text_identifier!(LocalAiEvidenceContextRefId);
text_identifier!(LocalAiEvidenceSourceId);
text_identifier!(LocalAiEvidenceAdapterId);
text_identifier!(LocalAiParentRuleContextRefId);
text_identifier!(LocalAiEvidenceContextId);
text_identifier!(LocalAiEvidenceContextSummary);
text_identifier!(LocalAiRejectedField);

pub type LocalAiProviderPrivacyMode = crate::local_ai_runtime_boundary::LocalAiProviderPrivacyMode;
pub type LocalAiAdapterBoundary = crate::local_ai_runtime_boundary::LocalAiAdapterBoundary;
pub type LocalAiExecutionState = crate::local_ai_runtime_boundary::LocalAiExecutionState;
pub type LocalAiProviderSource = crate::local_ai_runtime_boundary::LocalAiProviderSource;
pub type LocalAiAdapterProbeState = crate::local_ai_runtime_boundary::LocalAiAdapterProbeState;
pub type LocalAiProviderConfigurationState =
    crate::local_ai_runtime_boundary::LocalAiProviderConfigurationState;
pub type LocalAiAdapterReadinessState =
    crate::local_ai_runtime_boundary::LocalAiAdapterReadinessState;
pub type LocalAiMemoryReferenceKind = crate::activity::local_ai::LocalAiMemoryReferenceKind;
pub type LocalAiGraphReferenceKind = crate::activity::local_ai::LocalAiGraphReferenceKind;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalAiEvidenceRetentionState {
    Local,
    Temporary,
    DeletedSource,
    ExportCopy,
    ParentOwnedCopy,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalAiConfidenceKind {
    Observation,
    Correlation,
    Classifier,
    Model,
    MemoryMatch,
    GraphEdge,
    RuleMatch,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalAiContextCapabilityStatus {
    Available,
    Unsupported,
    PermissionLimited,
    Stale,
    Degraded,
    AdapterError,
    DisabledByParent,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalAiContextReasonCode {
    MissingEvidence,
    StaleEvidence,
    SourceConflict,
    UnsupportedSource,
    PermissionLimited,
    AdapterError,
    CapabilityDisabledByParent,
    CustodyUnavailable,
    ForbiddenRemoteSource,
    InvalidConfidence,
    InvalidAiOutput,
    ModelUnavailable,
    ModelOverloaded,
    ModelOutputUnparseable,
    MemoryUngrounded,
    GraphUngrounded,
    ParentRuleMissing,
    ParentRuleConflict,
    ScheduleUnresolved,
    ProtectedSurface,
    ScreenImageDeleted,
    ScreenDeletionUnconfirmed,
    NetworkEncryptedContentUnavailable,
    BrowserActiveTabUnknown,
    AppDurationIncomplete,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalAiRequestedEvaluationKind {
    Page,
    Url,
    Video,
    App,
    Game,
    Domain,
    NetworkDigest,
    ScreenSummary,
    RecentActivity,
    MixedContext,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalModelRuntimeStatusWire {
    pub runtime_reference_id: LocalAiRuntimeReferenceId,
    pub provider_id: LocalAiProviderId,
    pub model_id: LocalAiModelId,
    pub model_reference: LocalAiModelReference,
    pub privacy_mode: LocalAiProviderPrivacyMode,
    pub adapter_boundary: LocalAiAdapterBoundary,
    pub execution_state: LocalAiExecutionState,
    pub provider_source: LocalAiProviderSource,
    pub load_state: LocalAiModelLoadState,
    pub capability_flags: Vec<LocalAiCapabilityFlag>,
    pub resource_class: LocalAiResourceClass,
    pub degraded_state: LocalAiDegradedState,
    pub last_checked_at: LocalAiTimestamp,
    pub unavailable_reason: Option<LocalAiUnavailableReason>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalProviderAdapterProbeWire {
    pub provider_id: LocalAiProviderId,
    pub privacy_mode: LocalAiProviderPrivacyMode,
    pub adapter_boundary: LocalAiAdapterBoundary,
    pub execution_state: LocalAiExecutionState,
    pub provider_source: LocalAiProviderSource,
    pub probe_state: LocalAiAdapterProbeState,
    pub configuration_state: LocalAiProviderConfigurationState,
    pub readiness_state: LocalAiAdapterReadinessState,
    pub execution_allowed: bool,
    pub last_checked_at: LocalAiTimestamp,
    pub unavailable_reason: Option<LocalAiUnavailableReason>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalAiMemoryReferenceWire {
    pub memory_reference_id: LocalAiMemoryReferenceId,
    pub kind: LocalAiMemoryReferenceKind,
    pub source_evidence_references: Vec<ParentEvidenceReference>,
    pub source_policy_version: Option<ParentPolicyVersion>,
    pub generated_at: LocalAiTimestamp,
    pub confidence: f64,
    pub derived_index_version: LocalAiDerivedIndexVersion,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalAiGraphReferenceWire {
    pub graph_reference_id: LocalAiGraphReferenceId,
    pub kind: LocalAiGraphReferenceKind,
    pub source_evidence_references: Vec<ParentEvidenceReference>,
    pub source_policy_version: Option<ParentPolicyVersion>,
    pub generated_at: LocalAiTimestamp,
    pub confidence: f64,
    pub derived_index_version: LocalAiDerivedIndexVersion,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalAiEvidenceContextSourceRefWire {
    pub evidence_ref_id: LocalAiEvidenceContextRefId,
    pub evidence: ParentEvidenceReference,
    pub evidence_kind: LocalAiEvidenceContextKind,
    pub source_schema_version: ParentContractSchemaVersion,
    pub observed_at: LocalAiTimestamp,
    pub ingested_at: Option<LocalAiTimestamp>,
    pub fresh_until: Option<LocalAiTimestamp>,
    pub source_id: LocalAiEvidenceSourceId,
    pub adapter_id: LocalAiEvidenceAdapterId,
    pub device: ParentDeviceReference,
    pub child_profile: ChildProfileReference,
    pub custody: LocalAiEvidenceCustody,
    pub retention_state: LocalAiEvidenceRetentionState,
    pub confidence: Option<f64>,
    pub confidence_kind: Option<LocalAiConfidenceKind>,
    pub capability_status: LocalAiContextCapabilityStatus,
    pub degraded_reasons: Vec<LocalAiContextReasonCode>,
    pub unknown_reasons: Vec<LocalAiContextReasonCode>,
    pub source_evidence_references: Vec<ParentEvidenceReference>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalAiEvidenceContextValidationSummaryWire {
    pub evidence_reference_count: u64,
    pub source_evidence_reference_count: u64,
    pub runtime_reference_count: u64,
    pub memory_reference_count: u64,
    pub graph_reference_count: u64,
    pub parent_rule_reference_count: u64,
    pub ungrounded_parent_rule_reference_count: u64,
    pub forbidden_custody_reference_count: u64,
    pub unallowed_custody_reference_count: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalAiEvidenceContextWire {
    pub schema_version: ParentContractSchemaVersion,
    pub context_id: LocalAiEvidenceContextId,
    pub request_id: LocalAiEvaluationRequestId,
    pub child_profile: ChildProfileReference,
    pub device: ParentDeviceReference,
    pub evidence_references: Vec<LocalAiEvidenceContextSourceRefWire>,
    pub browser_evidence_refs: Vec<LocalAiEvidenceContextRefId>,
    pub app_game_evidence_refs: Vec<LocalAiEvidenceContextRefId>,
    pub network_flow_evidence_refs: Vec<LocalAiEvidenceContextRefId>,
    pub screen_summary_refs: Vec<LocalAiEvidenceContextRefId>,
    pub parent_rule_references: Vec<PolicyRuleId>,
    pub parent_rule_context_references: Vec<LocalAiParentRuleContextRef>,
    pub recent_activity_summary_refs: Vec<LocalAiEvidenceContextRefId>,
    pub memory_references: Vec<LocalAiMemoryReferenceWire>,
    pub graph_references: Vec<LocalAiGraphReferenceWire>,
    pub local_model_runtime_refs: Vec<LocalAiRuntimeReferenceId>,
    pub prompt_version: LocalAiPromptVersion,
    pub custody_labels: Vec<LocalAiEvidenceCustody>,
    pub degraded_reasons: Vec<LocalAiContextReasonCode>,
    pub unknown_reasons: Vec<LocalAiContextReasonCode>,
    pub validation_summary: LocalAiEvidenceContextValidationSummaryWire,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalAiEvidenceContextBuildRequestWire {
    pub schema_version: ParentContractSchemaVersion,
    pub request_id: LocalAiEvaluationRequestId,
    pub requested_at: LocalAiTimestamp,
    pub child_profile: ChildProfileReference,
    pub device: ParentDeviceReference,
    pub requested_evaluation_kind: LocalAiRequestedEvaluationKind,
    pub required_evidence_kinds: Vec<LocalAiEvidenceContextKind>,
    pub parent_rule_context_references: Vec<LocalAiParentRuleContextRef>,
    pub model_task_requirements: Vec<LocalAiCapabilityFlag>,
    pub allowed_custody: Vec<LocalAiEvidenceCustody>,
    pub prompt_version: LocalAiPromptVersion,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalAiEvidenceContextBuildResultWire {
    pub schema_version: ParentContractSchemaVersion,
    pub request_id: LocalAiEvaluationRequestId,
    pub state: LocalAiContextBuildState,
    pub context: Option<LocalAiEvidenceContextWire>,
    pub rejected_fields: Vec<LocalAiRejectedField>,
    pub missing_evidence_kinds: Vec<LocalAiEvidenceContextKind>,
    pub degraded_source_refs: Vec<LocalAiEvidenceContextRefId>,
    pub custody_boundary_summary: LocalAiEvidenceContextSummary,
    pub validation_gate_summary: LocalAiEvidenceContextSummary,
    pub audit_evidence_references: Vec<ParentEvidenceReference>,
}
