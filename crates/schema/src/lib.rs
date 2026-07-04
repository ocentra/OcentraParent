#![forbid(unsafe_code)]

//! Rust-owned cross-boundary contracts.
//!
//! This crate is the contract authority for shapes that cross runtime,
//! process, host, and UI bridge boundaries. Domain crates own behavior;
//! this crate owns serializable DTO shape.

pub(crate) fn schema_option_or_unreachable<T>(value: Option<T>, context: &str) -> T {
    value.expect(context)
}

pub(crate) fn schema_result_or_unreachable<T, E: std::fmt::Debug>(
    value: Result<T, E>,
    context: &str,
) -> T {
    value.expect(context)
}

pub mod app_game_preview_source_freshness;
pub mod app_game_preview_source_freshness_ts;
pub mod app_game_timer_service_readiness;
pub mod app_game_timer_service_readiness_ts;
pub mod app_risk_detection;
pub mod app_risk_detection_ts;
pub mod billing_contracts_ts;
pub mod browser_policy_control_catalog_ts;
pub mod child_ios_entitlement_capability_proof;
pub mod child_ios_entitlement_capability_proof_ts;
pub mod child_signing_store_device_owner_matrix;
pub mod child_signing_store_device_owner_matrix_ts;
pub mod data_custody_source_of_truth;
pub mod data_custody_source_of_truth_ts;
pub mod encryption_key_custody;
pub mod export_import_backup_recovery;
pub mod logging_contracts;
pub mod logging_contracts_ts;
mod parent_agent_protocol_bridge_ts;
pub mod parent_owned_sync_export;
pub mod parent_owned_sync_export_ts;
pub mod parent_storage_settings_apply_flow;
pub mod parent_storage_settings_apply_flow_ts;
pub mod parent_ui_bridge;
pub mod parent_ui_bridge_ts;
pub mod report_query_custody;
pub mod report_query_custody_ts;
pub mod retention_delete_tombstone;
pub mod setup_device_trust_handoff;
