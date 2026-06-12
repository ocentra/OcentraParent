#![forbid(unsafe_code)]

mod read_model;
mod read_model_rows;
mod retention_settings;
mod runtime_flow;

pub const CRATE_NAME: &str = "ocentra-tracking-core";

pub fn evidence_crate_name() -> &'static str {
    ocentra_evidence::CRATE_NAME
}

pub use read_model::tracking_read_model_for_connection;
pub use retention_settings::{
    apply_tracking_retention_settings_write, tracking_retention_settings_durable_store_path,
    TrackingRetentionSettingsWriteAppliedState,
};
pub use runtime_flow::{
    default_child_tracking_runtime_config, default_location_observed_event,
    observe_tracking_location, policy_eligible_child_tracking_runtime_config,
    record_tracking_evidence_from_location, tracking_ai_analysis_request_from_evidence,
    tracking_observation_is_portal_notification_candidate, TrackingRuntimeObservationReport,
};
