use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::{
    apple_network_extension_gate::{
        plan_network_apple_network_extension_gate, NetworkAppleNetworkExtensionCapabilityState,
        NetworkAppleNetworkExtensionGateBoundaryReason, NetworkAppleNetworkExtensionGateInput,
        NetworkAppleNetworkExtensionGateProof, NetworkAppleNetworkExtensionGateState,
        NetworkAppleNetworkExtensionPlatform, NetworkAppleNetworkExtensionRequiredArtifact,
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
use ocentra_parent_agent_protocol::network_apple_network_extension_gate_status::{
    NetworkAppleNetworkExtensionGateBoundaryReason as ProtocolNetworkAppleNetworkExtensionGateBoundaryReason,
    NetworkAppleNetworkExtensionGateCapabilityStatusState,
    NetworkAppleNetworkExtensionGateRequiredArtifact as ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact,
    NetworkAppleNetworkExtensionGateStatus, NetworkAppleNetworkExtensionGateStatusState,
    NetworkAppleNetworkExtensionPlatformStatus,
};
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{event_builder::build_event, fields::fields_from_pairs};

pub(crate) fn build_network_apple_network_extension_gate_status_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let correlation_id = command.message_id.clone();
    let target = command.source;
    match network_apple_network_extension_gate_status_payload() {
        Ok(payload) => build_event(
            constants::event_id::NETWORK_APPLE_NETWORK_EXTENSION_GATE_STATUS_REPORTED,
            &correlation_id,
            target,
            AgentEventName::AgentNetworkAppleNetworkExtensionGateStatusReported,
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
                    constants::network_flow::ERROR_NETWORK_APPLE_NETWORK_EXTENSION_GATE_STATUS
                        .to_string(),
                ),
            )]),
            None,
        ),
    }
}

pub(crate) fn network_apple_network_extension_gate_status_payload() -> Result<LogFields, ()> {
    let proof = plan_network_apple_network_extension_gate(gate_input()).map_err(|_error| ())?;
    let status = status_from_proof(&proof);
    let serialized = serde_json::to_string(&status).map_err(|_error| ())?;
    Ok(fields_from_pairs(vec![(
        constants::network_flow::FIELD_NETWORK_APPLE_NETWORK_EXTENSION_GATE_STATUS,
        LogFieldValue::String(serialized),
    )]))
}

fn status_from_proof(
    proof: &NetworkAppleNetworkExtensionGateProof,
) -> NetworkAppleNetworkExtensionGateStatus {
    NetworkAppleNetworkExtensionGateStatus {
        status_ref: constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_GATE_STATUS_REF
            .to_string(),
        apple_network_extension_gate_ref: proof.apple_network_extension_gate_ref.clone(),
        policy_decision_ref: proof.policy_decision_ref.clone(),
        parent_rule_ref: proof.parent_rule_ref.clone(),
        evidence_refs: proof.evidence_refs.clone(),
        local_ai_result_ref: proof.local_ai_result_ref.clone(),
        platform: protocol_platform(proof.platform),
        bundle_ref: proof.bundle_ref.clone(),
        network_extension_ref: proof.network_extension_ref.clone(),
        capability_state: protocol_capability_state(proof.capability_state),
        gate_state: protocol_gate_state(proof.gate_state),
        boundary_reasons: proof.boundary_reasons.iter().map(boundary_reason).collect(),
        missing_required_artifacts: proof
            .missing_required_artifacts
            .iter()
            .map(required_artifact)
            .collect(),
        developer_team_proof_ref: proof.developer_team_proof_ref.clone(),
        entitlement_approval_proof_ref: proof.entitlement_approval_proof_ref.clone(),
        provisioning_profile_proof_ref: proof.provisioning_profile_proof_ref.clone(),
        signing_proof_ref: proof.signing_proof_ref.clone(),
        device_or_testflight_proof_ref: proof.device_or_testflight_proof_ref.clone(),
        network_extension_declaration_ref: proof.network_extension_declaration_ref.clone(),
        extension_configuration_proof_ref: proof.extension_configuration_proof_ref.clone(),
        rollback_plan_ref: proof.rollback_plan_ref.clone(),
        audit_event_ref: proof.audit_event_ref.clone(),
        supervision_required: proof.supervision_required,
        supervision_or_mdm_proof_ref: proof.supervision_or_mdm_proof_ref.clone(),
        apple_entitlement_proof_ready: proof.apple_entitlement_proof_ready,
        supervision_authority_proved: proof.supervision_authority_proved,
        adapter_apply_authorized: proof.adapter_apply_authorized,
        enforcement_command_published: proof.enforcement_command_published,
        simulator_only_product_support_claimed: proof.simulator_only_product_support_claimed,
        live_network_extension_claimed: proof.live_network_extension_claimed,
        packet_block_claimed: proof.packet_block_claimed,
        app_level_control_claimed: proof.app_level_control_claimed,
        exact_url_available: proof.exact_url_available,
        decrypted_payload_available: proof.decrypted_payload_available,
        page_content_available: proof.page_content_available,
    }
}

