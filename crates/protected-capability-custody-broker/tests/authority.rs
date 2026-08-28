use std::error::Error;

use ocentra_protected_capability_custody_broker::{BrokerError, BROKER_SERVICE_NAME};
use ocentra_protected_capability_custody_protocol::constants;

#[cfg(not(windows))]
use ocentra_protected_capability_custody_broker::run_service;

#[test]
fn broker_service_authority_uses_the_fixed_protocol_service_name() {
    assert_eq!(BROKER_SERVICE_NAME, constants::BROKER_SERVICE_NAME);
}

#[test]
fn protocol_failures_remain_nested_as_broker_errors() {
    let error = BrokerError::from(
        ocentra_protected_capability_custody_protocol::types::ProtocolError::EmptyFrame,
    );
    assert!(error.source().is_some());
}

#[cfg(not(windows))]
#[test]
fn broker_authority_fails_closed_when_the_platform_owner_is_unavailable() {
    assert!(matches!(
        run_service(),
        Err(BrokerError::UnsupportedPlatform)
    ));
}
