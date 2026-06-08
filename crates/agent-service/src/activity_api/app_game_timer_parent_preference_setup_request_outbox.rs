use std::{
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::Path,
};

use ocentra_parent_agent_protocol::{constants, AppGameTimerParentPreferenceSetupRequestResult};
use serde_json::{Map, Value};

pub(crate) fn append_setup_outbox_record(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
    store_path: &Path,
) -> Result<(), ()> {
    let outbox_path = store_path.with_extension(
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_DURABLE_OUTBOX_FILE_EXTENSION,
    );
    if let Some(parent) = outbox_path.parent() {
        create_dir_all(parent).map_err(|_| ())?;
    }
    let record = setup_outbox_record(result);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(outbox_path)
        .map_err(|_| ())?;
    let line = serde_json::to_string(&record).map_err(|_| ())?;
    file.write_all(line.as_bytes()).map_err(|_| ())?;
    file.write_all(constants::delimiter::NEWLINE.to_string().as_bytes())
        .map_err(|_| ())
}

fn setup_outbox_record(result: &AppGameTimerParentPreferenceSetupRequestResult) -> Value {
    let mut record = Map::new();
    for (field, value) in setup_outbox_string_fields(result) {
        record.insert(field.to_string(), Value::String(value.to_string()));
    }
    for (field, value) in setup_outbox_bool_fields(result) {
        record.insert(field.to_string(), Value::Bool(value));
    }
    Value::Object(record)
}

fn setup_outbox_string_fields(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> [(&'static str, &String); 18] {
    [
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_SCHEMA_VERSION,
            &result.schema_version,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_RECORD_ID,
            &result.durable_outbox_record_id,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_REQUEST_ID,
            &result.request_id,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_RECORDED_AT,
            &result.accepted_at,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PARENT_SURFACE_INTENT_REFERENCE_ID,
            &result.parent_surface_intent_reference_id,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PARENT_PREFERENCE_SETUP_REFERENCE_ID,
            &result.parent_preference_setup_reference_id,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_ACTION_RESULT_REFERENCE_ID,
            &result.action_result_reference_id,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_CHILD_RUNTIME_DELIVERY_RECEIPT_INGESTED_ID,
            &result.child_runtime_delivery_receipt_ingested_id,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_ADAPTER_REQUIREMENT_ID,
            &result.provider_delivery_adapter_requirement_id,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_ADAPTER_REQUIREMENT_STATUS,
            &result.provider_delivery_adapter_requirement_status,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_CREDENTIAL_REQUIREMENT_ID,
            &result.provider_delivery_credential_requirement_id,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_CREDENTIAL_REQUIREMENT_STATUS,
            &result.provider_delivery_credential_requirement_status,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_QUEUE_ID,
            &result.provider_delivery_queue_id,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_QUEUE_STATUS,
            &result.provider_delivery_queue_status,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_RECEIPT_REQUIREMENT_ID,
            &result.provider_delivery_receipt_requirement_id,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_RECEIPT_REQUIREMENT_STATUS,
            &result.provider_delivery_receipt_requirement_status,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_RECEIPT_PENDING_ID,
            &result.provider_delivery_receipt_pending_id,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_RECEIPT_PENDING_STATUS,
            &result.provider_delivery_receipt_pending_status,
        ),
    ]
}

fn setup_outbox_bool_fields(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> [(&'static str, bool); 7] {
    [
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_CLAIMED,
            result.provider_delivery_claimed,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_RECEIPT_INGESTION_CLAIMED,
            result.provider_receipt_ingestion_claimed,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_ADAPTER_DISPATCH_CLAIMED,
            result.adapter_dispatch_claimed,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PLATFORM_ENFORCEMENT_CLAIMED,
            result.platform_enforcement_claimed,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_RAW_PRIVATE_SOURCE_ROWS_CLAIMED,
            result.raw_private_source_rows_claimed,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_RAW_TARGET_VALUES_CLAIMED,
            result.raw_target_values_claimed,
        ),
        (
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PRIVATE_DIAGNOSTICS_CLAIMED,
            result.private_diagnostics_claimed,
        ),
    ]
}
