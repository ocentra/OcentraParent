use std::{error::Error, fs};

use ocentra_parent_logging_core::{
    artifact_test_support::{
        publish_artifact_after_stale_temporary, publish_artifact_with_forced_fallback,
        publish_artifact_with_forced_fallback_fault, publish_artifact_with_hard_link_fault,
        publish_artifact_with_parent_sync_fault, ArtifactFallbackFault, HardLinkFault,
    },
    ndjson_test_support::{
        append_plain_record_with_external_after_sync_fault, append_plain_record_with_sync_fault,
        append_record_with_fault, append_record_with_marker_fault, AppendFault,
        OperationMarkerFault,
    },
    ndjson_writer::{append_record, append_record_for_operation},
};

#[test]
fn ndjson_writer_recovers_from_injected_write_and_sync_failures() {
    let result = ndjson_writer_recovers_from_injected_write_and_sync_failures_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn ndjson_operation_recovers_from_partial_intent_and_commit_markers() {
    let result = ndjson_operation_recovers_from_partial_intent_and_commit_markers_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn artifact_fallback_publishes_immutably_and_cleans_temporary_file() {
    let result = artifact_fallback_publishes_immutably_and_cleans_temporary_file_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn artifact_existing_replay_removes_owned_stale_temporary() {
    let result = artifact_existing_replay_removes_owned_stale_temporary_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn artifact_fallback_failure_removes_partial_destination_and_allows_retry() {
    let result = artifact_fallback_failure_removes_partial_destination_and_allows_retry_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn artifact_fallback_fault_does_not_delete_preexisting_artifact() {
    let result = artifact_fallback_fault_does_not_delete_preexisting_artifact_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn artifact_replay_retries_parent_directory_sync_before_success() {
    let result = artifact_replay_retries_parent_directory_sync_before_success_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn artifact_hard_link_fallback_handles_permission_errors_but_not_conflicts() {
    let result = artifact_hard_link_fallback_handles_permission_errors_but_not_conflicts_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn artifact_copy_fallback_recovers_owned_partial_temporary_after_process_death() {
    let result = artifact_copy_fallback_recovers_owned_partial_temporary_after_process_death_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

fn artifact_hard_link_fallback_handles_permission_errors_but_not_conflicts_impl(
) -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    fs::create_dir_all(&root)?;
    let fallback_path = root.join("permission-fallback.log");
    publish_artifact_with_hard_link_fault(
        &fallback_path,
        b"permission fallback content",
        HardLinkFault::PermissionDenied,
    )?;
    assert_eq!(fs::read(&fallback_path)?, b"permission fallback content");

    let conflict_path = root.join("already-exists.log");
    let conflict = expected_artifact_error(publish_artifact_with_hard_link_fault(
        &conflict_path,
        b"must not publish",
        HardLinkFault::AlreadyExists,
    ))?;
    assert_eq!(conflict.kind(), std::io::ErrorKind::AlreadyExists);
    assert!(!conflict_path.exists());
    Ok(())
}

fn artifact_copy_fallback_recovers_owned_partial_temporary_after_process_death_impl(
) -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    fs::create_dir_all(&root)?;
    let path = root.join("crash-recovery.log");
    let error = expected_artifact_error(publish_artifact_with_forced_fallback_fault(
        &path,
        b"complete artifact after restart",
        ArtifactFallbackFault::Crash,
    ))?;
    assert_eq!(
        error.to_string(),
        "injected artifact fallback process death"
    );
    assert!(!path.exists());
    assert!(root.join(".crash-recovery.log.copy.tmp").exists());

    publish_artifact_with_forced_fallback(&path, b"complete artifact after restart")?;
    assert_eq!(fs::read(&path)?, b"complete artifact after restart");
    assert!(!root.join(".crash-recovery.log.copy.tmp").exists());
    Ok(())
}

fn artifact_fallback_failure_removes_partial_destination_and_allows_retry_impl(
) -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    fs::create_dir_all(&root)?;
    for (name, fault, message) in [
        (
            "copy",
            ArtifactFallbackFault::Copy,
            "injected artifact fallback copy failure",
        ),
        (
            "sync",
            ArtifactFallbackFault::Sync,
            "injected artifact fallback sync failure",
        ),
    ] {
        let path = root.join(format!("{name}-failure.log"));
        let error = expected_artifact_error(publish_artifact_with_forced_fallback_fault(
            &path,
            b"retry-safe artifact content",
            fault,
        ))?;
        assert_eq!(error.kind(), std::io::ErrorKind::Other);
        assert_eq!(error.to_string(), message);
        assert_eq!(
            fs::metadata(&path).err().map(|error| error.kind()),
            Some(std::io::ErrorKind::NotFound)
        );
        publish_artifact_with_forced_fallback(&path, b"retry-safe artifact content")?;
        assert_eq!(fs::read(&path)?, b"retry-safe artifact content");
    }

    Ok(())
}

fn artifact_fallback_fault_does_not_delete_preexisting_artifact_impl() -> Result<(), Box<dyn Error>>
{
    let root = temp_dir!();
    fs::create_dir_all(&root)?;
    let existing_path = root.join("existing-artifact.log");
    publish_artifact_with_forced_fallback(&existing_path, b"valid existing artifact")?;
    let conflict = expected_artifact_error(publish_artifact_with_forced_fallback_fault(
        &existing_path,
        b"conflicting artifact",
        ArtifactFallbackFault::Copy,
    ))?;
    assert_eq!(conflict.kind(), std::io::ErrorKind::AlreadyExists);
    assert_eq!(fs::read(&existing_path)?, b"valid existing artifact");
    Ok(())
}

fn artifact_replay_retries_parent_directory_sync_before_success_impl() -> Result<(), Box<dyn Error>>
{
    let root = temp_dir!();
    fs::create_dir_all(&root)?;
    let path = root.join("parent-sync-retry.log");
    for _attempt in 0..2 {
        let error = expected_artifact_error(publish_artifact_with_parent_sync_fault(
            &path,
            b"durable after replay",
        ))?;
        assert_eq!(error.kind(), std::io::ErrorKind::Other);
        assert_eq!(error.to_string(), "injected parent directory sync failure");
        assert_eq!(fs::read(&path)?, b"durable after replay");
    }
    publish_artifact_with_forced_fallback(&path, b"durable after replay")?;
    Ok(())
}

fn artifact_fallback_publishes_immutably_and_cleans_temporary_file_impl(
) -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    fs::create_dir_all(&root)?;
    let path = root.join("artifact.log");
    publish_artifact_after_stale_temporary(&path, b"fallback content")?;
    assert_eq!(fs::read(&path)?, b"fallback content");
    fs::remove_file(&path)?;
    publish_artifact_with_forced_fallback(&path, b"fallback content")?;
    publish_artifact_with_forced_fallback(&path, b"fallback content")?;
    let conflict = match publish_artifact_with_forced_fallback(&path, b"different") {
        Ok(()) => {
            return Err(std::io::Error::other("fallback overwrote immutable artifact").into())
        }
        Err(error) => error,
    };
    assert_eq!(conflict.kind(), std::io::ErrorKind::AlreadyExists);
    assert_eq!(fs::read(&path)?, b"fallback content");
    let temporary_files = fs::read_dir(&root)?
        .filter_map(Result::ok)
        .filter(|entry| entry.file_name().to_string_lossy().ends_with(".tmp"))
        .count();
    assert_eq!(temporary_files, 0);
    Ok(())
}

fn artifact_existing_replay_removes_owned_stale_temporary_impl() -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    fs::create_dir_all(&root)?;
    let path = root.join("existing-replay.log");
    publish_artifact_with_forced_fallback(&path, b"immutable replay")?;
    let temporary = root.join(".existing-replay.log.tmp");
    fs::write(&temporary, b"crash leftover")?;

    publish_artifact_with_forced_fallback(&path, b"immutable replay")?;

    assert_eq!(fs::read(&path)?, b"immutable replay");
    assert!(!temporary.exists());
    Ok(())
}

fn ndjson_writer_recovers_from_injected_write_and_sync_failures_impl() -> Result<(), Box<dyn Error>>
{
    let root = temp_dir!();
    fs::create_dir_all(&root)?;
    let path = root.join("failure-retry.ndjson");
    let record = b"{\"recovery\":true}\n";
    let write = injected_error(append_record_with_fault(
        &path,
        "write-retry",
        record,
        AppendFault::Write,
    ))?;
    assert_eq!(write.kind(), std::io::ErrorKind::Other);
    assert_eq!(write.to_string(), "injected NDJSON append failure");
    append_record_for_operation(&path, "write-retry", record)?;
    let sync = injected_error(append_record_with_fault(
        &path,
        "sync-retry",
        record,
        AppendFault::Sync,
    ))?;
    assert_eq!(sync.kind(), std::io::ErrorKind::Other);
    assert_eq!(sync.to_string(), "injected NDJSON append failure");
    append_record_for_operation(&path, "sync-retry", record)?;
    assert_eq!(
        fs::read(&path)?,
        [record.as_slice(), record.as_slice()].concat()
    );
    let plain_path = root.join("plain-sync-retry.ndjson");
    let plain_sync = injected_error(append_plain_record_with_sync_fault(&plain_path, record))?;
    assert_eq!(plain_sync.kind(), std::io::ErrorKind::Other);
    assert_eq!(plain_sync.to_string(), "injected NDJSON sync failure");
    assert_eq!(fs::read(&plain_path)?, b"");
    append_record(&plain_path, record)?;
    assert_eq!(fs::read(&plain_path)?, record);
    let mixed_path = root.join("mixed-sync-failure.ndjson");
    let external = b"{\"external\":true}\n";
    let mixed_sync = injected_error(append_plain_record_with_external_after_sync_fault(
        &mixed_path,
        record,
        external,
    ))?;
    assert_eq!(mixed_sync.kind(), std::io::ErrorKind::Other);
    assert_eq!(
        fs::read(&mixed_path)?,
        [record.as_slice(), external.as_slice()].concat()
    );
    Ok(())
}

fn ndjson_operation_recovers_from_partial_intent_and_commit_markers_impl(
) -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    fs::create_dir_all(&root)?;
    for (name, fault) in [
        ("intent-write", OperationMarkerFault::IntentWrite),
        ("intent-sync", OperationMarkerFault::IntentSync),
        ("commit-write", OperationMarkerFault::CommitWrite),
        ("commit-sync", OperationMarkerFault::CommitSync),
    ] {
        let path = root.join(format!("{name}.ndjson"));
        let operation_id = format!("{name}-operation");
        let record = format!("{{\"markerFault\":\"{name}\"}}\n");
        let failure = expected_artifact_error(append_record_with_marker_fault(
            &path,
            &operation_id,
            record.as_bytes(),
            fault,
        ))?;
        assert_eq!(failure.kind(), std::io::ErrorKind::Other);
        append_record_for_operation(&path, &operation_id, record.as_bytes())?;
        append_record_for_operation(&path, &operation_id, record.as_bytes())?;
        assert_eq!(fs::read(&path)?, record.as_bytes());
    }
    Ok(())
}

fn injected_error(result: std::io::Result<()>) -> std::io::Result<std::io::Error> {
    match result {
        Ok(()) => Err(std::io::Error::other(
            "injected append failure was not observed",
        )),
        Err(error) => Ok(error),
    }
}

fn expected_artifact_error(result: std::io::Result<()>) -> std::io::Result<std::io::Error> {
    match result {
        Ok(()) => Err(std::io::Error::other(
            "injected artifact fallback failure was not observed",
        )),
        Err(error) => Ok(error),
    }
}
