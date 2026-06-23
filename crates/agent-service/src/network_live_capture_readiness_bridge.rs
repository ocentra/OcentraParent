use ocentra_network_evidence::{
    live_capture::{
        plan_network_live_capture_proof, NetworkLiveCapturePlatform, NetworkLiveCaptureProof,
        NetworkLiveCaptureProofInput, NetworkLiveCaptureProofState,
    },
    live_capture_execution::{
        prove_network_live_capture_execution, NetworkLiveCaptureExecutionProof,
    },
    raw_capture_storage::{
        plan_network_raw_capture_storage,
        types::{
            NetworkRawCaptureStorageInput, NetworkRawCaptureStorageProof,
            NetworkRawCaptureStorageState,
        },
    },
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::network_flow::NetworkLiveCaptureExecutionStatusState;
use ocentra_parent_agent_protocol::network_flow::NetworkLiveCaptureProofStatusState;
use ocentra_parent_agent_protocol::network_flow::NetworkLiveCaptureStatus;
use ocentra_parent_agent_protocol::network_flow::NetworkLiveCaptureStatusPlatform;
use ocentra_parent_agent_protocol::network_flow::NetworkLiveCaptureStatusRow;
use ocentra_parent_agent_protocol::network_flow::NetworkRawCaptureStorageStatusState;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::network_live_capture_execution_bridge::{execution_input, protocol_execution_state};
use crate::{event_builder::build_event, fields::fields_from_pairs};

const LIVE_CAPTURE_REQUIRED_ARTIFACTS_PER_ROW: u64 = 9;

pub(crate) fn build_network_live_capture_status_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let correlation_id = command.message_id.clone();
    let target = command.source;
    match network_live_capture_status_payload() {
        Ok(payload) => build_event(
            constants::event_id::NETWORK_LIVE_CAPTURE_STATUS_REPORTED,
            &correlation_id,
            target,
            AgentEventName::AgentNetworkLiveCaptureStatusReported,
            LogLevel::Info,
            payload,
            None,
        ),
        Err(()) => build_event(
            constants::event_id::COMMAND_REJECTED,
            &correlation_id,
            target,
            AgentEventName::AgentCommandRejected,
            LogLevel::Warn,
            fields_from_pairs(vec![(
                constants::field::REASON,
                LogFieldValue::String(
                    constants::network_flow::ERROR_NETWORK_LIVE_CAPTURE_STATUS.to_string(),
                ),
            )]),
            None,
        ),
    }
}

pub(crate) fn network_live_capture_status_payload() -> Result<LogFields, ()> {
    let status = network_live_capture_status()?;
    let serialized = serde_json::to_string(&status).map_err(|_error| ())?;
    Ok(fields_from_pairs(vec![(
        constants::network_flow::FIELD_NETWORK_LIVE_CAPTURE_STATUS,
        LogFieldValue::String(serialized),
    )]))
}

fn network_live_capture_status() -> Result<NetworkLiveCaptureStatus, ()> {
    let rows = live_capture_rows()?;
    Ok(status_from_rows(rows))
}

