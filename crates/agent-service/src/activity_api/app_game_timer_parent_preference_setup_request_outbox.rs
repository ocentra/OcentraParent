use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::Write,
    path::PathBuf,
};

use fs2::FileExt;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::AppGameTimerParentPreferenceSetupRequestResult;
use serde_json::{Map, Value};

use super::app_game_timer_parent_preference_setup_request::AppGameTimerSetupStorePath;

const OUTBOX_LOCK_EXTENSION: &str = "lock";

pub(crate) struct SetupOutboxPath(PathBuf);

struct SetupOutboxTextRef<'a>(&'a str);

struct SetupOutboxStringField<'a> {
    name: &'static str,
    value: SetupOutboxTextRef<'a>,
}

struct SetupOutboxBoolField {
    name: &'static str,
    value: bool,
}

struct SetupOutboxLock(File);

impl Drop for SetupOutboxLock {
    fn drop(&mut self) {
        let _ = self.0.unlock();
    }
}

pub(crate) trait SetupOutboxStorePathSource {
    fn setup_outbox_store_path(&self) -> SetupOutboxPath;
}

impl SetupOutboxStorePathSource for AppGameTimerSetupStorePath {
    fn setup_outbox_store_path(&self) -> SetupOutboxPath {
        SetupOutboxPath(self.0.clone())
    }
}

macro_rules! setup_outbox_string_field {
    ($name:expr, $value:expr) => {
        SetupOutboxStringField {
            name: $name,
            value: SetupOutboxTextRef($value),
        }
    };
}

macro_rules! setup_outbox_bool_field {
    ($name:expr, $value:expr) => {
        SetupOutboxBoolField {
            name: $name,
            value: $value,
        }
    };
}

pub(crate) fn append_setup_outbox_record(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
    store_path: &impl SetupOutboxStorePathSource,
) -> Result<(), ()> {
    let outbox_path = setup_outbox_path(store_path);
    if let Some(parent) = outbox_path.0.parent() {
        create_dir_all(parent).map_err(|_error| ())?;
    }
    let record = setup_outbox_record(result);
    let record_id = record
        .get(constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_RECORD_ID)
        .and_then(Value::as_str)
        .filter(|record_id| !record_id.is_empty() && *record_id == record_id.trim())
        .ok_or(())?;
    let line = serde_json::to_string(&record).map_err(|_error| ())?;
    let lock_path = outbox_path.0.with_extension(OUTBOX_LOCK_EXTENSION);
    let lock_file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(lock_path)
        .map_err(|_error| ())?;
    lock_file.lock_exclusive().map_err(|_error| ())?;
    let _lock = SetupOutboxLock(lock_file);
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(&outbox_path.0)
        .map_err(|_error| ())?;
    let existing = std::fs::read_to_string(&outbox_path.0).map_err(|_error| ())?;
    if !existing.is_empty()
        && !existing.ends_with(constants::delimiter::NEWLINE.to_string().as_str())
    {
        return Err(());
    }
    let existing_records: Vec<(Value, String)> = existing
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|existing_line| {
            let existing_record: Value =
                serde_json::from_str(existing_line).map_err(|_error| ())?;
            let existing_object = existing_record.as_object().ok_or(())?;
            let existing_record_id = existing_object
                .get(constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_RECORD_ID)
                .and_then(Value::as_str)
                .filter(|record_id| !record_id.is_empty() && *record_id == record_id.trim())
                .ok_or(())?
                .to_owned();
            Ok((existing_record, existing_record_id))
        })
        .collect::<Result<_, ()>>()?;
    for (existing_record, existing_record_id) in existing_records {
        if existing_record_id == record_id {
            if existing_record == record {
                return Ok(());
            }
            return Err(());
        }
    }
    let serialized_line = format!("{}{}", line, constants::delimiter::NEWLINE);
    file.write_all(serialized_line.as_bytes())
        .map_err(|_error| ())?;
    file.sync_all().map_err(|_error| ())?;
    if let Some(parent) = outbox_path.0.parent() {
        sync_parent_directory(parent)?;
    }
    Ok(())
}

#[cfg(not(windows))]
fn sync_parent_directory(parent: &std::path::Path) -> Result<(), ()> {
    File::open(parent)
        .map_err(|_error| ())?
        .sync_all()
        .map_err(|_error| ())
}

#[cfg(windows)]
fn sync_parent_directory(_parent: &std::path::Path) -> Result<(), ()> {
    Ok(())
}

fn setup_outbox_path(store_path: &impl SetupOutboxStorePathSource) -> SetupOutboxPath {
    SetupOutboxPath(store_path.setup_outbox_store_path().0.with_extension(
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_DURABLE_OUTBOX_FILE_EXTENSION,
    ))
}

