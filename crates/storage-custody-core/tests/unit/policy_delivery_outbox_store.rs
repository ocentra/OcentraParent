use std::{
    io,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};

use ocentra_storage_custody_core::policy_delivery_outbox_store::{
    PolicyDeliveryIntentState, PolicyDeliveryOutboxStore,
};

static DIRECTORY_SEQUENCE: AtomicUsize = AtomicUsize::new(0);

fn temporary_directory(label: &str) -> std::path::PathBuf {
    let directory = std::env::temp_dir().join(format!(
        "ocentra-policy-delivery-{label}-{}-{}",
        std::process::id(),
        DIRECTORY_SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&directory);
    directory
}

#[test]
fn policy_delivery_intent_survives_reopen_and_state_transition(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = temporary_directory("reopen");
    let store = PolicyDeliveryOutboxStore::open(&directory)?;
    store.persist_queued_intent(
        "delivery:one".to_string(),
        "decision:one".to_string(),
        "target:one".to_string(),
        7,
    )?;
    PolicyDeliveryOutboxStore::open(&directory)?
        .mark_delivery_state("delivery:one", PolicyDeliveryIntentState::Offline)?;

    let intents = PolicyDeliveryOutboxStore::open(&directory)?.intents()?;
    assert_eq!(intents.len(), 1);
    assert_eq!(intents[0].delivery_id, "delivery:one");
    assert_eq!(intents[0].policy_decision_ref, "decision:one");
    assert_eq!(intents[0].target_ref, "target:one");
    assert_eq!(intents[0].sequence, 7);
    assert_eq!(intents[0].state, PolicyDeliveryIntentState::Offline);
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[test]
fn duplicate_policy_delivery_intent_is_idempotent_but_conflicting_identity_fails_closed(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = temporary_directory("duplicate");
    let store = PolicyDeliveryOutboxStore::open(&directory)?;
    store.persist_queued_intent(
        "delivery:one".to_string(),
        "decision:one".to_string(),
        "target:one".to_string(),
        1,
    )?;
    store.persist_queued_intent(
        "delivery:one".to_string(),
        "decision:one".to_string(),
        "target:one".to_string(),
        1,
    )?;
    assert_eq!(store.intents()?.len(), 1);

    let error = match store.persist_queued_intent(
        "delivery:one".to_string(),
        "decision:other".to_string(),
        "target:one".to_string(),
        1,
    ) {
        Ok(()) => return Err("conflicting delivery identity unexpectedly persisted".into()),
        Err(error) => error,
    };
    assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[test]
fn concurrent_policy_delivery_intents_are_serialized() -> Result<(), Box<dyn std::error::Error>> {
    let directory = temporary_directory("concurrent");
    let workers = (0..8)
        .map(|index| {
            let directory = directory.clone();
            thread::spawn(move || {
                PolicyDeliveryOutboxStore::open(directory).and_then(|store| {
                    store.persist_queued_intent(
                        format!("delivery:{index}"),
                        format!("decision:{index}"),
                        format!("target:{index}"),
                        index,
                    )
                })
            })
        })
        .collect::<Vec<_>>();
    for worker in workers {
        worker
            .join()
            .map_err(|_| io::Error::other("policy delivery worker panicked"))??;
    }

    let intents = PolicyDeliveryOutboxStore::open(&directory)?.intents()?;
    assert_eq!(intents.len(), 8);
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[test]
fn corrupt_policy_delivery_metadata_fails_closed() -> Result<(), Box<dyn std::error::Error>> {
    let directory = temporary_directory("corrupt");
    let store = PolicyDeliveryOutboxStore::open(&directory)?;
    std::fs::write(directory.join("policy-delivery-outbox.json"), b"not-json")?;

    let error = match store.intents() {
        Ok(_) => return Err("corrupt delivery metadata unexpectedly decoded".into()),
        Err(error) => error,
    };
    assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}
