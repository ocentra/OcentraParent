use crate::{
    constants,
    network_android_vpn_service_gate_status::{
        NetworkAndroidVpnServiceGateBoundaryReason,
        NetworkAndroidVpnServiceGateCapabilityStatusState,
        NetworkAndroidVpnServiceGateRequiredArtifact, NetworkAndroidVpnServiceGateStatus,
        NetworkAndroidVpnServiceGateStatusState,
    },
};

#[test]
fn android_vpn_service_gate_status_serializes_to_camel_case_contract_shape(
) -> Result<(), serde_json::Error> {
    let status = NetworkAndroidVpnServiceGateStatus {
        status_ref: constants::network_flow::TEST_ANDROID_VPN_SERVICE_GATE_STATUS_REF.to_string(),
        android_vpn_service_gate_ref: constants::network_flow::TEST_ANDROID_VPN_SERVICE_GATE_REF
            .to_string(),
        policy_decision_ref: constants::network_flow::TEST_ANDROID_VPN_SERVICE_POLICY_DECISION_REF
            .to_string(),
        parent_rule_ref: constants::network_flow::TEST_ANDROID_VPN_SERVICE_PARENT_RULE_REF
            .to_string(),
        evidence_refs: vec![
            constants::network_flow::TEST_ANDROID_VPN_SERVICE_EVIDENCE_REF.to_string(),
        ],
        local_ai_result_ref: Some(
            constants::network_flow::TEST_ANDROID_VPN_SERVICE_LOCAL_AI_RESULT_REF.to_string(),
        ),
        package_ref: constants::network_flow::TEST_ANDROID_VPN_SERVICE_PACKAGE_REF.to_string(),
        vpn_service_ref: constants::network_flow::TEST_ANDROID_VPN_SERVICE_REF.to_string(),
        capability_state: NetworkAndroidVpnServiceGateCapabilityStatusState::PhysicalDeviceReady,
        gate_state: NetworkAndroidVpnServiceGateStatusState::ManualRequired,
        boundary_reasons: vec![
            NetworkAndroidVpnServiceGateBoundaryReason::PolicyNotVpnServiceApproved,
        ],
        missing_required_artifacts: vec![
            NetworkAndroidVpnServiceGateRequiredArtifact::DeviceOwnerProof,
        ],
        device_owner_required: true,
        ..NetworkAndroidVpnServiceGateStatus::default()
    };

    let serialized = serde_json::to_value(status)?;

    assert_eq!(
        serialized["statusRef"],
        constants::network_flow::TEST_ANDROID_VPN_SERVICE_GATE_STATUS_REF
    );
    assert_eq!(serialized["capabilityState"], "physical-device-ready");
    assert_eq!(serialized["gateState"], "manual-required");
    assert_eq!(
        serialized["boundaryReasons"][0],
        constants::network_flow::ANDROID_VPN_BOUNDARY_POLICY_NOT_VPN_SERVICE_APPROVED
    );
    assert_eq!(
        serialized["missingRequiredArtifacts"][0],
        constants::network_flow::ANDROID_VPN_ARTIFACT_DEVICE_OWNER_PROOF
    );
    assert_eq!(serialized["deviceOwnerRequired"], true);
    assert_eq!(serialized["liveVpnTunnelClaimed"], false);

    Ok(())
}
