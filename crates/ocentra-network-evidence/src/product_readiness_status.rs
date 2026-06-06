use serde::{Deserialize, Serialize};

use crate::{
    NetworkInterventionState, NetworkPerformanceBenchmarkProof, NetworkPerformanceBenchmarkState,
    NetworkPerformancePathState, NetworkPerformanceRegressionCode, NetworkPlatformClaimEntry,
    NetworkPlatformClaimManifestProof, NetworkPlatformClaimManualFollowup,
    NetworkPlatformClaimState, NetworkRiskBudgetAdapterProofState, NetworkRiskBudgetAgeBand,
    NetworkRiskBudgetEvaluation, NetworkRiskBudgetState,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkProductReadinessStatusState {
    ReadyForPortal,
    ManualRequired,
    Degraded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkProductReadinessStatusInput {
    pub status_ref: String,
    pub portal_read_model_ref: String,
    pub retention_export_ref: String,
    pub risk_budget: NetworkRiskBudgetEvaluation,
    pub performance: NetworkPerformanceBenchmarkProof,
    pub platform_claims: NetworkPlatformClaimManifestProof,
    pub exact_url_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub production_slo_claimed: bool,
    pub ui_policy_authority_claimed: bool,
    pub portal_adapter_dispatch_claimed: bool,
    pub live_adapter_execution_claimed: bool,
    pub enforcement_command_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkProductReadinessStatus {
    pub status_ref: String,
    pub portal_read_model_ref: String,
    pub retention_export_ref: String,
    pub readiness_state: NetworkProductReadinessStatusState,
    pub risk_evaluation_ref: String,
    pub risk_child_profile_ref: String,
    pub risk_household_policy_ref: String,
    pub risk_budget_ref: String,
    pub risk_cascade_ref: String,
    pub risk_age_band: NetworkRiskBudgetAgeBand,
    pub risk_budget_state: NetworkRiskBudgetState,
    pub risk_intervention_state: NetworkInterventionState,
    pub risk_total_points: u16,
    pub risk_age_profile_points: u16,
    pub risk_active_signal_points: u16,
    pub risk_prior_event_points: u16,
    pub risk_safe_behavior_credit_applied_points: u16,
    pub risk_triggered_threshold_points: u16,
    pub risk_cited_signal_refs: Vec<String>,
    pub risk_cited_audit_refs: Vec<String>,
    pub risk_cited_evidence_refs: Vec<String>,
    pub risk_cited_parent_rule_refs: Vec<String>,
    pub risk_cited_prior_event_refs: Vec<String>,
    pub risk_adapter_proof_state: NetworkRiskBudgetAdapterProofState,
    pub risk_budget_advisory_only: bool,
    pub performance_benchmark_run_ref: String,
    pub performance_fixture_set_ref: String,
    pub performance_event_history_ref: String,
    pub performance_resource_snapshot_ref: String,
    pub performance_state: NetworkPerformanceBenchmarkState,
    pub performance_regression_codes: Vec<NetworkPerformanceRegressionCode>,
    pub performance_scenario_count: usize,
    pub performance_fixture_count: u32,
    pub performance_packet_count: u32,
    pub performance_flow_count: u32,
    pub performance_event_count: u32,
    pub performance_max_packet_to_summary_latency_ms: u32,
    pub performance_max_packet_to_detection_latency_ms: u32,
    pub performance_max_detection_to_cascade_latency_ms: u32,
    pub performance_max_cascade_to_command_latency_ms: Option<u32>,
    pub performance_event_throughput_per_second: u32,
    pub performance_max_cpu_millis: u32,
    pub performance_max_memory_peak_kib: u32,
    pub performance_total_disk_written_bytes: u64,
    pub performance_max_queue_depth: u32,
    pub performance_dropped_event_count: u32,
    pub performance_high_concurrency_flow_count: u32,
    pub performance_false_positive_count: u32,
    pub performance_false_negative_count: u32,
    pub performance_path_states: Vec<NetworkPerformancePathState>,
    pub performance_realtime_response_claimed: bool,
    pub performance_adapter_action_executed: bool,
    pub performance_host_filtering_executed: bool,
    pub platform_ready_claims: usize,
    pub platform_dry_run_claims: usize,
    pub platform_research_only_claims: usize,
    pub platform_manual_required_claims: usize,
    pub platform_unavailable_claims: usize,
    pub platform_manual_followups: Vec<NetworkPlatformClaimManualFollowup>,
    pub platform_entries: Vec<NetworkPlatformClaimEntry>,
    pub portal_read_model_ready: bool,
    pub retention_export_refs_visible: bool,
    pub policy_authority: bool,
    pub adapter_authority: bool,
    pub ui_policy_authority: bool,
    pub live_adapter_execution_claimed: bool,
    pub portal_adapter_dispatch_claimed: bool,
    pub enforcement_commands_published: usize,
    pub production_slo_claimed: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkProductReadinessStatusError {
    EmptyStatusRef,
    EmptyPortalReadModelRef,
    EmptyRetentionExportRef,
    ExactUrlClaimRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    ProductionSloClaimRejected,
    UiPolicyAuthorityClaimRejected,
    PortalAdapterDispatchClaimRejected,
    LiveAdapterExecutionClaimRejected,
    EnforcementCommandClaimRejected,
    RiskBudgetContentClaimRejected,
    RiskBudgetAuthorityClaimRejected,
    RiskBudgetEnforcementClaimRejected,
    PerformanceProductionClaimRejected,
    PerformanceContentClaimRejected,
    PerformanceAdapterOrEnforcementClaimRejected,
    PlatformAuthorityClaimRejected,
    PlatformLiveAdapterClaimRejected,
    PlatformEnforcementClaimRejected,
    PlatformCountMismatch,
}

pub fn materialize_network_product_readiness_status(
    input: NetworkProductReadinessStatusInput,
) -> Result<NetworkProductReadinessStatus, NetworkProductReadinessStatusError> {
    validate_input(&input)?;

    let status_ref = normalize_ref(input.status_ref)
        .ok_or(NetworkProductReadinessStatusError::EmptyStatusRef)?;
    let portal_read_model_ref = normalize_ref(input.portal_read_model_ref)
        .ok_or(NetworkProductReadinessStatusError::EmptyPortalReadModelRef)?;
    let retention_export_ref = normalize_ref(input.retention_export_ref)
        .ok_or(NetworkProductReadinessStatusError::EmptyRetentionExportRef)?;

    let readiness_state = readiness_state(
        &input.risk_budget,
        &input.performance,
        &input.platform_claims,
    );

    let mut status = NetworkProductReadinessStatus {
        status_ref,
        portal_read_model_ref,
        retention_export_ref,
        readiness_state,
        portal_read_model_ready: true,
        retention_export_refs_visible: true,
        policy_authority: false,
        adapter_authority: false,
        ui_policy_authority: false,
        live_adapter_execution_claimed: false,
        portal_adapter_dispatch_claimed: false,
        enforcement_commands_published: 0,
        production_slo_claimed: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
        ..NetworkProductReadinessStatus::default()
    };
    apply_risk_details(&mut status, input.risk_budget);
    apply_performance_details(&mut status, input.performance);
    apply_platform_details(&mut status, input.platform_claims);
    Ok(status)
}

impl Default for NetworkProductReadinessStatus {
    fn default() -> Self {
        Self {
            status_ref: String::new(),
            portal_read_model_ref: String::new(),
            retention_export_ref: String::new(),
            readiness_state: NetworkProductReadinessStatusState::ReadyForPortal,
            risk_evaluation_ref: String::new(),
            risk_child_profile_ref: String::new(),
            risk_household_policy_ref: String::new(),
            risk_budget_ref: String::new(),
            risk_cascade_ref: String::new(),
            risk_age_band: NetworkRiskBudgetAgeBand::AdultOrUnknown,
            risk_budget_state: NetworkRiskBudgetState::WithinBudget,
            risk_intervention_state: NetworkInterventionState::Ignore,
            risk_total_points: 0,
            risk_age_profile_points: 0,
            risk_active_signal_points: 0,
            risk_prior_event_points: 0,
            risk_safe_behavior_credit_applied_points: 0,
            risk_triggered_threshold_points: 0,
            risk_cited_signal_refs: Vec::new(),
            risk_cited_audit_refs: Vec::new(),
            risk_cited_evidence_refs: Vec::new(),
            risk_cited_parent_rule_refs: Vec::new(),
            risk_cited_prior_event_refs: Vec::new(),
            risk_adapter_proof_state: NetworkRiskBudgetAdapterProofState::NotNeeded,
            risk_budget_advisory_only: true,
            performance_benchmark_run_ref: String::new(),
            performance_fixture_set_ref: String::new(),
            performance_event_history_ref: String::new(),
            performance_resource_snapshot_ref: String::new(),
            performance_state: NetworkPerformanceBenchmarkState::MeetsBenchmarkGate,
            performance_regression_codes: Vec::new(),
            performance_scenario_count: 0,
            performance_fixture_count: 0,
            performance_packet_count: 0,
            performance_flow_count: 0,
            performance_event_count: 0,
            performance_max_packet_to_summary_latency_ms: 0,
            performance_max_packet_to_detection_latency_ms: 0,
            performance_max_detection_to_cascade_latency_ms: 0,
            performance_max_cascade_to_command_latency_ms: None,
            performance_event_throughput_per_second: 0,
            performance_max_cpu_millis: 0,
            performance_max_memory_peak_kib: 0,
            performance_total_disk_written_bytes: 0,
            performance_max_queue_depth: 0,
            performance_dropped_event_count: 0,
            performance_high_concurrency_flow_count: 0,
            performance_false_positive_count: 0,
            performance_false_negative_count: 0,
            performance_path_states: Vec::new(),
            performance_realtime_response_claimed: false,
            performance_adapter_action_executed: false,
            performance_host_filtering_executed: false,
            platform_ready_claims: 0,
            platform_dry_run_claims: 0,
            platform_research_only_claims: 0,
            platform_manual_required_claims: 0,
            platform_unavailable_claims: 0,
            platform_manual_followups: Vec::new(),
            platform_entries: Vec::new(),
            portal_read_model_ready: false,
            retention_export_refs_visible: false,
            policy_authority: false,
            adapter_authority: false,
            ui_policy_authority: false,
            live_adapter_execution_claimed: false,
            portal_adapter_dispatch_claimed: false,
            enforcement_commands_published: 0,
            production_slo_claimed: false,
            exact_url_available: false,
            decrypted_payload_available: false,
            page_content_available: false,
        }
    }
}

fn apply_risk_details(
    status: &mut NetworkProductReadinessStatus,
    risk_budget: NetworkRiskBudgetEvaluation,
) {
    status.risk_evaluation_ref = risk_budget.evaluation_ref;
    status.risk_child_profile_ref = risk_budget.child_profile_ref;
    status.risk_household_policy_ref = risk_budget.household_policy_ref;
    status.risk_budget_ref = risk_budget.risk_budget_ref;
    status.risk_cascade_ref = risk_budget.cascade_ref;
    status.risk_age_band = risk_budget.age_band;
    status.risk_budget_state = risk_budget.risk_budget_state;
    status.risk_intervention_state = risk_budget.intervention_state;
    status.risk_total_points = risk_budget.total_risk_points;
    status.risk_age_profile_points = risk_budget.age_profile_points;
    status.risk_active_signal_points = risk_budget.active_signal_points;
    status.risk_prior_event_points = risk_budget.prior_event_points;
    status.risk_safe_behavior_credit_applied_points =
        risk_budget.safe_behavior_credit_applied_points;
    status.risk_triggered_threshold_points = risk_budget.triggered_threshold_points;
    status.risk_cited_signal_refs = risk_budget.cited_signal_refs;
    status.risk_cited_audit_refs = risk_budget.cited_audit_refs;
    status.risk_cited_evidence_refs = risk_budget.cited_evidence_refs;
    status.risk_cited_parent_rule_refs = risk_budget.cited_parent_rule_refs;
    status.risk_cited_prior_event_refs = risk_budget.cited_prior_event_refs;
    status.risk_adapter_proof_state = risk_budget.adapter_proof_state;
    status.risk_budget_advisory_only = risk_budget.advisory_only;
}

fn apply_performance_details(
    status: &mut NetworkProductReadinessStatus,
    performance: NetworkPerformanceBenchmarkProof,
) {
    status.performance_benchmark_run_ref = performance.benchmark_run_ref;
    status.performance_fixture_set_ref = performance.fixture_set_ref;
    status.performance_event_history_ref = performance.event_history_ref;
    status.performance_resource_snapshot_ref = performance.resource_snapshot_ref;
    status.performance_state = performance.benchmark_state;
    status.performance_regression_codes = performance.regression_codes;
    status.performance_scenario_count = performance.scenario_count;
    status.performance_fixture_count = performance.fixture_count;
    status.performance_packet_count = performance.packet_count;
    status.performance_flow_count = performance.flow_count;
    status.performance_event_count = performance.event_count;
    status.performance_max_packet_to_summary_latency_ms =
        performance.max_packet_to_summary_latency_ms;
    status.performance_max_packet_to_detection_latency_ms =
        performance.max_packet_to_detection_latency_ms;
    status.performance_max_detection_to_cascade_latency_ms =
        performance.max_detection_to_cascade_latency_ms;
    status.performance_max_cascade_to_command_latency_ms =
        performance.max_cascade_to_command_latency_ms;
    status.performance_event_throughput_per_second = performance.event_throughput_per_second;
    status.performance_max_cpu_millis = performance.max_cpu_millis;
    status.performance_max_memory_peak_kib = performance.max_memory_peak_kib;
    status.performance_total_disk_written_bytes = performance.total_disk_written_bytes;
    status.performance_max_queue_depth = performance.max_queue_depth;
    status.performance_dropped_event_count = performance.dropped_event_count;
    status.performance_high_concurrency_flow_count = performance.high_concurrency_flow_count;
    status.performance_false_positive_count = performance.false_positive_count;
    status.performance_false_negative_count = performance.false_negative_count;
    status.performance_path_states = performance.path_states;
    status.performance_realtime_response_claimed = performance.realtime_response_claimed;
    status.performance_adapter_action_executed = performance.adapter_action_executed;
    status.performance_host_filtering_executed = performance.host_filtering_executed;
}

fn apply_platform_details(
    status: &mut NetworkProductReadinessStatus,
    platform_claims: NetworkPlatformClaimManifestProof,
) {
    status.platform_ready_claims = platform_claims.ready_claims;
    status.platform_dry_run_claims = platform_claims.dry_run_claims;
    status.platform_research_only_claims = platform_claims.research_only_claims;
    status.platform_manual_required_claims = platform_claims.manual_required_claims;
    status.platform_unavailable_claims = platform_claims.unavailable_claims;
    status.platform_manual_followups = platform_claims.manual_followups;
    status.platform_entries = platform_claims.entries;
}

fn validate_input(
    input: &NetworkProductReadinessStatusInput,
) -> Result<(), NetworkProductReadinessStatusError> {
    validate_direct_claims(input)?;
    validate_risk_budget(&input.risk_budget)?;
    validate_performance(&input.performance)?;
    validate_platform_claims(&input.platform_claims)
}

fn validate_direct_claims(
    input: &NetworkProductReadinessStatusInput,
) -> Result<(), NetworkProductReadinessStatusError> {
    if input.exact_url_claimed {
        return Err(NetworkProductReadinessStatusError::ExactUrlClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkProductReadinessStatusError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkProductReadinessStatusError::PageContentClaimRejected);
    }
    if input.production_slo_claimed {
        return Err(NetworkProductReadinessStatusError::ProductionSloClaimRejected);
    }
    if input.ui_policy_authority_claimed {
        return Err(NetworkProductReadinessStatusError::UiPolicyAuthorityClaimRejected);
    }
    if input.portal_adapter_dispatch_claimed {
        return Err(NetworkProductReadinessStatusError::PortalAdapterDispatchClaimRejected);
    }
    if input.live_adapter_execution_claimed {
        return Err(NetworkProductReadinessStatusError::LiveAdapterExecutionClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkProductReadinessStatusError::EnforcementCommandClaimRejected);
    }
    Ok(())
}

fn validate_risk_budget(
    risk_budget: &NetworkRiskBudgetEvaluation,
) -> Result<(), NetworkProductReadinessStatusError> {
    if risk_budget.raw_pcap_available
        || risk_budget.exact_url_available
        || risk_budget.decrypted_payload_available
        || risk_budget.page_content_available
        || risk_budget.private_message_available
        || risk_budget.search_query_available
    {
        return Err(NetworkProductReadinessStatusError::RiskBudgetContentClaimRejected);
    }
    if risk_budget.policy_authority
        || risk_budget.adapter_authority
        || risk_budget.extra_privilege_granted
        || risk_budget.allowance_granted
        || risk_budget.time_granted
    {
        return Err(NetworkProductReadinessStatusError::RiskBudgetAuthorityClaimRejected);
    }
    if risk_budget.enforcement_commands_published > 0 {
        return Err(NetworkProductReadinessStatusError::RiskBudgetEnforcementClaimRejected);
    }
    Ok(())
}

fn validate_performance(
    performance: &NetworkPerformanceBenchmarkProof,
) -> Result<(), NetworkProductReadinessStatusError> {
    if performance.realtime_response_claimed || performance.production_slo_claimed {
        return Err(NetworkProductReadinessStatusError::PerformanceProductionClaimRejected);
    }
    if performance.raw_pcap_available
        || performance.exact_url_available
        || performance.decrypted_payload_available
        || performance.page_content_available
    {
        return Err(NetworkProductReadinessStatusError::PerformanceContentClaimRejected);
    }
    if performance.adapter_action_executed
        || performance.host_filtering_executed
        || performance.enforcement_commands_published > 0
    {
        return Err(
            NetworkProductReadinessStatusError::PerformanceAdapterOrEnforcementClaimRejected,
        );
    }
    Ok(())
}

fn validate_platform_claims(
    platform_claims: &NetworkPlatformClaimManifestProof,
) -> Result<(), NetworkProductReadinessStatusError> {
    if !platform_claims.ui_has_no_policy_authority {
        return Err(NetworkProductReadinessStatusError::PlatformAuthorityClaimRejected);
    }
    if !platform_claims.no_live_adapter_execution_claimed {
        return Err(NetworkProductReadinessStatusError::PlatformLiveAdapterClaimRejected);
    }
    if !platform_claims.no_enforcement_commands_published
        || platform_claims
            .entries
            .iter()
            .any(|entry| entry.enforcement_command_published)
    {
        return Err(NetworkProductReadinessStatusError::PlatformEnforcementClaimRejected);
    }
    if count_platform_entries(platform_claims, NetworkPlatformClaimState::Ready)
        != platform_claims.ready_claims
        || count_platform_entries(platform_claims, NetworkPlatformClaimState::DryRun)
            != platform_claims.dry_run_claims
        || count_platform_entries(platform_claims, NetworkPlatformClaimState::ResearchOnly)
            != platform_claims.research_only_claims
        || count_platform_entries(platform_claims, NetworkPlatformClaimState::ManualRequired)
            != platform_claims.manual_required_claims
        || count_platform_entries(platform_claims, NetworkPlatformClaimState::Unavailable)
            != platform_claims.unavailable_claims
    {
        return Err(NetworkProductReadinessStatusError::PlatformCountMismatch);
    }
    Ok(())
}

fn count_platform_entries(
    platform_claims: &NetworkPlatformClaimManifestProof,
    state: NetworkPlatformClaimState,
) -> usize {
    platform_claims
        .entries
        .iter()
        .filter(|entry| entry.claim_state == state)
        .count()
}

fn readiness_state(
    risk_budget: &NetworkRiskBudgetEvaluation,
    performance: &NetworkPerformanceBenchmarkProof,
    platform_claims: &NetworkPlatformClaimManifestProof,
) -> NetworkProductReadinessStatusState {
    let manual_required = risk_budget.intervention_state
        == NetworkInterventionState::ManualRequired
        || platform_claims.manual_required_claims > 0
        || platform_claims.research_only_claims > 0
        || platform_claims.unavailable_claims > 0
        || !platform_claims.manual_followups.is_empty();

    if manual_required {
        NetworkProductReadinessStatusState::ManualRequired
    } else if performance.benchmark_state == NetworkPerformanceBenchmarkState::BenchmarkGateExceeded
    {
        NetworkProductReadinessStatusState::Degraded
    } else {
        NetworkProductReadinessStatusState::ReadyForPortal
    }
}

fn normalize_ref(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}
