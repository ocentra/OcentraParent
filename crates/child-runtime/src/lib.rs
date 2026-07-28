#![forbid(unsafe_code)]

//! Child runtime orchestration ownership boundary.
//!
//! This crate composes child-side feature domains, shared eventing, runtime
//! preflight gates, and parent-to-child command application. Parent runtime and
//! portal UI code must route through this boundary instead of owning child
//! tracking, app/game, browser, LAN, network, screen, AI, policy, notification,
//! storage, remote-access, or enforcement decisions directly.

mod event_flow_scaffold;

pub mod child_domain_runtime_flow;
pub mod policy_control_runtime_flow;
pub mod runtime_gate;
pub mod runtime_gate_tombstone;
pub mod tracking_config_update_flow;
pub mod tracking_runtime_flow;

pub const CRATE_NAME: &str = "ocentra-child-runtime";
