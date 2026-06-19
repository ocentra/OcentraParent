use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::ai_audit::NetworkAiAuditReport;

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
    validate_input(&input)?;

    let score = calculate_score(&input);
    let risk_budget_state = score_state(score.total_risk_points, &input.thresholds);
    let intervention_state = intervention_state(&input, risk_budget_state);
    let triggered_threshold_points =
        threshold_points_for_state(risk_budget_state, &input.thresholds);

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

struct NetworkRiskBudgetScore {
    total_risk_points: u16,
    age_profile_points: u16,
    active_signal_points: u16,
    prior_event_points: u16,
    safe_behavior_credit_applied_points: u16,
    cited_signal_refs: Vec<String>,
    cited_audit_refs: Vec<String>,
    cited_evidence_refs: Vec<String>,
    cited_prior_event_refs: Vec<String>,
}

fn validate_input(
    input: &NetworkRiskBudgetThresholdInput,
) -> Result<(), NetworkRiskBudgetThresholdError> {
    if input.evaluation_ref.trim().is_empty() {
        return Err(NetworkRiskBudgetThresholdError::EmptyEvaluationRef);
    }
    if input.child_profile_ref.trim().is_empty() {
        return Err(NetworkRiskBudgetThresholdError::EmptyChildProfileRef);
    }
    if input.risk_budget_ref.trim().is_empty() {
        return Err(NetworkRiskBudgetThresholdError::EmptyRiskBudgetRef);
    }
    if input.cascade_ref.trim().is_empty() {
        return Err(NetworkRiskBudgetThresholdError::EmptyCascadeRef);
    }
    validate_claims(input)?;
    validate_thresholds(&input.thresholds)?;
    validate_policy(&input.household_policy)?;
    if input.signals.is_empty() {
        return Err(NetworkRiskBudgetThresholdError::EmptySignals);
    }

    let mut signal_refs = BTreeSet::new();
    let mut safe_behavior_credit_requested = false;
    for signal in &input.signals {
        validate_signal(signal)?;
        if !signal_refs.insert(signal.signal_ref.as_str()) {
            return Err(NetworkRiskBudgetThresholdError::DuplicateSignalRef);
        }
        safe_behavior_credit_requested |= signal.safe_behavior_credit_points > 0;
    }
    if safe_behavior_credit_requested {
        validate_safe_behavior_credit(&input.household_policy)?;
    }
    for prior_event in &input.prior_events {
        if prior_event.event_ref.trim().is_empty() {
            return Err(NetworkRiskBudgetThresholdError::EmptyPriorEventRef);
        }
    }
    Ok(())
}

fn validate_claims(
    input: &NetworkRiskBudgetThresholdInput,
) -> Result<(), NetworkRiskBudgetThresholdError> {
    if input.raw_pcap_claimed {
        return Err(NetworkRiskBudgetThresholdError::RawPcapClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkRiskBudgetThresholdError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkRiskBudgetThresholdError::PageContentClaimRejected);
    }
    if input.exact_url_claimed {
        return Err(NetworkRiskBudgetThresholdError::ExactUrlClaimRejected);
    }
    if input.private_message_claimed {
        return Err(NetworkRiskBudgetThresholdError::PrivateMessageClaimRejected);
    }
    if input.search_query_claimed {
        return Err(NetworkRiskBudgetThresholdError::SearchQueryClaimRejected);
    }
    if input.policy_authority_claimed {
        return Err(NetworkRiskBudgetThresholdError::PolicyAuthorityClaimRejected);
    }
    if input.adapter_authority_claimed {
        return Err(NetworkRiskBudgetThresholdError::AdapterAuthorityClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkRiskBudgetThresholdError::EnforcementCommandClaimRejected);
    }
    if input.extra_privilege_grant_claimed {
        return Err(NetworkRiskBudgetThresholdError::ExtraPrivilegeGrantRejected);
    }
    if input.allowance_grant_claimed {
        return Err(NetworkRiskBudgetThresholdError::AllowanceGrantRejected);
    }
    if input.time_grant_claimed {
        return Err(NetworkRiskBudgetThresholdError::TimeGrantRejected);
    }
    Ok(())
}

fn validate_thresholds(
    thresholds: &NetworkRiskBudgetThresholds,
) -> Result<(), NetworkRiskBudgetThresholdError> {
    let ordered = thresholds.monitor_points <= thresholds.ask_parent_points
        && thresholds.ask_parent_points <= thresholds.warn_child_points
        && thresholds.warn_child_points <= thresholds.limit_points
        && thresholds.limit_points <= thresholds.block_points
        && thresholds.block_points <= thresholds.max_points
        && thresholds.monitor_points > 0;
    if ordered {
        Ok(())
    } else {
        Err(NetworkRiskBudgetThresholdError::InvalidThresholdOrder)
    }
}

