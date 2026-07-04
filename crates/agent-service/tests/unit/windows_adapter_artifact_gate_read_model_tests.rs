use std::{error::Error, io::Error as IoError};

use ocentra_parent_agent_protocol::constants::windows_adapter_artifact_gate as artifact_gate;
use ocentra_parent_agent_protocol::constants::windows_adapter_capability as windows_adapter;
use ocentra_parent_agent_protocol::policy_constants as policy;
use ocentra_parent_agent_protocol::windows_adapter_artifact_gate::{
    WindowsAdapterArtifactEvidence, WindowsAdapterArtifactGateDecision,
    WindowsAdapterArtifactGateEntry, WindowsAdapterArtifactGateProof, WindowsAdapterArtifactKind,
};
use ocentra_parent_agent_protocol::windows_adapter_capability::WindowsAdapterCapabilitySurface;

use super::test_text::TestText;
use crate::windows_adapter_artifact_gate_read_model::{
    evaluate_windows_adapter_artifact_gate, windows_adapter_artifact_gate_proof,
};

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn windows_adapter_artifact_gate_refuses_missing_app_artifacts() -> TestResult {
    let proof = windows_adapter_artifact_gate_proof(policy::TEST_EVALUATED_AT);
    let app = entry_for(&proof, WindowsAdapterCapabilitySurface::AppTarget)?;

    assert_eq!(proof.read_model_id, artifact_gate::READ_MODEL_ID_V0_8);
    assert_eq!(
        proof.capability_read_model_id,
        windows_adapter::READ_MODEL_ID_V0_8
    );
    assert_eq!(proof.entries.len(), 6);
    assert_eq!(
        app.capability_entry_id,
        windows_adapter::ENTRY_ID_APP_TARGET
    );
    assert_eq!(
        app.decision,
        WindowsAdapterArtifactGateDecision::RefusedMissingArtifacts
    );
    assert!(!app.ready_for_manual_review);
    assert!(!app.claim_upgrade_allowed);
    assert_eq!(
        app.missing_artifact_kinds,
        vec![
            WindowsAdapterArtifactKind::SameIdentityAppPackageEvidence,
            WindowsAdapterArtifactKind::AdapterApplyResult,
            WindowsAdapterArtifactKind::AdapterRollbackResult,
            WindowsAdapterArtifactKind::AuditCustodyEvent
        ]
    );
    assert_eq!(
        app.refusal_reasons,
        vec![artifact_gate::REFUSAL_MISSING_APP_ARTIFACTS.to_string()]
    );

    Ok(())
}

#[test]
fn windows_adapter_artifact_gate_allows_manual_review_only_after_app_artifact_refs() -> TestResult {
    let proof = evaluate_windows_adapter_artifact_gate(
        policy::TEST_EVALUATED_AT,
        &complete_app_artifacts(),
    );
    let app = entry_for(&proof, WindowsAdapterCapabilitySurface::AppTarget)?;

    assert_eq!(
        app.present_artifact_ids,
        vec![
            artifact_gate::TEST_ARTIFACT_APP_IDENTITY.to_string(),
            artifact_gate::TEST_ARTIFACT_APP_APPLY.to_string(),
            artifact_gate::TEST_ARTIFACT_APP_ROLLBACK.to_string(),
            artifact_gate::TEST_ARTIFACT_AUDIT.to_string()
        ]
    );
    assert!(app.missing_artifact_kinds.is_empty());
    assert_eq!(
        app.decision,
        WindowsAdapterArtifactGateDecision::ReadyForManualReview
    );
    assert!(app.ready_for_manual_review);
    assert!(!app.claim_upgrade_allowed);
    assert!(app.refusal_reasons.is_empty());

    Ok(())
}

#[test]
fn windows_adapter_artifact_gate_keeps_browser_and_unsupported_claims_blocked() -> TestResult {
    let proof = windows_adapter_artifact_gate_proof(policy::TEST_EVALUATED_AT);
    let managed = entry_for(
        &proof,
        WindowsAdapterCapabilitySurface::ManagedBrowserTarget,
    )?;
    let unmanaged = entry_for(
        &proof,
        WindowsAdapterCapabilitySurface::UnmanagedBrowserTarget,
    )?;
    let unsupported = entry_for(&proof, WindowsAdapterCapabilitySurface::UnsupportedOsTarget)?;

    assert_eq!(
        managed.missing_artifact_kinds,
        vec![
            WindowsAdapterArtifactKind::ManagedBrowserExactUrlEvidence,
            WindowsAdapterArtifactKind::AuditCustodyEvent
        ]
    );
    assert_eq!(
        unmanaged.decision,
        WindowsAdapterArtifactGateDecision::RefusedUnsupportedSurface
    );
    assert_eq!(
        unmanaged.refusal_reasons,
        vec![artifact_gate::REFUSAL_UNMANAGED_BROWSER_PROCESS_ONLY.to_string()]
    );
    assert_eq!(
        unsupported.decision,
        WindowsAdapterArtifactGateDecision::RefusedUnsupportedSurface
    );
    assert!(proof
        .entries
        .iter()
        .all(|entry| !entry.claim_upgrade_allowed));

    Ok(())
}

