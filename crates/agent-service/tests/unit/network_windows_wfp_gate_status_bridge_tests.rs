use std::primitive::str as TestStr;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_windows_wfp_gate_status::{
    NetworkWindowsWfpGateCapabilityStatusState, NetworkWindowsWfpGateStatus,
    NetworkWindowsWfpGateStatusState,
};
use ocentra_parent_agent_service::test_support::network_windows_wfp_gate_status_payload_for_test;
use serde::de::DeserializeOwned;

#[test]
fn network_windows_wfp_gate_status_payload_reports_lab_ready_without_execution_claims(
) -> Result<(), Box<dyn std::error::Error>> {
    let payload = network_windows_wfp_gate_status_payload_for_test()?;
    let status: NetworkWindowsWfpGateStatus = status_value(
        &payload,
        constants::network_flow::FIELD_NETWORK_WINDOWS_WFP_GATE_STATUS,
    )?;

    assert_windows_wfp_status(&status);
    Ok(())
}

fn assert_windows_wfp_status(status: &NetworkWindowsWfpGateStatus) {
    assert_eq!(
        status.status_ref,
        constants::network_flow::TEST_WINDOWS_WFP_GATE_STATUS_REF
    );
    assert_eq!(
        status.wfp_gate_ref,
        constants::network_flow::TEST_WINDOWS_WFP_GATE_REF
    );
    assert_eq!(
        status.capability_state,
        NetworkWindowsWfpGateCapabilityStatusState::LabReady
    );
    assert_eq!(
        status.gate_state,
        NetworkWindowsWfpGateStatusState::LabProofReady
    );
    assert!(status.boundary_reasons.is_empty());
    assert!(status.missing_required_artifacts.is_empty());
    assert!(status.wfp_lab_proof_ready);
    assert_non_claims(status);
    assert_artifact_refs(status);
}

fn assert_non_claims(status: &NetworkWindowsWfpGateStatus) {
    assert!(!status.adapter_apply_authorized);
    assert!(!status.enforcement_command_published);
    assert!(!status.live_driver_install_claimed);
    assert!(!status.callout_registration_claimed);
    assert!(!status.packet_block_claimed);
    assert!(!status.kernel_payload_inspection_claimed);
    assert!(!status.command_invocation_claimed);
    assert!(!status.exact_url_available);
    assert!(!status.decrypted_payload_available);
    assert!(!status.page_content_available);
}

fn assert_artifact_refs(status: &NetworkWindowsWfpGateStatus) {
    assert_eq!(
        status.administrator_permission_proof_ref.as_deref(),
        Some(constants::network_flow::TEST_WINDOWS_WFP_ADMIN_PERMISSION_PROOF_REF)
    );
    assert_eq!(
        status.driver_signing_proof_ref.as_deref(),
        Some(constants::network_flow::TEST_WINDOWS_WFP_DRIVER_SIGNING_PROOF_REF)
    );
    assert_eq!(
        status.driver_package_proof_ref.as_deref(),
        Some(constants::network_flow::TEST_WINDOWS_WFP_DRIVER_PACKAGE_PROOF_REF)
    );
    assert_eq!(
        status.lab_result_artifact_ref.as_deref(),
        Some(constants::network_flow::TEST_WINDOWS_WFP_LAB_RESULT_ARTIFACT_REF)
    );
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
