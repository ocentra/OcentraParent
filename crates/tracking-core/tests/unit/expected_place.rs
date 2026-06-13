use ocentra_parent_agent_protocol::{constants, TrackingParentActionRequirement};

#[test]
fn expected_place_evaluation_marks_uncertain_location_without_parent_action() {
    let observed = ocentra_tracking_core::default_location_observed_event();
    let evidence = ocentra_tracking_core::record_tracking_evidence_from_location(&observed);

    let evaluation = ocentra_tracking_core::evaluate_expected_place_state(&evidence);

    assert_eq!(
        evaluation.expected_place_state,
        constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN
    );
    assert_eq!(
        evaluation.parent_action_requirement,
        TrackingParentActionRequirement::NotRequired
    );
    assert_eq!(evaluation.evidence_refs, vec![evidence.evidence_ref]);
}
