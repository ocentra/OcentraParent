use ocentra_evidence::{
    evaluate_evidence_reference, EvidenceCustodyScope, EvidenceReferenceInput,
    EvidenceReferenceState, ManualReviewState, PrivatePayloadState, RetentionState,
    RuntimeBoundaryState,
};

#[test]
fn family_shared_evidence_ref_can_cross_runtime_boundary_without_raw_payload() {
    let decision = evaluate_evidence_reference(EvidenceReferenceInput {
        custody_scope: EvidenceCustodyScope::FamilyShared,
        reference_state: EvidenceReferenceState::Stable,
        private_payload_state: PrivatePayloadState::Excluded,
        retention_state: RetentionState::Known,
    });

    assert_eq!(decision.reference_state, EvidenceReferenceState::Stable);
    assert_eq!(
        decision.runtime_boundary_state,
        RuntimeBoundaryState::MayCross
    );
    assert_eq!(decision.manual_review_state, ManualReviewState::NotRequired);
}

#[test]
fn raw_private_payload_rejected_even_when_reference_exists() {
    let decision = evaluate_evidence_reference(EvidenceReferenceInput {
        custody_scope: EvidenceCustodyScope::Exportable,
        reference_state: EvidenceReferenceState::Stable,
        private_payload_state: PrivatePayloadState::Included,
        retention_state: RetentionState::Known,
    });

    assert_eq!(decision.reference_state, EvidenceReferenceState::Stable);
    assert_eq!(
        decision.runtime_boundary_state,
        RuntimeBoundaryState::MustRemainLocal
    );
    assert_eq!(decision.manual_review_state, ManualReviewState::Required);
}

#[test]
fn local_only_evidence_ref_stays_inside_child_runtime_boundary() {
    let decision = evaluate_evidence_reference(EvidenceReferenceInput {
        custody_scope: EvidenceCustodyScope::LocalOnly,
        reference_state: EvidenceReferenceState::Stable,
        private_payload_state: PrivatePayloadState::Excluded,
        retention_state: RetentionState::Known,
    });

    assert_eq!(decision.reference_state, EvidenceReferenceState::Stable);
    assert_eq!(
        decision.runtime_boundary_state,
        RuntimeBoundaryState::MustRemainLocal
    );
}

#[test]
fn custody_scope_serializes_to_canonical_schema_literals() {
    let local_only = serde_json::to_string(&EvidenceCustodyScope::LocalOnly)
        .expect("serialize local-only scope");
    let family_shared = serde_json::to_string(&EvidenceCustodyScope::FamilyShared)
        .expect("serialize family-shared scope");
    let exportable = serde_json::to_string(&EvidenceCustodyScope::Exportable)
        .expect("serialize exportable scope");

    assert_eq!(local_only, "\"local-only\"");
    assert_eq!(family_shared, "\"family-shared\"");
    assert_eq!(exportable, "\"exportable\"");
}

#[test]
fn custody_scope_deserializes_from_canonical_schema_literals() {
    let local_only: EvidenceCustodyScope =
        serde_json::from_str("\"local-only\"").expect("deserialize local-only scope");
    let family_shared: EvidenceCustodyScope =
        serde_json::from_str("\"family-shared\"").expect("deserialize family-shared scope");
    let exportable: EvidenceCustodyScope =
        serde_json::from_str("\"exportable\"").expect("deserialize exportable scope");

    assert_eq!(local_only, EvidenceCustodyScope::LocalOnly);
    assert_eq!(family_shared, EvidenceCustodyScope::FamilyShared);
    assert_eq!(exportable, EvidenceCustodyScope::Exportable);
}