fn validate_policy(
    policy: &NetworkRiskBudgetHouseholdPolicy,
) -> Result<(), NetworkRiskBudgetThresholdError> {
    if policy.household_policy_ref.trim().is_empty() {
        return Err(NetworkRiskBudgetThresholdError::EmptyHouseholdPolicyRef);
    }
    if policy.parent_rule_refs.is_empty() {
        return Err(NetworkRiskBudgetThresholdError::EmptyParentRuleRefs);
    }
    if policy
        .parent_rule_refs
        .iter()
        .any(|parent_rule_ref| parent_rule_ref.trim().is_empty())
    {
        return Err(NetworkRiskBudgetThresholdError::EmptyParentRuleRef);
    }
    Ok(())
}

fn validate_safe_behavior_credit(
    policy: &NetworkRiskBudgetHouseholdPolicy,
) -> Result<(), NetworkRiskBudgetThresholdError> {
    let proof_refs_exist = policy
        .safe_behavior_credit_expiry_ref
        .as_ref()
        .is_some_and(|value| !value.trim().is_empty())
        && policy
            .safe_behavior_audit_reason_ref
            .as_ref()
            .is_some_and(|value| !value.trim().is_empty())
        && policy
            .safe_behavior_ui_explanation_ref
            .as_ref()
            .is_some_and(|value| !value.trim().is_empty());
    if policy.safe_behavior_credit_cap_points > 0 && proof_refs_exist {
        Ok(())
    } else {
        Err(NetworkRiskBudgetThresholdError::SafeBehaviorCreditRequiresPolicyProof)
    }
}

fn validate_signal(
    signal: &NetworkRiskBudgetSignal,
) -> Result<(), NetworkRiskBudgetThresholdError> {
    if signal.signal_ref.trim().is_empty() {
        return Err(NetworkRiskBudgetThresholdError::EmptySignalRef);
    }
    validate_audit_report(&signal.audit_report)
}

fn validate_audit_report(
    report: &NetworkAiAuditReport,
) -> Result<(), NetworkRiskBudgetThresholdError> {
    if report.audit_report_ref.trim().is_empty() {
        return Err(NetworkRiskBudgetThresholdError::EmptyAuditReportRef);
    }
    if report.cited_evidence_refs.is_empty() {
        return Err(NetworkRiskBudgetThresholdError::EmptyEvidenceRefs);
    }
    if report
        .cited_evidence_refs
        .iter()
        .any(|evidence_ref| evidence_ref.trim().is_empty())
    {
        return Err(NetworkRiskBudgetThresholdError::EmptyEvidenceRef);
    }
    if !report.parent_readable
        || !report.advisory_only
        || report.policy_authority
        || report.adapter_authority
        || report.enforcement_commands_published > 0
    {
        return Err(NetworkRiskBudgetThresholdError::AuditReportMustRemainAdvisory);
    }
    if report.raw_pcap_available
        || report.exact_url_available
        || report.decrypted_payload_available
        || report.page_content_available
        || report.private_message_available
        || report.search_query_available
        || report.remote_ai_used
    {
        return Err(NetworkRiskBudgetThresholdError::AuditReportUnsupportedClaim);
    }
    Ok(())
}

fn calculate_score(input: &NetworkRiskBudgetThresholdInput) -> NetworkRiskBudgetScore {
    let mut cited_signal_refs = Vec::new();
    let mut cited_audit_refs = Vec::new();
    let mut cited_evidence_refs = Vec::new();
    let mut cited_prior_event_refs = Vec::new();
    let mut active_signal_points = 0_u32;
    let mut requested_safe_credit_points = 0_u32;

    for signal in &input.signals {
        push_unique(&mut cited_signal_refs, &signal.signal_ref);
        push_unique(&mut cited_audit_refs, &signal.audit_report.audit_report_ref);
        for evidence_ref in &signal.audit_report.cited_evidence_refs {
            push_unique(&mut cited_evidence_refs, evidence_ref);
        }
        if signal.known_safe || signal.expected_activity {
            requested_safe_credit_points += u32::from(signal.safe_behavior_credit_points);
        } else {
            active_signal_points += u32::from(signal.base_risk_points);
        }
    }

    let mut prior_event_points = 0_u32;
    for prior_event in &input.prior_events {
        if prior_event.within_window && prior_event.same_household_rule {
            prior_event_points += u32::from(prior_event.risk_points);
            push_unique(&mut cited_prior_event_refs, &prior_event.event_ref);
        }
    }

    let age_profile_points =
        age_pressure_points(input.age_band) + u32::from(input.profile_risk_weight_points);
    let safe_behavior_credit_applied_points = requested_safe_credit_points.min(u32::from(
        input.household_policy.safe_behavior_credit_cap_points,
    ));
    let raw_points = age_profile_points + active_signal_points + prior_event_points;
    let total_risk_points = raw_points
        .saturating_sub(safe_behavior_credit_applied_points)
        .min(u32::from(input.thresholds.max_points));

    NetworkRiskBudgetScore {
        total_risk_points: total_risk_points as u16,
        age_profile_points: age_profile_points as u16,
        active_signal_points: active_signal_points as u16,
        prior_event_points: prior_event_points as u16,
        safe_behavior_credit_applied_points: safe_behavior_credit_applied_points as u16,
        cited_signal_refs,
        cited_audit_refs,
        cited_evidence_refs,
        cited_prior_event_refs,
    }
}

