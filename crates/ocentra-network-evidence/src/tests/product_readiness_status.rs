use crate::{
    materialize_network_product_readiness_status, NetworkInterventionState,
    NetworkPerformanceBenchmarkProof, NetworkPerformanceBenchmarkState,
    NetworkPerformancePathState, NetworkPerformanceRegressionCode, NetworkPlatformClaimEntry,
    NetworkPlatformClaimManifestProof, NetworkPlatformClaimManualFollowup,
    NetworkPlatformClaimState, NetworkPlatformClaimTarget, NetworkProductReadinessStatusError,
    NetworkProductReadinessStatusInput, NetworkProductReadinessStatusState,
    NetworkRiskBudgetAdapterProofState, NetworkRiskBudgetAgeBand, NetworkRiskBudgetEvaluation,
    NetworkRiskBudgetState,
};

#[test]
fn product_readiness_status_composes_ready_proofs_without_runtime_authority() {
    let status = materialize_network_product_readiness_status(status_input(
        risk_budget(NetworkInterventionState::AskParent),
        performance(NetworkPerformanceBenchmarkState::MeetsBenchmarkGate, vec![]),
        platform_claims(8, 0, 0, 0, vec![]),
    ))
    .expect("ready risk, performance, and platform proofs should materialize");

    assert_eq!(
        status.readiness_state,
        NetworkProductReadinessStatusState::ReadyForPortal
    );
    assert_eq!(status.risk_budget_ref, "network-risk-budget-row51a");
    assert_eq!(
        status.risk_intervention_state,
        NetworkInterventionState::AskParent
    );
    assert_eq!(
        status.performance_state,
        NetworkPerformanceBenchmarkState::MeetsBenchmarkGate
    );
    assert_eq!(status.platform_ready_claims, 8);
    assert!(status.portal_read_model_ready);
    assert!(status.retention_export_refs_visible);
    assert!(!status.policy_authority);
    assert!(!status.adapter_authority);
    assert!(!status.ui_policy_authority);
    assert!(!status.portal_adapter_dispatch_claimed);
    assert!(!status.live_adapter_execution_claimed);
    assert_eq!(status.enforcement_commands_published, 0);
    assert!(!status.production_slo_claimed);
    assert!(!status.exact_url_available);
    assert!(!status.decrypted_payload_available);
    assert!(!status.page_content_available);
}

#[test]
fn product_readiness_status_preserves_manual_followups() {
    let status = materialize_network_product_readiness_status(status_input(
        risk_budget(NetworkInterventionState::ManualRequired),
        performance(NetworkPerformanceBenchmarkState::MeetsBenchmarkGate, vec![]),
        platform_claims(
            6,
            1,
            0,
            1,
            vec![NetworkPlatformClaimManualFollowup {
                target: NetworkPlatformClaimTarget::WindowsWfp,
                missing_required_artifacts: vec!["windows-wfp.administrator-permission".to_owned()],
            }],
        ),
    ))
    .expect("manual-required risk and platform proof should stay visible");

    assert_eq!(
        status.readiness_state,
        NetworkProductReadinessStatusState::ManualRequired
    );
    assert_eq!(
        status.risk_intervention_state,
        NetworkInterventionState::ManualRequired
    );
    assert_eq!(status.platform_manual_required_claims, 1);
    assert_eq!(status.platform_unavailable_claims, 1);
    assert_eq!(status.platform_manual_followups.len(), 1);
    assert_eq!(
        status.platform_manual_followups[0].target,
        NetworkPlatformClaimTarget::WindowsWfp
    );
}

#[test]
fn product_readiness_status_reports_performance_regression_as_degraded() {
    let status = materialize_network_product_readiness_status(status_input(
        risk_budget(NetworkInterventionState::Monitor),
        performance(
            NetworkPerformanceBenchmarkState::BenchmarkGateExceeded,
            vec![NetworkPerformanceRegressionCode::PacketToDetectionLatencyExceeded],
        ),
        platform_claims(8, 0, 0, 0, vec![]),
    ))
    .expect("performance regression should materialize as degraded");

    assert_eq!(
        status.readiness_state,
        NetworkProductReadinessStatusState::Degraded
    );
    assert_eq!(
        status.performance_regression_codes,
        vec![NetworkPerformanceRegressionCode::PacketToDetectionLatencyExceeded]
    );
    assert_eq!(
        status.performance_path_states,
        vec![NetworkPerformancePathState::DryRun]
    );
}

