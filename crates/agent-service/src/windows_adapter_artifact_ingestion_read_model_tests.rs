use ocentra_parent_agent_protocol::{
    constants::{
        windows_adapter_artifact_gate as artifact_gate,
        windows_adapter_artifact_ingestion as artifact_ingestion,
    },
    policy_constants as policy, WindowsAdapterArtifactGateDecision, WindowsAdapterArtifactKind,
    WindowsAdapterCapabilitySurface,
};

use crate::windows_adapter_artifact_ingestion_read_model::{
    app_ingestion_records, domain_ingestion_records, evaluate_windows_adapter_artifact_ingestion,
    managed_browser_ingestion_records,
};

#[test]
fn windows_adapter_artifact_ingestion_refuses_empty_records() {
    let proof = evaluate_windows_adapter_artifact_ingestion(policy::TEST_EVALUATED_AT, &[]);
    let app = gate_entry_for(&proof, WindowsAdapterCapabilitySurface::AppTarget);

    assert_eq!(proof.read_model_id, artifact_ingestion::READ_MODEL_ID_V0_8);
    assert!(proof.accepted_records.is_empty());
    assert!(proof.rejected_records.is_empty());
    assert_eq!(
        app.decision,
        WindowsAdapterArtifactGateDecision::RefusedMissingArtifacts
    );
    assert!(!app.claim_upgrade_allowed);
}

#[test]
fn windows_adapter_artifact_ingestion_feeds_custodied_app_records_to_gate() {
    let proof = evaluate_windows_adapter_artifact_ingestion(
        policy::TEST_EVALUATED_AT,
        &app_ingestion_records(),
    );
    let app = gate_entry_for(&proof, WindowsAdapterCapabilitySurface::AppTarget);

    assert_eq!(proof.accepted_records.len(), 4);
    assert!(proof.rejected_records.is_empty());
    assert_eq!(
        app.present_artifact_ids,
        vec![
            artifact_gate::TEST_ARTIFACT_APP_IDENTITY.to_string(),
            artifact_gate::TEST_ARTIFACT_APP_APPLY.to_string(),
            artifact_gate::TEST_ARTIFACT_APP_ROLLBACK.to_string(),
            artifact_gate::TEST_ARTIFACT_AUDIT.to_string()
        ]
    );
    assert_eq!(
        app.decision,
        WindowsAdapterArtifactGateDecision::ReadyForManualReview
    );
    assert!(app.ready_for_manual_review);
    assert!(!app.claim_upgrade_allowed);
}

#[test]
fn windows_adapter_artifact_ingestion_rejects_mismatched_subjects() {
    let mut records = app_ingestion_records();
    records[0].artifact_subject_ref = artifact_ingestion::TEST_MISMATCHED_SUBJECT_REF.to_string();

    let proof = evaluate_windows_adapter_artifact_ingestion(policy::TEST_EVALUATED_AT, &records);
    let app = gate_entry_for(&proof, WindowsAdapterCapabilitySurface::AppTarget);

    assert_eq!(proof.accepted_records.len(), 3);
    assert_eq!(proof.rejected_records.len(), 1);
    assert_eq!(
        proof.rejected_records[0].refusal_reasons,
        vec![artifact_ingestion::REFUSAL_SUBJECT_MISMATCH.to_string()]
    );
    assert_eq!(
        app.missing_artifact_kinds,
        vec![WindowsAdapterArtifactKind::SameIdentityAppPackageEvidence]
    );
    assert_eq!(
        app.decision,
        WindowsAdapterArtifactGateDecision::RefusedMissingArtifacts
    );
}

#[test]
fn windows_adapter_artifact_ingestion_rejects_uncustodied_records() {
    let mut records = app_ingestion_records();
    records[1].custody_event_id = None;

    let proof = evaluate_windows_adapter_artifact_ingestion(policy::TEST_EVALUATED_AT, &records);
    let app = gate_entry_for(&proof, WindowsAdapterCapabilitySurface::AppTarget);

    assert_eq!(proof.accepted_records.len(), 3);
    assert_eq!(proof.rejected_records.len(), 1);
    assert_eq!(
        proof.rejected_records[0].refusal_reasons,
        vec![artifact_ingestion::REFUSAL_MISSING_CUSTODY_EVENT.to_string()]
    );
    assert_eq!(
        app.missing_artifact_kinds,
        vec![WindowsAdapterArtifactKind::AdapterApplyResult]
    );
    assert!(!app.claim_upgrade_allowed);
}

#[test]
fn windows_adapter_artifact_ingestion_rejects_wrong_surface_kind() {
    let mut records = app_ingestion_records();
    records[0].artifact_kind = WindowsAdapterArtifactKind::ManagedBrowserExactUrlEvidence;

    let proof = evaluate_windows_adapter_artifact_ingestion(policy::TEST_EVALUATED_AT, &records);

    assert_eq!(proof.accepted_records.len(), 3);
    assert_eq!(
        proof.rejected_records[0].refusal_reasons,
        vec![artifact_ingestion::REFUSAL_KIND_SURFACE_MISMATCH.to_string()]
    );
}

#[test]
fn windows_adapter_artifact_ingestion_builds_domain_and_managed_browser_gate_inputs() {
    let mut records = domain_ingestion_records();
    records.extend(managed_browser_ingestion_records());

    let proof = evaluate_windows_adapter_artifact_ingestion(policy::TEST_EVALUATED_AT, &records);
    let domain = gate_entry_for(&proof, WindowsAdapterCapabilitySurface::DomainNetworkTarget);
    let managed = gate_entry_for(
        &proof,
        WindowsAdapterCapabilitySurface::ManagedBrowserTarget,
    );

    assert_eq!(proof.accepted_records.len(), 5);
    assert!(proof.rejected_records.is_empty());
    assert_eq!(
        domain.decision,
        WindowsAdapterArtifactGateDecision::ReadyForManualReview
    );
    assert_eq!(
        managed.decision,
        WindowsAdapterArtifactGateDecision::ReadyForManualReview
    );
    assert!(!domain.claim_upgrade_allowed);
    assert!(!managed.claim_upgrade_allowed);
}

fn gate_entry_for(
    proof: &ocentra_parent_agent_protocol::WindowsAdapterArtifactIngestionProof,
    surface: WindowsAdapterCapabilitySurface,
) -> &ocentra_parent_agent_protocol::WindowsAdapterArtifactGateEntry {
    proof
        .gate_proof
        .entries
        .iter()
        .find(|entry| entry.surface == surface)
        .expect(artifact_ingestion::READ_MODEL_ID_V0_8)
}
