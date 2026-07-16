#[path = "ai_audit/build.rs"]
mod build;
#[path = "ai_audit/claims.rs"]
mod claims;
#[path = "ai_audit/recommendations.rs"]
mod recommendations;
#[path = "ai_audit/refs.rs"]
mod refs;

use serde::{Deserialize, Serialize};

use crate::ai_detection::{
    NetworkAiDetectionResult, NetworkAiDetectionRiskLevel, NetworkAiDetectionUncertaintyCode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAiAuditNarrativeState {
    Ready,
    UncertainReviewRequired,
    MonitorOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAiAuditRecommendationKind {
    ReviewWithParent,
    ConfirmWithManagedBrowser,
    ConfirmWithScreenSummary,
    ReviewPolicyRule,
    MonitorOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAiAuditUncertaintyCode {
    DetectionMismatch,
    FalsePositiveFixture,
    FalseNegativeFixture,
    UnknownPrediction,
    ConfidenceDriftExceeded,
    LowConfidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAiAuditReportInput {
    pub audit_report_ref: String,
    pub narrative_template_ref: String,
    pub model_version_ref: String,
    pub policy_context_ref: String,
    pub detection_results: Vec<NetworkAiDetectionResult>,
    pub parent_rule_refs: Vec<String>,
    pub remote_ai_claimed: bool,
    pub raw_pcap_input_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub exact_url_claimed: bool,
    pub private_message_claimed: bool,
    pub search_query_claimed: bool,
    pub policy_authority_claimed: bool,
    pub adapter_authority_claimed: bool,
    pub enforcement_command_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAiAuditRecommendation {
    pub recommendation_ref: String,
    pub kind: NetworkAiAuditRecommendationKind,
    pub cited_detection_refs: Vec<String>,
    pub cited_evidence_refs: Vec<String>,
    pub cited_parent_rule_refs: Vec<String>,
    pub advisory_only: bool,
    pub policy_authority: bool,
    pub adapter_authority: bool,
    pub enforcement_command_published: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkAiAuditReport {
    pub audit_report_ref: String,
    pub narrative_template_ref: String,
    pub model_version_ref: String,
    pub policy_context_ref: String,
    pub narrative_state: NetworkAiAuditNarrativeState,
    pub narrative_headline: String,
    pub cited_detection_refs: Vec<String>,
    pub cited_evidence_refs: Vec<String>,
    pub cited_analyzer_alert_refs: Vec<String>,
    pub cited_parent_rule_refs: Vec<String>,
    pub uncertainty_codes: Vec<NetworkAiAuditUncertaintyCode>,
    pub recommendations: Vec<NetworkAiAuditRecommendation>,
    pub parent_readable: bool,
    pub advisory_only: bool,
    pub raw_pcap_available: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
    pub private_message_available: bool,
    pub search_query_available: bool,
    pub remote_ai_used: bool,
    pub policy_authority: bool,
    pub adapter_authority: bool,
    pub enforcement_commands_published: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkAiAuditReportError {
    EmptyAuditReportRef,
    EmptyNarrativeTemplateRef,
    EmptyModelVersionRef,
    EmptyPolicyContextRef,
    EmptyDetectionResults,
    EmptyDetectionRef,
    DuplicateDetectionRef,
    EmptyEvidenceRefs,
    EmptyEvidenceRef,
    EmptyAnalyzerAlertRef,
    EmptyParentRuleRefs,
    EmptyParentRuleRef,
    RemoteAiClaimRejected,
    RawPcapInputRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    ExactUrlClaimRejected,
    PrivateMessageClaimRejected,
    SearchQueryClaimRejected,
    PolicyAuthorityClaimRejected,
    AdapterAuthorityClaimRejected,
    EnforcementCommandClaimRejected,
}

pub fn build_network_ai_audit_report(
    input: &NetworkAiAuditReportInput,
) -> Result<NetworkAiAuditReport, NetworkAiAuditReportError> {
    build::build_network_ai_audit_report(input)
}
