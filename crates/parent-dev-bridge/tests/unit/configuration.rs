use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_dev_bridge::{
    parent_dev_bridge_address_from_configuration, parent_dev_bridge_log_fields,
    ParentDevBridgeFailure,
};
use ocentra_parent_logging_core::field::LogFieldValue;

#[test]
fn configured_parent_dev_bridge_defaults_to_loopback() {
    let address = parent_dev_bridge_address_from_configuration(Some(4491), false);

    assert_eq!(
        address.map(|value| value.to_string()),
        Some("127.0.0.1:4491".to_string())
    );
}

#[test]
fn configured_parent_dev_bridge_uses_wildcard_for_lan_mode() {
    let address = parent_dev_bridge_address_from_configuration(Some(4491), true);

    assert_eq!(
        address.map(|value| value.to_string()),
        Some("0.0.0.0:4491".to_string())
    );
}

#[test]
fn configured_parent_dev_bridge_rejects_missing_or_invalid_ports() {
    assert_eq!(
        parent_dev_bridge_address_from_configuration(None, false),
        None
    );
}

#[test]
fn parent_dev_bridge_log_fields_include_bridge_address_when_present() {
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 4491);
    let fields = parent_dev_bridge_log_fields(
        Some(address),
        &ParentDevBridgeFailure::from_display(constants::error::PARENT_DEV_BRIDGE_RUNS),
    );

    assert_eq!(
        fields.get(constants::field::LOCAL_PORT),
        Some(&LogFieldValue::Number(4491.0))
    );
    assert_eq!(
        fields.get(constants::field::BRIDGE_ENDPOINT_REF),
        Some(&LogFieldValue::String(address.to_string()))
    );
    assert_eq!(
        fields.get(constants::field::REASON),
        Some(&LogFieldValue::String(
            constants::error::PARENT_DEV_BRIDGE_RUNS.to_string()
        ))
    );
}

#[test]
fn parent_dev_bridge_log_fields_omit_address_fields_when_absent() {
    let fields = parent_dev_bridge_log_fields(
        None,
        &ParentDevBridgeFailure::from_display(constants::error::PARENT_DEV_BRIDGE_RUNS),
    );

    assert_eq!(fields.get(constants::field::LOCAL_PORT), None);
    assert_eq!(fields.get(constants::field::BRIDGE_ENDPOINT_REF), None);
    assert_eq!(
        fields.get(constants::field::REASON),
        Some(&LogFieldValue::String(
            constants::error::PARENT_DEV_BRIDGE_RUNS.to_string()
        ))
    );
}
