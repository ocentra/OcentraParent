use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::{
    dns::types::NetworkEvidenceGrade,
    linux_adapter_gate::{
        plan_network_linux_adapter_gate, NetworkLinuxAdapterCapabilityState,
        NetworkLinuxAdapterGateInput, NetworkLinuxAdapterGateProof, NetworkLinuxAdapterKind,
    },
    linux_nftables_lab_execution::{
        prove_network_linux_nftables_lab_execution,
        types::{
            NetworkLinuxNftablesLabExecutionInput, NetworkLinuxNftablesLabExecutionProof,
            NetworkLinuxNftablesLabUnsupportedClaims,
        },
    },
    policy::{
        map_network_evidence_grade_to_policy, NetworkEvidencePolicyAction,
        NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingInput,
    },
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use self::command_evidence::command_evidence_rows;
use self::status::status_from_proof;
use crate::{event_builder::build_event, fields::fields_from_pairs};

#[path = "network_linux_nftables_lab_status_bridge/command_evidence.rs"]
mod command_evidence;
#[path = "network_linux_nftables_lab_status_bridge/status.rs"]
mod status;

const REQUIRED_COMMAND_COUNT: u64 = 6;

pub(crate) fn build_network_linux_nftables_lab_status_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let correlation_id = command.message_id.clone();
    let target = command.source;
    match network_linux_nftables_lab_status_payload() {
        Ok(payload) => build_event(
            constants::event_id::NETWORK_LINUX_NFTABLES_LAB_STATUS_REPORTED,
            &correlation_id,
            target,
            AgentEventName::AgentNetworkLinuxNftablesLabStatusReported,
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
                    constants::network_flow::ERROR_NETWORK_LINUX_NFTABLES_LAB_STATUS.to_string(),
                ),
            )]),
            None,
        ),
    }
}

pub(crate) fn network_linux_nftables_lab_status_payload() -> Result<LogFields, ()> {
    let proof = lab_execution_proof()?;
    let status = status_from_proof(&proof);
    let serialized = serde_json::to_string(&status).map_err(|_error| ())?;
    Ok(fields_from_pairs(vec![(
        constants::network_flow::FIELD_NETWORK_LINUX_NFTABLES_LAB_STATUS,
        LogFieldValue::String(serialized),
    )]))
}

fn lab_execution_proof() -> Result<NetworkLinuxNftablesLabExecutionProof, ()> {
    let gate = plan_network_linux_adapter_gate(gate_input()).map_err(|_error| ())?;
    prove_network_linux_nftables_lab_execution(lab_execution_input(gate)).map_err(|_error| ())
}

fn gate_input() -> NetworkLinuxAdapterGateInput {
    NetworkLinuxAdapterGateInput {
        linux_adapter_gate_ref: constants::network_flow::TEST_LINUX_ADAPTER_GATE_REF.to_string(),
        policy_mapping: policy_mapping(),
        adapter_kind: NetworkLinuxAdapterKind::Nftables,
        distro_ref: constants::network_flow::TEST_LINUX_DISTRO_REF.to_string(),
        kernel_ref: constants::network_flow::TEST_LINUX_KERNEL_REF.to_string(),
        capability_state: NetworkLinuxAdapterCapabilityState::DistroReady,
        distro_kernel_proof_ref: Some(
            constants::network_flow::TEST_LINUX_DISTRO_KERNEL_PROOF_REF.to_string(),
        ),
        permission_proof_ref: Some(
            constants::network_flow::TEST_LINUX_ADAPTER_PERMISSION_PROOF_REF.to_string(),
        ),
        adapter_api_capability_proof_ref: Some(
            constants::network_flow::TEST_LINUX_ADAPTER_API_CAPABILITY_PROOF_REF.to_string(),
        ),
        adapter_plan_proof_ref: Some(
            constants::network_flow::TEST_LINUX_ADAPTER_PLAN_PROOF_REF.to_string(),
        ),
        service_manager_scope_proof_ref: Some(
            constants::network_flow::TEST_LINUX_SERVICE_MANAGER_SCOPE_PROOF_REF.to_string(),
        ),
        rollback_plan_ref: Some(constants::network_flow::TEST_LINUX_ROLLBACK_PLAN_REF.to_string()),
        lab_result_artifact_ref: Some(
            constants::network_flow::TEST_LINUX_LAB_RESULT_ARTIFACT_REF.to_string(),
        ),
        audit_event_ref: Some(
            constants::network_flow::TEST_LINUX_ADAPTER_AUDIT_EVENT_REF.to_string(),
        ),
        research_only: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        generic_linux_support_claimed: false,
        live_adapter_install_claimed: false,
        packet_filtering_claimed: false,
        kernel_hook_loaded_claimed: false,
        tun_interface_mutation_claimed: false,
        service_manager_install_claimed: false,
    }
}

fn policy_mapping() -> NetworkEvidencePolicyMapping {
    map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
        policy_decision_ref: constants::network_flow::TEST_LINUX_ADAPTER_POLICY_DECISION_REF
            .to_string(),
        parent_rule_ref: constants::network_flow::TEST_LINUX_ADAPTER_PARENT_RULE_REF.to_string(),
        evidence_refs: vec![constants::network_flow::TEST_LINUX_ADAPTER_EVIDENCE_REF.to_string()],
        local_ai_result_ref: Some(
            constants::network_flow::TEST_LINUX_ADAPTER_LOCAL_AI_RESULT_REF.to_string(),
        ),
        evidence_grade: NetworkEvidenceGrade::A,
        requested_action: NetworkEvidencePolicyAction::Block,
        adapter_capability_proof_ref: Some(
            constants::network_flow::TEST_LINUX_ADAPTER_CAPABILITY_PROOF_REF.to_string(),
        ),
    })
    .expect_value(constants::event_id::NETWORK_LINUX_NFTABLES_LAB_STATUS_REPORTED)
}

fn lab_execution_input(
    gate_proof: NetworkLinuxAdapterGateProof,
) -> NetworkLinuxNftablesLabExecutionInput {
    NetworkLinuxNftablesLabExecutionInput {
        lab_ref: constants::network_flow::TEST_LINUX_NFTABLES_LAB_REF.to_string(),
        gate_proof,
        table_name: constants::network_flow::TEST_LINUX_NFTABLES_TABLE_NAME.to_string(),
        chain_name: constants::network_flow::TEST_LINUX_NFTABLES_CHAIN_NAME.to_string(),
        target_remote_address: constants::network_flow::TEST_LINUX_NFTABLES_TARGET_REMOTE_ADDRESS
            .to_string(),
        wsl_host_observed: true,
        root_permission_observed: true,
        nft_tool_observed: true,
        command_evidence: command_evidence_rows(),
        unsupported_claims: unsupported_claims(),
    }
}

fn unsupported_claims() -> NetworkLinuxNftablesLabUnsupportedClaims {
    NetworkLinuxNftablesLabUnsupportedClaims {
        production_enforcement_claimed: false,
        persistent_rule_claimed: false,
        generic_linux_support_claimed: false,
        service_manager_install_claimed: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        policy_engine_execution_claimed: false,
        enforcement_command_published: false,
    }
}
