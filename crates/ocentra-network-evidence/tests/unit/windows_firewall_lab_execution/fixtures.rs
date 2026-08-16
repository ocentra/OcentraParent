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
        NetworkWindowsFirewallCapabilityState, NetworkWindowsFirewallProofState,
        NetworkWindowsFirewallTargetKind,
    },
    windows_firewall_lab_execution::types::{
        NetworkWindowsFirewallLabCommandEvidence, NetworkWindowsFirewallLabCommandKind,
        NetworkWindowsFirewallLabExecutionInput, NetworkWindowsFirewallLabUnsupportedClaims,
    },
};

pub fn lab_execution_input() -> NetworkWindowsFirewallLabExecutionInput {
    NetworkWindowsFirewallLabExecutionInput {
        lab_ref: " windows-firewall-lab-execution-row38a ".to_owned(),
        adapter_proof: adapter_proof(adapter_input(
            NetworkWindowsFirewallTargetKind::RemoteAddress,
        )),
        rule_name: " OcentraParentNetworkLab-row38a ".to_owned(),
        target_remote_address: "203.0.113.254".to_owned(),
        windows_host_observed: true,
        administrator_permission_observed: true,
        command_evidence: vec![
            command(NetworkWindowsFirewallLabCommandKind::ApplyRule, true),
            command(
                NetworkWindowsFirewallLabCommandKind::VerifyRulePresent,
                true,
            ),
            command(NetworkWindowsFirewallLabCommandKind::RollbackRule, false),
            command(
                NetworkWindowsFirewallLabCommandKind::VerifyRuleRemoved,
                false,
            ),
        ],
        unsupported_claims: unsupported_claims(),
    }
}

pub fn command(
    kind: NetworkWindowsFirewallLabCommandKind,
    rule_present_after_command: bool,
) -> NetworkWindowsFirewallLabCommandEvidence {
    NetworkWindowsFirewallLabCommandEvidence {
        kind,
        command_ref: format!(" command-ref-{kind:?} "),
        exit_status: 0,
        output_sha256: format!(" output-sha256-{kind:?} "),
        rule_present_after_command,
    }
}

pub fn adapter_proof(
    input: NetworkWindowsFirewallAdapterProofInput,
) -> NetworkWindowsFirewallAdapterProof {
    let proof = plan_network_windows_firewall_adapter_proof(input)
        .expect_value("adapter proof fixture should be valid");
    if proof.proof_state == NetworkWindowsFirewallProofState::ApplyReady {
        assert!(proof.adapter_apply_authorized);
    }
    proof
}

pub fn adapter_input(
    target_kind: NetworkWindowsFirewallTargetKind,
) -> NetworkWindowsFirewallAdapterProofInput {
    NetworkWindowsFirewallAdapterProofInput {
        firewall_adapter_plan_ref: " windows-firewall-adapter-plan-38a ".to_owned(),
        policy_mapping: policy_mapping(NetworkEvidenceGrade::A, NetworkEvidencePolicyAction::Block),
        requested_action: NetworkWindowsFirewallAdapterAction::BlockOutbound,
        windows_os_scope_ref: " windows-os-scope-ref-38a ".to_owned(),
        target_kind,
        target_ref: " remote-endpoint-ref-38a ".to_owned(),
        firewall_rule_ref: " windows-firewall-rule-ref-38a ".to_owned(),
        capability_state: NetworkWindowsFirewallCapabilityState::Supported,
        adapter_authorization_ref: Some(" adapter-authorization-ref-38a ".to_owned()),
        adapter_capability_proof_ref: Some(
            " windows-firewall-capability-proof-ref-38a ".to_owned(),
        ),
        apply_artifact_ref: Some(" windows-firewall-apply-artifact-ref-38a ".to_owned()),
        result_artifact_ref: Some(" windows-firewall-result-artifact-ref-38a ".to_owned()),
        rollback_artifact_ref: Some(" windows-firewall-rollback-artifact-ref-38a ".to_owned()),
        audit_event_ref: Some(" windows-firewall-audit-event-ref-38a ".to_owned()),
        dry_run: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        host_firewall_mutation_claimed: false,
        netsh_command_invoked: false,
        powershell_command_invoked: false,
    }
}

pub fn unsupported_claims() -> NetworkWindowsFirewallLabUnsupportedClaims {
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

fn policy_mapping(
    evidence_grade: NetworkEvidenceGrade,
    requested_action: NetworkEvidencePolicyAction,
) -> NetworkEvidencePolicyMapping {
    map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
        policy_decision_ref: " policy-decision-network-38a ".to_owned(),
        parent_rule_ref: " parent-rule-network-38a ".to_owned(),
        evidence_refs: vec![" network-evidence-38a ".to_owned()],
        local_ai_result_ref: Some(" local-ai-result-ref-38a ".to_owned()),
        evidence_grade,
        requested_action,
        adapter_capability_proof_ref: Some(" adapter-capability-proof-38a ".to_owned()),
    })
    .expect_value("policy mapping input should be valid")
}
