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
