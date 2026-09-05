#![cfg(windows)]

use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

use interprocess::os::windows::named_pipe::{pipe_mode, DuplexPipeStream, PipeListenerOptions};
use interprocess::ConnectWaitMode;
use ocentra_parent_logging_local_artifact_windows_ffi::error::ArtifactError;
use ocentra_parent_logging_local_artifact_windows_ffi::owner::{
    DirectoryDurability, LocalArtifactMutation as Mutation, LocalArtifactMutationOwner,
};
use ocentra_parent_logging_local_artifact_windows_ffi::transport::ParentProcessObservation;
use serde_json::json;
use sha2::{Digest, Sha256};

fn temporary_root(label: &str) -> Result<PathBuf, Box<dyn Error>> {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let path = std::env::temp_dir().join(format!(
        "ocentra-local-artifact-{label}-{}-{stamp}",
        std::process::id()
    ));
    fs::create_dir(&path)?;
    Ok(path)
}

fn remove_temporary_root(path: &PathBuf) {
    let _ = fs::remove_dir_all(path);
}

fn required<T>(value: Option<T>, message: &str) -> Result<T, Box<dyn Error>> {
    value.ok_or_else(|| std::io::Error::other(message).into())
}

fn hex_digest(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn request_descriptor(operation: &str, relative_path: &str, payload: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"ocentra.local-artifact-request.v1\0");
    hasher.update(operation.as_bytes());
    hasher.update([0]);
    hasher.update(relative_path.as_bytes());
    hasher.update([0]);
    hasher.update(payload);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[test]
fn root_ancestor_and_target_identity_are_retained() -> Result<(), Box<dyn Error>> {
    let root = temporary_root("identity")?;
    let result = (|| -> Result<(), Box<dyn Error>> {
        let owner = LocalArtifactMutationOwner::open(&root)?;
        let identity = owner.root_identity();
        assert_eq!(identity, owner.root_identity());
        owner.verify_current()?;
        {
            let session = owner.session()?;
            assert_eq!(session.root_identity(), identity);
            assert_eq!(
                session.ensure_directory("a/b")?,
                DirectoryDurability::Synced
            );
            let stat = required(session.stat("a/b")?, "created directory is missing")?;
            assert!(stat.is_directory());
            assert_eq!(
                stat.identity().volume_serial_number(),
                identity.volume_serial_number()
            );
            assert_eq!(session.sync_directory("a/b")?, DirectoryDurability::Synced);
        }
        let renamed = root.with_extension("renamed");
        let rename_error = fs::rename(&root, &renamed).expect_err("held root must reject rename");
        assert_eq!(
            rename_error.raw_os_error(),
            Some(32),
            "held root rename must fail with ERROR_SHARING_VIOLATION",
        );
        owner.verify_current()?;
        Ok(())
    })();
    remove_temporary_root(&root);
    result
}

#[test]
fn append_replace_remove_tree_and_receipt_replay_are_real() -> Result<(), Box<dyn Error>> {
    let root = temporary_root("mutations")?;
    let result = (|| -> Result<(), Box<dyn Error>> {
        let owner = LocalArtifactMutationOwner::open(&root)?;
        fs::write(root.join("remove.txt"), b"remove")?;
        let first = {
            let mut session = owner.session()?;
            session.ensure_directory("scope/tree")?;
            let appended = session.append("append-1", "append.ndjson", b"one\n")?;
            assert!(!appended.replayed());
            let snapshot = required(
                session.read_snapshot("append.ndjson", 1024)?,
                "append snapshot is missing",
            )?;
            assert_eq!(snapshot.bytes(), b"one\n");
            assert_eq!(snapshot.stat().length(), 4);
            let removed = session.remove("remove-1", "remove.txt")?;
            assert!(matches!(
                removed.outcome(),
                ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactMutationOutcome::Removed {
                    existed: true
                }
            ));
            session.replace("replace-1", "replace.txt", b"replacement")?
        };
        assert!(!first.replayed());
        drop(owner);

        let owner = LocalArtifactMutationOwner::open(&root)?;
        let replay = {
            let mut session = owner.session()?;
            session.replace("replace-1", "replace.txt", b"replacement")?
        };
        assert!(replay.replayed());
        let remove_replay = {
            let mut session = owner.session()?;
            session.remove("remove-1", "remove.txt")?
        };
        assert!(remove_replay.replayed());
        assert!(matches!(
            remove_replay.outcome(),
            ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactMutationOutcome::Removed {
                existed: true
            }
        ));
        drop(owner);

        let owner = LocalArtifactMutationOwner::open(&root)?;
        let mut session = owner.session()?;
        let removed = session.remove_tree("tree-1", "scope/tree")?;
        assert!(matches!(removed.outcome(), ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactMutationOutcome::Removed { existed: true }));
        assert!(session.stat("scope/tree")?.is_none());
        Ok(())
    })();
    remove_temporary_root(&root);
    result
}

#[test]
fn mixed_replace_remove_remove_tree_transaction_is_atomic_owner_work() -> Result<(), Box<dyn Error>>
{
    let root = temporary_root("transaction")?;
    let result = (|| -> Result<(), Box<dyn Error>> {
        fs::create_dir(root.join("scope"))?;
        fs::create_dir(root.join("scope/tree"))?;
        fs::write(root.join("scope/replace.txt"), b"old")?;
        fs::write(root.join("scope/remove.txt"), b"remove")?;
        fs::write(root.join("scope/tree/child.txt"), b"child")?;
        let owner = LocalArtifactMutationOwner::open(&root)?;
        let receipt = {
            let mut session = owner.session()?;
            session.apply_transaction(
                "tx-mixed-1",
                &[
                    Mutation::Replace {
                        relative_path: "scope/replace.txt".to_owned(),
                        payload: b"new".to_vec(),
                    },
                    Mutation::Remove {
                        relative_path: "scope/remove.txt".to_owned(),
                    },
                    Mutation::RemoveTree {
                        relative_path: "scope/tree".to_owned(),
                    },
                ],
            )?
        };
        assert!(!receipt.replayed());
        assert_eq!(fs::read(root.join("scope/replace.txt"))?, b"new");
        assert!(!root.join("scope/remove.txt").exists());
        assert!(!root.join("scope/tree").exists());
        Ok(())
    })();
    remove_temporary_root(&root);
    result
}

#[test]
fn single_remove_tree_transaction_returns_durable_removed_receipt_and_replays(
) -> Result<(), Box<dyn Error>> {
    let root = temporary_root("transaction-remove-tree")?;
    let result = (|| -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(root.join("scope/tree"))?;
        fs::write(root.join("scope/tree/child.txt"), b"child")?;
        let mutations = [Mutation::RemoveTree {
            relative_path: "scope/tree".to_owned(),
        }];

        let first = {
            let owner = LocalArtifactMutationOwner::open(&root)?;
            let mut session = owner.session()?;
            session.apply_transaction("tx-tree-1", &mutations)?
        };
        assert!(!first.replayed());
        assert_eq!(first.operation(), "transaction");
        assert_eq!(first.relative_path(), "transaction");
        assert!(matches!(
            first.outcome(),
            ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactMutationOutcome::Removed {
                existed: true
            }
        ));
        assert!(!root.join("scope/tree").exists());

        let replay = {
            let owner = LocalArtifactMutationOwner::open(&root)?;
            let mut session = owner.session()?;
            session.apply_transaction("tx-tree-1", &mutations)?
        };
        assert!(replay.replayed());
        assert_eq!(replay.operation(), "transaction");
        assert_eq!(replay.relative_path(), "transaction");
        assert!(matches!(
            replay.outcome(),
            ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactMutationOutcome::Removed {
                existed: true
            }
        ));
        Ok(())
    })();
    remove_temporary_root(&root);
    result
}

