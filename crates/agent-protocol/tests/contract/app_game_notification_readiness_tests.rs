use super::{
    constants, AppGameNotificationReadinessReadModel, AppGameNotificationReadinessRow,
    APP_GAME_NOTIFICATION_READINESS_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_TIME_LIMIT,
    APP_GAME_NOTIFICATION_READINESS_REASON_TIME_LIMIT_EXCEEDED,
    APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
    APP_GAME_NOTIFICATION_READINESS_STATUS_PARTIAL, APP_GAME_SCHEMA_VERSION,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn app_game_notification_readiness_read_model_serializes_no_delivery_claims() {
    let read_model = readiness_read_model();

    let serialized = serde_json::to_value(read_model.clone())
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["custodyLabel"],
        APP_GAME_NOTIFICATION_READINESS_CUSTODY_CHILD_DEVICE_QUERY_STORE
    );
    assert_eq!(
        serialized["capabilityStatus"],
        APP_GAME_NOTIFICATION_READINESS_STATUS_PARTIAL
    );
    assert_eq!(serialized["providerDeliveryClaimed"], false);
    assert_eq!(serialized["providerReceiptIngestionClaimed"], false);
    assert_eq!(serialized["localOutboxRuntimeClaimed"], false);
    assert_eq!(serialized["schedulerRuntimeClaimed"], false);
    assert_eq!(serialized["adapterDispatchClaimed"], false);
    assert_eq!(serialized["parentUiClaimed"], false);
    assert_eq!(serialized["childDeliveryClaimed"], false);
    assert_eq!(
        serialized["rows"][0]["minimalPayloadRef"],
        APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_TIME_LIMIT
    );

    let decoded = serde_json::from_value::<AppGameNotificationReadinessReadModel>(serialized)
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    assert_eq!(decoded, read_model);
}

#[test]
fn app_game_notification_readiness_read_model_rejects_unknown_fields() {
    let serialized = serde_json::to_value(readiness_read_model())
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    let mut unknown_top_level_field = serialized.clone();
    unknown_top_level_field["unexpectedField"] = serde_json::Value::Bool(true);

    let mut unknown_row_field = serialized;
    unknown_row_field["rows"][0]["unexpectedField"] = serde_json::Value::Bool(true);

    for candidate in [unknown_top_level_field, unknown_row_field] {
        let parsed = serde_json::from_value::<AppGameNotificationReadinessReadModel>(candidate);
        assert_eq!(
            parsed.err().map(|error| error.classify()),
            Some(serde_json::error::Category::Data)
        );
    }
}

fn readiness_read_model() -> AppGameNotificationReadinessReadModel {
    AppGameNotificationReadinessReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT
            .to_string(),
        custody_label: APP_GAME_NOTIFICATION_READINESS_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        capability_status: APP_GAME_NOTIFICATION_READINESS_STATUS_PARTIAL.to_string(),
        returned: 1,
        ready_intent_count: 1,
        manual_required_count: 0,
        unavailable_count: 0,
        provider_delivery_claimed: false,
        provider_receipt_ingestion_claimed: false,
        local_outbox_runtime_claimed: false,
        scheduler_runtime_claimed: false,
        adapter_dispatch_claimed: false,
        parent_ui_claimed: false,
        child_delivery_claimed: false,
        rows: vec![readiness_model_row()],
    }
}

fn readiness_model_row() -> AppGameNotificationReadinessRow {
    AppGameNotificationReadinessRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id: APP_GAME_NOTIFICATION_READINESS_REASON_TIME_LIMIT_EXCEEDED.to_string(),
        reason: APP_GAME_NOTIFICATION_READINESS_REASON_TIME_LIMIT_EXCEEDED.to_string(),
        readiness_state: APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT.to_string(),
        row_count: 1,
        minimal_payload_ref: APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_TIME_LIMIT.to_string(),
        evidence_reference_ids: vec![
            constants::activity_store::TEST_TRACKING_RETENTION_DELETE_EVENT_ID.to_string(),
        ],
        evidence: Vec::new(),
    }
}
