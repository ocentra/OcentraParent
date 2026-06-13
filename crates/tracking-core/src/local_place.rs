use ocentra_parent_agent_protocol::{
    constants, TrackingEvidenceRef, TrackingParentDefinedPlaceId, TrackingParentDefinedPlaceState,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingParentDefinedPlaceInput {
    pub radius_meters: u16,
    pub evidence_refs: Vec<TrackingEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingParentDefinedPlaceDecision {
    pub place_id: TrackingParentDefinedPlaceId,
    pub place_state: TrackingParentDefinedPlaceState,
    pub evidence_refs: Vec<TrackingEvidenceRef>,
}

pub fn evaluate_parent_defined_place(
    input: TrackingParentDefinedPlaceInput,
) -> TrackingParentDefinedPlaceDecision {
    let place_state = if input.radius_meters == 0 {
        constants::tracking_runtime::PARENT_DEFINED_PLACE_STATE_REJECTED_INVALID_RADIUS
    } else {
        constants::tracking_runtime::PARENT_DEFINED_PLACE_STATE_ACCEPTED
    };

    TrackingParentDefinedPlaceDecision {
        place_id: TrackingParentDefinedPlaceId::parse(
            constants::tracking_runtime::DEFAULT_PARENT_DEFINED_PLACE_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_PARENT_DEFINED_PLACE_ID),
        place_state: TrackingParentDefinedPlaceState::parse(place_state)
            .expect(constants::tracking_runtime::PARENT_DEFINED_PLACE_STATE_ACCEPTED),
        evidence_refs: input.evidence_refs,
    }
}
