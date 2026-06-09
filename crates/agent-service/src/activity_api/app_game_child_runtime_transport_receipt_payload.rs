use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
    AppGameChildRuntimeTransportReceiptReadModel, AppGameChildRuntimeTransportReceiptRow,
    AppGameControlActionResult, AppGameInventoryEvidenceRow, AppGameServiceReadModel,
    LogFieldValue, LogFields, LogLevel, APP_GAME_ADAPTER_PRODUCT_NATIVE_APP,
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
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_ROW_ID_PREFIX,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_MANUAL_REQUIRED,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_TRANSPORT_REQUIRED,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_UNAVAILABLE,
    APP_GAME_CONTROL_ACTION_STATUS_ENFORCED, APP_GAME_CONTROL_ACTION_STATUS_MANUAL_REQUIRED,
    APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED, APP_GAME_INVENTORY_STATE_ADAPTER_ERROR,
    APP_GAME_INVENTORY_STATE_PERMISSION_LIMITED, APP_GAME_INVENTORY_STATE_UNAVAILABLE,
    APP_GAME_RUNTIME_ADAPTER_ERROR, APP_GAME_RUNTIME_DEGRADED, APP_GAME_RUNTIME_PERMISSION_LIMITED,
    APP_GAME_RUNTIME_RUNNING, APP_GAME_RUNTIME_UNAVAILABLE, APP_GAME_RUNTIME_UNKNOWN,
    APP_GAME_SCHEMA_VERSION,
};

use crate::{
    activity_surface_store::load_app_game_model, event_builder::build_event,
    fields::fields_from_pairs, time::timestamp_now,
};

pub async fn build_activity_app_game_child_runtime_transport_receipt_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let generated_at = timestamp_now();
    let read_model = match load_app_game_model().await {
        Some(model) => {
            app_game_child_runtime_transport_receipt_read_model_from_service_model(model)
        }
        None => app_game_child_runtime_transport_receipt_read_model(&generated_at),
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

pub fn app_game_child_runtime_transport_receipt_read_model(
    generated_at: &str,
) -> AppGameChildRuntimeTransportReceiptReadModel {
    read_model_from_rows(generated_at, Vec::new(), false, false)
}

pub fn app_game_child_runtime_transport_receipt_read_model_from_service_model(
    model: AppGameServiceReadModel,
) -> AppGameChildRuntimeTransportReceiptReadModel {
    let adapter_dispatch_claimed = model.approval_action_result_rows.iter().any(|row| {
        row.result_status == APP_GAME_CONTROL_ACTION_STATUS_ENFORCED
            && row.enforcement_result.as_ref().is_some_and(|result| {
                result.status == APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED
            })
    });
    read_model_from_rows(
        &model.generated_at,
        child_runtime_rows_from_service_model(&model),
        adapter_dispatch_claimed,
        adapter_dispatch_claimed,
    )
}

fn read_model_from_rows(
    generated_at: &str,
    rows: Vec<AppGameChildRuntimeTransportReceiptRow>,
    adapter_dispatch_claimed: bool,
    platform_enforcement_claimed: bool,
) -> AppGameChildRuntimeTransportReceiptReadModel {
    AppGameChildRuntimeTransportReceiptReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        read_model_id: APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL_ID.to_string(),
        generated_at: generated_at.to_string(),
        source_read_model_ids: vec![
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_REF_SOURCE_WRITER.to_string()
        ],
        custody_label: APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_CUSTODY_LABEL.to_string(),
        capability_status: APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_CAPABILITY_REQUIRED.to_string(),
        returned: rows.len() as u64,
        transport_required_count: count_rows(
            &rows,
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_TRANSPORT_REQUIRED,
        ),
        manual_required_count: count_rows(
            &rows,
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_MANUAL_REQUIRED,
        ),
        unavailable_count: count_rows(
            &rows,
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_UNAVAILABLE,
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

pub fn app_game_child_runtime_transport_receipt_payload(
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
            LogFieldValue::String(
                serde_json::to_string(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
    ])
}

fn child_runtime_rows_from_service_model(
    model: &AppGameServiceReadModel,
) -> Vec<AppGameChildRuntimeTransportReceiptRow> {
    let mut rows = Vec::new();
    for row in &model.running_now_rows {
        if let Some(boundary_state) = boundary_state_for_runtime_state(&row.runtime_state) {
            push_child_runtime_row(
                &mut rows,
                &row.runtime_evidence_id,
                boundary_state,
                evidence_reference_ids(&row.evidence),
            );
        }
    }
    for row in &model.foreground_now_rows {
        if let Some(boundary_state) = boundary_state_for_runtime_state(&row.runtime_state) {
            push_child_runtime_row(
                &mut rows,
                &row.foreground_evidence_id,
                boundary_state,
                evidence_reference_ids(&row.evidence),
            );
        }
    }
    for row in &model.inventory_rows {
        push_inventory_child_runtime_row(&mut rows, row);
    }
    for row in &model.approval_action_result_rows {
        push_action_result_child_runtime_row(&mut rows, row);
    }
    rows
}

fn push_action_result_child_runtime_row(
    rows: &mut Vec<AppGameChildRuntimeTransportReceiptRow>,
    row: &AppGameControlActionResult,
) {
    if row.result_status == APP_GAME_CONTROL_ACTION_STATUS_MANUAL_REQUIRED
        || row.enforcement_result.is_none()
    {
        push_child_runtime_row(
            rows,
            &row.result_id,
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_MANUAL_REQUIRED,
            row.request
                .evidence_references
                .iter()
                .map(|source_row| source_row.evidence_reference_id.clone())
                .filter(|value| !value.is_empty())
                .collect(),
        );
    }
}

fn push_inventory_child_runtime_row(
    rows: &mut Vec<AppGameChildRuntimeTransportReceiptRow>,
    row: &AppGameInventoryEvidenceRow,
) {
    if let Some(boundary_state) = boundary_state_for_inventory_source(row) {
        push_child_runtime_row(
            rows,
            &row.inventory_entry_id,
            boundary_state,
            evidence_reference_ids(&row.evidence),
        );
    }
}

fn push_child_runtime_row(
    rows: &mut Vec<AppGameChildRuntimeTransportReceiptRow>,
    source_row_id: &str,
    boundary_state: &'static str,
    source_refs: Vec<String>,
) {
    if rows
        .iter()
        .any(|row| row.source_runtime_writer_row_id == source_row_id)
    {
        return;
    }
    rows.push(child_runtime_row(
        source_row_id,
        boundary_state,
        source_refs,
    ));
}

fn child_runtime_row(
    source_row_id: &str,
    boundary_state: &'static str,
    required_transport_refs: Vec<String>,
) -> AppGameChildRuntimeTransportReceiptRow {
    let mut row_id = String::from(APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_ROW_ID_PREFIX);
    row_id.push_str(source_row_id);

    AppGameChildRuntimeTransportReceiptRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id,
        source_runtime_writer_row_id: source_row_id.to_string(),
        boundary_state: boundary_state.to_string(),
        product_meanings: vec![
            APP_GAME_ADAPTER_PRODUCT_NATIVE_APP.to_string(),
            APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME.to_string(),
        ],
        required_transport_refs: required_refs_for(boundary_state, required_transport_refs),
        required_receipt_refs: required_receipt_refs_for(boundary_state),
        open_gaps: open_gaps_for(boundary_state),
        runtime_transport_executed: false,
        runtime_receipt_ingested: false,
        provider_delivery_executed: false,
        platform_delivery_channel_claimed: false,
    }
}

fn count_rows(rows: &[AppGameChildRuntimeTransportReceiptRow], state: &str) -> u64 {
    rows.iter()
        .filter(|row| row.boundary_state == state)
        .count() as u64
}

fn boundary_state_for_inventory_source(row: &AppGameInventoryEvidenceRow) -> Option<&'static str> {
    if row.runtime_state == APP_GAME_RUNTIME_RUNNING {
        return Some(APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_TRANSPORT_REQUIRED);
    }
    if row.inventory_state == APP_GAME_INVENTORY_STATE_UNAVAILABLE {
        return Some(APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_UNAVAILABLE);
    }
    if row.inventory_state == APP_GAME_INVENTORY_STATE_PERMISSION_LIMITED
        || row.inventory_state == APP_GAME_INVENTORY_STATE_ADAPTER_ERROR
    {
        return Some(APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_MANUAL_REQUIRED);
    }
    None
}

fn boundary_state_for_runtime_state(runtime_state: &str) -> Option<&'static str> {
    match runtime_state {
        APP_GAME_RUNTIME_RUNNING => {
            Some(APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_TRANSPORT_REQUIRED)
        }
        APP_GAME_RUNTIME_UNAVAILABLE => {
            Some(APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_UNAVAILABLE)
        }
        APP_GAME_RUNTIME_PERMISSION_LIMITED
        | APP_GAME_RUNTIME_ADAPTER_ERROR
        | APP_GAME_RUNTIME_DEGRADED
        | APP_GAME_RUNTIME_UNKNOWN => {
            Some(APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_MANUAL_REQUIRED)
        }
        _ => None,
    }
}

