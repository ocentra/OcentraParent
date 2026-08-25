#![forbid(unsafe_code)]

//! Rust-owned cross-boundary contracts.
//!
//! This crate is the contract authority for shapes that cross runtime,
//! process, host, and UI bridge boundaries. Domain crates own behavior;
//! this crate owns serializable DTO shape.

pub(crate) fn schema_option_or_unreachable<T>(value: Option<T>, _context: &str) -> T {
    match value {
        Some(value) => value,
        None => std::process::abort(),
    }
}

pub(crate) fn schema_result_or_unreachable<T, E>(value: Result<T, E>, _context: &str) -> T {
    match value {
        Ok(value) => value,
        Err(_) => std::process::abort(),
    }
}

pub mod account_identity_authority;
pub mod account_identity_authority_producer;
pub mod account_identity_authority_ts;
pub mod activity_event_kind_ts;
pub mod ai_contracts;
pub mod ai_contracts_ts;
pub mod app_game_preview_source_freshness;
pub mod app_game_preview_source_freshness_ts;
pub mod app_game_source_freshness_policy_consumption;
pub mod app_game_source_freshness_policy_consumption_ts;
pub mod app_game_timer_service_readiness;
pub mod app_game_timer_service_readiness_ts;
pub mod app_risk_detection;
pub mod app_risk_detection_ts;
pub mod authenticated_delivery_grant;
pub mod authenticated_delivery_managed_process;
pub mod billing_checkout_portal_boundary_values_ts;
pub mod billing_contracts_ts;
pub mod billing_entitlement_proof;
pub mod billing_entitlement_proof_ts;
pub mod billing_entitlement_runtime_proof;
pub mod billing_entitlement_runtime_proof_schema_ts;
pub mod billing_entitlement_runtime_proof_ts;
pub mod billing_entitlement_values_ts;
pub mod billing_parent_visible_summary;
pub mod billing_parent_visible_summary_ts;
pub mod browser_generated_values_ts;
pub mod browser_policy_control_catalog_ts;
pub mod child_android_proof_contracts_ts;
pub mod child_domain_runtime_events_ts;
pub mod child_ios_entitlement_capability_proof;
pub mod child_ios_entitlement_capability_proof_ts;
pub mod child_service_package_proof_contracts_ts;
pub mod child_signing_store_device_owner_matrix;
pub mod child_signing_store_device_owner_matrix_ts;
pub mod data_custody_source_of_truth;
pub mod data_custody_source_of_truth_ts;
pub mod encryption_key_custody;
pub mod eventing_contracts_ts;
pub mod evidence_kinds_ts;
pub mod export_import_backup_recovery;
pub mod family_reference_primitives_ts;
pub mod family_references_ts;
pub mod logging_contracts;
pub mod logging_contracts_ts;
pub mod managed_browser_cdp_capture;
pub mod notification_local_outbox_ts;
pub mod notification_v3_provider_retry_ts;
mod parent_agent_protocol_bridge_ts;
pub mod parent_control_capabilities_ts;
pub mod parent_control_capability_data_ts;
pub mod parent_owned_sync_export;
pub mod parent_owned_sync_export_ts;
pub mod parent_step_up_receipt;
pub mod parent_storage_settings_apply_flow;
pub mod parent_storage_settings_apply_flow_ts;
pub mod parent_ui_bridge;
pub mod parent_ui_bridge_ts;
pub mod phone_qr_approval;
pub mod policy_enforcement_ts;
pub mod remote_capability_fabric;
pub mod report_query_custody;
pub mod report_query_custody_ts;
pub mod retention_delete_tombstone;
pub mod setup_device_trust_handoff;
pub mod tracking_event_contracts;
pub mod typescript_literal;
pub mod v0_8_notification_provider_status_boundary_ts;