fn live_capture_rows() -> Result<Vec<NetworkLiveCaptureStatusRow>, ()> {
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

fn status_from_rows(rows: Vec<NetworkLiveCaptureStatusRow>) -> NetworkLiveCaptureStatus {
    let mut status = NetworkLiveCaptureStatus {
        status_ref: constants::network_flow::TEST_LIVE_CAPTURE_STATUS_REF.to_string(),
        row13_status_ref: constants::network_flow::TEST_LIVE_CAPTURE_ROW13_STATUS_REF.to_string(),
        execution_status_ref: constants::network_flow::TEST_LIVE_CAPTURE_EXECUTION_STATUS_REF
            .to_string(),
        raw_storage_status_ref: constants::network_flow::TEST_LIVE_CAPTURE_STORAGE_STATUS_REF
            .to_string(),
        platform_row_count: count(rows.len()),
        required_artifact_count: count(rows.len()) * LIVE_CAPTURE_REQUIRED_ARTIFACTS_PER_ROW,
        ..NetworkLiveCaptureStatus::default()
    };

    for row in &rows {
        apply_row_counts(&mut status, row);
    }
    status.rows = rows;

    status
}

fn apply_row_counts(status: &mut NetworkLiveCaptureStatus, row: &NetworkLiveCaptureStatusRow) {
    let count_bool = |value| {
        if value {
            1
        } else {
            0
        }
    };

    match row.proof_state {
        NetworkLiveCaptureProofStatusState::ProofReady => status.proof_ready_count += 1,
        NetworkLiveCaptureProofStatusState::ManualRequired => status.manual_required_count += 1,
        NetworkLiveCaptureProofStatusState::Unavailable => status.unavailable_count += 1,
        NetworkLiveCaptureProofStatusState::Degraded => status.degraded_count += 1,
    }
    match row.storage_state {
        NetworkRawCaptureStorageStatusState::CustodyReady => {
            status.storage_custody_ready_count += 1
        }
        NetworkRawCaptureStorageStatusState::ManualRequired => {
            status.storage_manual_required_count += 1
        }
        NetworkRawCaptureStorageStatusState::Unavailable => status.storage_unavailable_count += 1,
        NetworkRawCaptureStorageStatusState::Degraded => status.storage_degraded_count += 1,
    }
    status.missing_artifact_count += row.missing_artifact_count;
    status.storage_missing_artifact_count += row.storage_missing_artifact_count;
    status.execution_missing_artifact_count += row.execution_missing_artifact_count;
    status.metadata_snapshot_executed_count += count_bool(row.metadata_snapshot_executed);
    status.captured_packet_count += row.captured_packet_count;
    status.raw_artifact_created_count += count_bool(row.raw_artifact_created);
    match row.execution_state {
        NetworkLiveCaptureExecutionStatusState::BoundedExecuted => {
            status.bounded_executed_count += 1
        }
        NetworkLiveCaptureExecutionStatusState::ManualRequired => {
            status.execution_manual_required_count += 1
        }
        NetworkLiveCaptureExecutionStatusState::Unavailable => {
            status.execution_unavailable_count += 1
        }
        NetworkLiveCaptureExecutionStatusState::Degraded => status.execution_degraded_count += 1,
    }
    status.capture_ready_count += count_bool(row.capture_ready);
    status.raw_artifact_storage_authorized_count += count_bool(row.raw_artifact_storage_authorized);
    status.driver_invoked_count += count_bool(row.driver_invoked);
    status.live_capture_executed_count += count_bool(row.live_capture_executed);
    status.remote_upload_enabled_count += count_bool(row.remote_upload_enabled);
    status.raw_pcap_without_custody_available_count +=
        count_bool(row.raw_pcap_without_custody_available);
    status.exact_url_available_count += count_bool(row.exact_url_available);
    status.decrypted_payload_available_count += count_bool(row.decrypted_payload_available);
    status.page_content_available_count += count_bool(row.page_content_available);
    status.private_message_available_count += count_bool(row.private_message_available);
    status.search_query_available_count += count_bool(row.search_query_available);
    status.policy_authority_count += count_bool(row.policy_authority);
    status.adapter_authority_count += count_bool(row.adapter_authority);
    status.enforcement_command_event_count += row.enforcement_commands_published;
    status.netstat_metadata_substitution_count +=
        count_bool(row.netstat_metadata_substituted_for_live_capture);
    status.host_filtering_claim_count += count_bool(row.host_filtering_claimed);
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

fn proof_ready_input() -> NetworkLiveCaptureProofInput {
    complete_live_capture_input(
        constants::network_flow::TEST_LIVE_CAPTURE_WINDOWS_PROOF_REF,
        NetworkLiveCapturePlatform::WindowsNpcap,
        true,
        false,
    )
}

fn manual_required_input() -> NetworkLiveCaptureProofInput {
    NetworkLiveCaptureProofInput {
        capture_proof_ref: constants::network_flow::TEST_LIVE_CAPTURE_MANUAL_PROOF_REF.to_string(),
        platform: NetworkLiveCapturePlatform::WindowsNpcap,
        interface_ref: None,
        driver_proof_ref: None,
        permission_proof_ref: None,
        bounded_capture_ref: None,
        clean_stop_ref: None,
        quota_rotation_ref: None,
        retention_delete_export_ref: None,
        custody_ref: None,
        private_traffic_exclusion_ref: None,
        platform_available: true,
        driver_available: false,
        permission_granted: false,
        interface_enumerated: false,
        bounded_capture_succeeded: false,
        clean_stop_succeeded: false,
        adapter_degraded: false,
        ..no_claim_live_capture_input()
    }
}

fn unavailable_input() -> NetworkLiveCaptureProofInput {
    NetworkLiveCaptureProofInput {
        capture_proof_ref: constants::network_flow::TEST_LIVE_CAPTURE_LINUX_PROOF_REF.to_string(),
        platform: NetworkLiveCapturePlatform::LinuxLibpcap,
        platform_available: false,
        ..manual_required_input()
    }
}

fn degraded_input() -> NetworkLiveCaptureProofInput {
    complete_live_capture_input(
        constants::network_flow::TEST_LIVE_CAPTURE_MACOS_PROOF_REF,
        NetworkLiveCapturePlatform::MacosBpfLibpcap,
        true,
        true,
    )
}

fn complete_live_capture_input(
    capture_proof_ref: &str,
    platform: NetworkLiveCapturePlatform,
    platform_available: bool,
    adapter_degraded: bool,
) -> NetworkLiveCaptureProofInput {
    NetworkLiveCaptureProofInput {
        capture_proof_ref: capture_proof_ref.to_string(),
        platform,
        interface_ref: Some(constants::network_flow::TEST_LIVE_CAPTURE_INTERFACE_REF.to_string()),
        driver_proof_ref: Some(constants::network_flow::TEST_LIVE_CAPTURE_DRIVER_REF.to_string()),
        permission_proof_ref: Some(
            constants::network_flow::TEST_LIVE_CAPTURE_PERMISSION_REF.to_string(),
        ),
        bounded_capture_ref: Some(
            constants::network_flow::TEST_LIVE_CAPTURE_BOUNDED_REF.to_string(),
        ),
        clean_stop_ref: Some(constants::network_flow::TEST_LIVE_CAPTURE_CLEAN_STOP_REF.to_string()),
        quota_rotation_ref: Some(constants::network_flow::TEST_LIVE_CAPTURE_QUOTA_REF.to_string()),
        retention_delete_export_ref: Some(
            constants::network_flow::TEST_LIVE_CAPTURE_RETENTION_REF.to_string(),
        ),
        custody_ref: Some(constants::network_flow::TEST_LIVE_CAPTURE_CUSTODY_REF.to_string()),
        private_traffic_exclusion_ref: Some(
            constants::network_flow::TEST_LIVE_CAPTURE_PRIVATE_TRAFFIC_EXCLUSION_REF.to_string(),
        ),
        platform_available,
        driver_available: true,
        permission_granted: true,
        interface_enumerated: true,
        bounded_capture_succeeded: true,
        clean_stop_succeeded: true,
        adapter_degraded,
        ..no_claim_live_capture_input()
    }
}

fn no_claim_live_capture_input() -> NetworkLiveCaptureProofInput {
    NetworkLiveCaptureProofInput {
        capture_proof_ref: String::new(),
        platform: NetworkLiveCapturePlatform::WindowsNpcap,
        interface_ref: None,
        driver_proof_ref: None,
        permission_proof_ref: None,
        bounded_capture_ref: None,
        clean_stop_ref: None,
        quota_rotation_ref: None,
        retention_delete_export_ref: None,
        custody_ref: None,
        private_traffic_exclusion_ref: None,
        platform_available: false,
        driver_available: false,
        permission_granted: false,
        interface_enumerated: false,
        bounded_capture_succeeded: false,
        clean_stop_succeeded: false,
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

fn storage_input(proof: &NetworkLiveCaptureProof) -> NetworkRawCaptureStorageInput {
    let ready = proof.proof_state == NetworkLiveCaptureProofState::ProofReady;
    NetworkRawCaptureStorageInput {
        storage_proof_ref: constants::network_flow::TEST_LIVE_CAPTURE_STORAGE_STATUS_REF
            .to_string(),
        live_capture_proof: proof.clone(),
        raw_artifact_manifest_ref: ready
            .then_some(constants::network_flow::TEST_RAW_CAPTURE_MANIFEST_REF.to_string()),
        storage_location_ref: ready
            .then_some(constants::network_flow::TEST_RAW_CAPTURE_STORAGE_LOCATION_REF.to_string()),
        encryption_at_rest_ref: ready
            .then_some(constants::network_flow::TEST_RAW_CAPTURE_ENCRYPTION_REF.to_string()),
        quota_rotation_ref: ready
            .then_some(constants::network_flow::TEST_RAW_CAPTURE_QUOTA_REF.to_string()),
        retention_policy_ref: ready
            .then_some(constants::network_flow::TEST_RAW_CAPTURE_RETENTION_REF.to_string()),
        delete_export_ref: ready
            .then_some(constants::network_flow::TEST_RAW_CAPTURE_DELETE_EXPORT_REF.to_string()),
        custody_chain_ref: ready
            .then_some(constants::network_flow::TEST_RAW_CAPTURE_CUSTODY_CHAIN_REF.to_string()),
        private_traffic_exclusion_ref: ready.then_some(
            constants::network_flow::TEST_RAW_CAPTURE_PRIVATE_TRAFFIC_EXCLUSION_REF.to_string(),
        ),
        raw_artifact_manifest_available: ready,
        storage_location_available: ready,
        encryption_at_rest_verified: ready,
        quota_rotation_verified: ready,
        retention_policy_verified: ready,
        delete_export_verified: ready,
        custody_chain_verified: ready,
        private_traffic_exclusion_verified: ready,
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

fn protocol_platform(platform: NetworkLiveCapturePlatform) -> NetworkLiveCaptureStatusPlatform {
    match platform {
        NetworkLiveCapturePlatform::WindowsNpcap => NetworkLiveCaptureStatusPlatform::WindowsNpcap,
        NetworkLiveCapturePlatform::LinuxLibpcap => NetworkLiveCaptureStatusPlatform::LinuxLibpcap,
        NetworkLiveCapturePlatform::MacosBpfLibpcap => {
            NetworkLiveCaptureStatusPlatform::MacosBpfLibpcap
        }
    }
}

fn protocol_proof_state(state: NetworkLiveCaptureProofState) -> NetworkLiveCaptureProofStatusState {
    match state {
        NetworkLiveCaptureProofState::ProofReady => NetworkLiveCaptureProofStatusState::ProofReady,
        NetworkLiveCaptureProofState::ManualRequired => {
            NetworkLiveCaptureProofStatusState::ManualRequired
        }
        NetworkLiveCaptureProofState::Unavailable => {
            NetworkLiveCaptureProofStatusState::Unavailable
        }
        NetworkLiveCaptureProofState::Degraded => NetworkLiveCaptureProofStatusState::Degraded,
    }
}

fn protocol_storage_state(
    state: NetworkRawCaptureStorageState,
) -> NetworkRawCaptureStorageStatusState {
    match state {
        NetworkRawCaptureStorageState::CustodyReady => {
            NetworkRawCaptureStorageStatusState::CustodyReady
        }
        NetworkRawCaptureStorageState::ManualRequired => {
            NetworkRawCaptureStorageStatusState::ManualRequired
        }
        NetworkRawCaptureStorageState::Unavailable => {
            NetworkRawCaptureStorageStatusState::Unavailable
        }
        NetworkRawCaptureStorageState::Degraded => NetworkRawCaptureStorageStatusState::Degraded,
    }
}

fn count(value: usize) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}
