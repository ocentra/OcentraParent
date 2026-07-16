#![forbid(unsafe_code)]
pub mod ai_boundary;
pub mod alerting;
pub mod child_check_in;
pub mod expected_place;
pub mod generated_bridge;
pub mod geofence;
pub mod local_place;
pub mod local_place_store;
pub mod location_validation;
pub mod missing_device;
pub mod nearby_place;
pub mod parent_acknowledgement;
pub mod read_model;
pub mod read_model_guard;
mod read_model_rows;
mod read_model_rows_aggregate;
pub mod retention_runtime;
pub mod retention_settings;
pub mod runtime_flow;
pub mod status;
pub mod temporary_live;

pub const CRATE_NAME: &str = "ocentra-tracking-core";

pub fn evidence_crate_name() -> &'static str {
    ocentra_evidence::CRATE_NAME
}
