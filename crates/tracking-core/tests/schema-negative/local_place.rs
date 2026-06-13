use ocentra_parent_agent_protocol::{
    constants, TrackingEvidenceRef, TrackingParentDefinedPlaceState,
};

#[test]
fn parent_defined_place_rejects_invalid_zero_radius() {
    let decision = ocentra_tracking_core::evaluate_parent_defined_place(
        ocentra_tracking_core::TrackingParentDefinedPlaceInput {
            radius_meters: 0,
            evidence_refs: vec![
                TrackingEvidenceRef::parse(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)
                    .expect(constants::tracking_runtime::DEFAULT_EVIDENCE_REF),
            ],
        },
    );

    assert_eq!(
        decision.place_state,
        TrackingParentDefinedPlaceState::parse(
            constants::tracking_runtime::PARENT_DEFINED_PLACE_STATE_REJECTED_INVALID_RADIUS,
        )
        .expect(constants::tracking_runtime::PARENT_DEFINED_PLACE_STATE_REJECTED_INVALID_RADIUS)
    );
    assert_eq!(
        decision.evidence_refs,
        vec![
            TrackingEvidenceRef::parse(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)
                .expect(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)
        ]
    );
}
