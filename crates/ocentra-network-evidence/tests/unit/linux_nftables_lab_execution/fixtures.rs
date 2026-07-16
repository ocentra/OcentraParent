use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::{
    dns::types::NetworkEvidenceGrade,
    linux_adapter_gate::{
        plan_network_linux_adapter_gate, NetworkLinuxAdapterCapabilityState,
        NetworkLinuxAdapterGateInput, NetworkLinuxAdapterGateProof, NetworkLinuxAdapterGateState,
        NetworkLinuxAdapterKind,
    },
    linux_nftables_lab_execution::types::{
        NetworkLinuxNftablesLabCommandEvidence, NetworkLinuxNftablesLabCommandKind,
        NetworkLinuxNftablesLabExecutionInput, NetworkLinuxNftablesLabUnsupportedClaims,
    },
    policy::{
        map_network_evidence_grade_to_policy, NetworkEvidencePolicyAction,
        NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingInput,
    },
};

pub fn lab_execution_input() -> NetworkLinuxNftablesLabExecutionInput {
    NetworkLinuxNftablesLabExecutionInput {
        lab_ref: " linux-nftables-lab-execution-row42a ".to_owned(),
        gate_proof: gate_proof(gate_input(NetworkLinuxAdapterKind::Nftables)),
        table_name: " ocentra_parent_lab_row42a ".to_owned(),
        chain_name: " ocentra_parent_lab_chain_row42a ".to_owned(),
        target_remote_address: "203.0.113.253".to_owned(),
        wsl_host_observed: true,
        root_permission_observed: true,
        nft_tool_observed: true,
        command_evidence: vec![
            command(
                NetworkLinuxNftablesLabCommandKind::CreateTable,
                true,
                false,
                false,
            ),
            command(
                NetworkLinuxNftablesLabCommandKind::CreateChain,
                true,
                true,
                false,
            ),
            command(
                NetworkLinuxNftablesLabCommandKind::AddRule,
                true,
                true,
                true,
            ),
            command(
                NetworkLinuxNftablesLabCommandKind::VerifyRulePresent,
                true,
                true,
                true,
            ),
            command(
                NetworkLinuxNftablesLabCommandKind::DeleteTable,
                false,
                false,
                false,
            ),
            command(
                NetworkLinuxNftablesLabCommandKind::VerifyTableRemoved,
                false,
                false,
                false,
            ),
        ],
        unsupported_claims: unsupported_claims(),
    }
}

pub fn command(
    kind: NetworkLinuxNftablesLabCommandKind,
    table_present_after_command: bool,
    chain_present_after_command: bool,
    rule_present_after_command: bool,
) -> NetworkLinuxNftablesLabCommandEvidence {
    NetworkLinuxNftablesLabCommandEvidence {
        kind,
        command_ref: format!(" linux-nftables-command-ref-{kind:?} "),
        exit_status: 0,
        output_sha256: format!(" linux-nftables-output-sha256-{kind:?} "),
        table_present_after_command,
        chain_present_after_command,
        rule_present_after_command,
    }
}

pub fn gate_proof(input: NetworkLinuxAdapterGateInput) -> NetworkLinuxAdapterGateProof {
    let proof = plan_network_linux_adapter_gate(input)
        .expect_value("Linux adapter gate fixture should be valid");
    if proof.gate_state == NetworkLinuxAdapterGateState::DistroProofReady {
        assert!(proof.distro_proof_ready);
    }
    proof
}

pub fn gate_input(adapter_kind: NetworkLinuxAdapterKind) -> NetworkLinuxAdapterGateInput {
    NetworkLinuxAdapterGateInput {
        linux_adapter_gate_ref: " linux-adapter-gate-ref-42a ".to_owned(),
        policy_mapping: policy_mapping(NetworkEvidenceGrade::A, NetworkEvidencePolicyAction::Block),
        adapter_kind,
        distro_ref: " linux-distro-ref-42a ".to_owned(),
        kernel_ref: " linux-kernel-ref-42a ".to_owned(),
        capability_state: NetworkLinuxAdapterCapabilityState::DistroReady,
        distro_kernel_proof_ref: Some(" distro-kernel-proof-ref-42a ".to_owned()),
        permission_proof_ref: Some(" permission-proof-ref-42a ".to_owned()),
        adapter_api_capability_proof_ref: Some(" adapter-api-capability-proof-ref-42a ".to_owned()),
        adapter_plan_proof_ref: Some(" adapter-plan-proof-ref-42a ".to_owned()),
        service_manager_scope_proof_ref: Some(" service-manager-scope-proof-ref-42a ".to_owned()),
        rollback_plan_ref: Some(" rollback-plan-ref-42a ".to_owned()),
        lab_result_artifact_ref: Some(" lab-result-artifact-ref-42a ".to_owned()),
        audit_event_ref: Some(" linux-adapter-audit-event-ref-42a ".to_owned()),
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

pub fn unsupported_claims() -> NetworkLinuxNftablesLabUnsupportedClaims {
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

fn policy_mapping(
    evidence_grade: NetworkEvidenceGrade,
    requested_action: NetworkEvidencePolicyAction,
) -> NetworkEvidencePolicyMapping {
    map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
        policy_decision_ref: " policy-decision-network-42a ".to_owned(),
        parent_rule_ref: " parent-rule-network-42a ".to_owned(),
        evidence_refs: vec![" network-evidence-42a ".to_owned()],
        local_ai_result_ref: Some(" local-ai-result-ref-42a ".to_owned()),
        evidence_grade,
        requested_action,
        adapter_capability_proof_ref: Some(" adapter-capability-proof-42a ".to_owned()),
    })
    .expect_value("policy mapping input should be valid")
}
