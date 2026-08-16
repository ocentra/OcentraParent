use ocentra_parent_agent_protocol::activity_query::ACTIVITY_QUERY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::constants::tracking_runtime::{
    READ_MODEL_SCHEMA_COMPATIBLE, READ_MODEL_SCHEMA_MIGRATION_REQUIRED,
};
use ocentra_tracking_core::read_model_guard::{
    evaluate_tracking_read_model_schema, TrackingReadModelMigrationState,
};

#[test]
fn tracking_read_model_schema_accepts_current_version() {
    let decision = evaluate_tracking_read_model_schema(
        ACTIVITY_QUERY_SCHEMA_VERSION,
        ACTIVITY_QUERY_SCHEMA_VERSION,
    );

    assert_eq!(
        decision.migration_state,
        TrackingReadModelMigrationState::Compatible
    );
    assert_eq!(decision.decision_state, READ_MODEL_SCHEMA_COMPATIBLE);
}

#[test]
fn tracking_read_model_schema_requires_migration_for_version_drift() {
    let decision = evaluate_tracking_read_model_schema(
        ACTIVITY_QUERY_SCHEMA_VERSION + 1,
        ACTIVITY_QUERY_SCHEMA_VERSION,
    );

    assert_eq!(
        decision.migration_state,
        TrackingReadModelMigrationState::MigrationRequired
    );
    assert_eq!(
        decision.decision_state,
        READ_MODEL_SCHEMA_MIGRATION_REQUIRED
    );
}