fn gate_input() -> NetworkAppleNetworkExtensionGateInput {
    NetworkAppleNetworkExtensionGateInput {
        apple_network_extension_gate_ref:
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_GATE_REF.to_string(),
        policy_mapping: policy_mapping(),
        platform: NetworkAppleNetworkExtensionPlatform::Ios,
        bundle_ref: constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_BUNDLE_REF.to_string(),
        network_extension_ref: constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_REF
            .to_string(),
        capability_state: NetworkAppleNetworkExtensionCapabilityState::AppleDeviceReady,
        developer_team_proof_ref: Some(
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_DEVELOPER_TEAM_PROOF_REF
                .to_string(),
        ),
        entitlement_approval_proof_ref: Some(
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_ENTITLEMENT_APPROVAL_PROOF_REF
                .to_string(),
        ),
        provisioning_profile_proof_ref: Some(
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_PROVISIONING_PROFILE_PROOF_REF
                .to_string(),
        ),
        signing_proof_ref: Some(
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_SIGNING_PROOF_REF.to_string(),
        ),
        device_or_testflight_proof_ref: Some(
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_DEVICE_OR_TESTFLIGHT_PROOF_REF
                .to_string(),
        ),
        network_extension_declaration_ref: Some(
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_DECLARATION_REF.to_string(),
        ),
        extension_configuration_proof_ref: Some(
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_CONFIGURATION_PROOF_REF
                .to_string(),
        ),
        rollback_plan_ref: Some(
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_ROLLBACK_PLAN_REF.to_string(),
        ),
        audit_event_ref: Some(
            constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_AUDIT_EVENT_REF.to_string(),
        ),
        supervision_required: false,
        supervision_or_mdm_proof_ref: None,
        research_only: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        simulator_only_product_support_claimed: false,
        live_network_extension_claimed: false,
        packet_block_claimed: false,
        app_level_control_claimed: false,
    }
}

fn policy_mapping() -> NetworkEvidencePolicyMapping {
    map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
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
        evidence_grade: NetworkEvidenceGrade::A,
        requested_action: NetworkEvidencePolicyAction::Block,
        adapter_capability_proof_ref: None,
    })
    .expect_value(constants::event_id::NETWORK_APPLE_NETWORK_EXTENSION_GATE_STATUS_REPORTED)
}

fn protocol_platform(
    platform: NetworkAppleNetworkExtensionPlatform,
) -> NetworkAppleNetworkExtensionPlatformStatus {
    match platform {
        NetworkAppleNetworkExtensionPlatform::MacOs => {
            NetworkAppleNetworkExtensionPlatformStatus::MacOs
        }
        NetworkAppleNetworkExtensionPlatform::Ios => {
            NetworkAppleNetworkExtensionPlatformStatus::Ios
        }
    }
}

fn protocol_capability_state(
    state: NetworkAppleNetworkExtensionCapabilityState,
) -> NetworkAppleNetworkExtensionGateCapabilityStatusState {
    match state {
        NetworkAppleNetworkExtensionCapabilityState::AppleDeviceReady => {
            NetworkAppleNetworkExtensionGateCapabilityStatusState::AppleDeviceReady
        }
        NetworkAppleNetworkExtensionCapabilityState::ManualRequired => {
            NetworkAppleNetworkExtensionGateCapabilityStatusState::ManualRequired
        }
        NetworkAppleNetworkExtensionCapabilityState::Unavailable => {
            NetworkAppleNetworkExtensionGateCapabilityStatusState::Unavailable
        }
    }
}