#[test]
fn hardlinks_and_reparse_roots_are_rejected() -> Result<(), Box<dyn Error>> {
    let root = temporary_root("links")?;
    let result = (|| -> Result<(), Box<dyn Error>> {
        let parent = required(root.parent(), "temporary root has no parent")?;
        let name = required(root.file_name(), "temporary root has no name")?.to_string_lossy();
        let outside = parent.join(format!("{name}-outside"));
        fs::create_dir(&outside)?;
        fs::write(outside.join("source.txt"), b"source")?;
        fs::hard_link(outside.join("source.txt"), root.join("hardlink.txt"))?;
        let owner = LocalArtifactMutationOwner::open(&root)?;
        let session = owner.session()?;
        let hardlink = session.read_snapshot("hardlink.txt", 1024);
        assert!(matches!(hardlink, Err(ArtifactError::HardlinkDetected)));
        drop(session);
        drop(owner);

        let symlink = root.join("reparse.txt");
        match std::os::windows::fs::symlink_file(outside.join("source.txt"), &symlink) {
            Ok(()) => {
                let owner = LocalArtifactMutationOwner::open(&root)?;
                let session = owner.session()?;
                let reparse = session.read_snapshot("reparse.txt", 1024);
                assert!(matches!(reparse, Err(ArtifactError::LinkOrReparseDetected)));
            }
            Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {}
            Err(error) => return Err(error.into()),
        }
        let linked_root = parent.join(format!("{name}-linked"));
        match std::os::windows::fs::symlink_dir(&root, &linked_root) {
            Ok(()) => {
                let linked = LocalArtifactMutationOwner::open(&linked_root);
                assert!(
                    matches!(linked, Err(ArtifactError::LinkOrReparseDetected)),
                    "linked root must be rejected as a reparse point: {linked:?}"
                );
            }
            Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {}
            Err(error) => return Err(error.into()),
        }
        let _ = fs::remove_dir_all(&outside);
        Ok(())
    })();
    remove_temporary_root(&root);
    result
}