fn intervention_state(
    input: &NetworkRiskBudgetThresholdInput,
    risk_budget_state: NetworkRiskBudgetState,
) -> NetworkInterventionState {
    match risk_budget_state {
        NetworkRiskBudgetState::WithinBudget => NetworkInterventionState::Ignore,
        NetworkRiskBudgetState::MonitorThreshold => NetworkInterventionState::Monitor,
        NetworkRiskBudgetState::AskParentThreshold => NetworkInterventionState::AskParent,
        NetworkRiskBudgetState::WarnChildThreshold => {
            if input.household_policy.child_warning_allowed {
                NetworkInterventionState::WarnChild
            } else {
                NetworkInterventionState::AskParent
            }
        }
        NetworkRiskBudgetState::LimitThreshold => {
            if input.household_policy.limit_policy_allowed && adapter_control_ready(input) {
                NetworkInterventionState::Limit
            } else {
                NetworkInterventionState::ManualRequired
            }
        }
        NetworkRiskBudgetState::BlockThreshold => {
            if input.household_policy.block_policy_allowed
                && input.household_policy.strict_block_policy_enabled
                && adapter_control_ready(input)
            {
                NetworkInterventionState::Block
            } else {
                NetworkInterventionState::ManualRequired
            }
        }
    }
}

fn score_state(
    total_risk_points: u16,
    thresholds: &NetworkRiskBudgetThresholds,
) -> NetworkRiskBudgetState {
    if total_risk_points >= thresholds.block_points {
        NetworkRiskBudgetState::BlockThreshold
    } else if total_risk_points >= thresholds.limit_points {
        NetworkRiskBudgetState::LimitThreshold
    } else if total_risk_points >= thresholds.warn_child_points {
        NetworkRiskBudgetState::WarnChildThreshold
    } else if total_risk_points >= thresholds.ask_parent_points {
        NetworkRiskBudgetState::AskParentThreshold
    } else if total_risk_points >= thresholds.monitor_points {
        NetworkRiskBudgetState::MonitorThreshold
    } else {
        NetworkRiskBudgetState::WithinBudget
    }
}

fn threshold_points_for_state(
    state: NetworkRiskBudgetState,
    thresholds: &NetworkRiskBudgetThresholds,
) -> u16 {
    match state {
        NetworkRiskBudgetState::WithinBudget => 0,
        NetworkRiskBudgetState::MonitorThreshold => thresholds.monitor_points,
        NetworkRiskBudgetState::AskParentThreshold => thresholds.ask_parent_points,
        NetworkRiskBudgetState::WarnChildThreshold => thresholds.warn_child_points,
        NetworkRiskBudgetState::LimitThreshold => thresholds.limit_points,
        NetworkRiskBudgetState::BlockThreshold => thresholds.block_points,
    }
}

fn adapter_control_ready(input: &NetworkRiskBudgetThresholdInput) -> bool {
    input.adapter_proof_state == NetworkRiskBudgetAdapterProofState::Ready
        && input.signals.iter().any(|signal| {
            signal.evidence_tier == NetworkRiskBudgetEvidenceTier::AdapterProofReady
                && !signal.signature_only
        })
}

fn age_pressure_points(age_band: NetworkRiskBudgetAgeBand) -> u32 {
    match age_band {
        NetworkRiskBudgetAgeBand::UnderTwelve => 15,
        NetworkRiskBudgetAgeBand::ThirteenToFifteen => 10,
        NetworkRiskBudgetAgeBand::SixteenToSeventeen => 5,
        NetworkRiskBudgetAgeBand::AdultOrUnknown => 0,
    }
}

fn push_unique(values: &mut Vec<String>, value: &str) {
    if !values.iter().any(|existing| existing == value) {
        values.push(value.to_owned());
    }
}