fn protocol_gate_state(
    state: NetworkAppleNetworkExtensionGateState,
) -> NetworkAppleNetworkExtensionGateStatusState {
    match state {
        NetworkAppleNetworkExtensionGateState::ResearchOnly => {
            NetworkAppleNetworkExtensionGateStatusState::ResearchOnly
        }
        NetworkAppleNetworkExtensionGateState::ManualRequired => {
            NetworkAppleNetworkExtensionGateStatusState::ManualRequired
        }
        NetworkAppleNetworkExtensionGateState::Unavailable => {
            NetworkAppleNetworkExtensionGateStatusState::Unavailable
        }
        NetworkAppleNetworkExtensionGateState::AppleEntitlementProofReady => {
            NetworkAppleNetworkExtensionGateStatusState::AppleEntitlementProofReady
        }
    }
}

fn boundary_reason(
    reason: &NetworkAppleNetworkExtensionGateBoundaryReason,
) -> ProtocolNetworkAppleNetworkExtensionGateBoundaryReason {
    match reason {
        NetworkAppleNetworkExtensionGateBoundaryReason::ResearchOnlyRequested => {
            ProtocolNetworkAppleNetworkExtensionGateBoundaryReason::ResearchOnlyRequested
        }
        NetworkAppleNetworkExtensionGateBoundaryReason::CapabilityManualRequired => {
            ProtocolNetworkAppleNetworkExtensionGateBoundaryReason::CapabilityManualRequired
        }
        NetworkAppleNetworkExtensionGateBoundaryReason::CapabilityUnavailable => {
            ProtocolNetworkAppleNetworkExtensionGateBoundaryReason::CapabilityUnavailable
        }
        NetworkAppleNetworkExtensionGateBoundaryReason::EvidenceGradeBelowProofThreshold => {
            ProtocolNetworkAppleNetworkExtensionGateBoundaryReason::EvidenceGradeBelowProofThreshold
        }
        NetworkAppleNetworkExtensionGateBoundaryReason::PolicyNotNetworkExtensionApproved => {
            ProtocolNetworkAppleNetworkExtensionGateBoundaryReason::PolicyNotNetworkExtensionApproved
        }
        NetworkAppleNetworkExtensionGateBoundaryReason::MissingRequiredArtifact => {
            ProtocolNetworkAppleNetworkExtensionGateBoundaryReason::MissingRequiredArtifact
        }
    }
}

fn required_artifact(
    artifact: &NetworkAppleNetworkExtensionRequiredArtifact,
) -> ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact {
    match artifact {
        NetworkAppleNetworkExtensionRequiredArtifact::DeveloperTeamProof => {
            ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact::DeveloperTeamProof
        }
        NetworkAppleNetworkExtensionRequiredArtifact::EntitlementApprovalProof => {
            ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact::EntitlementApprovalProof
        }
        NetworkAppleNetworkExtensionRequiredArtifact::ProvisioningProfileProof => {
            ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact::ProvisioningProfileProof
        }
        NetworkAppleNetworkExtensionRequiredArtifact::SigningProof => {
            ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact::SigningProof
        }
        NetworkAppleNetworkExtensionRequiredArtifact::DeviceOrTestFlightProof => {
            ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact::DeviceOrTestflightProof
        }
        NetworkAppleNetworkExtensionRequiredArtifact::NetworkExtensionDeclaration => {
            ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact::NetworkExtensionDeclaration
        }
        NetworkAppleNetworkExtensionRequiredArtifact::ExtensionConfigurationProof => {
            ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact::ExtensionConfigurationProof
        }
        NetworkAppleNetworkExtensionRequiredArtifact::RollbackPlan => {
            ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact::RollbackPlan
        }
        NetworkAppleNetworkExtensionRequiredArtifact::AuditEvent => {
            ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact::AuditEvent
        }
        NetworkAppleNetworkExtensionRequiredArtifact::SupervisionOrMdmProof => {
            ProtocolNetworkAppleNetworkExtensionGateRequiredArtifact::SupervisionOrMdmProof
        }
    }
}
