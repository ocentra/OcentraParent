#![forbid(unsafe_code)]

mod activity;
mod activity_capture;
mod activity_conversions;
mod activity_query;
mod browser;
pub mod constants;
mod journal;
mod logging;
mod network_flow;
mod transport;

pub use activity::*;
pub use activity_capture::*;
pub use activity_query::*;
pub use browser::*;
pub use journal::*;
pub use logging::*;
pub use network_flow::*;
pub use transport::*;

pub const CRATE_NAME: &str = "ocentra-parent-agent-protocol";
pub const LOG_SCHEMA_VERSION: u16 = 1;
pub const AGENT_PROTOCOL_SCHEMA_VERSION: u16 = 1;
pub const ACTIVITY_SCHEMA_VERSION: u16 = 1;
pub const ACTIVITY_JOURNAL_SCHEMA_VERSION: u16 = 1;
pub const ACTIVITY_QUERY_SCHEMA_VERSION: u16 = 1;
pub const BROWSER_EVIDENCE_SCHEMA_VERSION: u16 = 1;

pub fn crate_name() -> &'static str {
    CRATE_NAME
}

#[cfg(test)]
mod activity_query_tests;
#[cfg(test)]
mod activity_tests;
#[cfg(test)]
mod journal_tests;
#[cfg(test)]
mod network_flow_tests;
#[cfg(test)]
mod tests;
