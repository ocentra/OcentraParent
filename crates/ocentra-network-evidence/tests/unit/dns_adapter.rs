use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::dns::types::*;
use ocentra_network_evidence::dns_adapter::*;
use ocentra_network_evidence::policy::*;

#[test]
fn dns_adapter_allows_apply_ready_with_policy_capability_artifacts_and_audit_refs() {
    let proof = plan_network_dns_adapter_proof(apply_ready_input(NetworkDnsAdapterAction::Block))
        .expect_value("complete DNS adapter proof should become apply-ready");

    assert_eq!(proof.proof_state, NetworkDnsAdapterProofState::ApplyReady);
    assert_eq!(proof.target_domain, "video.example.test");
    assert_eq!(proof.redirect_target_domain, None);
    assert_eq!(proof.policy_decision_ref, "policy-decision-network-37");
    assert_eq!(proof.parent_rule_ref, "parent-rule-network-37");
    assert_eq!(proof.evidence_refs, vec!["network-evidence-37"]);
    assert_eq!(proof.boundary_reasons, Vec::new());
    assert_eq!(proof.missing_required_artifacts, Vec::new());
    assert_eq!(
        proof.adapter_authorization_ref,
        Some("adapter-authorization-ref-37".to_owned())
    );
    assert_eq!(
        proof.adapter_capability_proof_ref,
        Some("dns-capability-proof-ref-37".to_owned())
    );
    assert_eq!(
        proof.apply_artifact_ref,
        Some("dns-apply-artifact-ref-37".to_owned())
    );
    assert_eq!(
        proof.result_artifact_ref,
        Some("dns-result-artifact-ref-37".to_owned())
    );
    assert_eq!(
        proof.rollback_artifact_ref,
        Some("dns-rollback-artifact-ref-37".to_owned())
    );
    assert_eq!(
        proof.audit_event_ref,
        Some("dns-audit-event-ref-37".to_owned())
    );
    assert!(proof.adapter_apply_authorized);
    assert!(!proof.enforcement_command_published);
    assert!(!proof.host_dns_mutation_claimed);
    assert!(!proof.exact_url_available);
    assert!(!proof.decrypted_payload_available);
    assert!(!proof.page_content_available);
}

#[test]
fn dns_adapter_redirect_requires_a_domain_target_and_preserves_no_content_claims() {
    let proof =
        plan_network_dns_adapter_proof(apply_ready_input(NetworkDnsAdapterAction::Redirect))
            .expect_value("redirect proof should require and normalize a redirect target domain");

    assert_eq!(proof.proof_state, NetworkDnsAdapterProofState::ApplyReady);
    assert_eq!(proof.target_domain, "video.example.test");
    assert_eq!(
        proof.redirect_target_domain,
        Some("safe-search.example.test".to_owned())
    );
    assert!(proof.adapter_apply_authorized);
    assert!(!proof.enforcement_command_published);
    assert!(!proof.host_dns_mutation_claimed);

    assert_eq!(
        plan_network_dns_adapter_proof(NetworkDnsAdapterProofInput {
            redirect_target_domain: None,
            ..apply_ready_input(NetworkDnsAdapterAction::Redirect)
        }),
        Err(NetworkDnsAdapterProofError::MissingRedirectTargetDomain)
    );
}

#[test]
fn dns_adapter_dry_run_is_non_executable_without_adapter_artifacts() {
    let proof = plan_network_dns_adapter_proof(NetworkDnsAdapterProofInput {
        dry_run: true,
        adapter_authorization_ref: None,
        adapter_capability_proof_ref: None,
        apply_artifact_ref: None,
        result_artifact_ref: None,
        rollback_artifact_ref: None,
        audit_event_ref: None,
        ..apply_ready_input(NetworkDnsAdapterAction::Block)
    })
    .expect_value("dry-run DNS adapter proof should be allowed without apply artifacts");

    assert_eq!(proof.proof_state, NetworkDnsAdapterProofState::DryRun);
    assert_eq!(
        proof.boundary_reasons,
        vec![
            NetworkDnsAdapterBoundaryReason::DryRunRequested,
            NetworkDnsAdapterBoundaryReason::MissingRequiredArtifact
        ]
    );
    assert_eq!(
        proof.missing_required_artifacts,
        vec![
            NetworkDnsAdapterRequiredArtifact::AdapterAuthorization,
            NetworkDnsAdapterRequiredArtifact::CapabilityProof,
            NetworkDnsAdapterRequiredArtifact::ApplyArtifact,
            NetworkDnsAdapterRequiredArtifact::ResultArtifact,
            NetworkDnsAdapterRequiredArtifact::RollbackArtifact,
            NetworkDnsAdapterRequiredArtifact::AuditEvent
        ]
    );
    assert!(!proof.adapter_apply_authorized);
    assert!(!proof.enforcement_command_published);
}

