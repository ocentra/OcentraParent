use std::error::Error;

use ocentra_protected_capability_custody_broker::{BrokerError, BROKER_SERVICE_NAME};
use ocentra_protected_capability_custody_protocol::constants;

#[test]
fn account_issuer_uses_the_fixed_broker_service_identity() {
    assert_eq!(BROKER_SERVICE_NAME, constants::BROKER_SERVICE_NAME);
}

#[test]
fn account_issuer_protocol_failures_remain_nested_and_typed() {
    let error = BrokerError::from(
        ocentra_protected_capability_custody_protocol::types::ProtocolError::EmptyFrame,
    );
    assert_eq!(
        error.source().map(ToString::to_string),
        Some(constants::ERROR_EMPTY_FRAME.to_string())
    );
}

#[cfg(not(windows))]
#[test]
fn account_issuer_service_remains_unavailable_without_the_windows_owner() {
    assert!(matches!(
        ocentra_protected_capability_custody_broker::run_service(),
        Err(BrokerError::UnsupportedPlatform)
    ));
}
