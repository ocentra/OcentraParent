use super::*;

#[test]
fn recovery_reclaims_a_head_claim_after_wall_clock_rollback() -> TestResult {
    let store = DeliveryStore::new("rollback-stale-claim");
    let mut port = store.port()?;
    let journal_path = port.custody_decision_journal_path().to_path_buf();
    issue_challenge(&mut port, "rollback-stale-claim");
    let accepted = port
        .verify_and_consume(input("rollback-stale-claim", "rollback-stale-correlation")?)
        .map(|_accepted| ());
    assert_eq!(accepted, Ok(()));
    drop(port);
    fs::remove_file(&journal_path)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let future_claim = current_time_millis()?.saturating_add(60_000);
    reset_outbox_with_claimed_head(&store.store_path, future_claim)?;

    let store_path = store.store_path.clone();
    let recovery = thread::spawn(move || open_parent_presence_test_port(store_path).is_ok());
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(500);
    while !recovery.is_finished() && std::time::Instant::now() < deadline {
        thread::sleep(std::time::Duration::from_millis(20));
    }
    let recovered_without_wall_clock_catch_up = recovery.is_finished();
    if !recovered_without_wall_clock_catch_up {
        release_all_outbox_claims(&store.store_path)?;
    }

    assert!(recovery.join().unwrap_or(false));
    assert!(recovered_without_wall_clock_catch_up);
    assert_eq!(outbox_state(&store.store_path)?, "delivered");
    let artifacts = decode_artifacts(&journal_entries(&journal_path)?)?;
    assert_eq!(artifacts.len(), 1);
    assert_eq!(
        artifacts[0].correlation_id.as_str(),
        "rollback-stale-correlation"
    );
    Ok(())
}

#[test]
fn isolated_delivery_runtime_waits_for_append_lock_contention() -> TestResult {
    let store = DeliveryStore::new("isolated-runtime-timer");
    let mut port = store.port()?;
    let journal_path = port.custody_decision_journal_path().to_path_buf();
    let lock = hold_journal_append_lock(&journal_path)?;
    issue_challenge(&mut port, "isolated-runtime-timer");
    let release = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(75));
        drop(lock);
    });

    let result = port
        .verify_and_consume(input("isolated-runtime-timer", "timer-correlation")?)
        .map(|_accepted| ());

    release
        .join()
        .map_err(|_panic| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    assert_eq!(result, Ok(()));
    assert_eq!(outbox_state(&store.store_path)?, "delivered");
    assert_eq!(journal_entries(&journal_path)?.len(), 1);
    Ok(())
}

fn journal_append_lock_path(path: &Path) -> PathBuf {
    let mut file_name = path.file_name().map_or_else(
        || std::ffi::OsString::from("journal"),
        std::ffi::OsString::from,
    );
    file_name.push(".append.lock");
    path.with_file_name(file_name)
}

#[cfg(target_os = "windows")]
fn hold_journal_append_lock(path: &Path) -> Result<fs::File, ParentPresenceStorageFailureReason> {
    use std::os::windows::fs::OpenOptionsExt;

    fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .share_mode(0)
        .open(journal_append_lock_path(path))
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)
}

#[cfg(not(target_os = "windows"))]
fn hold_journal_append_lock(path: &Path) -> Result<fs::File, ParentPresenceStorageFailureReason> {
    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(journal_append_lock_path(path))
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    file.try_lock()
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    Ok(file)
}
