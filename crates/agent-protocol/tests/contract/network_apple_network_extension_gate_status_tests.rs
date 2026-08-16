use crate::{
    constants,
    network_apple_network_extension_gate_status::{
        NetworkAppleNetworkExtensionGateBoundaryReason,
        NetworkAppleNetworkExtensionGateCapabilityStatusState,
        NetworkAppleNetworkExtensionGateRequiredArtifact, NetworkAppleNetworkExtensionGateStatus,
        NetworkAppleNetworkExtensionGateStatusState, NetworkAppleNetworkExtensionPlatformStatus,
    },
};

#[test]
fn apple_network_extension_gate_status_serializes_to_camel_case_contract_shape(
) -> Result<(), serde_json::Error> {
    let status = NetworkAppleNetworkExtensionGateStatus {
        status_ref: constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_GATE_STATUS_REF
            .to_string(),
        apple_network_extension_gate_ref:
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_GATE_REF.to_string(),
        policy_decision_ref:
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_POLICY_DECISION_REF.to_string(),
        parent_rule_ref: constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_PARENT_RULE_REF
            .to_string(),
        evidence_refs: vec![
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_EVIDENCE_REF.to_string(),
        ],
        local_ai_result_ref: Some(
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_LOCAL_AI_RESULT_REF.to_string(),
        ),
        platform: NetworkAppleNetworkExtensionPlatformStatus::MacOs,
        bundle_ref: constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_BUNDLE_REF.to_string(),
        network_extension_ref: constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_REF
            .to_string(),
        capability_state: NetworkAppleNetworkExtensionGateCapabilityStatusState::AppleDeviceReady,
        gate_state: NetworkAppleNetworkExtensionGateStatusState::ManualRequired,
        boundary_reasons: vec![
            NetworkAppleNetworkExtensionGateBoundaryReason::MissingRequiredArtifact,
        ],
        missing_required_artifacts: vec![
            NetworkAppleNetworkExtensionGateRequiredArtifact::EntitlementApprovalProof,
        ],
        supervision_required: true,
        ..NetworkAppleNetworkExtensionGateStatus::default()
    };

    let serialized = serde_json::to_value(status)?;

    assert_eq!(
        serialized["statusRef"],
        constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_GATE_STATUS_REF
    );
    assert_eq!(serialized["platform"], "mac-os");
    assert_eq!(serialized["capabilityState"], "apple-device-ready");
    assert_eq!(serialized["gateState"], "manual-required");
    assert_eq!(
        serialized["boundaryReasons"][0],
        constants::network_flow::APPLE_NETWORK_EXTENSION_BOUNDARY_MISSING_REQUIRED_ARTIFACT
    );
    assert_eq!(
        serialized["missingRequiredArtifacts"][0],
        constants::network_flow::APPLE_NETWORK_EXTENSION_ARTIFACT_ENTITLEMENT_APPROVAL_PROOF
    );
    assert_eq!(serialized["supervisionRequired"], true);
    assert_eq!(serialized["liveNetworkExtensionClaimed"], false);

    Ok(())
}
