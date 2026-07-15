use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    tracking_parent_defined_place_id_from_evidence_ref, TrackingEvidenceRef,
    TrackingParentDefinedPlaceState,
};

#[test]
fn parent_defined_place_rejects_invalid_zero_radius() {
    let evidence_ref =
        TrackingEvidenceRef::parse(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)
            .expect_value(constants::tracking_runtime::DEFAULT_EVIDENCE_REF);
    let decision = ocentra_tracking_core::local_place::evaluate_parent_defined_place(
        ocentra_tracking_core::local_place::TrackingParentDefinedPlaceInput {
            source_evidence_ref: evidence_ref.clone(),
            radius_meters: 0,
            evidence_refs: vec![evidence_ref.clone()],
        },
    );

    assert_eq!(
        decision.place_state,
        TrackingParentDefinedPlaceState::parse(
            constants::tracking_runtime::PARENT_DEFINED_PLACE_STATE_REJECTED_INVALID_RADIUS,
        )
        .expect_value(
            constants::tracking_runtime::PARENT_DEFINED_PLACE_STATE_REJECTED_INVALID_RADIUS
        )
    );
    assert_eq!(decision.evidence_refs, vec![evidence_ref.clone()]);
    assert_eq!(
        decision.place_id,
        tracking_parent_defined_place_id_from_evidence_ref(&evidence_ref)
    );
}
