use super::{
    constants, AppGameTimerParentSurfaceReadModel, AppGameTimerParentSurfaceRow,
    APP_GAME_SCHEMA_VERSION, APP_GAME_TIMER_PARENT_SURFACE_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    APP_GAME_TIMER_PARENT_SURFACE_STATE_READY_FOR_PARENT_SURFACE,
    APP_GAME_TIMER_PARENT_SURFACE_STATUS_PARTIAL, APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
};

#[test]
fn app_game_timer_parent_surface_read_model_serializes_no_runtime_claims() {
    let read_model = AppGameTimerParentSurfaceReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT
            .to_string(),
        custody_label: APP_GAME_TIMER_PARENT_SURFACE_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        capability_status: APP_GAME_TIMER_PARENT_SURFACE_STATUS_PARTIAL.to_string(),
        returned: 1,
        ready_for_parent_surface_count: 1,
        blocked_by_source_freshness_count: 0,
        blocked_by_compiler_decision_count: 0,
        runtime_manual_required_count: 0,
        control_action_result_count: 0,
        control_action_result_reference_ids: Vec::new(),
        control_action_result_statuses: Vec::new(),
        control_action_result_capability_states: Vec::new(),
        control_action_result_enforcement_statuses: Vec::new(),
        child_facing_reason_reference_ids: Vec::new(),
        child_facing_status_reference_ids: Vec::new(),
        timer_runtime_claimed: false,
        scheduler_persistence_claimed: false,
        durable_scheduler_storage_claimed: false,
        audit_runtime_claimed: false,
        rollback_runtime_claimed: false,
        adapter_dispatch_claimed: false,
        child_delivery_claimed: false,
        platform_enforcement_claimed: false,
        raw_private_source_rows_included: false,
        rows: vec![AppGameTimerParentSurfaceRow {
            schema_version: APP_GAME_SCHEMA_VERSION,
            row_id: APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME.to_string(),
            target_domain: APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME.to_string(),
            timer_surface_state: APP_GAME_TIMER_PARENT_SURFACE_STATE_READY_FOR_PARENT_SURFACE
                .to_string(),
            row_count: 1,
            evidence_reference_ids: vec![
                constants::activity_store::TEST_TRACKING_RETENTION_DELETE_EVENT_ID.to_string(),
            ],
            evidence: Vec::new(),
        }],
    };

    let serialized =
        serde_json::to_value(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["custodyLabel"],
        APP_GAME_TIMER_PARENT_SURFACE_CUSTODY_CHILD_DEVICE_QUERY_STORE
    );
    assert_eq!(
        serialized["capabilityStatus"],
        APP_GAME_TIMER_PARENT_SURFACE_STATUS_PARTIAL
    );
    assert_eq!(serialized["timerRuntimeClaimed"], false);
    assert_eq!(serialized["schedulerPersistenceClaimed"], false);
    assert_eq!(serialized["durableSchedulerStorageClaimed"], false);
    assert_eq!(serialized["auditRuntimeClaimed"], false);
    assert_eq!(serialized["rollbackRuntimeClaimed"], false);
    assert_eq!(serialized["adapterDispatchClaimed"], false);
    assert_eq!(serialized["childDeliveryClaimed"], false);
    assert_eq!(serialized["platformEnforcementClaimed"], false);
    assert_eq!(serialized["rawPrivateSourceRowsIncluded"], false);
    assert_eq!(
        serialized["rows"][0]["timerSurfaceState"],
        APP_GAME_TIMER_PARENT_SURFACE_STATE_READY_FOR_PARENT_SURFACE
    );
}
