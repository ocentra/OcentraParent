use std::primitive::str as TestStr;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::{
    NetworkLiveCaptureExecutionStatusState, NetworkLiveCaptureProofStatusState,
    NetworkLiveCaptureStatus, NetworkRawCaptureStorageStatusState,
};
use ocentra_parent_agent_service::test_support::network_live_capture_status_payload_for_test;
use serde::de::DeserializeOwned;

#[test]
fn network_live_capture_status_payload_reports_row13_and_row03a_readiness_without_live_claims(
) -> Result<(), Box<dyn std::error::Error>> {
    let payload = network_live_capture_status_payload_for_test()?;
    let status: NetworkLiveCaptureStatus = status_value(
        &payload,
        constants::network_flow::FIELD_NETWORK_LIVE_CAPTURE_STATUS,
    )?;

    assert_live_capture_status(&status);
    Ok(())
}

fn assert_live_capture_status(status: &NetworkLiveCaptureStatus) {
    assert_eq!(
        status.status_ref,
        constants::network_flow::TEST_LIVE_CAPTURE_STATUS_REF
    );
    assert_eq!(
        status.row13_status_ref,
        constants::network_flow::TEST_LIVE_CAPTURE_ROW13_STATUS_REF
    );
    assert_eq!(
        status.raw_storage_status_ref,
        constants::network_flow::TEST_LIVE_CAPTURE_STORAGE_STATUS_REF
    );
    assert_eq!(
        status.execution_status_ref,
        constants::network_flow::TEST_LIVE_CAPTURE_EXECUTION_STATUS_REF
    );
    assert_eq!(status.platform_row_count, 4);
    assert_eq!(status.proof_ready_count, 1);
    assert_eq!(status.manual_required_count, 1);
    assert_eq!(status.unavailable_count, 1);
    assert_eq!(status.degraded_count, 1);
    assert_eq!(status.storage_custody_ready_count, 1);
    assert_eq!(status.storage_manual_required_count, 1);
    assert_eq!(status.storage_unavailable_count, 1);
    assert_eq!(status.storage_degraded_count, 1);
    assert_eq!(status.capture_ready_count, 1);
    assert_eq!(status.raw_artifact_storage_authorized_count, 1);
    assert_eq!(status.bounded_executed_count, 1);
    assert_eq!(status.execution_manual_required_count, 1);
    assert_eq!(status.execution_unavailable_count, 1);
    assert_eq!(status.execution_degraded_count, 1);
    assert_eq!(status.execution_missing_artifact_count, 30);
    assert_eq!(status.captured_packet_count, 3);
    assert!(status.missing_artifact_count > 0);
    assert!(status.storage_missing_artifact_count > 0);
    assert_live_capture_rows(status);
    assert_live_capture_non_claims(status);
}

fn assert_live_capture_rows(status: &NetworkLiveCaptureStatus) {
    let proof_ready = &status.rows[0];
    assert_eq!(
        proof_ready.capture_proof_ref,
        constants::network_flow::TEST_LIVE_CAPTURE_WINDOWS_PROOF_REF
    );
    assert_eq!(
        proof_ready.proof_state,
        NetworkLiveCaptureProofStatusState::ProofReady
    );
    assert_eq!(
        proof_ready.storage_state,
        NetworkRawCaptureStorageStatusState::CustodyReady
    );
    assert_eq!(
        proof_ready.execution_ref.as_deref(),
        Some(constants::network_flow::TEST_LIVE_CAPTURE_WINDOWS_EXECUTION_REF)
    );
    assert_eq!(
        proof_ready.execution_state,
        NetworkLiveCaptureExecutionStatusState::BoundedExecuted
    );
    assert_eq!(proof_ready.missing_artifact_count, 0);
    assert_eq!(proof_ready.storage_missing_artifact_count, 0);
    assert_eq!(proof_ready.execution_missing_artifact_count, 0);
    assert!(proof_ready.capture_ready);
    assert!(proof_ready.raw_artifact_storage_authorized);
    assert!(proof_ready.driver_invoked);
    assert!(proof_ready.live_capture_executed);
    assert_eq!(proof_ready.captured_packet_count, 3);
    assert!(!proof_ready.raw_artifact_created);

    assert_eq!(
        status.rows[1].proof_state,
        NetworkLiveCaptureProofStatusState::ManualRequired
    );
    assert_eq!(
        status.rows[1].execution_state,
        NetworkLiveCaptureExecutionStatusState::ManualRequired
    );
    assert_eq!(
        status.rows[2].proof_state,
        NetworkLiveCaptureProofStatusState::Unavailable
    );
    assert_eq!(
        status.rows[2].execution_state,
        NetworkLiveCaptureExecutionStatusState::Unavailable
    );
    assert_eq!(
        status.rows[3].proof_state,
        NetworkLiveCaptureProofStatusState::Degraded
    );
    assert_eq!(
        status.rows[3].execution_state,
        NetworkLiveCaptureExecutionStatusState::Degraded
    );
}

fn assert_live_capture_non_claims(status: &NetworkLiveCaptureStatus) {
    assert_eq!(status.driver_invoked_count, 1);
    assert_eq!(status.live_capture_executed_count, 1);
    assert_eq!(status.remote_upload_enabled_count, 0);
    assert_eq!(status.raw_artifact_created_count, 0);
    assert_eq!(status.raw_pcap_without_custody_available_count, 0);
    assert_eq!(status.exact_url_available_count, 0);
    assert_eq!(status.decrypted_payload_available_count, 0);
    assert_eq!(status.page_content_available_count, 0);
    assert_eq!(status.private_message_available_count, 0);
    assert_eq!(status.search_query_available_count, 0);
    assert_eq!(status.policy_authority_count, 0);
    assert_eq!(status.adapter_authority_count, 0);
    assert_eq!(status.enforcement_command_event_count, 0);
    assert_eq!(status.netstat_metadata_substitution_count, 0);
    assert_eq!(status.host_filtering_claim_count, 0);

    for row in &status.rows {
        assert!(!row.raw_artifact_created);
        assert!(!row.remote_upload_enabled);
        assert!(!row.raw_pcap_without_custody_available);
        assert!(!row.exact_url_available);
        assert!(!row.decrypted_payload_available);
        assert!(!row.page_content_available);
        assert!(!row.private_message_available);
        assert!(!row.search_query_available);
        assert!(!row.policy_authority);
        assert!(!row.adapter_authority);
        assert_eq!(row.enforcement_commands_published, 0);
        assert!(!row.netstat_metadata_substituted_for_live_capture);
        assert!(!row.host_filtering_claimed);
    }
}

fn status_value<TStatus: DeserializeOwned>(
    payload: &ocentra_parent_agent_protocol::logging::LogFields,
    field: &TestStr,
) -> Result<TStatus, Box<dyn std::error::Error>> {
    let text = match payload.get(field) {
        Some(ocentra_parent_agent_protocol::logging::LogFieldValue::String(text)) => text,
        other => {
            return Err(std::io::Error::other(format!(
                "{}: {other:?}",
                constants::error::AGENT_EVENT_SERIALIZES
            ))
            .into());
        }
    };
    Ok(serde_json::from_str(text)?)
}
