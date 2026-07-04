use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Mutex, OnceLock};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_dev_bridge::{
    configured_parent_dev_bridge_address, parent_dev_bridge_log_fields, ParentDevBridgeFailure,
};
use ocentra_parent_logging_core::field::LogFieldValue;

fn parent_dev_bridge_env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

#[test]
fn configured_parent_dev_bridge_defaults_to_loopback() {
    let _guard = parent_dev_bridge_env_lock().lock().expect("env lock");
    unsafe {
        std::env::set_var(constants::env_var::PARENT_DEV_BRIDGE_PORT, "4491");
        std::env::remove_var(constants::env_var::DEV_NETWORK_MODE);
    }
    let address = configured_parent_dev_bridge_address();
    unsafe {
        std::env::remove_var(constants::env_var::PARENT_DEV_BRIDGE_PORT);
    }

    assert_eq!(
        address.map(|value| value.to_string()),
        Some("127.0.0.1:4491".to_string())
    );
}

#[test]
fn configured_parent_dev_bridge_uses_wildcard_for_lan_mode() {
    let _guard = parent_dev_bridge_env_lock().lock().expect("env lock");
    unsafe {
        std::env::set_var(constants::env_var::PARENT_DEV_BRIDGE_PORT, "4491");
        std::env::set_var(
            constants::env_var::DEV_NETWORK_MODE,
            constants::value::LOCAL_NETWORK_MODE,
        );
    }
    let address = configured_parent_dev_bridge_address();
    unsafe {
        std::env::remove_var(constants::env_var::PARENT_DEV_BRIDGE_PORT);
        std::env::remove_var(constants::env_var::DEV_NETWORK_MODE);
    }

    assert_eq!(
        address.map(|value| value.to_string()),
        Some("0.0.0.0:4491".to_string())
    );
}

#[test]
fn configured_parent_dev_bridge_rejects_invalid_ports() {
    let _guard = parent_dev_bridge_env_lock().lock().expect("env lock");
    unsafe {
        std::env::set_var(constants::env_var::PARENT_DEV_BRIDGE_PORT, "not-a-port");
        std::env::remove_var(constants::env_var::DEV_NETWORK_MODE);
    }
    assert_eq!(configured_parent_dev_bridge_address(), None);
    unsafe {
        std::env::remove_var(constants::env_var::PARENT_DEV_BRIDGE_PORT);
    }
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
