use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::{
    dns::types::NetworkEvidenceGrade,
    policy::{
        map_network_evidence_grade_to_policy, NetworkEvidencePolicyAction,
        NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingInput,
    },
    windows_firewall_adapter::{
        plan_network_windows_firewall_adapter_proof, NetworkWindowsFirewallAdapterAction,
        NetworkWindowsFirewallAdapterProof, NetworkWindowsFirewallAdapterProofInput,
        NetworkWindowsFirewallCapabilityState, NetworkWindowsFirewallTargetKind,
    },
    windows_firewall_lab_execution::{
        prove_network_windows_firewall_lab_execution,
        types::{
            NetworkWindowsFirewallLabCommandEvidence, NetworkWindowsFirewallLabCommandKind,
            NetworkWindowsFirewallLabExecutionInput, NetworkWindowsFirewallLabExecutionProof,
            NetworkWindowsFirewallLabExecutionState, NetworkWindowsFirewallLabUnsupportedClaims,
        },
    },
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::network_windows_firewall_lab_status::{
    NetworkWindowsFirewallLabCommandStatusKind, NetworkWindowsFirewallLabCommandStatusRow,
    NetworkWindowsFirewallLabStatus, NetworkWindowsFirewallLabStatusState,
};
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{event_builder::build_event, fields::fields_from_pairs};

const REQUIRED_COMMAND_COUNT: u64 = 4;

pub(crate) fn build_network_windows_firewall_lab_status_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let correlation_id = command.message_id.clone();
    let target = command.source;
    match network_windows_firewall_lab_status_payload() {
        Ok(payload) => build_event(
            constants::event_id::NETWORK_WINDOWS_FIREWALL_LAB_STATUS_REPORTED,
            &correlation_id,
            target,
            AgentEventName::AgentNetworkWindowsFirewallLabStatusReported,
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
                    constants::network_flow::ERROR_NETWORK_WINDOWS_FIREWALL_LAB_STATUS.to_string(),
                ),
            )]),
            None,
        ),
    }
}

pub(crate) fn network_windows_firewall_lab_status_payload() -> Result<LogFields, ()> {
    let proof = lab_execution_proof()?;
    let status = status_from_proof(&proof);
    let serialized = serde_json::to_string(&status).map_err(|_error| ())?;
    Ok(fields_from_pairs(vec![(
        constants::network_flow::FIELD_NETWORK_WINDOWS_FIREWALL_LAB_STATUS,
        LogFieldValue::String(serialized),
    )]))
}

fn lab_execution_proof() -> Result<NetworkWindowsFirewallLabExecutionProof, ()> {
    let adapter_proof =
        plan_network_windows_firewall_adapter_proof(adapter_input()).map_err(|_error| ())?;
    prove_network_windows_firewall_lab_execution(lab_execution_input(adapter_proof))
        .map_err(|_error| ())
}

fn status_from_proof(
    proof: &NetworkWindowsFirewallLabExecutionProof,
) -> NetworkWindowsFirewallLabStatus {
    NetworkWindowsFirewallLabStatus {
        status_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_LAB_STATUS_REF.to_string(),
        lab_ref: proof.lab_ref.clone(),
        firewall_adapter_plan_ref: proof.adapter_plan_ref.clone(),
        policy_decision_ref: proof.policy_decision_ref.clone(),
        parent_rule_ref: proof.parent_rule_ref.clone(),
        evidence_refs: proof.evidence_refs.clone(),
        windows_os_scope_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_OS_SCOPE_REF
            .to_string(),
        target_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_TARGET_REF.to_string(),
        firewall_rule_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_RULE_REF.to_string(),
        rule_name: proof.rule_name.clone(),
        target_remote_address: proof.target_remote_address.clone(),
        state: protocol_state(proof.state),
        windows_host_observed: true,
        administrator_permission_observed: true,
        command_count: count(proof.command_count),
        required_command_count: REQUIRED_COMMAND_COUNT,
        apply_command_observed: proof.apply_command_observed,
        verify_present_observed: proof.verify_present_observed,
        rollback_command_observed: proof.rollback_command_observed,
        verify_removed_observed: proof.verify_removed_observed,
        lab_firewall_mutation_executed: proof.lab_firewall_mutation_executed,
        rollback_verified: proof.rollback_verified,
        adapter_apply_authorized: true,
        production_enforcement_claimed: proof.production_enforcement_claimed,
        persistent_rule_claimed: proof.persistent_rule_claimed,
        exact_url_available: proof.exact_url_available,
        decrypted_payload_available: proof.decrypted_payload_available,
        page_content_available: proof.page_content_available,
        host_firewall_mutation_claimed: false,
        netsh_command_invoked: false,
        powershell_command_invoked: false,
        policy_engine_execution_claimed: proof.policy_engine_execution_claimed,
        enforcement_command_published: proof.enforcement_command_published,
        command_evidence: proof.command_evidence.iter().map(command_row).collect(),
    }
}

