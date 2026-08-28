use std::primitive::str as TestStr;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_android_vpn_service_gate_status::{
    NetworkAndroidVpnServiceGateCapabilityStatusState, NetworkAndroidVpnServiceGateStatus,
    NetworkAndroidVpnServiceGateStatusState,
};
use ocentra_parent_agent_service::test_support::network_android_vpn_service_gate_status_payload_for_test;
use serde::de::DeserializeOwned;

#[test]
fn network_android_vpn_service_gate_status_payload_reports_physical_device_ready_without_execution_claims(
) -> Result<(), Box<dyn std::error::Error>> {
    let payload = network_android_vpn_service_gate_status_payload_for_test()?;
    let status: NetworkAndroidVpnServiceGateStatus = status_value(
        &payload,
        constants::network_flow::FIELD_NETWORK_ANDROID_VPN_SERVICE_GATE_STATUS,
    )?;

    assert_android_vpn_service_status(&status);
    Ok(())
}

fn assert_android_vpn_service_status(status: &NetworkAndroidVpnServiceGateStatus) {
    assert_eq!(
        status.status_ref,
        constants::network_flow::TEST_ANDROID_VPN_SERVICE_GATE_STATUS_REF
    );
    assert_eq!(
        status.android_vpn_service_gate_ref,
        constants::network_flow::TEST_ANDROID_VPN_SERVICE_GATE_REF
    );
    assert_eq!(
        status.capability_state,
        NetworkAndroidVpnServiceGateCapabilityStatusState::PhysicalDeviceReady
    );
    assert_eq!(
        status.gate_state,
        NetworkAndroidVpnServiceGateStatusState::PhysicalDeviceProofReady
    );
    assert!(status.boundary_reasons.is_empty());
    assert!(status.missing_required_artifacts.is_empty());
    assert!(status.physical_device_proof_ready);
    assert!(!status.device_owner_required);
    assert!(!status.device_owner_authority_proved);
    assert_artifact_refs(status);
    assert_non_claims(status);
}

fn assert_artifact_refs(status: &NetworkAndroidVpnServiceGateStatus) {
    assert_eq!(
        status.vpn_service_declaration_ref.as_deref(),
        Some(constants::network_flow::TEST_ANDROID_VPN_SERVICE_DECLARATION_REF)
    );
    assert_eq!(
        status.user_consent_proof_ref.as_deref(),
        Some(constants::network_flow::TEST_ANDROID_VPN_USER_CONSENT_PROOF_REF)
    );
    assert_eq!(
        status.physical_device_proof_ref.as_deref(),
        Some(constants::network_flow::TEST_ANDROID_VPN_PHYSICAL_DEVICE_PROOF_REF)
    );
    assert_eq!(
        status.rollback_plan_ref.as_deref(),
        Some(constants::network_flow::TEST_ANDROID_VPN_ROLLBACK_PLAN_REF)
    );
}

fn assert_non_claims(status: &NetworkAndroidVpnServiceGateStatus) {
    assert!(!status.adapter_apply_authorized);
    assert!(!status.enforcement_command_published);
    assert!(!status.emulator_only_product_support_claimed);
    assert!(!status.live_vpn_tunnel_claimed);
    assert!(!status.packet_block_claimed);
    assert!(!status.app_package_correlation_claimed);
    assert!(!status.exact_url_available);
    assert!(!status.decrypted_payload_available);
    assert!(!status.page_content_available);
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
