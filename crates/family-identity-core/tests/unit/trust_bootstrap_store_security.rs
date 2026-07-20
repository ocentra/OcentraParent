use std::fs;
use std::sync::{Arc, Barrier};
use std::thread;

use ocentra_family_identity_core::parent_presence::{
    ParentPresenceStorageFailureReason, ParentPresenceVerificationPort,
};

use super::open_parent_presence_test_port;
use super::trust_bootstrap_store_schema::{TestResult, TestStore};

#[test]
fn first_creation_is_complete_before_publication_and_survives_restart() -> TestResult {
    let store = TestStore::new("atomic-first-publication");
    assert!(!store.path().exists());
    let opened = open_parent_presence_test_port(store.path());
    assert!(
        opened.is_ok(),
        "open failed after publication state: final_exists={} entries={:?}",
        store.path().exists(),
        fs::read_dir(store.path().parent().unwrap_or(store.path()))
            .ok()
            .map(|entries| entries
                .filter_map(Result::ok)
                .map(|entry| entry.file_name())
                .collect::<Vec<_>>())
    );
    let port = opened.map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    assert!(store.path().is_file());
    assert!(
        fs::metadata(store.path())
            .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?
            .len()
            > 0
    );
    assert_no_initialization_artifacts(&store)?;
    drop(port);

    let restarted = open_parent_presence_test_port(store.path())?;
    assert_canonical_schema(store.path())?;
    drop(restarted);
    Ok(())
}

#[test]
fn stale_unpublished_initialization_artifact_cannot_poison_restart() -> TestResult {
    let store = TestStore::new("stale-unpublished-artifact");
    let stale = store
        .path()
        .parent()
        .ok_or(ParentPresenceStorageFailureReason::CustodyUnavailable)?
        .join(".parent-presence.sqlite.initialize-stale-partial");
    let stale_bytes = b"partial initialization that was never published";
    fs::write(&stale, stale_bytes)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;

    let port = open_parent_presence_test_port(store.path())?;
    drop(port);
    assert_eq!(
        fs::read(&stale)
            .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?,
        stale_bytes
    );
    assert_canonical_schema(store.path())
}

#[test]
fn concurrent_first_creators_publish_one_restartable_store() -> TestResult {
    let store = TestStore::new("concurrent-first-creators");
    let path = Arc::new(store.path().to_path_buf());
    let barrier = Arc::new(Barrier::new(5));
    let workers = (0..4)
        .map(|_| {
            let path = Arc::clone(&path);
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();
                open_parent_presence_test_port(path.as_ref()).map(drop)
            })
        })
        .collect::<Vec<_>>();
    barrier.wait();
    for worker in workers {
        let result = worker
            .join()
            .map_err(|_panic| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
        result?;
    }
    assert_canonical_schema(store.path())?;
    assert_no_initialization_artifacts(&store)?;
    let restarted = open_parent_presence_test_port(store.path())?;
    drop(restarted);
    Ok(())
}

#[cfg(unix)]
#[test]
fn unix_test_custody_creates_owner_private_file_and_rejects_permissive_existing_file() -> TestResult
{
    use std::os::unix::fs::{MetadataExt, PermissionsExt};

    let store = TestStore::new("unix-private-permissions");
    let port = open_parent_presence_test_port(store.path())?;
    drop(port);
    let metadata = fs::metadata(store.path())
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    assert_eq!(metadata.uid(), rustix::process::geteuid().as_raw());
    assert_eq!(metadata.mode() & 0o777, 0o600);
    let before = fs::read(store.path())
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o644);
    fs::set_permissions(store.path(), permissions)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;

    assert!(matches!(
        open_parent_presence_test_port(store.path()),
        Err(ParentPresenceStorageFailureReason::CustodyUnavailable)
    ));
    assert_eq!(
        fs::read(store.path())
            .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?,
        before
    );
    Ok(())
}

#[test]
fn production_custody_fails_closed_before_creating_a_path() {
    let store = TestStore::new("production-fail-closed");
    assert!(matches!(
        ParentPresenceVerificationPort::open(store.path()),
        Err(ParentPresenceStorageFailureReason::CustodyUnavailable)
    ));
    assert!(!store.path().exists());
}

#[cfg(windows)]
#[test]
fn windows_custody_pins_final_file_and_every_ancestor_against_substitution() -> TestResult {
    let store = TestStore::new("windows-path-pinning");
    let port = open_parent_presence_test_port(store.path())?;
    let displaced = store.path().with_extension("displaced");
    let final_rename = fs::rename(store.path(), &displaced);
    assert!(matches!(
        final_rename,
        Err(error) if matches!(error.raw_os_error(), Some(5 | 32))
    ));
    let parent = store
        .path()
        .parent()
        .ok_or(ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let displaced_parent = parent.with_extension("displaced-directory");
    let ancestor_rename = fs::rename(parent, &displaced_parent);
    assert!(matches!(
        ancestor_rename,
        Err(error) if matches!(error.raw_os_error(), Some(5 | 32))
    ));
    drop(port);
    assert_canonical_schema(store.path())
}

fn assert_canonical_schema(path: &std::path::Path) -> TestResult {
    let connection = rusqlite::Connection::open(path)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let objects = connection
        .query_row(
            "SELECT COUNT(*) FROM sqlite_schema WHERE name IN ('parent_presence_challenges', 'parent_presence_receipts', 'parent_presence_nonce_identity')",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    assert_eq!(objects, 3);
    Ok(())
}

fn assert_no_initialization_artifacts(store: &TestStore) -> TestResult {
    let parent = store
        .path()
        .parent()
        .ok_or(ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let entries = fs::read_dir(parent)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    for entry in entries {
        let entry =
            entry.map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
        let name = entry.file_name();
        assert!(!name.to_string_lossy().contains(".initialize-"));
    }
    Ok(())
}