fn adapter_input() -> NetworkWindowsFirewallAdapterProofInput {
    NetworkWindowsFirewallAdapterProofInput {
        firewall_adapter_plan_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_ADAPTER_PLAN_REF
            .to_string(),
        policy_mapping: policy_mapping(),
        requested_action: NetworkWindowsFirewallAdapterAction::BlockOutbound,
        windows_os_scope_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_OS_SCOPE_REF
            .to_string(),
        target_kind: NetworkWindowsFirewallTargetKind::RemoteAddress,
        target_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_TARGET_REF.to_string(),
        firewall_rule_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_RULE_REF.to_string(),
        capability_state: NetworkWindowsFirewallCapabilityState::Supported,
        adapter_authorization_ref: Some(
            constants::network_flow::TEST_WINDOWS_FIREWALL_AUTHORIZATION_REF.to_string(),
        ),
        adapter_capability_proof_ref: Some(
            constants::network_flow::TEST_WINDOWS_FIREWALL_CAPABILITY_PROOF_REF.to_string(),
        ),
        apply_artifact_ref: Some(
            constants::network_flow::TEST_WINDOWS_FIREWALL_APPLY_ARTIFACT_REF.to_string(),
        ),
        result_artifact_ref: Some(
            constants::network_flow::TEST_WINDOWS_FIREWALL_RESULT_ARTIFACT_REF.to_string(),
        ),
        rollback_artifact_ref: Some(
            constants::network_flow::TEST_WINDOWS_FIREWALL_ROLLBACK_ARTIFACT_REF.to_string(),
        ),
        audit_event_ref: Some(
            constants::network_flow::TEST_WINDOWS_FIREWALL_AUDIT_EVENT_REF.to_string(),
        ),
        dry_run: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        host_firewall_mutation_claimed: false,
        netsh_command_invoked: false,
        powershell_command_invoked: false,
    }
}

fn policy_mapping() -> NetworkEvidencePolicyMapping {
    map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
        policy_decision_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_POLICY_DECISION_REF
            .to_string(),
        parent_rule_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_PARENT_RULE_REF.to_string(),
        evidence_refs: vec![
            constants::network_flow::TEST_WINDOWS_FIREWALL_EVIDENCE_REF.to_string(),
        ],
        local_ai_result_ref: Some(
            constants::network_flow::TEST_WINDOWS_FIREWALL_LOCAL_AI_RESULT_REF.to_string(),
        ),
        evidence_grade: NetworkEvidenceGrade::A,
        requested_action: NetworkEvidencePolicyAction::Block,
        adapter_capability_proof_ref: Some(
            constants::network_flow::TEST_WINDOWS_FIREWALL_CAPABILITY_PROOF_REF.to_string(),
        ),
    })
    .expect_value(constants::event_id::NETWORK_WINDOWS_FIREWALL_LAB_STATUS_REPORTED)
}

fn lab_execution_input(
    adapter_proof: NetworkWindowsFirewallAdapterProof,
) -> NetworkWindowsFirewallLabExecutionInput {
    NetworkWindowsFirewallLabExecutionInput {
        lab_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_LAB_REF.to_string(),
        adapter_proof,
        rule_name: constants::network_flow::TEST_WINDOWS_FIREWALL_RULE_NAME.to_string(),
        target_remote_address: constants::network_flow::TEST_WINDOWS_FIREWALL_TARGET_REMOTE_ADDRESS
            .to_string(),
        windows_host_observed: true,
        administrator_permission_observed: true,
        command_evidence: command_evidence_rows(),
        unsupported_claims: unsupported_claims(),
    }
}

