use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::readiness::*;

#[derive(Clone, Copy)]
enum ExternalAuditRef {
    Missing,
    Present(&'static str),
}

#[test]
fn readiness_proof_accepts_internal_security_privacy_support_and_rollout_gates() {
    let proof = evaluate_network_readiness_proof(readiness_input(false, ExternalAuditRef::Missing))
        .expect_value("complete internal proof refs should pass");

    assert_eq!(
        proof.readiness_state,
        NetworkReadinessState::InternalProofReady
    );
    assert_eq!(proof.gates.len(), 9);
    assert!(proof
        .gates
        .contains(&NetworkReadinessGate::SecurityThreatModel));
    assert!(proof
        .gates
        .contains(&NetworkReadinessGate::RetentionDeleteExport));
    assert!(proof
        .gates
        .contains(&NetworkReadinessGate::SupportAndTraining));
    assert!(proof.finding_codes.is_empty());
    assert_eq!(proof.retention_refs.len(), 6);
    assert_eq!(proof.hardening_refs.len(), 6);
    assert_eq!(proof.support_refs.len(), 5);
    assert_eq!(proof.rollout_refs.len(), 6);
    assert!(!proof.production_rollout_ready);
    assert!(!proof.default_remote_upload_enabled);
    assert!(!proof.policy_authority);
    assert!(!proof.adapter_authority);
    assert_eq!(proof.enforcement_commands_published, 0);
}

#[test]
fn readiness_proof_blocks_production_claim_without_external_signoff() {
    let proof = evaluate_network_readiness_proof(readiness_input(true, ExternalAuditRef::Missing))
        .expect_value("production claim without signoff should be represented as blocked");

    assert_eq!(
        proof.readiness_state,
        NetworkReadinessState::ProductionBlockedPendingExternalSignoff
    );
    assert_eq!(
        proof.finding_codes,
        vec![NetworkReadinessFindingCode::ExternalAuditOrPenTestMissing]
    );
    assert!(!proof.production_rollout_ready);
    assert_eq!(proof.external_audit_or_pen_test_ref, None);
}

#[test]
fn readiness_proof_allows_production_ready_only_with_external_signoff() {
    let proof = evaluate_network_readiness_proof(readiness_input(
        true,
        ExternalAuditRef::Present("external-pen-test-signoff-row50"),
    ))
    .expect_value("external signoff should allow production-ready readiness state");

    assert_eq!(
        proof.readiness_state,
        NetworkReadinessState::ProductionReadyWithExternalSignoff
    );
    assert!(proof.finding_codes.is_empty());
    assert!(proof.production_rollout_ready);
    assert_eq!(
        proof.external_audit_or_pen_test_ref,
        Some("external-pen-test-signoff-row50".to_owned())
    );
}

#[test]
fn readiness_proof_rejects_default_upload_content_authority_and_commands() {
    assert_eq!(
        evaluate_network_readiness_proof(NetworkReadinessProofInput {
            default_remote_upload_claimed: true,
            ..readiness_input(false, ExternalAuditRef::Missing)
        }),
        Err(NetworkReadinessProofError::DefaultRemoteUploadClaimRejected)
    );
    assert_eq!(
        evaluate_network_readiness_proof(NetworkReadinessProofInput {
            exact_url_claimed: true,
            ..readiness_input(false, ExternalAuditRef::Missing)
        }),
        Err(NetworkReadinessProofError::ExactUrlClaimRejected)
    );
    assert_eq!(
        evaluate_network_readiness_proof(NetworkReadinessProofInput {
            adapter_authority_claimed: true,
            ..readiness_input(false, ExternalAuditRef::Missing)
        }),
        Err(NetworkReadinessProofError::AdapterAuthorityClaimRejected)
    );
    assert_eq!(
        evaluate_network_readiness_proof(NetworkReadinessProofInput {
            enforcement_command_claimed: true,
            ..readiness_input(false, ExternalAuditRef::Missing)
        }),
        Err(NetworkReadinessProofError::EnforcementCommandClaimRejected)
    );
}

#[test]
fn readiness_proof_rejects_missing_hardening_support_and_rollout_refs() {
    assert_eq!(
        evaluate_network_readiness_proof(NetworkReadinessProofInput {
            hardening: NetworkHardeningReadinessProof {
                key_rotation_ref: " ".to_owned(),
                ..hardening_proof()
            },
            ..readiness_input(false, ExternalAuditRef::Missing)
        }),
        Err(NetworkReadinessProofError::EmptyHardeningRef)
    );
    assert_eq!(
        evaluate_network_readiness_proof(NetworkReadinessProofInput {
            support: NetworkSupportReadinessProof {
                support_playbook_ref: " ".to_owned(),
                ..support_proof()
            },
            ..readiness_input(false, ExternalAuditRef::Missing)
        }),
        Err(NetworkReadinessProofError::EmptySupportRef)
    );
    assert_eq!(
        evaluate_network_readiness_proof(NetworkReadinessProofInput {
            rollout: NetworkRolloutReadinessProof {
                known_gap_signoff_ref: " ".to_owned(),
                ..rollout_proof()
            },
            ..readiness_input(false, ExternalAuditRef::Missing)
        }),
        Err(NetworkReadinessProofError::EmptyRolloutRef)
    );
}

fn readiness_input(
    production_rollout_claimed: bool,
    external_audit_or_pen_test_ref: ExternalAuditRef,
) -> NetworkReadinessProofInput {
    NetworkReadinessProofInput {
        readiness_report_ref: "network-readiness-row50".to_owned(),
        threat_model_ref: "network-threat-model-row50".to_owned(),
        privacy_review_ref: "network-privacy-review-row50".to_owned(),
        compliance_review_ref: "network-compliance-review-row50".to_owned(),
        retention: retention_proof(),
        hardening: hardening_proof(),
        support: support_proof(),
        rollout: rollout_proof(),
        external_audit_or_pen_test_ref: match external_audit_or_pen_test_ref {
            ExternalAuditRef::Missing => None,
            ExternalAuditRef::Present(value) => Some(value.to_owned()),
        },
        production_rollout_claimed,
        default_remote_upload_claimed: false,
        raw_pcap_without_custody_claimed: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        private_message_claimed: false,
        search_query_claimed: false,
        policy_authority_claimed: false,
        adapter_authority_claimed: false,
        enforcement_command_claimed: false,
    }
}

fn retention_proof() -> NetworkRetentionReadinessProof {
    NetworkRetentionReadinessProof {
        encryption_at_rest_ref: "network-encryption-at-rest-row50".to_owned(),
        quota_rotation_ref: "network-quota-rotation-row50".to_owned(),
        retention_policy_ref: "network-retention-policy-row50".to_owned(),
        delete_export_ref: "network-delete-export-row50".to_owned(),
        custody_ref: "network-custody-row50".to_owned(),
        private_family_traffic_exclusion_ref: "network-private-family-exclusion-row50".to_owned(),
    }
}

fn hardening_proof() -> NetworkHardeningReadinessProof {
    NetworkHardeningReadinessProof {
        key_rotation_ref: "network-key-rotation-row50".to_owned(),
        secret_handling_ref: "network-secret-handling-row50".to_owned(),
        rule_set_provenance_ref: "network-rule-set-provenance-row50".to_owned(),
        rule_set_rollback_ref: "network-rule-set-rollback-row50".to_owned(),
        ai_model_version_promotion_ref: "network-ai-model-promotion-row50".to_owned(),
        ai_model_rollback_ref: "network-ai-model-rollback-row50".to_owned(),
    }
}

fn support_proof() -> NetworkSupportReadinessProof {
    NetworkSupportReadinessProof {
        parent_guide_ref: "network-parent-guide-row50".to_owned(),
        user_guide_ref: "network-user-guide-row50".to_owned(),
        faq_ref: "network-faq-row50".to_owned(),
        support_playbook_ref: "network-support-playbook-row50".to_owned(),
        staff_training_ref: "network-staff-training-row50".to_owned(),
    }
}

fn rollout_proof() -> NetworkRolloutReadinessProof {
    NetworkRolloutReadinessProof {
        deployment_runbook_ref: "network-deployment-runbook-row50".to_owned(),
        rollback_runbook_ref: "network-rollback-runbook-row50".to_owned(),
        staged_rollout_plan_ref: "network-staged-rollout-row50".to_owned(),
        monitoring_ref: "network-monitoring-row50".to_owned(),
        incident_response_ref: "network-incident-response-row50".to_owned(),
        known_gap_signoff_ref: "network-known-gap-signoff-row50".to_owned(),
    }
}
