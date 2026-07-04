use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::live_capture::*;
use ocentra_network_evidence::live_capture_execution::*;

pub fn executed_input(
    live_capture_proof: NetworkLiveCaptureProof,
) -> NetworkLiveCaptureExecutionInput {
    NetworkLiveCaptureExecutionInput {
        execution_ref: "network-live-capture-execution-row13b".to_owned(),
        live_capture_proof,
        source: NetworkLiveCaptureExecutionSource::WindowsNpcapDriver,
        driver_invocation_ref: Some("npcap-driver-invocation-row13b".to_owned()),
        interface_observation_ref: Some("npcap-interface-observation-row13b".to_owned()),
        permission_ref: Some("npcap-permission-row13b".to_owned()),
        bounded_window_ref: Some("bounded-window-row13b".to_owned()),
        clean_stop_ref: Some("clean-stop-row13b".to_owned()),
        custody_ref: Some("execution-custody-row13b".to_owned()),
        retention_delete_export_ref: Some("execution-retention-delete-export-row13b".to_owned()),
        metadata_only_sanitization_ref: Some("metadata-only-sanitization-row13b".to_owned()),
        private_traffic_exclusion_ref: Some("private-traffic-exclusion-row13b".to_owned()),
        driver_invoked: true,
        live_capture_executed: true,
        metadata_snapshot_executed: false,
        captured_packet_count: 3,
        raw_artifact_created: false,
        netstat_metadata_substitution_claimed: false,
        unbounded_capture_claimed: false,
        raw_pcap_without_custody_claimed: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        private_message_claimed: false,
        search_query_claimed: false,
        policy_authority_claimed: false,
        adapter_authority_claimed: false,
        host_filtering_claimed: false,
        enforcement_command_claimed: false,
    }
}

pub fn proof_ready_live_capture() -> NetworkLiveCaptureProof {
    plan_network_live_capture_proof(live_capture_input())
        .expect_value("complete live capture proof refs should parse")
}

pub fn live_capture_with_state(state: NetworkLiveCaptureProofState) -> NetworkLiveCaptureProof {
    match state {
        NetworkLiveCaptureProofState::Unavailable => {
            plan_network_live_capture_proof(NetworkLiveCaptureProofInput {
                platform_available: false,
                ..live_capture_input()
            })
        }
        NetworkLiveCaptureProofState::Degraded => {
            plan_network_live_capture_proof(NetworkLiveCaptureProofInput {
                adapter_degraded: true,
                ..live_capture_input()
            })
        }
        NetworkLiveCaptureProofState::ProofReady | NetworkLiveCaptureProofState::ManualRequired => {
            plan_network_live_capture_proof(live_capture_input())
        }
    }
    .expect_value("live capture state fixture should parse")
}

fn live_capture_input() -> NetworkLiveCaptureProofInput {
    NetworkLiveCaptureProofInput {
        capture_proof_ref: "network-live-capture-row13".to_owned(),
        platform: NetworkLiveCapturePlatform::WindowsNpcap,
        interface_ref: Some("npcap-interface-row13".to_owned()),
        driver_proof_ref: Some("npcap-driver-proof-row13".to_owned()),
        permission_proof_ref: Some("npcap-permission-proof-row13".to_owned()),
        bounded_capture_ref: Some("bounded-capture-proof-row13".to_owned()),
        clean_stop_ref: Some("clean-stop-proof-row13".to_owned()),
        quota_rotation_ref: Some("quota-rotation-proof-row13".to_owned()),
        retention_delete_export_ref: Some("live-capture-retention-delete-export-row13".to_owned()),
        custody_ref: Some("live-capture-custody-row13".to_owned()),
        private_traffic_exclusion_ref: Some("private-family-traffic-exclusion-row13".to_owned()),
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
    }
}
