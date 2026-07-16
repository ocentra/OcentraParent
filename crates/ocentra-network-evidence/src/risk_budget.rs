use serde::{Deserialize, Serialize};

use crate::ai_audit::NetworkAiAuditReport;
mod score;
mod validation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRiskBudgetAgeBand {
    UnderTwelve,
    ThirteenToFifteen,
    SixteenToSeventeen,
    AdultOrUnknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRiskBudgetEvidenceTier {
    WeakNetworkMetadata,
    StructuredNetworkSummary,
    AiAuditWithCitations,
    AdapterProofReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRiskBudgetAdapterProofState {
    NotNeeded,
    Missing,
    Ready,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRiskBudgetState {
    WithinBudget,
    MonitorThreshold,
    AskParentThreshold,
    WarnChildThreshold,
    LimitThreshold,
    BlockThreshold,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkInterventionState {
    Ignore,
    Monitor,
    AskParent,
    WarnChild,
    Limit,
    Block,
    ManualRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRiskBudgetThresholds {
    pub monitor_points: u16,
    pub ask_parent_points: u16,
    pub warn_child_points: u16,
    pub limit_points: u16,
    pub block_points: u16,
    pub max_points: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRiskBudgetHouseholdPolicy {
    pub household_policy_ref: String,
    pub parent_rule_refs: Vec<String>,
    pub child_warning_allowed: bool,
    pub limit_policy_allowed: bool,
    pub block_policy_allowed: bool,
    pub strict_block_policy_enabled: bool,
    pub safe_behavior_credit_cap_points: u16,
    pub safe_behavior_credit_expiry_ref: Option<String>,
    pub safe_behavior_audit_reason_ref: Option<String>,
    pub safe_behavior_ui_explanation_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRiskBudgetSignal {
    pub signal_ref: String,
    pub audit_report: NetworkAiAuditReport,
    pub evidence_tier: NetworkRiskBudgetEvidenceTier,
    pub base_risk_points: u16,
    pub safe_behavior_credit_points: u16,
    pub known_safe: bool,
    pub expected_activity: bool,
    pub signature_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRiskBudgetPriorEvent {
    pub event_ref: String,
    pub risk_points: u16,
    pub within_window: bool,
    pub same_household_rule: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRiskBudgetThresholdInput {
    pub evaluation_ref: String,
    pub child_profile_ref: String,
    pub risk_budget_ref: String,
    pub cascade_ref: String,
    pub age_band: NetworkRiskBudgetAgeBand,
    pub profile_risk_weight_points: u16,
    pub thresholds: NetworkRiskBudgetThresholds,
    pub household_policy: NetworkRiskBudgetHouseholdPolicy,
    pub signals: Vec<NetworkRiskBudgetSignal>,
    pub prior_events: Vec<NetworkRiskBudgetPriorEvent>,
    pub adapter_proof_state: NetworkRiskBudgetAdapterProofState,
    pub raw_pcap_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub exact_url_claimed: bool,
    pub private_message_claimed: bool,
    pub search_query_claimed: bool,
    pub policy_authority_claimed: bool,
    pub adapter_authority_claimed: bool,
    pub enforcement_command_claimed: bool,
    pub extra_privilege_grant_claimed: bool,
    pub allowance_grant_claimed: bool,
    pub time_grant_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRiskBudgetEvaluation {
    pub evaluation_ref: String,
    pub child_profile_ref: String,
    pub household_policy_ref: String,
    pub risk_budget_ref: String,
    pub cascade_ref: String,
    pub age_band: NetworkRiskBudgetAgeBand,
    pub risk_budget_state: NetworkRiskBudgetState,
    pub intervention_state: NetworkInterventionState,
    pub total_risk_points: u16,
    pub age_profile_points: u16,
    pub active_signal_points: u16,
    pub prior_event_points: u16,
    pub safe_behavior_credit_applied_points: u16,
    pub triggered_threshold_points: u16,
    pub cited_signal_refs: Vec<String>,
    pub cited_audit_refs: Vec<String>,
    pub cited_evidence_refs: Vec<String>,
    pub cited_parent_rule_refs: Vec<String>,
    pub cited_prior_event_refs: Vec<String>,
    pub adapter_proof_state: NetworkRiskBudgetAdapterProofState,
    pub advisory_only: bool,
    pub policy_authority: bool,
    pub adapter_authority: bool,
    pub enforcement_commands_published: usize,
    pub raw_pcap_available: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
    pub private_message_available: bool,
    pub search_query_available: bool,
    pub extra_privilege_granted: bool,
    pub allowance_granted: bool,
    pub time_granted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkRiskBudgetThresholdError {
    EmptyEvaluationRef,
    EmptyChildProfileRef,
    EmptyRiskBudgetRef,
    EmptyCascadeRef,
    EmptyHouseholdPolicyRef,
    EmptyParentRuleRefs,
    EmptyParentRuleRef,
    InvalidThresholdOrder,
    EmptySignals,
    EmptySignalRef,
    DuplicateSignalRef,
    EmptyAuditReportRef,
    EmptyEvidenceRefs,
    EmptyEvidenceRef,
    EmptyPriorEventRef,
    AuditReportMustRemainAdvisory,
    AuditReportUnsupportedClaim,
    SafeBehaviorCreditRequiresPolicyProof,
    RawPcapClaimRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    ExactUrlClaimRejected,
    PrivateMessageClaimRejected,
    SearchQueryClaimRejected,
    PolicyAuthorityClaimRejected,
    AdapterAuthorityClaimRejected,
    EnforcementCommandClaimRejected,
    ExtraPrivilegeGrantRejected,
    AllowanceGrantRejected,
    TimeGrantRejected,
}

pub fn evaluate_network_risk_budget_threshold(
    input: NetworkRiskBudgetThresholdInput,
) -> Result<NetworkRiskBudgetEvaluation, NetworkRiskBudgetThresholdError> {
    validation::validate_input(&input)?;

    let score = score::calculate_score(&input);
    let risk_budget_state = score::score_state(score.total_risk_points, &input.thresholds);
    let intervention_state = score::intervention_state(&input, risk_budget_state);
    let triggered_threshold_points =
        score::threshold_points_for_state(risk_budget_state, &input.thresholds);

    Ok(NetworkRiskBudgetEvaluation {
        evaluation_ref: input.evaluation_ref,
        child_profile_ref: input.child_profile_ref,
        household_policy_ref: input.household_policy.household_policy_ref,
        risk_budget_ref: input.risk_budget_ref,
        cascade_ref: input.cascade_ref,
        age_band: input.age_band,
        risk_budget_state,
        intervention_state,
        total_risk_points: score.total_risk_points,
        age_profile_points: score.age_profile_points,
        active_signal_points: score.active_signal_points,
        prior_event_points: score.prior_event_points,
        safe_behavior_credit_applied_points: score.safe_behavior_credit_applied_points,
        triggered_threshold_points,
        cited_signal_refs: score.cited_signal_refs,
        cited_audit_refs: score.cited_audit_refs,
        cited_evidence_refs: score.cited_evidence_refs,
        cited_parent_rule_refs: input.household_policy.parent_rule_refs,
        cited_prior_event_refs: score.cited_prior_event_refs,
        adapter_proof_state: input.adapter_proof_state,
        advisory_only: true,
        policy_authority: false,
        adapter_authority: false,
        enforcement_commands_published: 0,
        raw_pcap_available: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
        private_message_available: false,
        search_query_available: false,
        extra_privilege_granted: false,
        allowance_granted: false,
        time_granted: false,
    })
}