#[test]
fn application_bridge_is_allowed_but_owner_subtree_and_lock_are_not() -> Result<(), Box<dyn Error>>
{
    let root = temporary_root("boundary")?;
    let result = (|| -> Result<(), Box<dyn Error>> {
        let owner = LocalArtifactMutationOwner::open(&root)?;
        let session = owner.session()?;
        let second_owner = LocalArtifactMutationOwner::open(&root)?;
        let second_session = second_owner.session();
        assert!(matches!(second_session, Err(ArtifactError::LockConflict)));
        drop(second_session);
        drop(second_owner);
        drop(session);
        let owner = LocalArtifactMutationOwner::open(&root)?;
        let mut session = owner.session()?;
        session.append("lifecycle-1", ".bridge/lifecycle-state.json", b"state")?;
        let internal = session.append("internal-1", ".bridge/.mutation-owner/forbidden", b"no");
        assert!(matches!(internal, Err(ArtifactError::InvalidPath(_))));
        Ok(())
    })();
    remove_temporary_root(&root);
    result
}

#[test]
fn interrupted_private_temps_are_reconciled_on_session_start() -> Result<(), Box<dyn Error>> {
    let root = temporary_root("recovery")?;
    let result = (|| -> Result<(), Box<dyn Error>> {
        let owner = LocalArtifactMutationOwner::open(&root)?;
        let _session = owner.session()?;
        drop(_session);
        drop(owner);
        let intents = root.join(".bridge/.mutation-owner/intents");
        let receipts = root.join(".bridge/.mutation-owner/receipts");
        fs::write(intents.join("recovery.stage-0"), b"stale-stage")?;
        fs::write(intents.join("recovery.intent.tmp"), b"stale-intent")?;
        fs::write(receipts.join("recovery.receipt.tmp"), b"stale-receipt")?;
        let owner = LocalArtifactMutationOwner::open(&root)?;
        let _session = owner.session()?;
        assert!(!intents.join("recovery.stage-0").exists());
        assert!(!intents.join("recovery.intent.tmp").exists());
        assert!(!receipts.join("recovery.receipt.tmp").exists());
        Ok(())
    })();
    remove_temporary_root(&root);
    result
}

