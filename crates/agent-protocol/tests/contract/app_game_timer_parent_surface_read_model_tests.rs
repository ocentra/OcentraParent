use super::{
    constants, AppGameTimerParentSurfaceChildUxLocalArtifactRecord,
    AppGameTimerParentSurfaceChildUxParentPreferenceSetupRecord,
    AppGameTimerParentSurfaceChildUxParentSurfaceIntentRecord, AppGameTimerParentSurfaceReadModel,
    AppGameTimerParentSurfaceRow, APP_GAME_SCHEMA_VERSION,
    APP_GAME_TIMER_PARENT_SURFACE_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    APP_GAME_TIMER_PARENT_SURFACE_STATE_READY_FOR_PARENT_SURFACE,
    APP_GAME_TIMER_PARENT_SURFACE_STATUS_PARTIAL, APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn app_game_timer_parent_surface_read_model_serializes_no_runtime_claims() {
    let serialized = serde_json::to_value(app_game_timer_parent_surface_read_model())
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

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
    assert_eq!(serialized["childUxLocalHandoffArtifactRecordCount"], 1);
    assert_eq!(serialized["childUxLocalHandoffArtifactSkippedCount"], 0);
    assert_eq!(
        serialized["childUxParentSurfaceIntentManualActionRequiredCount"],
        1
    );
    assert_eq!(
        serialized["childUxParentSurfaceIntentUnavailableVisibleCount"],
        0
    );
    assert_eq!(
        serialized["childUxParentSurfaceIntentHistoryVisibleCount"],
        1
    );
    assert_eq!(
        serialized["childUxParentSurfaceIntentPreferenceSetupRequiredCount"],
        1
    );
    assert_eq!(
        serialized["childUxParentSurfaceIntentRecords"][0]["parentNotificationUiRendered"],
        false
    );
    assert_eq!(serialized["childUxParentPreferenceSetupDraftReadyCount"], 1);
    assert_eq!(
        serialized["childUxParentPreferenceSetupUnavailableVisibleCount"],
        0
    );
    assert_eq!(
        serialized["childUxParentPreferenceSetupRequestReadyCount"],
        1
    );
    assert_eq!(
        serialized["childUxParentPreferenceSetupRequestUnavailableVisibleCount"],
        0
    );
    assert_eq!(
        serialized["childUxParentPreferenceSetupRecords"][0]["parentPreferenceSetupRequestStatus"],
        constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_REQUEST_READY
    );
    assert_eq!(
        serialized["childUxParentPreferenceSetupRecords"][0]["parentPreferenceUiRendered"],
        false
    );
    assert_eq!(
        serialized["childUxParentPreferenceSetupRecords"][0]["notificationRuleMutationClaimed"],
        false
    );
    assert_eq!(
        serialized["childUxLocalHandoffArtifactRecords"][0]["childDeliveryClaimed"],
        false
    );
    assert_eq!(serialized["adapterDispatchClaimed"], false);
    assert_eq!(serialized["childDeliveryClaimed"], false);
    assert_eq!(serialized["platformEnforcementClaimed"], false);
    assert_eq!(serialized["rawPrivateSourceRowsIncluded"], false);
    assert_eq!(
        serialized["rows"][0]["timerSurfaceState"],
        APP_GAME_TIMER_PARENT_SURFACE_STATE_READY_FOR_PARENT_SURFACE
    );
}

fn app_game_timer_parent_surface_read_model() -> AppGameTimerParentSurfaceReadModel {
    AppGameTimerParentSurfaceReadModel {
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
        child_ux_handoff_ready_count: 0,
        child_ux_handoff_blocked_count: 0,
        child_ux_handoff_reference_ids: Vec::new(),
        child_ux_local_handoff_artifact_record_count: 1,
        child_ux_local_handoff_artifact_skipped_count: 0,
        child_ux_local_handoff_artifact_reference_ids: vec![[
            constants::value::APP_GAME_CHILD_UX_LOCAL_HANDOFF_ARTIFACT_PREFIX,
            APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
        ]
        .concat()],
        child_ux_local_handoff_artifact_records: vec![child_ux_artifact_record()],
        child_ux_parent_surface_intent_manual_action_required_count: 1,
        child_ux_parent_surface_intent_unavailable_visible_count: 0,
        child_ux_parent_surface_intent_history_visible_count: 1,
        child_ux_parent_surface_intent_preference_setup_required_count: 1,
        child_ux_parent_surface_intent_reference_ids: vec![[
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX,
            APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
        ]
        .concat()],
        child_ux_parent_surface_intent_records: vec![child_ux_parent_surface_record()],
        child_ux_parent_preference_setup_draft_ready_count: 1,
        child_ux_parent_preference_setup_unavailable_visible_count: 0,
        child_ux_parent_preference_setup_reference_ids: vec![[
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX,
            APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
        ]
        .concat()],
        child_ux_parent_preference_setup_request_ready_count: 1,
        child_ux_parent_preference_setup_request_unavailable_visible_count: 0,
        child_ux_parent_preference_setup_request_reference_ids: vec![[
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX,
            APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
        ]
        .concat()],
        child_ux_parent_preference_setup_records: vec![child_ux_parent_preference_setup_record()],
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
    }
}

fn child_ux_parent_surface_record() -> AppGameTimerParentSurfaceChildUxParentSurfaceIntentRecord {
    AppGameTimerParentSurfaceChildUxParentSurfaceIntentRecord {
        schema_version: APP_GAME_SCHEMA_VERSION,
        parent_surface_intent_reference_id: [
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX,
            APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
        ]
        .concat(),
        source_result_id: APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME.to_string(),
        source_artifact_reference_id: [
            constants::value::APP_GAME_CHILD_UX_LOCAL_HANDOFF_ARTIFACT_PREFIX,
            APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
        ]
        .concat(),
        target_domain: APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME.to_string(),
        history_visibility: "history-row-visible".to_string(),
        parent_surface_status: "manual-action-required".to_string(),
        preference_visibility: "preference-setup-required".to_string(),
        drill_in_reference_ids: vec![[
            constants::value::APP_GAME_CHILD_UX_LOCAL_HANDOFF_ARTIFACT_PREFIX,
            APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
        ]
        .concat()],
        manual_proof_reference_ids: vec![[
            constants::value::APP_GAME_CHILD_UX_LOCAL_HANDOFF_ARTIFACT_PREFIX,
            APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
        ]
        .concat()],
        sensitive_detail_included: false,
        parent_notification_ui_rendered: false,
        parent_preference_mutation_claimed: false,
        provider_delivery_claimed: false,
        child_delivery_claimed: false,
        adapter_dispatch_claimed: false,
        platform_enforcement_claimed: false,
        raw_private_source_rows_included: false,
    }
}

fn child_ux_parent_preference_setup_record(
) -> AppGameTimerParentSurfaceChildUxParentPreferenceSetupRecord {
    AppGameTimerParentSurfaceChildUxParentPreferenceSetupRecord {
        schema_version: APP_GAME_SCHEMA_VERSION,
        parent_preference_setup_reference_id: [
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX,
            APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
        ]
        .concat(),
        source_parent_surface_intent_reference_id: [
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX,
            APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
        ]
        .concat(),
        source_result_id: APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME.to_string(),
        source_artifact_reference_id: [
            constants::value::APP_GAME_CHILD_UX_LOCAL_HANDOFF_ARTIFACT_PREFIX,
            APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
        ]
        .concat(),
        target_domain: APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME.to_string(),
        draft_status: constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_DRAFT_READY
            .to_string(),
        parent_preference_setup_request_status:
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_REQUEST_READY.to_string(),
        parent_preference_setup_request_reference_ids: vec![[
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX,
            APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
        ]
        .concat()],
        drill_in_reference_ids: vec![[
            constants::value::APP_GAME_CHILD_UX_LOCAL_HANDOFF_ARTIFACT_PREFIX,
            APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
        ]
        .concat()],
        manual_proof_reference_ids: vec![[
            constants::value::APP_GAME_CHILD_UX_LOCAL_HANDOFF_ARTIFACT_PREFIX,
            APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
        ]
        .concat()],
        parent_preference_ui_rendered: false,
        parent_frequency_control_ui_rendered: false,
        parent_preference_mutation_claimed: false,
        notification_rule_mutation_claimed: false,
        provider_delivery_claimed: false,
        child_delivery_claimed: false,
        adapter_dispatch_claimed: false,
        platform_enforcement_claimed: false,
        raw_private_source_rows_included: false,
    }
}

fn child_ux_artifact_record() -> AppGameTimerParentSurfaceChildUxLocalArtifactRecord {
    AppGameTimerParentSurfaceChildUxLocalArtifactRecord {
        schema_version: APP_GAME_SCHEMA_VERSION,
        artifact_reference_id: [
            constants::value::APP_GAME_CHILD_UX_LOCAL_HANDOFF_ARTIFACT_PREFIX,
            APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
        ]
        .concat(),
        source_result_id: APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME.to_string(),
        target_domain: APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME.to_string(),
        child_reason_reference_ids: Vec::new(),
        child_status_reference_ids: Vec::new(),
        child_delivery_claimed: false,
        notification_delivery_claimed: false,
        adapter_dispatch_claimed: false,
        platform_enforcement_claimed: false,
        raw_private_source_rows_included: false,
    }
}
