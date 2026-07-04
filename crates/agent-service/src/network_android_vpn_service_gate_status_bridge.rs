use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::{
    android_vpn_service_gate::{
        plan_network_android_vpn_service_gate, NetworkAndroidVpnServiceCapabilityState,
        NetworkAndroidVpnServiceGateBoundaryReason, NetworkAndroidVpnServiceGateInput,
        NetworkAndroidVpnServiceGateProof, NetworkAndroidVpnServiceGateState,
        NetworkAndroidVpnServiceRequiredArtifact,
    },
    dns::types::NetworkEvidenceGrade,
    policy::{
        map_network_evidence_grade_to_policy, NetworkEvidencePolicyAction,
        NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingInput,
    },
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::network_android_vpn_service_gate_status::{
    NetworkAndroidVpnServiceGateBoundaryReason as ProtocolNetworkAndroidVpnServiceGateBoundaryReason,
    NetworkAndroidVpnServiceGateCapabilityStatusState,
    NetworkAndroidVpnServiceGateRequiredArtifact as ProtocolNetworkAndroidVpnServiceGateRequiredArtifact,
    NetworkAndroidVpnServiceGateStatus, NetworkAndroidVpnServiceGateStatusState,
};
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{event_builder::build_event, fields::fields_from_pairs};

pub(crate) fn build_network_android_vpn_service_gate_status_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let correlation_id = command.message_id.clone();
    let target = command.source;
    match network_android_vpn_service_gate_status_payload() {
        Ok(payload) => build_event(
            constants::event_id::NETWORK_ANDROID_VPN_SERVICE_GATE_STATUS_REPORTED,
            &correlation_id,
            target,
            AgentEventName::AgentNetworkAndroidVpnServiceGateStatusReported,
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
                    constants::network_flow::ERROR_NETWORK_ANDROID_VPN_SERVICE_GATE_STATUS
                        .to_string(),
                ),
            )]),
            None,
        ),
    }
}

pub(crate) fn network_android_vpn_service_gate_status_payload() -> Result<LogFields, ()> {
    let proof = plan_network_android_vpn_service_gate(gate_input()).map_err(|_error| ())?;
    let status = status_from_proof(&proof);
    let serialized = serde_json::to_string(&status).map_err(|_error| ())?;
    Ok(fields_from_pairs(vec![(
        constants::network_flow::FIELD_NETWORK_ANDROID_VPN_SERVICE_GATE_STATUS,
        LogFieldValue::String(serialized),
    )]))
}

fn status_from_proof(
    proof: &NetworkAndroidVpnServiceGateProof,
) -> NetworkAndroidVpnServiceGateStatus {
    NetworkAndroidVpnServiceGateStatus {
        status_ref: constants::network_flow::TEST_ANDROID_VPN_SERVICE_GATE_STATUS_REF.to_string(),
        android_vpn_service_gate_ref: proof.android_vpn_service_gate_ref.clone(),
        policy_decision_ref: proof.policy_decision_ref.clone(),
        parent_rule_ref: proof.parent_rule_ref.clone(),
        evidence_refs: proof.evidence_refs.clone(),
        local_ai_result_ref: proof.local_ai_result_ref.clone(),
        package_ref: proof.package_ref.clone(),
        vpn_service_ref: proof.vpn_service_ref.clone(),
        capability_state: protocol_capability_state(proof.capability_state),
        gate_state: protocol_gate_state(proof.gate_state),
        boundary_reasons: proof.boundary_reasons.iter().map(boundary_reason).collect(),
        missing_required_artifacts: proof
            .missing_required_artifacts
            .iter()
            .map(required_artifact)
            .collect(),
        vpn_service_declaration_ref: proof.vpn_service_declaration_ref.clone(),
        user_consent_proof_ref: proof.user_consent_proof_ref.clone(),
        physical_device_proof_ref: proof.physical_device_proof_ref.clone(),
        package_identity_proof_ref: proof.package_identity_proof_ref.clone(),
        virtual_interface_proof_ref: proof.virtual_interface_proof_ref.clone(),
        traffic_observation_proof_ref: proof.traffic_observation_proof_ref.clone(),
        rollback_plan_ref: proof.rollback_plan_ref.clone(),
        audit_event_ref: proof.audit_event_ref.clone(),
        device_owner_required: proof.device_owner_required,
        device_owner_proof_ref: proof.device_owner_proof_ref.clone(),
        physical_device_proof_ready: proof.physical_device_proof_ready,
        device_owner_authority_proved: proof.device_owner_authority_proved,
        adapter_apply_authorized: proof.adapter_apply_authorized,
        enforcement_command_published: proof.enforcement_command_published,
        emulator_only_product_support_claimed: proof.emulator_only_product_support_claimed,
        live_vpn_tunnel_claimed: proof.live_vpn_tunnel_claimed,
        packet_block_claimed: proof.packet_block_claimed,
        app_package_correlation_claimed: proof.app_package_correlation_claimed,
        exact_url_available: proof.exact_url_available,
        decrypted_payload_available: proof.decrypted_payload_available,
        page_content_available: proof.page_content_available,
    }
}