#[test]
fn interrupted_replace_stage_is_rolled_back_on_restart() -> Result<(), Box<dyn Error>> {
    let root = temporary_root("replace-recovery")?;
    let result = (|| -> Result<(), Box<dyn Error>> {
        fs::create_dir(root.join("scope"))?;
        fs::write(root.join("scope/replaced.txt"), b"old")?;
        let owner = LocalArtifactMutationOwner::open(&root)?;
        let target_identity = {
            let session = owner.session()?;
            required(
                session.stat("scope/replaced.txt")?,
                "replace recovery target is missing",
            )?
            .identity()
        };
        drop(owner);

        let request_id = "replace-recovery-1";
        let relative_path = "scope/replaced.txt";
        let payload = b"new";
        let stage_name = format!("{request_id}.replace.tmp");
        let intents = root.join(".bridge/.mutation-owner/intents");
        fs::write(intents.join(&stage_name), payload)?;
        let intent = json!({
            "kind": "replace",
            "schema": 1,
            "request_id": request_id,
            "relative_path": relative_path,
            "descriptor": request_descriptor("replace", relative_path, payload),
            "payload_digest": hex_digest(payload),
            "temp_name": stage_name,
            "quarantine_name": format!("{request_id}.replace.quarantine"),
            "target_identity": {
                "volume_serial_number": target_identity.volume_serial_number(),
                "file_id": target_identity.file_id()
            },
            "staged_identity": null,
            "phase": "prepared"
        });
        fs::write(
            intents.join(format!("{request_id}.json")),
            serde_json::to_vec(&intent)?,
        )?;

        let owner = LocalArtifactMutationOwner::open(&root)?;
        let mut session = owner.session()?;
        session.recover()?;
        assert_eq!(fs::read(root.join(relative_path))?, b"old");
        assert!(!intents.join(&stage_name).exists());
        assert!(!intents.join(format!("{request_id}.json")).exists());
        assert_eq!(
            required(
                session.read_snapshot(relative_path, 1024)?,
                "replace recovery snapshot is missing",
            )?
            .bytes(),
            b"old"
        );
        Ok(())
    })();
    remove_temporary_root(&root);
    result
}

#[test]
fn retained_parent_process_observation_is_current() -> Result<(), Box<dyn Error>> {
    let observation = ParentProcessObservation::open(std::process::id())?;
    let identity = observation.identity();
    assert_eq!(identity.pid(), std::process::id());
    assert!(observation.is_alive()?);
    assert_eq!(observation.current()?, identity);
    Ok(())
}

#[test]
fn retained_parent_binds_accepted_interprocess_pipe() -> Result<(), Box<dyn Error>> {
    let unique = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let pipe_name = format!(
        r"\\.\pipe\ocentra-local-artifact-ffi-bind-{}-{unique:x}",
        std::process::id()
    );
    let listener = PipeListenerOptions::new()
        .path(pipe_name.as_str())
        .nonblocking(false)
        .accept_remote(false)
        .inheritable(false)
        .create_duplex::<pipe_mode::Bytes>()?;
    let client_name = pipe_name.clone();
    let client = thread::spawn(move || {
        DuplexPipeStream::<pipe_mode::Bytes>::connect_by_path_with_wait_mode(
            client_name.as_str(),
            ConnectWaitMode::Timeout(Duration::from_secs(5)),
        )
    });
    let stream = listener.accept()?;
    stream.set_nonblocking(true)?;
    let observation = ParentProcessObservation::open(std::process::id())?;
    let proof = observation
        .bind_named_pipe_client(&stream)
        .map_err(|_| io::Error::other("accepted interprocess pipe was not parent-bound"))?;
    assert_eq!(proof.client_pid(), std::process::id());
    proof.verify_current()?;
    let _client = client
        .join()
        .map_err(|_| io::Error::other("pipe client thread panicked"))??;
    Ok(())
}

