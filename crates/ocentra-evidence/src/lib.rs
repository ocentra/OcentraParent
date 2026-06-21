#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

pub const CRATE_NAME: &str = "ocentra-evidence";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceCustodyScope {
    #[serde(rename = "local-only")]
    LocalOnly,
    #[serde(rename = "family-shared")]
    FamilyShared,
    #[serde(rename = "exportable")]
    Exportable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceReferenceState {
    #[serde(rename = "stable")]
    Stable,
    #[serde(rename = "missing")]
    Missing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrivatePayloadState {
    #[serde(rename = "excluded")]
    Excluded,
    #[serde(rename = "included")]
    Included,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RetentionState {
    #[serde(rename = "known")]
    Known,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeBoundaryState {
    #[serde(rename = "may-cross")]
    MayCross,
    #[serde(rename = "must-remain-local")]
    MustRemainLocal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManualReviewState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvidenceReferenceInput {
    pub custody_scope: EvidenceCustodyScope,
    pub reference_state: EvidenceReferenceState,
    pub private_payload_state: PrivatePayloadState,
    pub retention_state: RetentionState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvidenceReferenceDecision {
    pub reference_state: EvidenceReferenceState,
    pub runtime_boundary_state: RuntimeBoundaryState,
    pub manual_review_state: ManualReviewState,
}

pub fn evaluate_evidence_reference(input: EvidenceReferenceInput) -> EvidenceReferenceDecision {
    let accepted = input.reference_state == EvidenceReferenceState::Stable
        && input.private_payload_state == PrivatePayloadState::Excluded
        && input.retention_state == RetentionState::Known;
    let runtime_boundary_state =
        if accepted && input.custody_scope != EvidenceCustodyScope::LocalOnly {
            RuntimeBoundaryState::MayCross
        } else {
            RuntimeBoundaryState::MustRemainLocal
        };
    let manual_review_state = if accepted {
        ManualReviewState::NotRequired
    } else {
        ManualReviewState::Required
    };

    EvidenceReferenceDecision {
        reference_state: input.reference_state,
        runtime_boundary_state,
        manual_review_state,
    }
}
