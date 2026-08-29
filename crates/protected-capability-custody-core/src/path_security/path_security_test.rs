use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use super::{journal, platform, validation, PathSecurityError, PendingSecuredPath};

static NEXT_PROBE: AtomicU64 = AtomicU64::new(1);

fn absolute_fixture() -> PathBuf {
    if cfg!(windows) {
        PathBuf::from(r"C:\ProgramData\Ocentra\ProtectedCapabilityCustody\custody.sqlite")
    } else {
        PathBuf::from("/var/lib/ocentra/protected-capability-custody/custody.sqlite")
    }
}

fn probe_database_path() -> PathBuf {
    let suffix = NEXT_PROBE.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "ocentra-pcc-path-security-{}-{suffix}.sqlite",
        std::process::id()
    ))
}

#[test]
fn path_shape_rejects_relative_uri_and_parent_traversal_forms() {
    assert!(matches!(
        super::reject_unsafe_shape(Path::new("custody.sqlite")),
        Err(PathSecurityError::UnsafePath)
    ));
    assert!(matches!(
        super::reject_unsafe_shape(Path::new(":memory:")),
        Err(PathSecurityError::UnsafePath)
    ));
    assert!(matches!(
        super::reject_unsafe_shape(Path::new("file:custody.sqlite?mode=memory")),
        Err(PathSecurityError::UnsafePath)
    ));

    let traversal = std::env::current_dir()
        .expect("the test worktree must exist")
        .join("..")
        .join("custody.sqlite");
    assert!(matches!(
        validation::components(&traversal),
        Err(PathSecurityError::UnsafePath)
    ));
    assert!(super::reject_unsafe_shape(&absolute_fixture()).is_ok());
}

#[test]
fn tracked_sidecars_are_rejected_while_the_expected_journal_name_is_stable() {
    let database = probe_database_path();
    let journal_path = journal::sidecar(&database, "-journal");
    assert_eq!(
        journal_path,
        PathBuf::from(format!("{}-journal", database.display()))
    );

    let clean = journal::reject_untracked_sidecars(&database);
    assert!(clean.is_ok());

    let wal = journal::sidecar(&database, "-wal");
    let create = std::fs::File::create(&wal);
    assert!(create.is_ok());
    drop(create);
    assert!(matches!(
        journal::reject_untracked_sidecars(&database),
        Err(PathSecurityError::UnsafePath)
    ));
    let _ = std::fs::remove_file(&wal);
    let _ = std::fs::remove_file(&database);
}

#[test]
fn stable_path_support_and_open_fail_closed_by_platform() {
    assert_eq!(platform::stable_sqlite_paths_supported(), cfg!(windows));

    let missing = probe_database_path();
    let result = PendingSecuredPath::open(&missing);
    #[cfg(windows)]
    assert!(matches!(
        result,
        Err(PathSecurityError::Unavailable) | Err(PathSecurityError::UnsafePath)
    ));
    #[cfg(not(windows))]
    assert!(matches!(
        result,
        Err(PathSecurityError::UnsupportedPlatform)
    ));
}
