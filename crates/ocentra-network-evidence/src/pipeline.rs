use serde::{Deserialize, Serialize};

mod builders;
mod proofs;
mod validation;

use self::validation::{reject_unsupported_claims, validate_refs};

use crate::{
    build_network_ai_audit_report, build_network_cross_slice_evidence_bundle,
    evaluate_network_ai_detection_fixtures, evaluate_network_risk_budget_threshold,
    map_network_evidence_grade_to_policy, plan_network_action_result_state,
    plan_network_dns_adapter_proof, plan_network_local_ai_queue,
    NetworkActionResultAdapterProofState, NetworkActionResultCapabilityState,
    NetworkActionResultError, NetworkActionResultInput, NetworkActionResultProof,
    NetworkActionResultRequestedAction, NetworkActionResultTargetKind, NetworkAiAuditReport,
    NetworkAiAuditReportError, NetworkAiAuditReportInput, NetworkAiDetectionEvaluationError,
    NetworkAiDetectionEvaluationInput, NetworkAiDetectionEvaluationProof,
    NetworkAiDetectionFixtureCase, NetworkAiDetectionInputKind, NetworkAiDetectionLabel,
    NetworkAiDetectionRiskLevel, NetworkCrossSliceEvidenceBundle,
    NetworkCrossSliceEvidenceBundleError, NetworkCrossSliceEvidenceBundleInput,
    NetworkCrossSliceEvidenceSource, NetworkDnsAdapterAction, NetworkDnsAdapterCapabilityState,
    NetworkDnsAdapterProof, NetworkDnsAdapterProofError, NetworkDnsAdapterProofInput,
    NetworkDnsAdapterProofState, NetworkEvidencePolicyAction, NetworkEvidencePolicyMapping,
    NetworkEvidencePolicyMappingError, NetworkEvidencePolicyMappingInput, NetworkLocalAiQueueError,
    NetworkLocalAiQueueInput, NetworkLocalAiQueuePlan, NetworkRiskBudgetAdapterProofState,
    NetworkRiskBudgetAgeBand, NetworkRiskBudgetEvaluation, NetworkRiskBudgetEvidenceTier,
    NetworkRiskBudgetHouseholdPolicy, NetworkRiskBudgetPriorEvent, NetworkRiskBudgetSignal,
    NetworkRiskBudgetThresholdError, NetworkRiskBudgetThresholdInput, NetworkRiskBudgetThresholds,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkEndToEndPipelineRefs {
    pub trigger_ref: String,
    pub capture_ref: String,
    pub ingest_ref: String,
    pub typed_event_ref: String,
    pub summary_refs: Vec<String>,
    pub analyzer_alert_refs: Vec<String>,
    pub queue_job_ref: String,
    pub queue_ref: String,
    pub model_runtime_ref: String,
    pub ai_detection_ref: String,
    pub ai_fixture_ref: String,
    pub ai_evaluation_run_ref: String,
    pub ai_fixture_set_ref: String,
    pub ai_model_card_ref: String,
    pub ai_model_version_ref: String,
    pub ai_baseline_ref: String,
    pub ai_audit_report_ref: String,
    pub ai_narrative_template_ref: String,
    pub policy_context_ref: String,
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub risk_evaluation_ref: String,
    pub child_profile_ref: String,
    pub risk_budget_ref: String,
    pub cascade_ref: String,
    pub household_policy_ref: String,
    pub dns_adapter_plan_ref: String,
    pub action_result_ref: String,
    pub target_domain: String,
    pub adapter_authorization_ref: Option<String>,
    pub adapter_capability_proof_ref: Option<String>,
    pub apply_artifact_ref: Option<String>,
    pub result_artifact_ref: Option<String>,
    pub rollback_artifact_ref: Option<String>,
    pub audit_event_ref: String,
    pub portal_read_model_ref: String,
    pub retention_ref: String,
    pub deletion_ref: String,
    pub export_ref: String,
    pub tombstone_ref: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkEndToEndUnsupportedClaims {
    pub raw_network_payload_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub exact_url_claimed: bool,
    pub ai_policy_authority_claimed: bool,
    pub ui_policy_authority_claimed: bool,
    pub network_adapter_authority_claimed: bool,
    pub enforcement_command_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkEndToEndPipelineInput {
    pub refs: NetworkEndToEndPipelineRefs,
    pub sources: Vec<NetworkCrossSliceEvidenceSource>,
    pub requested_policy_action: NetworkEvidencePolicyAction,
    pub adapter_capability_state: NetworkDnsAdapterCapabilityState,
    pub adapter_dry_run: bool,
    pub local_ai_enabled: bool,
    pub model_runtime_available: bool,
    pub queue_available: bool,
    pub unsupported_claims: NetworkEndToEndUnsupportedClaims,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRetentionDeleteExportProof {
    pub retention_ref: String,
    pub deletion_ref: String,
    pub export_ref: String,
    pub tombstone_ref: String,
    pub audit_event_ref: String,
    pub portal_read_model_ref: String,
    pub evidence_refs: Vec<String>,
    pub same_product_path: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkCaptureIngestProof {
    pub trigger_ref: String,
    pub capture_ref: String,
    pub ingest_ref: String,
    pub typed_event_ref: String,
    pub summary_refs: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub audit_event_ref: String,
    pub same_product_path: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkEndToEndPipelineProof {
    pub trigger_ref: String,
    pub capture_ref: String,
    pub ingest_ref: String,
    pub typed_event_ref: String,
    pub capture_ingest: NetworkCaptureIngestProof,
    pub evidence_bundle: NetworkCrossSliceEvidenceBundle,
    pub local_ai_queue: NetworkLocalAiQueuePlan,
    pub ai_detection: NetworkAiDetectionEvaluationProof,
    pub ai_audit: NetworkAiAuditReport,
    pub risk_budget: NetworkRiskBudgetEvaluation,
    pub policy_mapping: NetworkEvidencePolicyMapping,
    pub adapter_proof: NetworkDnsAdapterProof,
    pub action_result: NetworkActionResultProof,
    pub retention_delete_export: NetworkRetentionDeleteExportProof,
    pub ai_advisory_only: bool,
    pub policy_is_action_authority: bool,
    pub ui_policy_authority: bool,
    pub network_adapter_authority: bool,
    pub adapter_action_executed: bool,
    pub enforcement_commands_published: usize,
    pub weak_or_unavailable_evidence_enforcement_blocked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkEndToEndPipelineError {
    EmptyTriggerRef,
    EmptyCaptureRef,
    EmptyIngestRef,
    EmptyTypedEventRef,
    EmptyActionResultRef,
    EmptySummaryRef,
    EmptyAnalyzerAlertRef,
    EmptyAuditEventRef,
    EmptyPortalReadModelRef,
    EmptyRetentionRef,
    EmptyDeletionRef,
    EmptyExportRef,
    EmptyTombstoneRef,
    RawNetworkPayloadRejected,
    DecryptedPayloadRejected,
    PageContentRejected,
    ExactUrlRejected,
    AiPolicyAuthorityRejected,
    UiPolicyAuthorityRejected,
    NetworkAdapterAuthorityRejected,
    EnforcementCommandRejected,
    Bundle(NetworkCrossSliceEvidenceBundleError),
    LocalAi(NetworkLocalAiQueueError),
    AiDetection(NetworkAiDetectionEvaluationError),
    AiAudit(NetworkAiAuditReportError),
    RiskBudget(NetworkRiskBudgetThresholdError),
    Policy(NetworkEvidencePolicyMappingError),
    DnsAdapter(NetworkDnsAdapterProofError),
    ActionResult(NetworkActionResultError),
}

struct NetworkEndToEndPipelineParts {
    capture_ingest: NetworkCaptureIngestProof,
    evidence_bundle: NetworkCrossSliceEvidenceBundle,
    local_ai_queue: NetworkLocalAiQueuePlan,
    ai_detection: NetworkAiDetectionEvaluationProof,
    ai_audit: NetworkAiAuditReport,
    risk_budget: NetworkRiskBudgetEvaluation,
    policy_mapping: NetworkEvidencePolicyMapping,
    adapter_proof: NetworkDnsAdapterProof,
    action_result: NetworkActionResultProof,
    retention_delete_export: NetworkRetentionDeleteExportProof,
}

pub fn prove_network_end_to_end_pipeline(
    input: NetworkEndToEndPipelineInput,
) -> Result<NetworkEndToEndPipelineProof, NetworkEndToEndPipelineError> {
    validate_refs(&input.refs)?;
    reject_unsupported_claims(input.unsupported_claims)?;
    let parts = build_pipeline_parts(&input)?;
    let NetworkEndToEndPipelineInput { refs, .. } = input;
    let enforcement_commands_published = parts.adapter_proof.enforcement_command_published as usize;
    let weak_or_unavailable_evidence_enforcement_blocked =
        !parts.adapter_proof.adapter_apply_authorized;
    let adapter_proof = parts.adapter_proof;

    Ok(NetworkEndToEndPipelineProof {
        trigger_ref: refs.trigger_ref,
        capture_ref: refs.capture_ref,
        ingest_ref: refs.ingest_ref,
        typed_event_ref: refs.typed_event_ref,
        capture_ingest: parts.capture_ingest,
        evidence_bundle: parts.evidence_bundle,
        local_ai_queue: parts.local_ai_queue,
        ai_detection: parts.ai_detection,
        ai_audit: parts.ai_audit,
        risk_budget: parts.risk_budget,
        policy_mapping: parts.policy_mapping,
        adapter_proof,
        action_result: parts.action_result,
        retention_delete_export: parts.retention_delete_export,
        ai_advisory_only: true,
        policy_is_action_authority: true,
        ui_policy_authority: false,
        network_adapter_authority: false,
        adapter_action_executed: false,
        enforcement_commands_published,
        weak_or_unavailable_evidence_enforcement_blocked,
    })
}

fn build_pipeline_parts(
    input: &NetworkEndToEndPipelineInput,
) -> Result<NetworkEndToEndPipelineParts, NetworkEndToEndPipelineError> {
    builders::build_pipeline_parts(input)
}