fn gate_input() -> NetworkAndroidVpnServiceGateInput {
    NetworkAndroidVpnServiceGateInput {
        android_vpn_service_gate_ref: constants::network_flow::TEST_ANDROID_VPN_SERVICE_GATE_REF
            .to_string(),
        policy_mapping: policy_mapping(),
        package_ref: constants::network_flow::TEST_ANDROID_VPN_SERVICE_PACKAGE_REF.to_string(),
        vpn_service_ref: constants::network_flow::TEST_ANDROID_VPN_SERVICE_REF.to_string(),
        capability_state: NetworkAndroidVpnServiceCapabilityState::PhysicalDeviceReady,
        vpn_service_declaration_ref: Some(
            constants::network_flow::TEST_ANDROID_VPN_SERVICE_DECLARATION_REF.to_string(),
        ),
        user_consent_proof_ref: Some(
            constants::network_flow::TEST_ANDROID_VPN_USER_CONSENT_PROOF_REF.to_string(),
        ),
        physical_device_proof_ref: Some(
            constants::network_flow::TEST_ANDROID_VPN_PHYSICAL_DEVICE_PROOF_REF.to_string(),
        ),
        package_identity_proof_ref: Some(
            constants::network_flow::TEST_ANDROID_VPN_PACKAGE_IDENTITY_PROOF_REF.to_string(),
        ),
        virtual_interface_proof_ref: Some(
            constants::network_flow::TEST_ANDROID_VPN_VIRTUAL_INTERFACE_PROOF_REF.to_string(),
        ),
        traffic_observation_proof_ref: Some(
            constants::network_flow::TEST_ANDROID_VPN_TRAFFIC_OBSERVATION_PROOF_REF.to_string(),
        ),
        rollback_plan_ref: Some(
            constants::network_flow::TEST_ANDROID_VPN_ROLLBACK_PLAN_REF.to_string(),
        ),
        audit_event_ref: Some(
            constants::network_flow::TEST_ANDROID_VPN_AUDIT_EVENT_REF.to_string(),
        ),
        device_owner_required: false,
        device_owner_proof_ref: None,
        research_only: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        emulator_only_product_support_claimed: false,
        live_vpn_tunnel_claimed: false,
        packet_block_claimed: false,
        app_package_correlation_claimed: false,
    }
}

fn policy_mapping() -> NetworkEvidencePolicyMapping {
    map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
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
        evidence_grade: NetworkEvidenceGrade::A,
        requested_action: NetworkEvidencePolicyAction::Block,
        adapter_capability_proof_ref: None,
    })
    .expect_value(constants::event_id::NETWORK_ANDROID_VPN_SERVICE_GATE_STATUS_REPORTED)
}

fn protocol_capability_state(
    state: NetworkAndroidVpnServiceCapabilityState,
) -> NetworkAndroidVpnServiceGateCapabilityStatusState {
    match state {
        NetworkAndroidVpnServiceCapabilityState::PhysicalDeviceReady => {
            NetworkAndroidVpnServiceGateCapabilityStatusState::PhysicalDeviceReady
        }
        NetworkAndroidVpnServiceCapabilityState::ManualRequired => {
            NetworkAndroidVpnServiceGateCapabilityStatusState::ManualRequired
        }
        NetworkAndroidVpnServiceCapabilityState::Unavailable => {
            NetworkAndroidVpnServiceGateCapabilityStatusState::Unavailable
        }
    }
}