#[test]
fn dns_adapter_routes_weak_or_parent_review_policy_to_manual_required() {
    let proof = plan_network_dns_adapter_proof(NetworkDnsAdapterProofInput {
        policy_mapping: policy_mapping(NetworkEvidenceGrade::B, NetworkEvidencePolicyAction::Block),
        ..apply_ready_input(NetworkDnsAdapterAction::Block)
    })
    .expect_value("grade B block policy handoff should not become DNS apply-ready");

    assert_eq!(
        proof.proof_state,
        NetworkDnsAdapterProofState::ManualRequired
    );
    assert_eq!(
        proof.boundary_reasons,
        vec![
            NetworkDnsAdapterBoundaryReason::EvidenceGradeBelowApplyThreshold,
            NetworkDnsAdapterBoundaryReason::PolicyNotAdapterApproved
        ]
    );
    assert!(!proof.adapter_apply_authorized);

    let observe = plan_network_dns_adapter_proof(NetworkDnsAdapterProofInput {
        policy_mapping: policy_mapping(NetworkEvidenceGrade::D, NetworkEvidencePolicyAction::Block),
        ..apply_ready_input(NetworkDnsAdapterAction::Block)
    })
    .expect_value("grade D network evidence should stay non-enforcing");
    assert_eq!(
        observe.proof_state,
        NetworkDnsAdapterProofState::ManualRequired
    );
    assert!(!observe.adapter_apply_authorized);
    assert!(observe
        .boundary_reasons
        .contains(&NetworkDnsAdapterBoundaryReason::EvidenceGradeBelowApplyThreshold));
}

#[test]
fn dns_adapter_marks_capability_manual_required_or_unavailable_without_commands() {
    let manual = plan_network_dns_adapter_proof(NetworkDnsAdapterProofInput {
        capability_state: NetworkDnsAdapterCapabilityState::ManualRequired,
        ..apply_ready_input(NetworkDnsAdapterAction::Block)
    })
    .expect_value("manual-required capability should stay reportable");
    assert_eq!(
        manual.proof_state,
        NetworkDnsAdapterProofState::ManualRequired
    );
    assert_eq!(
        manual.boundary_reasons,
        vec![NetworkDnsAdapterBoundaryReason::CapabilityManualRequired]
    );
    assert!(!manual.adapter_apply_authorized);

    let unavailable = plan_network_dns_adapter_proof(NetworkDnsAdapterProofInput {
        capability_state: NetworkDnsAdapterCapabilityState::Unavailable,
        ..apply_ready_input(NetworkDnsAdapterAction::Block)
    })
    .expect_value("unavailable capability should stay reportable");
    assert_eq!(
        unavailable.proof_state,
        NetworkDnsAdapterProofState::Unavailable
    );
    assert_eq!(
        unavailable.boundary_reasons,
        vec![NetworkDnsAdapterBoundaryReason::CapabilityUnavailable]
    );
    assert!(!unavailable.adapter_apply_authorized);
}

#[test]
fn dns_adapter_requires_authorization_capability_apply_result_rollback_and_audit_refs() {
    let proof = plan_network_dns_adapter_proof(NetworkDnsAdapterProofInput {
        adapter_authorization_ref: None,
        adapter_capability_proof_ref: None,
        apply_artifact_ref: None,
        result_artifact_ref: None,
        rollback_artifact_ref: None,
        audit_event_ref: None,
        ..apply_ready_input(NetworkDnsAdapterAction::Block)
    })
    .expect_value("missing DNS adapter artifacts should produce a manual-required proof");

    assert_eq!(
        proof.proof_state,
        NetworkDnsAdapterProofState::ManualRequired
    );
    assert_eq!(
        proof.boundary_reasons,
        vec![NetworkDnsAdapterBoundaryReason::MissingRequiredArtifact]
    );
    assert_eq!(
        proof.missing_required_artifacts,
        vec![
            NetworkDnsAdapterRequiredArtifact::AdapterAuthorization,
            NetworkDnsAdapterRequiredArtifact::CapabilityProof,
            NetworkDnsAdapterRequiredArtifact::ApplyArtifact,
            NetworkDnsAdapterRequiredArtifact::ResultArtifact,
            NetworkDnsAdapterRequiredArtifact::RollbackArtifact,
            NetworkDnsAdapterRequiredArtifact::AuditEvent
        ]
    );
    assert!(!proof.adapter_apply_authorized);
}