#[test]
fn persisted_receipt_rejects_operation_outcome_mismatch() -> Result<(), Box<dyn Error>> {
    let root = temporary_root("receipt-invariant")?;
    let result = (|| -> Result<(), Box<dyn Error>> {
        let owner = LocalArtifactMutationOwner::open(&root)?;
        let session = owner.session()?;
        drop(session);

        let request_id = "receipt-invariant-1";
        let relative_path = "artifact.txt";
        let payload = b"one";
        let record = json!({
            "schema": 1,
            "request_id": request_id,
            "operation": "append",
            "relative_path": relative_path,
            "descriptor": request_descriptor("append", relative_path, payload),
            "outcome": { "kind": "replaced" }
        });
        fs::write(
            root.join(".bridge/.mutation-owner/receipts")
                .join(format!("{request_id}.json")),
            serde_json::to_vec(&record)?,
        )?;

        let mut session = owner.session()?;
        let error = session
            .append(request_id, relative_path, payload)
            .expect_err("mismatched persisted outcome must fail closed");
        assert_eq!(error, ArtifactError::RecoveryRequired);
        Ok(())
    })();
    remove_temporary_root(&root);
    result
}

#[test]
fn persisted_intent_rejects_inconsistent_append_phase_state() -> Result<(), Box<dyn Error>> {
    let root = temporary_root("intent-invariant")?;
    let result = (|| -> Result<(), Box<dyn Error>> {
        let owner = LocalArtifactMutationOwner::open(&root)?;
        let session = owner.session()?;
        drop(session);

        let request_id = "intent-invariant-1";
        let relative_path = "artifact.txt";
        let payload = b"one";
        let record = json!({
            "kind": "append",
            "schema": 1,
            "request_id": request_id,
            "relative_path": relative_path,
            "descriptor": request_descriptor("append", relative_path, payload),
            "payload_digest": hex_digest(payload),
            "payload_length": payload.len(),
            "prior_length": 0,
            "created": false,
            "target_identity": {
                "volume_serial_number": 1,
                "file_id": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
            },
            "temp_name": format!("{request_id}.append.tmp"),
            "phase": "prepared"
        });
        fs::write(
            root.join(".bridge/.mutation-owner/intents")
                .join(format!("{request_id}.json")),
            serde_json::to_vec(&record)?,
        )?;

        assert!(matches!(
            owner.session(),
            Err(ArtifactError::RecoveryRequired)
        ));
        Ok(())
    })();
    remove_temporary_root(&root);
    result
}

#[test]
fn persisted_receipt_rejects_unknown_fields() -> Result<(), Box<dyn Error>> {
    let root = temporary_root("receipt-schema")?;
    let result = (|| -> Result<(), Box<dyn Error>> {
        let owner = LocalArtifactMutationOwner::open(&root)?;
        let session = owner.session()?;
        drop(session);

        let request_id = "receipt-schema-1";
        let relative_path = "artifact.txt";
        let payload = b"one";
        let record = json!({
            "schema": 1,
            "request_id": request_id,
            "operation": "append",
            "relative_path": relative_path,
            "descriptor": request_descriptor("append", relative_path, payload),
            "outcome": { "kind": "appended", "offset": 0, "length": 3 },
            "unexpected": true
        });
        fs::write(
            root.join(".bridge/.mutation-owner/receipts")
                .join(format!("{request_id}.json")),
            serde_json::to_vec(&record)?,
        )?;

        let mut session = owner.session()?;
        let error = session
            .append(request_id, relative_path, payload)
            .expect_err("unknown persisted receipt fields must fail closed");
        assert_eq!(error, ArtifactError::RecoveryRequired);
        Ok(())
    })();
    remove_temporary_root(&root);
    result
}
