#![cfg(windows)]

use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use ocentra_parent_logging_core::local_artifact_mutation::{
    LocalArtifactMutation, LocalArtifactMutationOutcome, LocalArtifactMutationOwner,
};

fn temporary_root() -> Result<PathBuf, Box<dyn Error>> {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let path = std::env::temp_dir().join(format!(
        "ocentra-logging-core-local-artifact-{}-{stamp}",
        std::process::id()
    ));
    fs::create_dir(&path)?;
    Ok(path)
}

#[test]
fn facade_preserves_owner_currentness_and_replayable_mixed_mutation() -> Result<(), Box<dyn Error>>
{
    let root = temporary_root()?;
    let result = (|| -> Result<(), Box<dyn Error>> {
        fs::create_dir(root.join("scope"))?;
        fs::create_dir(root.join("scope/tree"))?;
        fs::write(root.join("scope/remove.txt"), b"remove")?;
        fs::write(root.join("scope/replaced.txt"), b"old")?;
        fs::write(root.join("scope/tree/child.txt"), b"child")?;

        let owner = LocalArtifactMutationOwner::open(&root)?;
        let root_identity = owner.root_identity()?;
        let mut session = owner.session()?;
        assert_eq!(session.root_identity()?, root_identity);
        session.verify_current()?;
        let appended = session.append("facade-append-1", "scope/append.ndjson", b"line\n")?;
        assert!(!appended.replayed());
        assert!(matches!(
            appended.outcome(),
            LocalArtifactMutationOutcome::Appended {
                offset: 0,
                length: 5
            }
        ));
        let snapshot = session
            .read_snapshot("scope/append.ndjson", 1024)?
            .ok_or_else(|| std::io::Error::other("append snapshot is missing"))?;
        assert_eq!(snapshot.bytes(), b"line\n");
        assert_eq!(snapshot.stat().size(), 5);
        assert_eq!(
            snapshot.stat().identity(),
            session
                .stat("scope/append.ndjson")?
                .ok_or_else(|| { std::io::Error::other("append stat is missing") })?
                .identity()
        );

        let committed = session.apply_transaction(
            "facade-transaction-1",
            &[
                LocalArtifactMutation::Replace {
                    relative_path: "scope/replaced.txt".to_owned(),
                    payload: b"new".to_vec(),
                },
                LocalArtifactMutation::Remove {
                    relative_path: "scope/remove.txt".to_owned(),
                },
                LocalArtifactMutation::RemoveTree {
                    relative_path: "scope/tree".to_owned(),
                },
            ],
        )?;
        assert!(matches!(
            committed.outcome(),
            LocalArtifactMutationOutcome::TransactionCommitted { count: 3 }
        ));
        assert!(session.stat("scope/remove.txt")?.is_none());
        assert!(session.stat("scope/tree")?.is_none());
        assert_eq!(
            session
                .read_snapshot("scope/replaced.txt", 1024)?
                .ok_or_else(|| std::io::Error::other("replacement is missing"))?
                .bytes(),
            b"new"
        );
        session.verify_current()?;
        drop(session);
        drop(owner);

        let owner = LocalArtifactMutationOwner::open(&root)?;
        let mut session = owner.session()?;
        let replay = session.append("facade-append-1", "scope/append.ndjson", b"line\n")?;
        assert!(replay.replayed());
        assert_eq!(replay.request_id(), "facade-append-1");
        session.verify_current()?;
        Ok(())
    })();
    let _ = fs::remove_dir_all(&root);
    result
}