#[test]
fn dns_adapter_rejects_network_only_exact_url_content_or_decrypted_payload_claims() {
    assert_eq!(
        plan_network_dns_adapter_proof(NetworkDnsAdapterProofInput {
            exact_url_claimed: true,
            ..apply_ready_input(NetworkDnsAdapterAction::Block)
        }),
        Err(NetworkDnsAdapterProofError::ExactUrlClaimRejected)
    );
    assert_eq!(
        plan_network_dns_adapter_proof(NetworkDnsAdapterProofInput {
            decrypted_payload_claimed: true,
            ..apply_ready_input(NetworkDnsAdapterAction::Block)
        }),
        Err(NetworkDnsAdapterProofError::DecryptedPayloadClaimRejected)
    );
    assert_eq!(
        plan_network_dns_adapter_proof(NetworkDnsAdapterProofInput {
            page_content_claimed: true,
            ..apply_ready_input(NetworkDnsAdapterAction::Block)
        }),
        Err(NetworkDnsAdapterProofError::PageContentClaimRejected)
    );
}

#[test]
fn dns_adapter_rejects_invalid_domains_and_upstream_authority_bypass() {
    assert!(matches!(
        plan_network_dns_adapter_proof(NetworkDnsAdapterProofInput {
            target_domain: "https://video.example.test/watch".to_owned(),
            ..apply_ready_input(NetworkDnsAdapterAction::Block)
        }),
        Err(NetworkDnsAdapterProofError::InvalidTargetDomain(_))
    ));

    let mut mapping = policy_mapping(NetworkEvidenceGrade::A, NetworkEvidencePolicyAction::Block);
    mapping.enforcement_command_authorized = true;
    assert_eq!(
        plan_network_dns_adapter_proof(NetworkDnsAdapterProofInput {
            policy_mapping: mapping,
            ..apply_ready_input(NetworkDnsAdapterAction::Block)
        }),
        Err(NetworkDnsAdapterProofError::PolicyMappingAuthorityRejected)
    );
}

fn apply_ready_input(action: NetworkDnsAdapterAction) -> NetworkDnsAdapterProofInput {
    NetworkDnsAdapterProofInput {
        dns_adapter_plan_ref: " dns-adapter-plan-37 ".to_owned(),
        policy_mapping: policy_mapping(NetworkEvidenceGrade::A, NetworkEvidencePolicyAction::Block),
        requested_action: action,
        target_domain: " Video.Example.Test ".to_owned(),
        redirect_target_domain: match action {
            NetworkDnsAdapterAction::Redirect => Some(" Safe-Search.Example.Test ".to_owned()),
            NetworkDnsAdapterAction::Block => None,
        },
        capability_state: NetworkDnsAdapterCapabilityState::Supported,
        adapter_authorization_ref: Some(" adapter-authorization-ref-37 ".to_owned()),
        adapter_capability_proof_ref: Some(" dns-capability-proof-ref-37 ".to_owned()),
        apply_artifact_ref: Some(" dns-apply-artifact-ref-37 ".to_owned()),
        result_artifact_ref: Some(" dns-result-artifact-ref-37 ".to_owned()),
        rollback_artifact_ref: Some(" dns-rollback-artifact-ref-37 ".to_owned()),
        audit_event_ref: Some(" dns-audit-event-ref-37 ".to_owned()),
        dry_run: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
    }
}

fn policy_mapping(
    evidence_grade: NetworkEvidenceGrade,
    requested_action: NetworkEvidencePolicyAction,
) -> NetworkEvidencePolicyMapping {
    map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
        policy_decision_ref: " policy-decision-network-37 ".to_owned(),
        parent_rule_ref: " parent-rule-network-37 ".to_owned(),
        evidence_refs: vec![
            " network-evidence-37 ".to_owned(),
            "network-evidence-37".to_owned(),
        ],
        local_ai_result_ref: Some(" local-ai-result-ref-37 ".to_owned()),
        evidence_grade,
        requested_action,
        adapter_capability_proof_ref: None,
    })
    .expect_value("policy mapping input should be valid")
}
