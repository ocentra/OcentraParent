use ocentra_network_evidence::{
    dns::types::NetworkEvidenceGrade,
    linux_adapter_gate::{
        plan_network_linux_adapter_gate, NetworkLinuxAdapterCapabilityState,
        NetworkLinuxAdapterGateInput, NetworkLinuxAdapterGateProof, NetworkLinuxAdapterKind,
    },
    linux_nftables_lab_execution::{
        prove_network_linux_nftables_lab_execution,
        types::{
            NetworkLinuxNftablesLabCommandEvidence, NetworkLinuxNftablesLabCommandKind,
            NetworkLinuxNftablesLabExecutionInput, NetworkLinuxNftablesLabExecutionProof,
            NetworkLinuxNftablesLabExecutionState, NetworkLinuxNftablesLabUnsupportedClaims,
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
use ocentra_parent_agent_protocol::network_linux_nftables_lab_status::{
    NetworkLinuxNftablesLabCommandStatusKind, NetworkLinuxNftablesLabCommandStatusRow,
    NetworkLinuxNftablesLabStatus, NetworkLinuxNftablesLabStatusState,
};
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{event_builder::build_event, fields::fields_from_pairs};

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

fn status_from_proof(
    proof: &NetworkLinuxNftablesLabExecutionProof,
) -> NetworkLinuxNftablesLabStatus {
    NetworkLinuxNftablesLabStatus {
        status_ref: constants::network_flow::TEST_LINUX_NFTABLES_LAB_STATUS_REF.to_string(),
        lab_ref: proof.lab_ref.clone(),
        linux_adapter_gate_ref: proof.linux_adapter_gate_ref.clone(),
        policy_decision_ref: proof.policy_decision_ref.clone(),
        parent_rule_ref: proof.parent_rule_ref.clone(),
        evidence_refs: proof.evidence_refs.clone(),
        distro_ref: proof.distro_ref.clone(),
        kernel_ref: proof.kernel_ref.clone(),
        table_name: proof.table_name.clone(),
        chain_name: proof.chain_name.clone(),
        target_remote_address: proof.target_remote_address.clone(),
        state: protocol_state(proof.state),
        wsl_host_observed: proof.wsl_host_observed,
        root_permission_observed: proof.root_permission_observed,
        nft_tool_observed: proof.nft_tool_observed,
        command_count: count(proof.command_count),
        required_command_count: REQUIRED_COMMAND_COUNT,
        table_create_observed: proof.table_create_observed,
        chain_create_observed: proof.chain_create_observed,
        rule_add_observed: proof.rule_add_observed,
        verify_present_observed: proof.verify_present_observed,
        rollback_observed: proof.rollback_observed,
        verify_removed_observed: proof.verify_removed_observed,
        lab_packet_filter_rule_executed: proof.lab_packet_filter_rule_executed,
        rollback_verified: proof.rollback_verified,
        production_enforcement_claimed: proof.production_enforcement_claimed,
        persistent_rule_claimed: proof.persistent_rule_claimed,
        generic_linux_support_claimed: proof.generic_linux_support_claimed,
        service_manager_install_claimed: proof.service_manager_install_claimed,
        exact_url_available: proof.exact_url_available,
        decrypted_payload_available: proof.decrypted_payload_available,
        page_content_available: proof.page_content_available,
        policy_engine_execution_claimed: proof.policy_engine_execution_claimed,
        enforcement_command_published: proof.enforcement_command_published,
        command_evidence: proof.command_evidence.iter().map(command_row).collect(),
    }
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
    .unwrap_or_else(|_| panic!("{}", constants::error::AGENT_EVENT_SERIALIZES))
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

fn command_evidence_rows() -> Vec<NetworkLinuxNftablesLabCommandEvidence> {
    vec![
        command_evidence(
            NetworkLinuxNftablesLabCommandKind::CreateTable,
            constants::network_flow::TEST_LINUX_NFTABLES_CREATE_TABLE_COMMAND_REF,
            constants::network_flow::TEST_LINUX_NFTABLES_CREATE_TABLE_OUTPUT_SHA256,
            true,
            false,
            false,
        ),
        command_evidence(
            NetworkLinuxNftablesLabCommandKind::CreateChain,
            constants::network_flow::TEST_LINUX_NFTABLES_CREATE_CHAIN_COMMAND_REF,
            constants::network_flow::TEST_LINUX_NFTABLES_CREATE_CHAIN_OUTPUT_SHA256,
            true,
            true,
            false,
        ),
        command_evidence(
            NetworkLinuxNftablesLabCommandKind::AddRule,
            constants::network_flow::TEST_LINUX_NFTABLES_ADD_RULE_COMMAND_REF,
            constants::network_flow::TEST_LINUX_NFTABLES_ADD_RULE_OUTPUT_SHA256,
            true,
            true,
            true,
        ),
        command_evidence(
            NetworkLinuxNftablesLabCommandKind::VerifyRulePresent,
            constants::network_flow::TEST_LINUX_NFTABLES_VERIFY_RULE_COMMAND_REF,
            constants::network_flow::TEST_LINUX_NFTABLES_VERIFY_RULE_OUTPUT_SHA256,
            true,
            true,
            true,
        ),
        command_evidence(
            NetworkLinuxNftablesLabCommandKind::DeleteTable,
            constants::network_flow::TEST_LINUX_NFTABLES_DELETE_TABLE_COMMAND_REF,
            constants::network_flow::TEST_LINUX_NFTABLES_DELETE_TABLE_OUTPUT_SHA256,
            false,
            false,
            false,
        ),
        command_evidence(
            NetworkLinuxNftablesLabCommandKind::VerifyTableRemoved,
            constants::network_flow::TEST_LINUX_NFTABLES_VERIFY_REMOVED_COMMAND_REF,
            constants::network_flow::TEST_LINUX_NFTABLES_VERIFY_REMOVED_OUTPUT_SHA256,
            false,
            false,
            false,
        ),
    ]
}

fn command_evidence(
    kind: NetworkLinuxNftablesLabCommandKind,
    command_ref: &str,
    output_sha256: &str,
    table_present_after_command: bool,
    chain_present_after_command: bool,
    rule_present_after_command: bool,
) -> NetworkLinuxNftablesLabCommandEvidence {
    NetworkLinuxNftablesLabCommandEvidence {
        kind,
        command_ref: command_ref.to_string(),
        exit_status: 0,
        output_sha256: output_sha256.to_string(),
        table_present_after_command,
        chain_present_after_command,
        rule_present_after_command,
    }
}

fn command_row(
    command: &NetworkLinuxNftablesLabCommandEvidence,
) -> NetworkLinuxNftablesLabCommandStatusRow {
    NetworkLinuxNftablesLabCommandStatusRow {
        kind: protocol_command_kind(command.kind),
        command_ref: command.command_ref.clone(),
        exit_status: command.exit_status,
        output_sha256: command.output_sha256.clone(),
        table_present_after_command: command.table_present_after_command,
        chain_present_after_command: command.chain_present_after_command,
        rule_present_after_command: command.rule_present_after_command,
    }
}

fn protocol_state(
    state: NetworkLinuxNftablesLabExecutionState,
) -> NetworkLinuxNftablesLabStatusState {
    match state {
        NetworkLinuxNftablesLabExecutionState::ExecutedAndRolledBack => {
            NetworkLinuxNftablesLabStatusState::ExecutedAndRolledBack
        }
        NetworkLinuxNftablesLabExecutionState::ManualRequired => {
            NetworkLinuxNftablesLabStatusState::ManualRequired
        }
        NetworkLinuxNftablesLabExecutionState::Unavailable => {
            NetworkLinuxNftablesLabStatusState::Unavailable
        }
    }
}

fn protocol_command_kind(
    kind: NetworkLinuxNftablesLabCommandKind,
) -> NetworkLinuxNftablesLabCommandStatusKind {
    match kind {
        NetworkLinuxNftablesLabCommandKind::CreateTable => {
            NetworkLinuxNftablesLabCommandStatusKind::CreateTable
        }
        NetworkLinuxNftablesLabCommandKind::CreateChain => {
            NetworkLinuxNftablesLabCommandStatusKind::CreateChain
        }
        NetworkLinuxNftablesLabCommandKind::AddRule => {
            NetworkLinuxNftablesLabCommandStatusKind::AddRule
        }
        NetworkLinuxNftablesLabCommandKind::VerifyRulePresent => {
            NetworkLinuxNftablesLabCommandStatusKind::VerifyRulePresent
        }
        NetworkLinuxNftablesLabCommandKind::DeleteTable => {
            NetworkLinuxNftablesLabCommandStatusKind::DeleteTable
        }
        NetworkLinuxNftablesLabCommandKind::VerifyTableRemoved => {
            NetworkLinuxNftablesLabCommandStatusKind::VerifyTableRemoved
        }
    }
}

fn count(value: usize) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}
