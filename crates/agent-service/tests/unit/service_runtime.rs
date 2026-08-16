#![forbid(unsafe_code)]

use ocentra_parent_agent_service::network::NetworkPolicy;

fn startup_log_fields(
    network: &NetworkPolicy,
) -> ocentra_parent_agent_protocol::logging::LogFields {
    ocentra_parent_agent_service::service_runtime::startup_log_fields(network)
}

#[path = "service_runtime.rs"]
mod service_runtime_tests;
