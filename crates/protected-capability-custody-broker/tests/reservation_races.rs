#[cfg(not(windows))]
use std::thread;

#[cfg(not(windows))]
use ocentra_protected_capability_custody_broker::{run_service, BrokerError};

#[cfg(not(windows))]
#[test]
fn concurrent_service_entrypoints_do_not_mint_reservation_authority() {
    let first = thread::spawn(run_service);
    let second = thread::spawn(run_service);

    let first_result = first.join().expect("first broker service thread joins");
    let second_result = second.join().expect("second broker service thread joins");

    assert!(matches!(
        first_result,
        Err(BrokerError::UnsupportedPlatform)
    ));
    assert!(matches!(
        second_result,
        Err(BrokerError::UnsupportedPlatform)
    ));
}