fn required_refs_for(boundary_state: &str, source_refs: Vec<String>) -> Vec<String> {
    if boundary_state == APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_TRANSPORT_REQUIRED {
        return transport_refs(source_refs);
    }
    let mut required_refs = source_refs;
    required_refs
        .push(APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_TRANSPORT_NOT_EXECUTED.to_string());
    required_refs
}

fn required_receipt_refs_for(boundary_state: &str) -> Vec<String> {
    if boundary_state == APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_TRANSPORT_REQUIRED {
        return vec![APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_REF_RECEIPT_CONTRACT.to_string()];
    }
    vec![APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_TRANSPORT_NOT_EXECUTED.to_string()]
}

fn open_gaps_for(boundary_state: &str) -> Vec<String> {
    if boundary_state == APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_UNAVAILABLE {
        return vec![
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_UNAVAILABLE.to_string(),
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_TRANSPORT_NOT_EXECUTED.to_string(),
        ];
    }
    if boundary_state == APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_MANUAL_REQUIRED {
        return vec![
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_MANUAL_REQUIRED.to_string(),
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_TRANSPORT_NOT_EXECUTED.to_string(),
        ];
    }
    vec![
        APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_TRANSPORT_NOT_EXECUTED.to_string(),
        APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_RECEIPT_NOT_INGESTED.to_string(),
        APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_PROVIDER_NOT_EXECUTED.to_string(),
        APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_PLATFORM_CHANNEL_NOT_PROVED.to_string(),
    ]
}

fn evidence_reference_ids(
    evidence: &[ocentra_parent_agent_protocol::ActivityEvidenceRef],
) -> Vec<String> {
    evidence
        .iter()
        .map(|row| row.evidence_id.clone())
        .filter(|value| !value.is_empty())
        .collect()
}

fn transport_refs(source_refs: Vec<String>) -> Vec<String> {
    if source_refs.is_empty() {
        return vec![APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_REF_TRANSPORT_CONTRACT.to_string()];
    }
    source_refs
}
