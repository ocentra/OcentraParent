use super::windows_adapter_artifact_gate::{
    WindowsAdapterArtifactEvidence, WindowsAdapterArtifactGateProof, WindowsAdapterArtifactKind,
};
use super::windows_adapter_artifact_ingestion::{
    WindowsAdapterArtifactIngestionAcceptedRecord, WindowsAdapterArtifactIngestionProof,
    WindowsAdapterArtifactIngestionRecord, WindowsAdapterArtifactIngestionRejection,
};
use super::{
    constants, constants::windows_adapter_artifact_gate as artifact_gate,
    constants::windows_adapter_artifact_ingestion as artifact_ingestion,
    policy_constants as policy, WindowsAdapterCapabilitySurface,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn windows_adapter_artifact_ingestion_record_serializes_custody_and_subjects() {
    let record = ingestion_record();

    let serialized =
        serde_json::to_value(record).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["ingestionRecordId"],
        artifact_ingestion::RECORD_ID_APP_IDENTITY
    );
    assert_eq!(
        serialized["artifactKind"],
        artifact_gate::ARTIFACT_KIND_SAME_IDENTITY_APP
    );
    assert_eq!(
        serialized["targetSubjectRef"],
        artifact_gate::TEST_SUBJECT_REF
    );
    assert_eq!(
        serialized["custodyEventId"],
        artifact_gate::TEST_CUSTODY_EVENT_ID
    );
}

#[test]
fn windows_adapter_artifact_ingestion_proof_serializes_gate_boundary() {
    let proof = WindowsAdapterArtifactIngestionProof {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: artifact_ingestion::READ_MODEL_ID_V0_8.to_string(),
        generated_at: policy::TEST_EVALUATED_AT.to_string(),
        accepted_records: vec![accepted_record()],
        rejected_records: vec![rejected_record()],
        gate_proof: WindowsAdapterArtifactGateProof {
            schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            read_model_id: artifact_gate::READ_MODEL_ID_V0_8.to_string(),
            generated_at: policy::TEST_EVALUATED_AT.to_string(),
            capability_read_model_id: String::new(),
            entries: Vec::new(),
        },
        product_claim_boundary: artifact_ingestion::CLAIM_BOUNDARY.to_string(),
    };

    let serialized =
        serde_json::to_value(proof).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized[constants::field::READ_MODEL_ID],
        artifact_ingestion::READ_MODEL_ID_V0_8
    );
    assert_eq!(
        serialized["acceptedRecords"][0]["evidence"]["artifactId"],
        artifact_gate::TEST_ARTIFACT_APP_IDENTITY
    );
    assert_eq!(
        serialized["rejectedRecords"][0]["refusalReasons"][0],
        artifact_ingestion::REFUSAL_SUBJECT_MISMATCH
    );
    assert_eq!(
        serialized["gateProof"]["readModelId"],
        artifact_gate::READ_MODEL_ID_V0_8
    );
}

fn ingestion_record() -> WindowsAdapterArtifactIngestionRecord {
    WindowsAdapterArtifactIngestionRecord {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        ingestion_record_id: artifact_ingestion::RECORD_ID_APP_IDENTITY.to_string(),
        artifact_id: artifact_gate::TEST_ARTIFACT_APP_IDENTITY.to_string(),
        artifact_kind: WindowsAdapterArtifactKind::SameIdentityAppPackageEvidence,
        surface: WindowsAdapterCapabilitySurface::AppTarget,
        target_subject_ref: artifact_gate::TEST_SUBJECT_REF.to_string(),
        artifact_subject_ref: artifact_gate::TEST_SUBJECT_REF.to_string(),
        custody_event_id: Some(artifact_gate::TEST_CUSTODY_EVENT_ID.to_string()),
        collected_at: artifact_ingestion::TEST_INGESTED_AT.to_string(),
    }
}

fn accepted_record() -> WindowsAdapterArtifactIngestionAcceptedRecord {
    WindowsAdapterArtifactIngestionAcceptedRecord {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        ingestion_record_id: artifact_ingestion::RECORD_ID_APP_IDENTITY.to_string(),
        evidence: WindowsAdapterArtifactEvidence {
            schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            artifact_id: artifact_gate::TEST_ARTIFACT_APP_IDENTITY.to_string(),
            artifact_kind: WindowsAdapterArtifactKind::SameIdentityAppPackageEvidence,
            surface: WindowsAdapterCapabilitySurface::AppTarget,
            subject_ref: artifact_gate::TEST_SUBJECT_REF.to_string(),
            custody_event_id: Some(artifact_gate::TEST_CUSTODY_EVENT_ID.to_string()),
            verified_at: policy::TEST_EVALUATED_AT.to_string(),
        },
    }
}

fn rejected_record() -> WindowsAdapterArtifactIngestionRejection {
    WindowsAdapterArtifactIngestionRejection {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        ingestion_record_id: artifact_ingestion::RECORD_ID_APP_IDENTITY.to_string(),
        artifact_id: artifact_gate::TEST_ARTIFACT_APP_IDENTITY.to_string(),
        artifact_kind: WindowsAdapterArtifactKind::SameIdentityAppPackageEvidence,
        surface: WindowsAdapterCapabilitySurface::AppTarget,
        refusal_reasons: vec![artifact_ingestion::REFUSAL_SUBJECT_MISMATCH.to_string()],
        rejected_at: policy::TEST_EVALUATED_AT.to_string(),
    }
}