fn setup_outbox_record(result: &AppGameTimerParentPreferenceSetupRequestResult) -> Value {
    let mut record = Map::new();
    for field in setup_outbox_string_fields(result) {
        record.insert(
            field.name.to_string(),
            Value::String(field.value.0.to_string()),
        );
    }
    for field in setup_outbox_bool_fields(result) {
        record.insert(field.name.to_string(), Value::Bool(field.value));
    }
    Value::Object(record)
}

fn setup_outbox_string_fields(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> Vec<SetupOutboxStringField<'_>> {
    let mut fields = Vec::from(setup_outbox_base_string_fields(result));
    fields.extend(setup_outbox_provider_string_fields(result));
    fields
}

fn setup_outbox_base_string_fields(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> [SetupOutboxStringField<'_>; 8] {
    [
        setup_outbox_string_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_SCHEMA_VERSION,
            result.schema_version.as_str()
        ),
        setup_outbox_string_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_RECORD_ID,
            result.durable_outbox_record_id.as_str()
        ),
        setup_outbox_string_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_REQUEST_ID,
            result.request_id.as_str()
        ),
        setup_outbox_string_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_RECORDED_AT,
            result.accepted_at.as_str()
        ),
        setup_outbox_string_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PARENT_SURFACE_INTENT_REFERENCE_ID,
            result.parent_surface_intent_reference_id.as_str()
        ),
        setup_outbox_string_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PARENT_PREFERENCE_SETUP_REFERENCE_ID,
            result.parent_preference_setup_reference_id.as_str()
        ),
        setup_outbox_string_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_ACTION_RESULT_REFERENCE_ID,
            result.action_result_reference_id.as_str()
        ),
        setup_outbox_string_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_CHILD_RUNTIME_DELIVERY_RECEIPT_INGESTED_ID,
            result.child_runtime_delivery_receipt_ingested_id.as_str()
        ),
    ]
}

fn setup_outbox_provider_string_fields(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> [SetupOutboxStringField<'_>; 12] {
    [
        setup_outbox_string_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_ADAPTER_REQUIREMENT_ID,
            result.provider_delivery_adapter_requirement_id.as_str()
        ),
        setup_outbox_string_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_ADAPTER_REQUIREMENT_STATUS,
            result.provider_delivery_adapter_requirement_status.as_str()
        ),
        setup_outbox_string_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_CREDENTIAL_REQUIREMENT_ID,
            result.provider_delivery_credential_requirement_id.as_str()
        ),
        setup_outbox_string_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_CREDENTIAL_REQUIREMENT_STATUS,
            result.provider_delivery_credential_requirement_status.as_str()
        ),
        setup_outbox_string_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_QUEUE_ID,
            result.provider_delivery_queue_id.as_str()
        ),
        setup_outbox_string_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_QUEUE_STATUS,
            result.provider_delivery_queue_status.as_str()
        ),
        setup_outbox_string_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_RECEIPT_REQUIREMENT_ID,
            result.provider_delivery_receipt_requirement_id.as_str()
        ),
        setup_outbox_string_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_RECEIPT_REQUIREMENT_STATUS,
            result.provider_delivery_receipt_requirement_status.as_str()
        ),
        setup_outbox_string_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_RECEIPT_PENDING_ID,
            result.provider_delivery_receipt_pending_id.as_str()
        ),
        setup_outbox_string_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_RECEIPT_PENDING_STATUS,
            result.provider_delivery_receipt_pending_status.as_str()
        ),
        setup_outbox_string_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_RECEIPT_INGESTED_ID,
            result.provider_delivery_receipt_ingested_id.as_str()
        ),
        setup_outbox_string_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_RECEIPT_INGESTED_STATUS,
            result.provider_delivery_receipt_ingested_status.as_str()
        ),
    ]
}

fn setup_outbox_bool_fields(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> [SetupOutboxBoolField; 7] {
    [
        setup_outbox_bool_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_DELIVERY_CLAIMED,
            result.provider_delivery_claimed
        ),
        setup_outbox_bool_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PROVIDER_RECEIPT_INGESTION_CLAIMED,
            result.provider_receipt_ingestion_claimed
        ),
        setup_outbox_bool_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_ADAPTER_DISPATCH_CLAIMED,
            result.adapter_dispatch_claimed
        ),
        setup_outbox_bool_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PLATFORM_ENFORCEMENT_CLAIMED,
            result.platform_enforcement_claimed
        ),
        setup_outbox_bool_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_RAW_PRIVATE_SOURCE_ROWS_CLAIMED,
            result.raw_private_source_rows_claimed
        ),
        setup_outbox_bool_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_RAW_TARGET_VALUES_CLAIMED,
            result.raw_target_values_claimed
        ),
        setup_outbox_bool_field!(
            constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_PRIVATE_DIAGNOSTICS_CLAIMED,
            result.private_diagnostics_claimed
        ),
    ]
}
