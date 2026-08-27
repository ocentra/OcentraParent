use ocentra_protected_capability_custody_broker::BROKER_SERVICE_NAME;
use ocentra_protected_capability_custody_protocol::constants as protocol_constants;

#[cfg(not(windows))]
use ocentra_protected_capability_custody_broker::run_service;

#[test]
fn broker_service_name_uses_the_protocol_contract() {
    assert_eq!(BROKER_SERVICE_NAME, protocol_constants::BROKER_SERVICE_NAME);
}

#[cfg(not(windows))]
#[test]
fn broker_service_fails_closed_outside_windows() {
    assert!(matches!(
        run_service(),
        Err(ocentra_protected_capability_custody_broker::BrokerError::UnsupportedPlatform)
    ));
}