fn unsupported_claims() -> NetworkWindowsFirewallLabUnsupportedClaims {
    NetworkWindowsFirewallLabUnsupportedClaims {
        production_enforcement_claimed: false,
        persistent_rule_claimed: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        policy_engine_execution_claimed: false,
        enforcement_command_published: false,
    }
}

fn command_evidence_rows() -> Vec<NetworkWindowsFirewallLabCommandEvidence> {
    vec![
        NetworkWindowsFirewallLabCommandEvidence {
            kind: NetworkWindowsFirewallLabCommandKind::ApplyRule,
            command_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_APPLY_RULE_COMMAND_REF
                .to_string(),
            exit_status: 0,
            output_sha256: constants::network_flow::TEST_WINDOWS_FIREWALL_APPLY_RULE_OUTPUT_SHA256
                .to_string(),
            rule_present_after_command: true,
        },
        NetworkWindowsFirewallLabCommandEvidence {
            kind: NetworkWindowsFirewallLabCommandKind::VerifyRulePresent,
            command_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_VERIFY_PRESENT_COMMAND_REF
                .to_string(),
            exit_status: 0,
            output_sha256:
                constants::network_flow::TEST_WINDOWS_FIREWALL_VERIFY_PRESENT_OUTPUT_SHA256
                    .to_string(),
            rule_present_after_command: true,
        },
        NetworkWindowsFirewallLabCommandEvidence {
            kind: NetworkWindowsFirewallLabCommandKind::RollbackRule,
            command_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_ROLLBACK_RULE_COMMAND_REF
                .to_string(),
            exit_status: 0,
            output_sha256:
                constants::network_flow::TEST_WINDOWS_FIREWALL_ROLLBACK_RULE_OUTPUT_SHA256
                    .to_string(),
            rule_present_after_command: false,
        },
        NetworkWindowsFirewallLabCommandEvidence {
            kind: NetworkWindowsFirewallLabCommandKind::VerifyRuleRemoved,
            command_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_VERIFY_REMOVED_COMMAND_REF
                .to_string(),
            exit_status: 0,
            output_sha256:
                constants::network_flow::TEST_WINDOWS_FIREWALL_VERIFY_REMOVED_OUTPUT_SHA256
                    .to_string(),
            rule_present_after_command: false,
        },
    ]
}

fn command_row(
    command: &NetworkWindowsFirewallLabCommandEvidence,
) -> NetworkWindowsFirewallLabCommandStatusRow {
    NetworkWindowsFirewallLabCommandStatusRow {
        kind: protocol_command_kind(command.kind),
        command_ref: command.command_ref.clone(),
        exit_status: command.exit_status,
        output_sha256: command.output_sha256.clone(),
        rule_present_after_command: command.rule_present_after_command,
    }
}

fn protocol_state(
    state: NetworkWindowsFirewallLabExecutionState,
) -> NetworkWindowsFirewallLabStatusState {
    match state {
        NetworkWindowsFirewallLabExecutionState::ExecutedAndRolledBack => {
            NetworkWindowsFirewallLabStatusState::ExecutedAndRolledBack
        }
        NetworkWindowsFirewallLabExecutionState::ManualRequired => {
            NetworkWindowsFirewallLabStatusState::ManualRequired
        }
        NetworkWindowsFirewallLabExecutionState::Unavailable => {
            NetworkWindowsFirewallLabStatusState::Unavailable
        }
    }
}

fn protocol_command_kind(
    kind: NetworkWindowsFirewallLabCommandKind,
) -> NetworkWindowsFirewallLabCommandStatusKind {
    match kind {
        NetworkWindowsFirewallLabCommandKind::ApplyRule => {
            NetworkWindowsFirewallLabCommandStatusKind::ApplyRule
        }
        NetworkWindowsFirewallLabCommandKind::VerifyRulePresent => {
            NetworkWindowsFirewallLabCommandStatusKind::VerifyRulePresent
        }
        NetworkWindowsFirewallLabCommandKind::RollbackRule => {
            NetworkWindowsFirewallLabCommandStatusKind::RollbackRule
        }
        NetworkWindowsFirewallLabCommandKind::VerifyRuleRemoved => {
            NetworkWindowsFirewallLabCommandStatusKind::VerifyRuleRemoved
        }
    }
}

fn count(value: usize) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}
