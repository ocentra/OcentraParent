use crate::windows_adapter_artifact_gate::{
    WindowsAdapterArtifactEvidence, WindowsAdapterArtifactGateDecision,
    WindowsAdapterArtifactGateEntry, WindowsAdapterArtifactGateProof, WindowsAdapterArtifactKind,
};
use ocentra_eventing::expect_value::ExpectValue;

use super::{
    constants, constants::windows_adapter_artifact_gate as artifact_gate,
    constants::windows_adapter_artifact_ingestion as artifact_ingestion,
    constants::windows_adapter_capability as windows_adapter, policy_constants as policy,
    WindowsAdapterCapabilitySurface,
};

#[test]
fn windows_adapter_artifact_gate_serializes_refusal_boundaries() {
    let proof = WindowsAdapterArtifactGateProof {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: artifact_gate::READ_MODEL_ID_V0_8.to_string(),
        generated_at: policy::TEST_EVALUATED_AT.to_string(),
        capability_read_model_id: windows_adapter::READ_MODEL_ID_V0_8.to_string(),
        entries: vec![app_gate_entry()],
    };

    let serialized =
        serde_json::to_value(proof).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized[constants::field::READ_MODEL_ID],
        artifact_gate::READ_MODEL_ID_V0_8
    );
    assert_eq!(
        serialized[constants::field::ENTRIES][0]["requiredArtifactKinds"][0],
        artifact_gate::ARTIFACT_KIND_SAME_IDENTITY_APP
    );
    assert_eq!(
        serialized[constants::field::ENTRIES][0]["decision"],
        artifact_gate::DECISION_REFUSED_MISSING_ARTIFACTS
    );
    assert_eq!(
        serialized[constants::field::ENTRIES][0]["claimUpgradeAllowed"],
        false
    );
}

#[test]
fn unsupported_windows_adapter_artifact_gate_decision_does_not_deserialize() {
    let payload = serde_json::json!({
        "schemaVersion": policy::CONTRACT_SCHEMA_VERSION_V0_6,
        "gateEntryId": artifact_gate::ENTRY_ID_APP_TARGET,
        "capabilityEntryId": windows_adapter::ENTRY_ID_APP_TARGET,
        "surface": windows_adapter::SURFACE_APP_TARGET,
        "requiredArtifactKinds": [artifact_gate::ARTIFACT_KIND_SAME_IDENTITY_APP],
        "presentArtifactIds": [],
        "missingArtifactKinds": [artifact_gate::ARTIFACT_KIND_SAME_IDENTITY_APP],
        "refusalReasons": [artifact_gate::REFUSAL_MISSING_APP_ARTIFACTS],
        "decision": "product-ready",
        "readyForManualReview": false,
        "claimUpgradeAllowed": false,
        "productClaimBoundary": artifact_gate::CLAIM_BOUNDARY_APP_TARGET,
        "lastCheckedAt": policy::TEST_EVALUATED_AT
    });

    let parsed = serde_json::from_value::<WindowsAdapterArtifactGateEntry>(payload);

    assert_eq!(
        parsed.err().map(|error| error.classify()),
        Some(serde_json::error::Category::Data)
    );
}

#[test]
fn windows_adapter_artifact_evidence_serializes_custody_event_refs() {
    let evidence = WindowsAdapterArtifactEvidence {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        artifact_id: artifact_gate::TEST_ARTIFACT_AUDIT.to_string(),
        artifact_kind: WindowsAdapterArtifactKind::AuditCustodyEvent,
        surface: WindowsAdapterCapabilitySurface::AppTarget,
        subject_ref: artifact_gate::TEST_SUBJECT_REF.to_string(),
        custody_event_id: Some(artifact_gate::TEST_CUSTODY_EVENT_ID.to_string()),
        verified_at: artifact_ingestion::TEST_INGESTED_AT.to_string(),
    };

    let serialized =
        serde_json::to_value(evidence).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["artifactKind"],
        artifact_gate::ARTIFACT_KIND_AUDIT_CUSTODY_EVENT
    );
    assert_eq!(
        serialized["custodyEventId"],
        artifact_gate::TEST_CUSTODY_EVENT_ID
    );
}

fn app_gate_entry() -> WindowsAdapterArtifactGateEntry {
    WindowsAdapterArtifactGateEntry {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        gate_entry_id: artifact_gate::ENTRY_ID_APP_TARGET.to_string(),
        capability_entry_id: windows_adapter::ENTRY_ID_APP_TARGET.to_string(),
        surface: WindowsAdapterCapabilitySurface::AppTarget,
        required_artifact_kinds: vec![WindowsAdapterArtifactKind::SameIdentityAppPackageEvidence],
        present_artifact_ids: Vec::new(),
        missing_artifact_kinds: vec![WindowsAdapterArtifactKind::SameIdentityAppPackageEvidence],
        refusal_reasons: vec![artifact_gate::REFUSAL_MISSING_APP_ARTIFACTS.to_string()],
        decision: WindowsAdapterArtifactGateDecision::RefusedMissingArtifacts,
        ready_for_manual_review: false,
        claim_upgrade_allowed: false,
        product_claim_boundary: artifact_gate::CLAIM_BOUNDARY_APP_TARGET.to_string(),
        last_checked_at: policy::TEST_EVALUATED_AT.to_string(),
    }
}