#[test]
fn product_readiness_status_rejects_content_authority_adapter_and_production_claims() {
    assert_eq!(
        materialize_network_product_readiness_status(NetworkProductReadinessStatusInput {
            exact_url_claimed: true,
            ..status_input(
                risk_budget(NetworkInterventionState::AskParent),
                performance(NetworkPerformanceBenchmarkState::MeetsBenchmarkGate, vec![]),
                platform_claims(8, 0, 0, 0, vec![]),
            )
        }),
        Err(NetworkProductReadinessStatusError::ExactUrlClaimRejected)
    );

    let mut risk = risk_budget(NetworkInterventionState::AskParent);
    risk.policy_authority = true;
    assert_eq!(
        materialize_network_product_readiness_status(status_input(
            risk,
            performance(NetworkPerformanceBenchmarkState::MeetsBenchmarkGate, vec![]),
            platform_claims(8, 0, 0, 0, vec![]),
        )),
        Err(NetworkProductReadinessStatusError::RiskBudgetAuthorityClaimRejected)
    );

    let mut perf = performance(NetworkPerformanceBenchmarkState::MeetsBenchmarkGate, vec![]);
    perf.production_slo_claimed = true;
    assert_eq!(
        materialize_network_product_readiness_status(status_input(
            risk_budget(NetworkInterventionState::AskParent),
            perf,
            platform_claims(8, 0, 0, 0, vec![]),
        )),
        Err(NetworkProductReadinessStatusError::PerformanceProductionClaimRejected)
    );

    let mut platform = platform_claims(8, 0, 0, 0, vec![]);
    platform.ui_has_no_policy_authority = false;
    assert_eq!(
        materialize_network_product_readiness_status(status_input(
            risk_budget(NetworkInterventionState::AskParent),
            performance(NetworkPerformanceBenchmarkState::MeetsBenchmarkGate, vec![]),
            platform,
        )),
        Err(NetworkProductReadinessStatusError::PlatformAuthorityClaimRejected)
    );

    assert_eq!(
        materialize_network_product_readiness_status(NetworkProductReadinessStatusInput {
            portal_adapter_dispatch_claimed: true,
            ..status_input(
                risk_budget(NetworkInterventionState::AskParent),
                performance(NetworkPerformanceBenchmarkState::MeetsBenchmarkGate, vec![]),
                platform_claims(8, 0, 0, 0, vec![]),
            )
        }),
        Err(NetworkProductReadinessStatusError::PortalAdapterDispatchClaimRejected)
    );
}

fn status_input(
    risk_budget: NetworkRiskBudgetEvaluation,
    performance: NetworkPerformanceBenchmarkProof,
    platform_claims: NetworkPlatformClaimManifestProof,
) -> NetworkProductReadinessStatusInput {
    NetworkProductReadinessStatusInput {
        status_ref: " network-product-readiness-row51a ".to_owned(),
        portal_read_model_ref: "portal-network-read-model-row51a".to_owned(),
        retention_export_ref: "network-retention-export-row51a".to_owned(),
        risk_budget,
        performance,
        platform_claims,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        production_slo_claimed: false,
        ui_policy_authority_claimed: false,
        portal_adapter_dispatch_claimed: false,
        live_adapter_execution_claimed: false,
        enforcement_command_claimed: false,
    }
}

fn risk_budget(intervention_state: NetworkInterventionState) -> NetworkRiskBudgetEvaluation {
    NetworkRiskBudgetEvaluation {
        evaluation_ref: "network-risk-evaluation-row51a".to_owned(),
        child_profile_ref: "child-profile-row51a".to_owned(),
        household_policy_ref: "household-policy-row51a".to_owned(),
        risk_budget_ref: "network-risk-budget-row51a".to_owned(),
        cascade_ref: "network-cascade-row51a".to_owned(),
        age_band: NetworkRiskBudgetAgeBand::UnderTwelve,
        risk_budget_state: NetworkRiskBudgetState::AskParentThreshold,
        intervention_state,
        total_risk_points: 42,
        age_profile_points: 15,
        active_signal_points: 27,
        prior_event_points: 0,
        safe_behavior_credit_applied_points: 0,
        triggered_threshold_points: 40,
        cited_signal_refs: vec!["network-signal-row51a".to_owned()],
        cited_audit_refs: vec!["network-audit-row51a".to_owned()],
        cited_evidence_refs: vec!["network-evidence-row51a".to_owned()],
        cited_parent_rule_refs: vec!["parent-rule-row51a".to_owned()],
        cited_prior_event_refs: vec![],
        adapter_proof_state: NetworkRiskBudgetAdapterProofState::Ready,
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
    }
}

