#[cfg(not(windows))]
use std::thread;

#[cfg(not(windows))]
use ocentra_protected_capability_custody_broker::{run_service, BrokerError};

#[cfg(not(windows))]
#[test]
fn concurrent_service_entrypoints_do_not_mint_reservation_authority() {
    let first = thread::spawn(run_service);
    let second = thread::spawn(run_service);

    let first_join = first.join();
    assert!(first_join.is_ok());
    let first_result = match first_join {
        Ok(result) => result,
        Err(_) => return,
    };
    let second_join = second.join();
    assert!(second_join.is_ok());
    let second_result = match second_join {
        Ok(result) => result,
        Err(_) => return,
    };

    assert!(matches!(
        first_result,
        Err(BrokerError::UnsupportedPlatform)
    ));
    assert!(matches!(
        second_result,
        Err(BrokerError::UnsupportedPlatform)
    ));
}
