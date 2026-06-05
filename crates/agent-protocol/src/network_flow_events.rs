use serde::{Deserialize, Serialize};

pub trait NetworkRuntimeEventContract {
    const EVENT_TYPE: &'static str;
    const SCHEMA_VERSION: u16 = crate::constants::network_flow::EVENT_SCHEMA_VERSION;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkClaimBoundary {
    pub exact_url_available: bool,
    pub decrypted_https_payload_available: bool,
    pub message_content_available: bool,
    pub search_query_available: bool,
    pub adapter_action_executed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkFlowObservedEvent {
    pub schema_version: u16,
    pub flow_event_ref: String,
    pub observed_at: String,
    pub device_ref: String,
    pub flow_evidence_ref: String,
    pub custody: String,
    pub evidence_grade: NetworkEvidenceGrade,
    pub claim_boundary: NetworkClaimBoundary,
}

impl NetworkRuntimeEventContract for NetworkFlowObservedEvent {
    const EVENT_TYPE: &'static str = crate::constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkDomainObservedEvent {
    pub schema_version: u16,
    pub domain_event_ref: String,
    pub previous_event_ref: String,
    pub flow_evidence_ref: String,
    pub domain_evidence_ref: String,
    pub attribution: NetworkDomainAttributionKind,
    pub evidence_grade: NetworkEvidenceGrade,
    pub uncertainty_codes: Vec<String>,
    pub claim_boundary: NetworkClaimBoundary,
}

impl NetworkRuntimeEventContract for NetworkDomainObservedEvent {
    const EVENT_TYPE: &'static str = crate::constants::network_flow::EVENT_NETWORK_DOMAIN_OBSERVED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkActivityClassifiedEvent {
    pub schema_version: u16,
    pub classification_event_ref: String,
    pub previous_event_ref: String,
    pub evidence_refs: Vec<String>,
    pub activity_kind: NetworkActivityKind,
    pub confidence: f32,
    pub evidence_grade: NetworkEvidenceGrade,
    pub uncertainty_codes: Vec<String>,
}

impl NetworkRuntimeEventContract for NetworkActivityClassifiedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::network_flow::EVENT_NETWORK_ACTIVITY_CLASSIFIED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkAiAnalysisRequestedEvent {
    pub schema_version: u16,
    pub ai_request_ref: String,
    pub previous_event_ref: String,
    pub evidence_refs: Vec<String>,
    pub prompt_template_ref: String,
    pub custody: String,
    pub raw_packet_payload_included: bool,
}

impl NetworkRuntimeEventContract for NetworkAiAnalysisRequestedEvent {
    const EVENT_TYPE: &'static str = crate::constants::network_flow::EVENT_AI_ANALYSIS_REQUESTED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkAiAnalysisCompletedEvent {
    pub schema_version: u16,
    pub ai_analysis_ref: String,
    pub ai_request_ref: String,
    pub previous_event_ref: String,
    pub advisory_state: NetworkAiAdvisoryState,
    pub evidence_refs: Vec<String>,
    pub unsupported_claims: Vec<String>,
}

impl NetworkRuntimeEventContract for NetworkAiAnalysisCompletedEvent {
    const EVENT_TYPE: &'static str = crate::constants::network_flow::EVENT_AI_ANALYSIS_COMPLETED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyEvaluationRequestedEvent {
    pub schema_version: u16,
    pub policy_evaluation_ref: String,
    pub previous_event_ref: String,
    pub evidence_refs: Vec<String>,
    pub ai_analysis_ref: Option<String>,
    pub parent_rule_refs: Vec<String>,
    pub dry_run: bool,
}

impl NetworkRuntimeEventContract for NetworkPolicyEvaluationRequestedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::network_flow::EVENT_POLICY_EVALUATION_REQUESTED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyDecisionCompletedEvent {
    pub schema_version: u16,
    pub policy_decision_ref: String,
    pub policy_evaluation_ref: String,
    pub previous_event_ref: String,
    pub decision_action: NetworkPolicyDecisionAction,
    pub evidence_refs: Vec<String>,
    pub parent_rule_refs: Vec<String>,
    pub adapter_capability_required: bool,
}

impl NetworkRuntimeEventContract for NetworkPolicyDecisionCompletedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::network_flow::EVENT_POLICY_DECISION_COMPLETED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkEnforcementCommandIssuedEvent {
    pub schema_version: u16,
    pub enforcement_command_ref: String,
    pub previous_event_ref: String,
    pub policy_decision_ref: String,
    pub adapter_capability_ref: String,
    pub enforcement_mode: NetworkEnforcementMode,
    pub evidence_refs: Vec<String>,
    pub rollback_ref: Option<String>,
}

impl NetworkRuntimeEventContract for NetworkEnforcementCommandIssuedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkEnforcementResultObservedEvent {
    pub schema_version: u16,
    pub enforcement_result_ref: String,
    pub enforcement_command_ref: String,
    pub previous_event_ref: String,
    pub result_status: NetworkEnforcementResultStatus,
    pub adapter_action_executed: bool,
    pub rollback_ref: Option<String>,
    pub unavailable_reason_code: Option<String>,
}

impl NetworkRuntimeEventContract for NetworkEnforcementResultObservedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::network_flow::EVENT_ENFORCEMENT_RESULT_OBSERVED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkAuditEntryCommittedEvent {
    pub schema_version: u16,
    pub audit_entry_ref: String,
    pub previous_event_ref: String,
    pub policy_decision_ref: String,
    pub enforcement_command_ref: Option<String>,
    pub enforcement_result_ref: Option<String>,
    pub evidence_refs: Vec<String>,
    pub audit_outcome: NetworkAuditOutcome,
}

impl NetworkRuntimeEventContract for NetworkAuditEntryCommittedEvent {
    const EVENT_TYPE: &'static str = crate::constants::network_flow::EVENT_AUDIT_ENTRY_COMMITTED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPortalReadModelUpdatedEvent {
    pub schema_version: u16,
    pub read_model_ref: String,
    pub previous_event_ref: String,
    pub audit_entry_ref: String,
    pub update_kind: NetworkPortalUpdateKind,
    pub visible_manual_required: bool,
    pub visible_unavailable: bool,
}

impl NetworkRuntimeEventContract for NetworkPortalReadModelUpdatedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::network_flow::EVENT_PORTAL_READ_MODEL_UPDATED;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkEvidenceGrade {
    #[serde(rename = "A")]
    A,
    #[serde(rename = "B")]
    B,
    #[serde(rename = "C")]
    C,
    #[serde(rename = "D")]
    D,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkDomainAttributionKind {
    DnsAnswer,
    SniVisible,
    HttpHost,
    ReverseLookup,
    IpOnly,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkActivityKind {
    SocialCandidate,
    VideoCandidate,
    GameCandidate,
    VpnProxyTunnelCandidate,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkAiAdvisoryState {
    Requested,
    Completed,
    ManualReviewRequired,
    ProviderUnavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkPolicyDecisionAction {
    Observe,
    Warn,
    AskParent,
    Limit,
    Block,
    ManualReview,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkEnforcementMode {
    DryRun,
    ManualRequired,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkEnforcementResultStatus {
    DryRun,
    ManualRequired,
    Unavailable,
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkAuditOutcome {
    Committed,
    Failed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkPortalUpdateKind {
    NetworkReadModel,
    CapabilityState,
    ManualRequiredState,
}
