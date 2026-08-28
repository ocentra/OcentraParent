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

#[test]
fn consumed_staged_ceremony_cannot_be_replayed() {
    let now = Instant::now();
    let mut staged_ceremonies = ExpiringStagedCeremonies::new();
    staged_ceremonies.insert("one-shot".to_owned(), (), now);

    assert_eq!(staged_ceremonies.remove("one-shot"), Some(()));
    assert_eq!(staged_ceremonies.remove("one-shot"), None);
}

#[test]
fn a_new_runtime_cache_does_not_restore_staged_ceremonies_after_restart() {
    let now = Instant::now();
    let mut before_restart = ExpiringStagedCeremonies::new();
    before_restart.insert("before-restart".to_owned(), (), now);
    drop(before_restart);

    let mut after_restart = ExpiringStagedCeremonies::new();
    assert_eq!(after_restart.remove("before-restart"), None);
}
