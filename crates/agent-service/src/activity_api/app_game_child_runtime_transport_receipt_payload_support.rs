#[path = "app_game_child_runtime_transport_receipt_payload_support/boundary.rs"]
mod boundary;
#[path = "app_game_child_runtime_transport_receipt_payload_support/rows.rs"]
mod rows;

use super::AppGameReceiptGeneratedAt;

use ocentra_parent_agent_protocol::app_game::{AppGameServiceReadModel, APP_GAME_SCHEMA_VERSION};
use ocentra_parent_agent_protocol::app_game_authority_classifier::{
    APP_GAME_CONTROL_ACTION_STATUS_ENFORCED, APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED,
};
use ocentra_parent_agent_protocol::app_game_child_runtime_transport_receipt::{
    AppGameChildRuntimeTransportReceiptReadModel, AppGameChildRuntimeTransportReceiptRow,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_CAPABILITY_REQUIRED,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_CUSTODY_LABEL,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL_ID,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_REF_SOURCE_WRITER,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_MANUAL_REQUIRED,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_TRANSPORT_REQUIRED,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_UNAVAILABLE,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};

use self::boundary::count_rows;
use self::rows::child_runtime_rows_from_service_model;

use crate::{
    activity_surface_store::load_app_game_model, event_builder::build_event,
    fields::fields_from_pairs, time::timestamp_now,
};

#[derive(Clone, Debug)]
pub(super) struct AppGameReceiptSourceRowId(pub(super) String);

#[derive(Clone, Debug)]
pub(super) struct AppGameReceiptBoundaryState(pub(super) String);

#[derive(Clone, Debug)]
pub(super) struct AppGameReceiptReferenceIds(pub(super) Vec<String>);

pub(super) async fn build_activity_app_game_child_runtime_transport_receipt_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let generated_at: String = timestamp_now();
    let read_model = match load_app_game_model().await {
        Some(model) => {
            app_game_child_runtime_transport_receipt_read_model_from_service_model(model)
        }
        None => app_game_child_runtime_transport_receipt_read_model(AppGameReceiptGeneratedAt(
            generated_at,
        )),
    };
    build_event(
        constants::event_id::ACTIVITY_APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelReported,
        LogLevel::Info,
        app_game_child_runtime_transport_receipt_payload(&read_model),
        None,
    )
}

pub(super) fn app_game_child_runtime_transport_receipt_read_model(
    generated_at: AppGameReceiptGeneratedAt,
) -> AppGameChildRuntimeTransportReceiptReadModel {
    read_model_from_rows(generated_at, Vec::new(), false, false)
}

pub(super) fn app_game_child_runtime_transport_receipt_read_model_from_service_model(
    model: AppGameServiceReadModel,
) -> AppGameChildRuntimeTransportReceiptReadModel {
    let adapter_dispatch_claimed = model.approval_action_result_rows.iter().any(|row| {
        row.result_status == APP_GAME_CONTROL_ACTION_STATUS_ENFORCED
            && row.enforcement_result.as_ref().is_some_and(|result| {
                result.status == APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED
            })
    });
    let rows = child_runtime_rows_from_service_model(&model);
    let generated_at = AppGameReceiptGeneratedAt(model.generated_at);
    read_model_from_rows(
        generated_at,
        rows,
        adapter_dispatch_claimed,
        adapter_dispatch_claimed,
    )
}

fn read_model_from_rows(
    generated_at: AppGameReceiptGeneratedAt,
    rows: Vec<AppGameChildRuntimeTransportReceiptRow>,
    adapter_dispatch_claimed: bool,
    platform_enforcement_claimed: bool,
) -> AppGameChildRuntimeTransportReceiptReadModel {
    AppGameChildRuntimeTransportReceiptReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        read_model_id: APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL_ID.to_string(),
        generated_at: generated_at.0,
        source_read_model_ids: vec![
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_REF_SOURCE_WRITER.to_string()
        ],
        custody_label: APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_CUSTODY_LABEL.to_string(),
        capability_status: APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_CAPABILITY_REQUIRED.to_string(),
        returned: rows.len() as u64,
        transport_required_count: count_rows(
            &rows,
            &AppGameReceiptBoundaryState(
                APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_TRANSPORT_REQUIRED.to_string(),
            ),
        ),
        manual_required_count: count_rows(
            &rows,
            &AppGameReceiptBoundaryState(
                APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_MANUAL_REQUIRED.to_string(),
            ),
        ),
        unavailable_count: count_rows(
            &rows,
            &AppGameReceiptBoundaryState(
                APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_UNAVAILABLE.to_string(),
            ),
        ),
        runtime_transport_executed: false,
        runtime_receipt_ingested: false,
        provider_delivery_executed: false,
        platform_delivery_channel_claimed: false,
        adapter_dispatch_claimed,
        platform_enforcement_claimed,
        raw_private_source_rows_included: false,
        rows,
    }
}

pub(super) fn app_game_child_runtime_transport_receipt_payload(
    read_model: &AppGameChildRuntimeTransportReceiptReadModel,
) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        (
            constants::field::CUSTODY_LABEL,
            LogFieldValue::String(read_model.custody_label.clone()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(read_model.capability_status.clone()),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.returned as f64),
        ),
        (
            constants::field::APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL,
            LogFieldValue::String(serde_json::to_string(read_model).unwrap_or_default()),
        ),
    ])
}
