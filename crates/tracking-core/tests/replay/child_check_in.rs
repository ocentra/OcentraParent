use ocentra_tracking_core::child_check_in::record_child_check_in;

#[test]
fn child_check_in_is_replay_safe_for_same_observation() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();

    let first = record_child_check_in(&observed);
    let second = record_child_check_in(&observed);

    assert_eq!(first.check_in_id, second.check_in_id);
    assert_eq!(first.source_observation_id, second.source_observation_id);
    assert_eq!(first.checked_in_at, observed.observed_at);
    assert_eq!(second.checked_in_at, observed.observed_at);
    assert_eq!(first.evidence_refs, second.evidence_refs);
}
