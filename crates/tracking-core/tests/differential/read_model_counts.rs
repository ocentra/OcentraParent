use ocentra_parent_agent_protocol::constants::tracking_runtime::{
    READ_MODEL_DIFFERENTIAL_CONSISTENT, READ_MODEL_DIFFERENTIAL_REJECTED_COUNT_DRIFT,
    READ_MODEL_DIFFERENTIAL_REJECTED_TOMBSTONE_DRIFT,
};
use ocentra_tracking_core::read_model_guard::{
    evaluate_tracking_read_model_differential, TrackingReadModelDifferentialState,
};

#[test]
fn tracking_read_model_counts_accept_active_and_tombstone_total() {
    let decision = evaluate_tracking_read_model_differential(5, 3, 2, 2);

    assert_eq!(
        decision.differential_state,
        TrackingReadModelDifferentialState::Accepted
    );
    assert_eq!(decision.decision_state, READ_MODEL_DIFFERENTIAL_CONSISTENT);
}

#[test]
fn tracking_read_model_counts_reject_returned_count_drift() {
    let decision = evaluate_tracking_read_model_differential(4, 3, 2, 2);

    assert_eq!(
        decision.differential_state,
        TrackingReadModelDifferentialState::Rejected
    );
    assert_eq!(
        decision.decision_state,
        READ_MODEL_DIFFERENTIAL_REJECTED_COUNT_DRIFT
    );
}

#[test]
fn tracking_read_model_counts_reject_deleted_refs_without_tombstone_rows() {
    let decision = evaluate_tracking_read_model_differential(3, 3, 0, 1);

    assert_eq!(
        decision.differential_state,
        TrackingReadModelDifferentialState::Rejected
    );
    assert_eq!(
        decision.decision_state,
        READ_MODEL_DIFFERENTIAL_REJECTED_TOMBSTONE_DRIFT
    );
}
