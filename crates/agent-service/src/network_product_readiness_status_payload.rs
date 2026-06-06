use ocentra_network_evidence::{
    materialize_network_live_capture_custody_status, materialize_network_product_readiness_status,
    plan_network_live_capture_proof, plan_network_raw_capture_storage, NetworkInterventionState,
    NetworkLiveCaptureCustodyStatus, NetworkLiveCaptureCustodyStatusInput,
    NetworkLiveCapturePlatform, NetworkLiveCaptureProof, NetworkLiveCaptureProofInput,
    NetworkPerformanceBenchmarkProof, NetworkPerformanceBenchmarkState,
    NetworkPerformancePathState, NetworkPlatformClaimEntry, NetworkPlatformClaimManifestProof,
    NetworkPlatformClaimManualFollowup, NetworkPlatformClaimState, NetworkPlatformClaimTarget,
    NetworkProductReadinessStatus, NetworkProductReadinessStatusInput,
    NetworkRawCaptureStorageInput, NetworkRiskBudgetAdapterProofState, NetworkRiskBudgetAgeBand,
    NetworkRiskBudgetEvaluation, NetworkRiskBudgetState,
};
use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LogFieldValue, LogFields,
    LogLevel,
};

use crate::{event_builder::build_event, fields::fields_from_pairs};

pub(crate) fn build_network_product_readiness_status_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    build_event(
        constants::event_id::NETWORK_PRODUCT_READINESS_STATUS_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentNetworkProductReadinessStatusReported,
        LogLevel::Info,
        network_product_readiness_status_payload(),
        None,
    )
}

pub fn network_product_readiness_status_payload() -> LogFields {
    let live_capture_status = live_capture_custody_status();
    let product_status = product_readiness_status();

    fields_from_pairs(vec![
        (
            constants::field::NETWORK_LIVE_CAPTURE_CUSTODY_STATUS,
            status_field(&live_capture_status),
        ),
        (
            constants::field::NETWORK_PRODUCT_READINESS_STATUS,
            status_field(&product_status),
        ),
    ])
}

fn status_field<TStatus: serde::Serialize>(status: &TStatus) -> LogFieldValue {
    LogFieldValue::String(
        serde_json::to_string(status).expect(constants::error::AGENT_EVENT_SERIALIZES),
    )
}

