use ocentra_parent_agent_protocol::app_game::{
    AppGameInventoryEvidenceRow, APP_GAME_INVENTORY_STATE_ADAPTER_ERROR,
    APP_GAME_INVENTORY_STATE_PERMISSION_LIMITED, APP_GAME_INVENTORY_STATE_UNAVAILABLE,
    APP_GAME_RUNTIME_ADAPTER_ERROR, APP_GAME_RUNTIME_DEGRADED, APP_GAME_RUNTIME_PERMISSION_LIMITED,
    APP_GAME_RUNTIME_RUNNING, APP_GAME_RUNTIME_UNAVAILABLE, APP_GAME_RUNTIME_UNKNOWN,
};
use ocentra_parent_agent_protocol::app_game_child_runtime_transport_receipt::{
    AppGameChildRuntimeTransportReceiptRow,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_PLATFORM_CHANNEL_NOT_PROVED,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_PROVIDER_NOT_EXECUTED,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_RECEIPT_NOT_INGESTED,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_TRANSPORT_NOT_EXECUTED,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_REF_RECEIPT_CONTRACT,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_MANUAL_REQUIRED,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_TRANSPORT_REQUIRED,
    APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_UNAVAILABLE,
};

use super::{AppGameReceiptBoundaryState, AppGameReceiptReferenceIds};

pub(super) fn count_rows(
    rows: &[AppGameChildRuntimeTransportReceiptRow],
    state: &AppGameReceiptBoundaryState,
) -> u64 {
    rows.iter()
        .filter(|row| row.boundary_state == state.0)
        .count() as u64
}

pub(super) fn boundary_state_for_inventory_source(
    row: &AppGameInventoryEvidenceRow,
) -> Option<AppGameReceiptBoundaryState> {
    if row.runtime_state == APP_GAME_RUNTIME_RUNNING {
        return Some(AppGameReceiptBoundaryState(
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_TRANSPORT_REQUIRED.to_string(),
        ));
    }
    if row.inventory_state == APP_GAME_INVENTORY_STATE_UNAVAILABLE {
        return Some(AppGameReceiptBoundaryState(
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_UNAVAILABLE.to_string(),
        ));
    }
    if row.inventory_state == APP_GAME_INVENTORY_STATE_PERMISSION_LIMITED
        || row.inventory_state == APP_GAME_INVENTORY_STATE_ADAPTER_ERROR
    {
        return Some(AppGameReceiptBoundaryState(
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_MANUAL_REQUIRED.to_string(),
        ));
    }
    None
}

pub(super) fn boundary_state_for_runtime_state(
    runtime_state: &AppGameReceiptBoundaryState,
) -> Option<AppGameReceiptBoundaryState> {
    match runtime_state.0.as_str() {
        APP_GAME_RUNTIME_RUNNING => Some(AppGameReceiptBoundaryState(
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_TRANSPORT_REQUIRED.to_string(),
        )),
        APP_GAME_RUNTIME_UNAVAILABLE => Some(AppGameReceiptBoundaryState(
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_UNAVAILABLE.to_string(),
        )),
        APP_GAME_RUNTIME_PERMISSION_LIMITED
        | APP_GAME_RUNTIME_ADAPTER_ERROR
        | APP_GAME_RUNTIME_DEGRADED
        | APP_GAME_RUNTIME_UNKNOWN => Some(AppGameReceiptBoundaryState(
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_MANUAL_REQUIRED.to_string(),
        )),
        _ => None,
    }
}

pub(super) fn required_receipt_refs_for(
    boundary_state: &AppGameReceiptBoundaryState,
) -> AppGameReceiptReferenceIds {
    if boundary_state.0 == APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_TRANSPORT_REQUIRED {
        return AppGameReceiptReferenceIds(vec![
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_REF_RECEIPT_CONTRACT.to_string(),
        ]);
    }
    AppGameReceiptReferenceIds(vec![
        APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_TRANSPORT_NOT_EXECUTED.to_string(),
    ])
}

pub(super) fn open_gaps_for(
    boundary_state: &AppGameReceiptBoundaryState,
) -> AppGameReceiptReferenceIds {
    if boundary_state.0 == APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_UNAVAILABLE {
        return AppGameReceiptReferenceIds(vec![
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_UNAVAILABLE.to_string(),
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_TRANSPORT_NOT_EXECUTED.to_string(),
        ]);
    }
    if boundary_state.0 == APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_MANUAL_REQUIRED {
        return AppGameReceiptReferenceIds(vec![
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_STATE_MANUAL_REQUIRED.to_string(),
            APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_TRANSPORT_NOT_EXECUTED.to_string(),
        ]);
    }
    AppGameReceiptReferenceIds(vec![
        APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_TRANSPORT_NOT_EXECUTED.to_string(),
        APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_RECEIPT_NOT_INGESTED.to_string(),
        APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_PROVIDER_NOT_EXECUTED.to_string(),
        APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_GAP_PLATFORM_CHANNEL_NOT_PROVED.to_string(),
    ])
}
