use crate::app_game_child_runtime_transport_receipt::{
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_PARITY_MANIFEST,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_PAYLOAD_FIELD,
};
use crate::{
    AgentCommandName, AgentEventName, AppGameChildRuntimeTransportReceiptReadModel,
    AppGameChildRuntimeTransportReceiptRow, APP_GAME_ADAPTER_PRODUCT_NATIVE_APP,
    APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_CAPABILITY_REQUIRED,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_CUSTODY_LABEL,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_PLATFORM_CHANNEL_NOT_PROVED,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_PROVIDER_NOT_EXECUTED,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_RECEIPT_NOT_INGESTED,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_TRANSPORT_NOT_EXECUTED,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL_ID,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_REF_RECEIPT_CONTRACT,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_REF_SOURCE_WRITER,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_REF_TRANSPORT_CONTRACT,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_TRANSPORT_REQUIRED, APP_GAME_SCHEMA_VERSION,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn app_game_child_runtime_transport_receipt_command_and_event_names_are_stable() {
    assert_eq!(
        serde_json::to_value(
            AgentCommandName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelGet
        )
        .expect_value(crate::constants::error::AGENT_EVENT_SERIALIZES),
        "agent.activity.app-game.child-runtime-transport-receipt.read-model.get"
    );
    assert_eq!(
        serde_json::to_value(
            AgentEventName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelReported
        )
        .expect_value(crate::constants::error::AGENT_EVENT_SERIALIZES),
        "agent.activity.app-game.child-runtime-transport-receipt.read-model.reported"
    );
}

#[test]
fn app_game_child_runtime_transport_receipt_parity_manifest_matches_contract_constants() {
    let manifest: serde_json::Value =
        serde_json::from_str(APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_PARITY_MANIFEST)
            .expect_value("child runtime transport receipt parity manifest parses");

    assert_eq!(
        manifest,
        serde_json::json!({
            "schemaVersion": APP_GAME_SCHEMA_VERSION,
            "payloadField": APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_PAYLOAD_FIELD,
            "readModelId": APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL_ID,
            "sourceRuntimeWriterRef": APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_REF_SOURCE_WRITER,
            "custodyLabel": APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_CUSTODY_LABEL,
            "capabilityStatus": APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_CAPABILITY_REQUIRED,
            "stateValues": [
                APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_TRANSPORT_REQUIRED,
                "manual-required",
                "unavailable"
            ],
            "productMeanings": [
                APP_GAME_ADAPTER_PRODUCT_NATIVE_APP,
                APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME
            ],
            "canonicalRefs": [
                APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_REF_TRANSPORT_CONTRACT,
                APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_REF_RECEIPT_CONTRACT
            ],
            "canonicalGaps": [
                APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_TRANSPORT_NOT_EXECUTED,
                APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_RECEIPT_NOT_INGESTED,
                APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_PROVIDER_NOT_EXECUTED,
                APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_PLATFORM_CHANNEL_NOT_PROVED
            ],
            "rowFields": [
                "schemaVersion",
                "rowId",
                "sourceRuntimeWriterRowId",
                "boundaryState",
                "productMeanings",
                "requiredTransportRefs",
                "requiredReceiptRefs",
                "openGaps",
                "runtimeTransportExecuted",
                "runtimeReceiptIngested",
                "providerDeliveryExecuted",
                "platformDeliveryChannelClaimed"
            ],
            "readModelFields": [
                "schemaVersion",
                "readModelId",
                "generatedAt",
                "sourceReadModelIds",
                "custodyLabel",
                "capabilityStatus",
                "returned",
                "transportRequiredCount",
                "manualRequiredCount",
                "unavailableCount",
                "runtimeTransportExecuted",
                "runtimeReceiptIngested",
                "providerDeliveryExecuted",
                "platformDeliveryChannelClaimed",
                "adapterDispatchClaimed",
                "platformEnforcementClaimed",
                "rawPrivateSourceRowsIncluded",
                "rows"
            ]
        })
    );
}

#[test]
fn app_game_child_runtime_transport_receipt_serializes_without_delivery_claims() {
    let read_model = AppGameChildRuntimeTransportReceiptReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        read_model_id: APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL_ID.to_string(),
        generated_at: crate::policy_constants::TEST_EVALUATED_AT.to_string(),
        source_read_model_ids: vec!["app-game-child-device-runtime-writer".to_string()],
        custody_label: "app-game-child-runtime-transport-receipt".to_string(),
        capability_status: "app-game-child-runtime-transport-required".to_string(),
        returned: 1,
        transport_required_count: 1,
        manual_required_count: 0,
        unavailable_count: 0,
        runtime_transport_executed: false,
        runtime_receipt_ingested: false,
        provider_delivery_executed: false,
        platform_delivery_channel_claimed: false,
        adapter_dispatch_claimed: false,
        platform_enforcement_claimed: false,
        raw_private_source_rows_included: false,
        rows: vec![transport_required_row()],
    };

    let reparsed = serde_json::from_value::<AppGameChildRuntimeTransportReceiptReadModel>(
        serde_json::to_value(read_model)
            .expect_value(crate::constants::error::AGENT_EVENT_SERIALIZES),
    )
    .expect_value("child runtime transport receipt read model reparses");

    assert_eq!(
        reparsed.read_model_id,
        APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL_ID
    );
    assert_eq!(reparsed.transport_required_count, 1);
    assert!(!reparsed.runtime_transport_executed);
    assert!(!reparsed.rows[0].runtime_receipt_ingested);
}

fn transport_required_row() -> AppGameChildRuntimeTransportReceiptRow {
    AppGameChildRuntimeTransportReceiptRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id: "app-game-child-runtime-transport-receipt-limit-reached".to_string(),
        source_runtime_writer_row_id: "app-game-child-device-runtime-writer-limit-reached"
            .to_string(),
        boundary_state: APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_TRANSPORT_REQUIRED
            .to_string(),
        product_meanings: vec![
            APP_GAME_ADAPTER_PRODUCT_NATIVE_APP.to_string(),
            APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME.to_string(),
        ],
        required_transport_refs: vec![
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_REF_TRANSPORT_CONTRACT.to_string(),
        ],
        required_receipt_refs: vec![
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_REF_RECEIPT_CONTRACT.to_string(),
        ],
        open_gaps: vec![
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_TRANSPORT_NOT_EXECUTED.to_string(),
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_RECEIPT_NOT_INGESTED.to_string(),
        ],
        runtime_transport_executed: false,
        runtime_receipt_ingested: false,
        provider_delivery_executed: false,
        platform_delivery_channel_claimed: false,
    }
}