fn live_capture_custody_status() -> NetworkLiveCaptureCustodyStatus {
    let live_capture_proof = live_capture_proof();
    let raw_capture_storage_proof =
        plan_network_raw_capture_storage(raw_capture_storage_input(live_capture_proof.clone()))
            .expect(constants::error::AGENT_EVENT_SERIALIZES);

    materialize_network_live_capture_custody_status(NetworkLiveCaptureCustodyStatusInput {
        status_ref: constants::network_flow::TEST_LIVE_CAPTURE_CUSTODY_STATUS_REF.to_owned(),
        live_capture_proof,
        raw_capture_storage_proof,
        live_capture_execution_claimed: false,
        raw_artifact_creation_claimed: false,
        remote_upload_claimed: false,
        raw_pcap_without_custody_claimed: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        private_message_claimed: false,
        search_query_claimed: false,
        policy_authority_claimed: false,
        adapter_authority_claimed: false,
        enforcement_command_claimed: false,
    })
    .expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn live_capture_proof() -> NetworkLiveCaptureProof {
    plan_network_live_capture_proof(NetworkLiveCaptureProofInput {
        capture_proof_ref: constants::network_flow::TEST_LIVE_CAPTURE_PROOF_REF.to_owned(),
        platform: NetworkLiveCapturePlatform::WindowsNpcap,
        interface_ref: Some(constants::network_flow::TEST_LIVE_CAPTURE_INTERFACE_REF.to_owned()),
        driver_proof_ref: Some(
            constants::network_flow::TEST_LIVE_CAPTURE_DRIVER_PROOF_REF.to_owned(),
        ),
        permission_proof_ref: Some(
            constants::network_flow::TEST_LIVE_CAPTURE_PERMISSION_PROOF_REF.to_owned(),
        ),
        bounded_capture_ref: Some(
            constants::network_flow::TEST_LIVE_CAPTURE_BOUNDED_PROOF_REF.to_owned(),
        ),
        clean_stop_ref: Some(constants::network_flow::TEST_LIVE_CAPTURE_CLEAN_STOP_REF.to_owned()),
        quota_rotation_ref: Some(
            constants::network_flow::TEST_LIVE_CAPTURE_QUOTA_ROTATION_REF.to_owned(),
        ),
        retention_delete_export_ref: Some(
            constants::network_flow::TEST_LIVE_CAPTURE_RETENTION_DELETE_EXPORT_REF.to_owned(),
        ),
        custody_ref: Some(constants::network_flow::TEST_LIVE_CAPTURE_CUSTODY_REF.to_owned()),
        private_traffic_exclusion_ref: Some(
            constants::network_flow::TEST_LIVE_CAPTURE_PRIVATE_TRAFFIC_EXCLUSION_REF.to_owned(),
        ),
        platform_available: true,
        driver_available: true,
        permission_granted: true,
        interface_enumerated: true,
        bounded_capture_succeeded: true,
        clean_stop_succeeded: true,
        adapter_degraded: false,
        live_capture_execution_claimed: false,
        unbounded_capture_claimed: false,
        raw_pcap_without_custody_claimed: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        private_message_claimed: false,
        search_query_claimed: false,
        policy_authority_claimed: false,
        adapter_authority_claimed: false,
        enforcement_command_claimed: false,
    })
    .expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn raw_capture_storage_input(
    live_capture_proof: NetworkLiveCaptureProof,
) -> NetworkRawCaptureStorageInput {
    NetworkRawCaptureStorageInput {
        storage_proof_ref: constants::network_flow::TEST_RAW_CAPTURE_STORAGE_PROOF_REF.to_owned(),
        live_capture_proof,
        raw_artifact_manifest_ref: Some(
            constants::network_flow::TEST_RAW_CAPTURE_ARTIFACT_MANIFEST_REF.to_owned(),
        ),
        storage_location_ref: Some(
            constants::network_flow::TEST_RAW_CAPTURE_STORAGE_LOCATION_REF.to_owned(),
        ),
        encryption_at_rest_ref: Some(
            constants::network_flow::TEST_RAW_CAPTURE_ENCRYPTION_AT_REST_REF.to_owned(),
        ),
        quota_rotation_ref: Some(
            constants::network_flow::TEST_RAW_CAPTURE_QUOTA_ROTATION_REF.to_owned(),
        ),
        retention_policy_ref: Some(
            constants::network_flow::TEST_RAW_CAPTURE_RETENTION_POLICY_REF.to_owned(),
        ),
        delete_export_ref: Some(
            constants::network_flow::TEST_RAW_CAPTURE_DELETE_EXPORT_REF.to_owned(),
        ),
        custody_chain_ref: Some(
            constants::network_flow::TEST_RAW_CAPTURE_CUSTODY_CHAIN_REF.to_owned(),
        ),
        private_traffic_exclusion_ref: Some(
            constants::network_flow::TEST_RAW_CAPTURE_PRIVATE_TRAFFIC_EXCLUSION_REF.to_owned(),
        ),
        raw_artifact_manifest_available: true,
        storage_location_available: true,
        encryption_at_rest_verified: true,
        quota_rotation_verified: true,
        retention_policy_verified: true,
        delete_export_verified: true,
        custody_chain_verified: true,
        private_traffic_exclusion_verified: true,
        live_capture_execution_claimed: false,
        remote_upload_claimed: false,
        raw_pcap_without_custody_claimed: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        private_message_claimed: false,
        search_query_claimed: false,
        policy_authority_claimed: false,
        adapter_authority_claimed: false,
        enforcement_command_claimed: false,
    }
}

fn product_readiness_status() -> NetworkProductReadinessStatus {
    materialize_network_product_readiness_status(NetworkProductReadinessStatusInput {
        status_ref: constants::network_flow::TEST_PRODUCT_READINESS_STATUS_REF.to_owned(),
        portal_read_model_ref:
            constants::network_flow::TEST_PRODUCT_READINESS_PORTAL_READ_MODEL_REF.to_owned(),
        retention_export_ref: constants::network_flow::TEST_PRODUCT_READINESS_RETENTION_EXPORT_REF
            .to_owned(),
        risk_budget: risk_budget(),
        performance: performance(),
        platform_claims: platform_claims(),
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        production_slo_claimed: false,
        ui_policy_authority_claimed: false,
        portal_adapter_dispatch_claimed: false,
        live_adapter_execution_claimed: false,
        enforcement_command_claimed: false,
    })
    .expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn risk_budget() -> NetworkRiskBudgetEvaluation {
    NetworkRiskBudgetEvaluation {
        evaluation_ref: constants::network_flow::TEST_RISK_EVALUATION_REF.to_owned(),
        child_profile_ref: constants::network_flow::TEST_CHILD_PROFILE_REF.to_owned(),
        household_policy_ref: constants::network_flow::TEST_HOUSEHOLD_POLICY_REF.to_owned(),
        risk_budget_ref: constants::network_flow::TEST_RISK_BUDGET_REF.to_owned(),
        cascade_ref: constants::network_flow::TEST_CASCADE_REF.to_owned(),
        age_band: NetworkRiskBudgetAgeBand::UnderTwelve,
        risk_budget_state: NetworkRiskBudgetState::AskParentThreshold,
        intervention_state: NetworkInterventionState::AskParent,
        total_risk_points: 42,
        age_profile_points: 15,
        active_signal_points: 27,
        prior_event_points: 0,
        safe_behavior_credit_applied_points: 0,
        triggered_threshold_points: 40,
        cited_signal_refs: vec![constants::network_flow::TEST_RISK_SIGNAL_REF.to_owned()],
        cited_audit_refs: vec![constants::network_flow::TEST_RISK_AUDIT_REF.to_owned()],
        cited_evidence_refs: vec![constants::network_flow::TEST_FLOW_EVIDENCE_REF.to_owned()],
        cited_parent_rule_refs: vec![constants::network_flow::TEST_PARENT_RULE_REF.to_owned()],
        cited_prior_event_refs: Vec::new(),
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

fn performance() -> NetworkPerformanceBenchmarkProof {
    NetworkPerformanceBenchmarkProof {
        benchmark_run_ref: constants::network_flow::TEST_PERFORMANCE_BENCHMARK_REF.to_owned(),
        fixture_set_ref: constants::network_flow::TEST_PERFORMANCE_FIXTURE_SET_REF.to_owned(),
        event_history_ref: constants::network_flow::TEST_PERFORMANCE_EVENT_HISTORY_REF.to_owned(),
        resource_snapshot_ref: constants::network_flow::TEST_PERFORMANCE_RESOURCE_SNAPSHOT_REF
            .to_owned(),
        benchmark_state: NetworkPerformanceBenchmarkState::MeetsBenchmarkGate,
        regression_codes: Vec::new(),
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

fn platform_claims() -> NetworkPlatformClaimManifestProof {
    NetworkPlatformClaimManifestProof {
        manifest_ref: constants::network_flow::TEST_PLATFORM_MANIFEST_REF.to_owned(),
        entries: vec![
            platform_entry(
                NetworkPlatformClaimTarget::WindowsFirewall,
                NetworkPlatformClaimState::Ready,
            ),
            platform_entry(
                NetworkPlatformClaimTarget::WindowsWfp,
                NetworkPlatformClaimState::ManualRequired,
            ),
        ],
        ready_claims: 1,
        dry_run_claims: 0,
        research_only_claims: 0,
        manual_required_claims: 1,
        unavailable_claims: 0,
        manual_followups: vec![NetworkPlatformClaimManualFollowup {
            target: NetworkPlatformClaimTarget::WindowsWfp,
            missing_required_artifacts: vec![
                constants::network_flow::TEST_PLATFORM_MANUAL_FOLLOWUP_REF.to_owned(),
            ],
        }],
        every_claim_names_platform: true,
        every_claim_names_permission_or_manual_followup: true,
        no_enforcement_commands_published: true,
        no_live_adapter_execution_claimed: true,
        ui_has_no_policy_authority: true,
    }
}

fn platform_entry(
    target: NetworkPlatformClaimTarget,
    claim_state: NetworkPlatformClaimState,
) -> NetworkPlatformClaimEntry {
    NetworkPlatformClaimEntry {
        target,
        claim_state,
        policy_decision_ref: constants::network_flow::TEST_POLICY_DECISION_REF.to_owned(),
        parent_rule_ref: constants::network_flow::TEST_PARENT_RULE_REF.to_owned(),
        evidence_refs: vec![constants::network_flow::TEST_FLOW_EVIDENCE_REF.to_owned()],
        device_or_os_refs: vec![constants::network_flow::TEST_DEVICE_REF.to_owned()],
        permission_or_entitlement_refs: vec![
            constants::network_flow::TEST_LIVE_CAPTURE_PERMISSION_PROOF_REF.to_owned(),
        ],
        adapter_capability_refs: vec![
            constants::network_flow::TEST_ADAPTER_CAPABILITY_REF.to_owned()
        ],
        missing_required_artifacts: Vec::new(),
        audit_refs: vec![constants::network_flow::TEST_AUDIT_ENTRY_REF.to_owned()],
        adapter_authorized_by_proof: claim_state == NetworkPlatformClaimState::Ready,
        enforcement_command_published: false,
    }
}
