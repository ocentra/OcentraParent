use ocentra_network_evidence::{
    live_capture::{plan_network_live_capture_proof, NetworkLiveCaptureProof},
    live_capture_execution::{
        prove_network_live_capture_execution, NetworkLiveCaptureExecutionProof,
    },
    raw_capture_storage::{plan_network_raw_capture_storage, types::NetworkRawCaptureStorageProof},
};
use ocentra_parent_agent_protocol::network_flow::NetworkLiveCaptureStatusRow;

use super::inputs::{
    degraded_input, manual_required_input, proof_ready_input, storage_input, unavailable_input,
};
use super::mapping::count;
use super::platform_mapping::protocol_platform;
use super::proof_state_mapping::protocol_proof_state;
use super::storage_state_mapping::protocol_storage_state;
use crate::network_live_capture_execution_bridge::{execution_input, protocol_execution_state};

pub(super) fn live_capture_rows() -> Result<Vec<NetworkLiveCaptureStatusRow>, ()> {
    let inputs = vec![
        proof_ready_input(),
        manual_required_input(),
        unavailable_input(),
        degraded_input(),
    ];
    inputs
        .into_iter()
        .map(|input| {
            let proof = plan_network_live_capture_proof(input).map_err(|_error| ())?;
            let storage =
                plan_network_raw_capture_storage(storage_input(&proof)).map_err(|_error| ())?;
            let execution = prove_network_live_capture_execution(execution_input(&proof))
                .map_err(|_error| ())?;
            Ok(status_row(&proof, &storage, &execution))
        })
        .collect()
}

fn status_row(
    proof: &NetworkLiveCaptureProof,
    storage: &NetworkRawCaptureStorageProof,
    execution: &NetworkLiveCaptureExecutionProof,
) -> NetworkLiveCaptureStatusRow {
    NetworkLiveCaptureStatusRow {
        platform: protocol_platform(proof.platform),
        capture_proof_ref: proof.capture_proof_ref.clone(),
        proof_state: protocol_proof_state(proof.proof_state),
        storage_proof_ref: storage.storage_proof_ref.clone(),
        storage_state: protocol_storage_state(storage.storage_state),
        interface_ref: proof.interface_ref.clone(),
        driver_proof_ref: proof.driver_proof_ref.clone(),
        permission_proof_ref: proof.permission_proof_ref.clone(),
        bounded_capture_ref: proof.bounded_capture_ref.clone(),
        clean_stop_ref: proof.clean_stop_ref.clone(),
        quota_rotation_ref: proof.quota_rotation_ref.clone(),
        retention_delete_export_ref: proof.retention_delete_export_ref.clone(),
        custody_ref: proof.custody_ref.clone(),
        private_traffic_exclusion_ref: proof.private_traffic_exclusion_ref.clone(),
        raw_artifact_manifest_ref: storage.raw_artifact_manifest_ref.clone(),
        storage_location_ref: storage.storage_location_ref.clone(),
        encryption_at_rest_ref: storage.encryption_at_rest_ref.clone(),
        storage_quota_rotation_ref: storage.quota_rotation_ref.clone(),
        retention_policy_ref: storage.retention_policy_ref.clone(),
        storage_delete_export_ref: storage.delete_export_ref.clone(),
        custody_chain_ref: storage.custody_chain_ref.clone(),
        storage_private_traffic_exclusion_ref: storage.private_traffic_exclusion_ref.clone(),
        execution_ref: Some(execution.execution_ref.clone()),
        execution_state: protocol_execution_state(execution.execution_state),
        execution_missing_artifact_count: count(execution.missing_artifacts.len()),
        driver_invocation_ref: execution.driver_invocation_ref.clone(),
        interface_observation_ref: execution.interface_observation_ref.clone(),
        execution_permission_ref: execution.permission_ref.clone(),
        bounded_window_ref: execution.bounded_window_ref.clone(),
        execution_clean_stop_ref: execution.clean_stop_ref.clone(),
        execution_custody_ref: execution.custody_ref.clone(),
        execution_retention_delete_export_ref: execution.retention_delete_export_ref.clone(),
        metadata_only_sanitization_ref: execution.metadata_only_sanitization_ref.clone(),
        execution_private_traffic_exclusion_ref: execution.private_traffic_exclusion_ref.clone(),
        metadata_snapshot_executed: execution.metadata_snapshot_executed,
        captured_packet_count: count(execution.captured_packet_count),
        raw_artifact_created: execution.raw_artifact_created,
        missing_artifact_count: count(proof.missing_artifacts.len()),
        storage_missing_artifact_count: count(storage.missing_artifacts.len()),
        capture_ready: proof.capture_ready,
        raw_artifact_storage_authorized: storage.raw_artifact_storage_authorized,
        driver_invoked: proof.driver_invoked || execution.driver_invoked,
        live_capture_executed: proof.live_capture_executed
            || storage.live_capture_executed
            || execution.live_capture_executed,
        remote_upload_enabled: storage.remote_upload_enabled,
        raw_pcap_without_custody_available: proof.raw_pcap_without_custody_available
            || storage.raw_pcap_without_custody_available,
        exact_url_available: proof.exact_url_available || storage.exact_url_available,
        decrypted_payload_available: proof.decrypted_payload_available
            || storage.decrypted_payload_available,
        page_content_available: proof.page_content_available || storage.page_content_available,
        private_message_available: proof.private_message_available
            || storage.private_message_available,
        search_query_available: proof.search_query_available || storage.search_query_available,
        policy_authority: proof.policy_authority || storage.policy_authority,
        adapter_authority: proof.adapter_authority || storage.adapter_authority,
        enforcement_commands_published: count(
            proof.enforcement_commands_published + storage.enforcement_commands_published,
        ),
        netstat_metadata_substituted_for_live_capture: execution
            .netstat_metadata_substituted_for_live_capture,
        host_filtering_claimed: execution.host_filtering_claimed,
    }
}
