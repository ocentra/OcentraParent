#![forbid(unsafe_code)]

mod activity;
mod activity_capture;
mod activity_conversions;
mod activity_query;
mod app_game;
mod browser;
mod browser_managed;
mod browser_read_model;
mod browser_values;
pub mod constants;
mod journal;
mod local_ai_runtime;
mod local_ai_runtime_boundary;
mod logging;
mod network_flow;
mod screen_evidence;
mod transport;

pub use activity::*;
pub use activity_capture::*;
pub use activity_query::*;
pub use app_game::*;
pub use browser::*;
pub use browser_managed::*;
pub use browser_read_model::*;
pub use journal::*;
pub use local_ai_runtime::*;
pub use local_ai_runtime_boundary::*;
pub use logging::*;
pub use network_flow::*;
pub use screen_evidence::*;
pub use transport::*;

pub const CRATE_NAME: &str = "ocentra-parent-agent-protocol";
pub const LOG_SCHEMA_VERSION: u16 = 1;
pub const AGENT_PROTOCOL_SCHEMA_VERSION: u16 = 1;
pub const ACTIVITY_SCHEMA_VERSION: u16 = 1;
pub const ACTIVITY_JOURNAL_SCHEMA_VERSION: u16 = 1;
pub const ACTIVITY_QUERY_SCHEMA_VERSION: u16 = 1;
pub const BROWSER_EVIDENCE_SCHEMA_VERSION: u16 = 1;
pub const NETWORK_FLOW_SCHEMA_VERSION: u16 = 1;
pub const SCREEN_EVIDENCE_SCHEMA_VERSION: u16 = 1;

pub fn crate_name() -> &'static str {
    CRATE_NAME
}

#[cfg(test)]
mod activity_query_tests;
#[cfg(test)]
mod activity_tests;
#[cfg(test)]
mod app_game_tests;
#[cfg(test)]
mod browser_managed_tests;
#[cfg(test)]
mod browser_read_model_tests;
#[cfg(test)]
mod journal_tests;
#[cfg(test)]
mod local_ai_runtime_tests;
#[cfg(test)]
mod local_provider_adapter_readiness_tests;
#[cfg(test)]
mod network_flow_tests;
#[cfg(test)]
mod policy_preview_tests;
#[cfg(test)]
mod screen_evidence_tests;
#[cfg(test)]
mod tests;
