#[path = "parent_agent_protocol_bridge_defaults_generated.rs"]
mod generated;

use generated::assert_parent_agent_protocol_bridge_defaults;

#[test]
fn generated_agent_protocol_bridge_exposes_transport_defaults() {
    assert_parent_agent_protocol_bridge_defaults();
}

#[test]
fn generated_agent_protocol_domain_artifact_stays_checked_in() {
    assert_parent_agent_protocol_bridge_defaults();
}

#[test]
fn generated_agent_protocol_bridge_exposes_lan_values_from_rust() {
    assert_parent_agent_protocol_bridge_defaults();
}
