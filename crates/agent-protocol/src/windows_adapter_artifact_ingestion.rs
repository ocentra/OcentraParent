use serde::{Deserialize, Serialize};

use crate::{
    windows_adapter_artifact_gate::{
        WindowsAdapterArtifactEvidence, WindowsAdapterArtifactGateProof, WindowsAdapterArtifactKind,
    },
    WindowsAdapterCapabilitySurface,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsAdapterArtifactIngestionRecord {
    pub schema_version: String,
    pub ingestion_record_id: String,
    pub artifact_id: String,
    pub artifact_kind: WindowsAdapterArtifactKind,
    pub surface: WindowsAdapterCapabilitySurface,
    pub target_subject_ref: String,
    pub artifact_subject_ref: String,
    pub custody_event_id: Option<String>,
    pub collected_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsAdapterArtifactIngestionAcceptedRecord {
    pub schema_version: String,
    pub ingestion_record_id: String,
    pub evidence: WindowsAdapterArtifactEvidence,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsAdapterArtifactIngestionRejection {
    pub schema_version: String,
    pub ingestion_record_id: String,
    pub artifact_id: String,
    pub artifact_kind: WindowsAdapterArtifactKind,
    pub surface: WindowsAdapterCapabilitySurface,
    pub refusal_reasons: Vec<String>,
    pub rejected_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsAdapterArtifactIngestionProof {
    pub schema_version: String,
    pub read_model_id: String,
    pub generated_at: String,
    pub accepted_records: Vec<WindowsAdapterArtifactIngestionAcceptedRecord>,
    pub rejected_records: Vec<WindowsAdapterArtifactIngestionRejection>,
    pub gate_proof: WindowsAdapterArtifactGateProof,
    pub product_claim_boundary: String,
}
