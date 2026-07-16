use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::app_game::{
    AppGameInventoryEvidenceRow, AppGameServiceReadModel, APP_GAME_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::app_game_adapter_execution_readiness::{
    APP_GAME_ADAPTER_PRODUCT_NATIVE_APP, APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME,
};
use ocentra_parent_agent_protocol::app_game_authority_classifier::{
    AppGameControlActionResult, APP_GAME_CONTROL_ACTION_STATUS_MANUAL_REQUIRED,
};
use ocentra_parent_agent_protocol::app_game_child_runtime_transport_receipt::{
    AppGameChildRuntimeTransportReceiptRow,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_TRANSPORT_NOT_EXECUTED,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_REF_TRANSPORT_CONTRACT,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_ROW_ID_PREFIX,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_MANUAL_REQUIRED,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_TRANSPORT_REQUIRED,
};

use super::boundary::{
    boundary_state_for_inventory_source, boundary_state_for_runtime_state, open_gaps_for,
    required_receipt_refs_for,
};
use super::{AppGameReceiptBoundaryState, AppGameReceiptReferenceIds, AppGameReceiptSourceRowId};

pub(super) fn child_runtime_rows_from_service_model(
    model: &AppGameServiceReadModel,
) -> Vec<AppGameChildRuntimeTransportReceiptRow> {
    let mut rows = Vec::new();
    for row in &model.running_now_rows {
        if let Some(boundary_state) = boundary_state_for_runtime_state(
            &AppGameReceiptBoundaryState(row.runtime_state.clone()),
        ) {
            push_child_runtime_row(
                &mut rows,
                AppGameReceiptSourceRowId(row.runtime_evidence_id.clone()),
                &boundary_state,
                evidence_reference_ids(&row.evidence),
            );
        }
    }
    for row in &model.foreground_now_rows {
        if let Some(boundary_state) = boundary_state_for_runtime_state(
            &AppGameReceiptBoundaryState(row.runtime_state.clone()),
        ) {
            push_child_runtime_row(
                &mut rows,
                AppGameReceiptSourceRowId(row.foreground_evidence_id.clone()),
                &boundary_state,
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
            AppGameReceiptSourceRowId(row.result_id.clone()),
            &AppGameReceiptBoundaryState(
                APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_MANUAL_REQUIRED.to_string(),
            ),
            AppGameReceiptReferenceIds(
                row.request
                    .evidence_references
                    .iter()
                    .map(|source_row| source_row.evidence_reference_id.clone())
                    .filter(|value| !value.is_empty())
                    .collect(),
            ),
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
            AppGameReceiptSourceRowId(row.inventory_entry_id.clone()),
            &boundary_state,
            evidence_reference_ids(&row.evidence),
        );
    }
}

fn push_child_runtime_row(
    rows: &mut Vec<AppGameChildRuntimeTransportReceiptRow>,
    source_row_id: AppGameReceiptSourceRowId,
    boundary_state: &AppGameReceiptBoundaryState,
    source_refs: AppGameReceiptReferenceIds,
) {
    if rows
        .iter()
        .any(|row| row.source_runtime_writer_row_id == source_row_id.0)
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
    source_row_id: AppGameReceiptSourceRowId,
    boundary_state: &AppGameReceiptBoundaryState,
    required_transport_refs: AppGameReceiptReferenceIds,
) -> AppGameChildRuntimeTransportReceiptRow {
    let mut row_id = String::from(APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_ROW_ID_PREFIX);
    row_id.push_str(&source_row_id.0);

    AppGameChildRuntimeTransportReceiptRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id,
        source_runtime_writer_row_id: source_row_id.0,
        boundary_state: boundary_state.0.clone(),
        product_meanings: vec![
            APP_GAME_ADAPTER_PRODUCT_NATIVE_APP.to_string(),
            APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME.to_string(),
        ],
        required_transport_refs: required_refs_for(boundary_state, required_transport_refs).0,
        required_receipt_refs: required_receipt_refs_for(boundary_state).0,
        open_gaps: open_gaps_for(boundary_state).0,
        runtime_transport_executed: false,
        runtime_receipt_ingested: false,
        provider_delivery_executed: false,
        platform_delivery_channel_claimed: false,
    }
}

fn evidence_reference_ids(evidence: &[ActivityEvidenceRef]) -> AppGameReceiptReferenceIds {
    AppGameReceiptReferenceIds(
        evidence
            .iter()
            .map(|row| row.evidence_id.clone())
            .filter(|value| !value.is_empty())
            .collect(),
    )
}

fn required_refs_for(
    boundary_state: &AppGameReceiptBoundaryState,
    source_refs: AppGameReceiptReferenceIds,
) -> AppGameReceiptReferenceIds {
    if boundary_state.0 == APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_TRANSPORT_REQUIRED {
        return transport_refs(source_refs);
    }
    let mut required_refs = source_refs.0;
    required_refs
        .push(APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_TRANSPORT_NOT_EXECUTED.to_string());
    AppGameReceiptReferenceIds(required_refs)
}

fn transport_refs(source_refs: AppGameReceiptReferenceIds) -> AppGameReceiptReferenceIds {
    if source_refs.0.is_empty() {
        return AppGameReceiptReferenceIds(vec![
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_REF_TRANSPORT_CONTRACT.to_string(),
        ]);
    }
    source_refs
}
