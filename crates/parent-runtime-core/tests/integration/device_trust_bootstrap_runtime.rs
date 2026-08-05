#![cfg(windows)]

use std::time::{Duration, Instant};

use ocentra_parent_runtime_core::device_trust_bootstrap_runtime::ExpiringStagedCeremonies;

#[test]
fn abandoned_staged_ceremonies_are_reaped_while_fresh_handles_remain() {
    let now = Instant::now();
    let mut staged_ceremonies = ExpiringStagedCeremonies::new();
    staged_ceremonies.insert("abandoned".to_owned(), (), now - Duration::from_secs(300));
    staged_ceremonies.insert("fresh".to_owned(), (), now);

    staged_ceremonies.reap_expired(now);

    assert_eq!(staged_ceremonies.remove("abandoned"), None);
    assert_eq!(staged_ceremonies.remove("fresh"), Some(()));
}