fn protocol_gate_state(
    state: NetworkAndroidVpnServiceGateState,
) -> NetworkAndroidVpnServiceGateStatusState {
    match state {
        NetworkAndroidVpnServiceGateState::ResearchOnly => {
            NetworkAndroidVpnServiceGateStatusState::ResearchOnly
        }
        NetworkAndroidVpnServiceGateState::ManualRequired => {
            NetworkAndroidVpnServiceGateStatusState::ManualRequired
        }
        NetworkAndroidVpnServiceGateState::Unavailable => {
            NetworkAndroidVpnServiceGateStatusState::Unavailable
        }
        NetworkAndroidVpnServiceGateState::PhysicalDeviceProofReady => {
            NetworkAndroidVpnServiceGateStatusState::PhysicalDeviceProofReady
        }
    }
}

fn boundary_reason(
    reason: &NetworkAndroidVpnServiceGateBoundaryReason,
) -> ProtocolNetworkAndroidVpnServiceGateBoundaryReason {
    match reason {
        NetworkAndroidVpnServiceGateBoundaryReason::ResearchOnlyRequested => {
            ProtocolNetworkAndroidVpnServiceGateBoundaryReason::ResearchOnlyRequested
        }
        NetworkAndroidVpnServiceGateBoundaryReason::CapabilityManualRequired => {
            ProtocolNetworkAndroidVpnServiceGateBoundaryReason::CapabilityManualRequired
        }
        NetworkAndroidVpnServiceGateBoundaryReason::CapabilityUnavailable => {
            ProtocolNetworkAndroidVpnServiceGateBoundaryReason::CapabilityUnavailable
        }
        NetworkAndroidVpnServiceGateBoundaryReason::EvidenceGradeBelowProofThreshold => {
            ProtocolNetworkAndroidVpnServiceGateBoundaryReason::EvidenceGradeBelowProofThreshold
        }
        NetworkAndroidVpnServiceGateBoundaryReason::PolicyNotVpnServiceApproved => {
            ProtocolNetworkAndroidVpnServiceGateBoundaryReason::PolicyNotVpnServiceApproved
        }
        NetworkAndroidVpnServiceGateBoundaryReason::MissingRequiredArtifact => {
            ProtocolNetworkAndroidVpnServiceGateBoundaryReason::MissingRequiredArtifact
        }
    }
}

fn required_artifact(
    artifact: &NetworkAndroidVpnServiceRequiredArtifact,
) -> ProtocolNetworkAndroidVpnServiceGateRequiredArtifact {
    match artifact {
        NetworkAndroidVpnServiceRequiredArtifact::VpnServiceDeclaration => {
            ProtocolNetworkAndroidVpnServiceGateRequiredArtifact::VpnServiceDeclaration
        }
        NetworkAndroidVpnServiceRequiredArtifact::UserConsentProof => {
            ProtocolNetworkAndroidVpnServiceGateRequiredArtifact::UserConsentProof
        }
        NetworkAndroidVpnServiceRequiredArtifact::PhysicalDeviceProof => {
            ProtocolNetworkAndroidVpnServiceGateRequiredArtifact::PhysicalDeviceProof
        }
        NetworkAndroidVpnServiceRequiredArtifact::PackageIdentityProof => {
            ProtocolNetworkAndroidVpnServiceGateRequiredArtifact::PackageIdentityProof
        }
        NetworkAndroidVpnServiceRequiredArtifact::VirtualInterfaceProof => {
            ProtocolNetworkAndroidVpnServiceGateRequiredArtifact::VirtualInterfaceProof
        }
        NetworkAndroidVpnServiceRequiredArtifact::TrafficObservationProof => {
            ProtocolNetworkAndroidVpnServiceGateRequiredArtifact::TrafficObservationProof
        }
        NetworkAndroidVpnServiceRequiredArtifact::RollbackPlan => {
            ProtocolNetworkAndroidVpnServiceGateRequiredArtifact::RollbackPlan
        }
        NetworkAndroidVpnServiceRequiredArtifact::AuditEvent => {
            ProtocolNetworkAndroidVpnServiceGateRequiredArtifact::AuditEvent
        }
        NetworkAndroidVpnServiceRequiredArtifact::DeviceOwnerProof => {
            ProtocolNetworkAndroidVpnServiceGateRequiredArtifact::DeviceOwnerProof
        }
    }
}
