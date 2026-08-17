#![forbid(unsafe_code)]

//! Parent/controller runtime ownership boundary.
//!
//! This crate owns parent desktop/mobile runtime orchestration,
//! controller-side event handling, local assistant handoff, discovery entry
//! points, and parent-visible service state. Child evidence logic must stay in
//! child runtime feature crates.

mod agent_service_client;
pub mod device_trust_bootstrap_runtime;
pub mod parent_ui_bridge;
pub mod policy_control_dispatch;
pub mod policy_control_update_flow;
pub(crate) mod setup_first_run;
pub mod tracking_child_check_in_request_flow;
pub mod tracking_config_update_flow;
pub mod tracking_dispatch;

pub const CRATE_NAME: &str = "ocentra-parent-runtime-core";
