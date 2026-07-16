use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    tracking_parent_defined_place_id_from_evidence_ref, TrackingEvidenceRef,
    TrackingParentDefinedPlaceId, TrackingParentDefinedPlaceState,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingParentDefinedPlaceInput {
    pub source_evidence_ref: TrackingEvidenceRef,
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
    let parsed_place_state = TrackingParentDefinedPlaceState::parse(place_state)
        .expect_value("tracking parent-defined place contract drift");

    TrackingParentDefinedPlaceDecision {
        place_id: tracking_parent_defined_place_id_from_evidence_ref(&input.source_evidence_ref),
        place_state: parsed_place_state,
        evidence_refs: input.evidence_refs,
    }
}
