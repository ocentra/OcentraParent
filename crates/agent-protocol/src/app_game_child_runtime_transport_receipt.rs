use serde::{Deserialize, Serialize};

pub const APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL_ID: &str =
    "app-game-child-runtime-transport-receipt";
pub const APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_PAYLOAD_FIELD: &str =
    "appGameChildRuntimeTransportReceiptReadModel";
pub const APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_CUSTODY_LABEL: &str =
    "app-game-child-runtime-transport-receipt";
pub const APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_CAPABILITY_REQUIRED: &str =
    "app-game-child-runtime-transport-required";
pub const APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_ROW_ID_PREFIX: &str =
    "app-game-child-runtime-transport-receipt-";
pub const APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_TRANSPORT_REQUIRED: &str =
    "child-runtime-transport-required";
pub const APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_MANUAL_REQUIRED: &str = "manual-required";
pub const APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_UNAVAILABLE: &str = "unavailable";
pub const APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_TRANSPORT_NOT_EXECUTED: &str =
    "child-runtime-transport-not-executed";
pub const APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_RECEIPT_NOT_INGESTED: &str =
    "child-runtime-receipt-not-ingested";
pub const APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_PROVIDER_NOT_EXECUTED: &str =
    "provider-delivery-not-executed";
pub const APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_PLATFORM_CHANNEL_NOT_PROVED: &str =
    "platform-delivery-channel-not-proved";
pub const APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_REF_SOURCE_WRITER: &str =
    "app-game-child-device-runtime-writer";
pub const APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_REF_TRANSPORT_CONTRACT: &str =
    "child-runtime-transport-contract-ref";
pub const APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_REF_RECEIPT_CONTRACT: &str =
    "child-runtime-delivery-receipt-contract-ref";
pub const APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_PARITY_MANIFEST: &str = r#"{"schemaVersion":1,"payloadField":"appGameChildRuntimeTransportReceiptReadModel","readModelId":"app-game-child-runtime-transport-receipt","sourceRuntimeWriterRef":"app-game-child-device-runtime-writer","custodyLabel":"app-game-child-runtime-transport-receipt","capabilityStatus":"app-game-child-runtime-transport-required","stateValues":["child-runtime-transport-required","manual-required","unavailable"],"productMeanings":["native-app","native-game"],"canonicalRefs":["child-runtime-transport-contract-ref","child-runtime-delivery-receipt-contract-ref"],"canonicalGaps":["child-runtime-transport-not-executed","child-runtime-receipt-not-ingested","provider-delivery-not-executed","platform-delivery-channel-not-proved"],"rowFields":["schemaVersion","rowId","sourceRuntimeWriterRowId","boundaryState","productMeanings","requiredTransportRefs","requiredReceiptRefs","openGaps","runtimeTransportExecuted","runtimeReceiptIngested","providerDeliveryExecuted","platformDeliveryChannelClaimed"],"readModelFields":["schemaVersion","readModelId","generatedAt","sourceReadModelIds","custodyLabel","capabilityStatus","returned","transportRequiredCount","manualRequiredCount","unavailableCount","runtimeTransportExecuted","runtimeReceiptIngested","providerDeliveryExecuted","platformDeliveryChannelClaimed","adapterDispatchClaimed","platformEnforcementClaimed","rawPrivateSourceRowsIncluded","rows"]}"#;

pub fn app_game_child_runtime_transport_receipt_typescript() -> String {
    include_str!(
        "../../../packages/schema-domain/src/generated-app-game-child-runtime-transport-receipt.ts"
    )
    .to_string()
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameChildRuntimeTransportReceiptRow {
    pub schema_version: u16,
    pub row_id: String,
    pub source_runtime_writer_row_id: String,
    pub boundary_state: String,
    pub product_meanings: Vec<String>,
    pub required_transport_refs: Vec<String>,
    pub required_receipt_refs: Vec<String>,
    pub open_gaps: Vec<String>,
    pub runtime_transport_executed: bool,
    pub runtime_receipt_ingested: bool,
    pub provider_delivery_executed: bool,
    pub platform_delivery_channel_claimed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameChildRuntimeTransportReceiptReadModel {
    pub schema_version: u16,
    pub read_model_id: String,
    pub generated_at: String,
    pub source_read_model_ids: Vec<String>,
    pub custody_label: String,
    pub capability_status: String,
    pub returned: u64,
    pub transport_required_count: u64,
    pub manual_required_count: u64,
    pub unavailable_count: u64,
    pub runtime_transport_executed: bool,
    pub runtime_receipt_ingested: bool,
    pub provider_delivery_executed: bool,
    pub platform_delivery_channel_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub raw_private_source_rows_included: bool,
    pub rows: Vec<AppGameChildRuntimeTransportReceiptRow>,
}
