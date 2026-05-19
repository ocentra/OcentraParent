#![forbid(unsafe_code)]

pub mod constants;
mod logging;
mod transport;

pub use logging::*;
pub use transport::*;

pub const CRATE_NAME: &str = "ocentra-parent-agent-protocol";
pub const LOG_SCHEMA_VERSION: u16 = 1;
pub const AGENT_PROTOCOL_SCHEMA_VERSION: u16 = 1;

pub fn crate_name() -> &'static str {
    CRATE_NAME
}

#[cfg(test)]
mod tests;
