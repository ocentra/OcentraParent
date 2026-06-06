use serde::{Deserialize, Serialize};

use crate::{
    NetworkInterventionState, NetworkPerformanceBenchmarkProof, NetworkPerformanceBenchmarkState,
    NetworkPerformancePathState, NetworkPerformanceRegressionCode, NetworkPlatformClaimEntry,
    NetworkPlatformClaimManifestProof, NetworkPlatformClaimManualFollowup,
    NetworkPlatformClaimState, NetworkRiskBudgetEvaluation, NetworkRiskBudgetState,
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
    pub risk_budget_ref: String,
    pub risk_budget_state: NetworkRiskBudgetState,
    pub risk_intervention_state: NetworkInterventionState,
    pub risk_total_points: u16,
    pub risk_budget_advisory_only: bool,
    pub performance_state: NetworkPerformanceBenchmarkState,
    pub performance_regression_codes: Vec<NetworkPerformanceRegressionCode>,
    pub performance_path_states: Vec<NetworkPerformancePathState>,
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

    Ok(NetworkProductReadinessStatus {
        status_ref,
        portal_read_model_ref,
        retention_export_ref,
        readiness_state,
        risk_budget_ref: input.risk_budget.risk_budget_ref,
        risk_budget_state: input.risk_budget.risk_budget_state,
        risk_intervention_state: input.risk_budget.intervention_state,
        risk_total_points: input.risk_budget.total_risk_points,
        risk_budget_advisory_only: input.risk_budget.advisory_only,
        performance_state: input.performance.benchmark_state,
        performance_regression_codes: input.performance.regression_codes,
        performance_path_states: input.performance.path_states,
        platform_ready_claims: input.platform_claims.ready_claims,
        platform_dry_run_claims: input.platform_claims.dry_run_claims,
        platform_research_only_claims: input.platform_claims.research_only_claims,
        platform_manual_required_claims: input.platform_claims.manual_required_claims,
        platform_unavailable_claims: input.platform_claims.unavailable_claims,
        platform_manual_followups: input.platform_claims.manual_followups,
        platform_entries: input.platform_claims.entries,
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
    })
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