fn performance(
    state: NetworkPerformanceBenchmarkState,
    regression_codes: Vec<NetworkPerformanceRegressionCode>,
) -> NetworkPerformanceBenchmarkProof {
    NetworkPerformanceBenchmarkProof {
        benchmark_run_ref: "network-performance-row51a".to_owned(),
        fixture_set_ref: "network-performance-fixtures-row51a".to_owned(),
        event_history_ref: "network-performance-event-history-row51a".to_owned(),
        resource_snapshot_ref: "network-performance-resource-row51a".to_owned(),
        benchmark_state: state,
        regression_codes,
        scenario_count: 2,
        fixture_count: 20,
        packet_count: 2_000,
        flow_count: 600,
        event_count: 1_200,
        max_packet_to_summary_latency_ms: 80,
        max_packet_to_detection_latency_ms: 700,
        max_detection_to_cascade_latency_ms: 90,
        max_cascade_to_command_latency_ms: None,
        event_throughput_per_second: 3_200,
        max_cpu_millis: 120,
        max_memory_peak_kib: 40_000,
        total_disk_written_bytes: 20_000,
        max_queue_depth: 4,
        dropped_event_count: 0,
        high_concurrency_flow_count: 2_100,
        path_states: vec![NetworkPerformancePathState::DryRun],
        false_positive_count: 0,
        false_negative_count: 0,
        realtime_response_claimed: false,
        production_slo_claimed: false,
        adapter_action_executed: false,
        host_filtering_executed: false,
        enforcement_commands_published: 0,
        raw_pcap_available: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
    }
}

fn platform_claims(
    ready_claims: usize,
    manual_required_claims: usize,
    research_only_claims: usize,
    unavailable_claims: usize,
    manual_followups: Vec<NetworkPlatformClaimManualFollowup>,
) -> NetworkPlatformClaimManifestProof {
    NetworkPlatformClaimManifestProof {
        manifest_ref: "network-platform-manifest-row51a".to_owned(),
        entries: platform_entries(ready_claims, manual_required_claims),
        ready_claims,
        dry_run_claims: 0,
        research_only_claims,
        manual_required_claims,
        unavailable_claims,
        manual_followups,
        every_claim_names_platform: true,
        every_claim_names_permission_or_manual_followup: true,
        no_enforcement_commands_published: true,
        no_live_adapter_execution_claimed: true,
        ui_has_no_policy_authority: true,
    }
}

fn platform_entries(
    ready_claims: usize,
    manual_required_claims: usize,
) -> Vec<NetworkPlatformClaimEntry> {
    let mut entries = Vec::new();
    for index in 0..ready_claims {
        entries.push(platform_entry(
            format!("ready-{index}"),
            NetworkPlatformClaimState::Ready,
        ));
    }
    for index in 0..manual_required_claims {
        entries.push(platform_entry(
            format!("manual-{index}"),
            NetworkPlatformClaimState::ManualRequired,
        ));
    }
    entries
}

fn platform_entry(
    suffix: String,
    claim_state: NetworkPlatformClaimState,
) -> NetworkPlatformClaimEntry {
    NetworkPlatformClaimEntry {
        target: NetworkPlatformClaimTarget::WindowsFirewall,
        claim_state,
        policy_decision_ref: format!("policy-row51a-{suffix}"),
        parent_rule_ref: format!("parent-rule-row51a-{suffix}"),
        evidence_refs: vec![format!("evidence-row51a-{suffix}")],
        device_or_os_refs: vec![format!("windows-row51a-{suffix}")],
        permission_or_entitlement_refs: vec![format!("permission-row51a-{suffix}")],
        adapter_capability_refs: vec![format!("adapter-row51a-{suffix}")],
        missing_required_artifacts: vec![],
        audit_refs: vec![format!("audit-row51a-{suffix}")],
        adapter_authorized_by_proof: claim_state == NetworkPlatformClaimState::Ready,
        enforcement_command_published: false,
    }
}