#[test]
fn windows_adapter_artifact_gate_rejects_audit_artifacts_without_custody_event_ids() -> TestResult {
    let mut artifacts = complete_app_artifacts();
    let audit = artifacts
        .iter_mut()
        .find(|artifact| artifact.artifact_kind == WindowsAdapterArtifactKind::AuditCustodyEvent)
        .ok_or_else(|| IoError::other(artifact_gate::READ_MODEL_ID_V0_8))?;
    audit.custody_event_id = None;

    let proof = evaluate_windows_adapter_artifact_gate(policy::TEST_EVALUATED_AT, &artifacts);
    let app = entry_for(&proof, WindowsAdapterCapabilitySurface::AppTarget)?;

    assert_eq!(
        app.decision,
        WindowsAdapterArtifactGateDecision::RefusedMissingArtifacts
    );
    assert_eq!(
        app.missing_artifact_kinds,
        vec![WindowsAdapterArtifactKind::AuditCustodyEvent]
    );
    assert!(!app.claim_upgrade_allowed);

    Ok(())
}

#[test]
fn windows_adapter_artifact_gate_rejects_non_audit_artifacts_without_custody_event_ids(
) -> TestResult {
    let mut artifacts = complete_app_artifacts();
    let apply = artifacts
        .iter_mut()
        .find(|artifact| artifact.artifact_kind == WindowsAdapterArtifactKind::AdapterApplyResult)
        .ok_or_else(|| IoError::other(artifact_gate::READ_MODEL_ID_V0_8))?;
    apply.custody_event_id = None;

    let proof = evaluate_windows_adapter_artifact_gate(policy::TEST_EVALUATED_AT, &artifacts);
    let app = entry_for(&proof, WindowsAdapterCapabilitySurface::AppTarget)?;

    assert_eq!(
        app.decision,
        WindowsAdapterArtifactGateDecision::RefusedMissingArtifacts
    );
    assert_eq!(
        app.missing_artifact_kinds,
        vec![WindowsAdapterArtifactKind::AdapterApplyResult]
    );
    assert!(!app.claim_upgrade_allowed);

    Ok(())
}

fn entry_for(
    proof: &WindowsAdapterArtifactGateProof,
    surface: WindowsAdapterCapabilitySurface,
) -> Result<&WindowsAdapterArtifactGateEntry, IoError> {
    proof
        .entries
        .iter()
        .find(|entry| entry.surface == surface)
        .ok_or_else(|| IoError::other(artifact_gate::READ_MODEL_ID_V0_8))
}

fn complete_app_artifacts() -> Vec<WindowsAdapterArtifactEvidence> {
    vec![
        evidence(
            artifact_gate::TEST_ARTIFACT_APP_IDENTITY,
            WindowsAdapterArtifactKind::SameIdentityAppPackageEvidence,
            Some(TestText::from_display(artifact_gate::TEST_CUSTODY_EVENT_ID)),
        ),
        evidence(
            artifact_gate::TEST_ARTIFACT_APP_APPLY,
            WindowsAdapterArtifactKind::AdapterApplyResult,
            Some(TestText::from_display(artifact_gate::TEST_CUSTODY_EVENT_ID)),
        ),
        evidence(
            artifact_gate::TEST_ARTIFACT_APP_ROLLBACK,
            WindowsAdapterArtifactKind::AdapterRollbackResult,
            Some(TestText::from_display(artifact_gate::TEST_CUSTODY_EVENT_ID)),
        ),
        evidence(
            artifact_gate::TEST_ARTIFACT_AUDIT,
            WindowsAdapterArtifactKind::AuditCustodyEvent,
            Some(TestText::from_display(artifact_gate::TEST_CUSTODY_EVENT_ID)),
        ),
    ]
}

fn evidence(
    artifact_id: impl std::fmt::Display,
    artifact_kind: WindowsAdapterArtifactKind,
    custody_event_id: Option<TestText>,
) -> WindowsAdapterArtifactEvidence {
    WindowsAdapterArtifactEvidence {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        artifact_id: artifact_id.to_string(),
        artifact_kind,
        surface: WindowsAdapterCapabilitySurface::AppTarget,
        subject_ref: artifact_gate::TEST_SUBJECT_REF.to_string(),
        custody_event_id: custody_event_id.map(|value| value.to_string()),
        verified_at: policy::TEST_EVALUATED_AT.to_string(),
    }
}
