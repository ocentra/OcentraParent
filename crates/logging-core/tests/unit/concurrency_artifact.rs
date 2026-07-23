use std::{
    collections::BTreeSet,
    env,
    error::Error,
    fs,
    path::Path,
    process::Command,
    sync::{mpsc, Arc},
    thread,
    time::Duration,
};

use ocentra_parent_logging_core::{
    artifact::{ArtifactKind, ArtifactWriter},
    ndjson_writer::{
        append_record, append_record_for_operation, remove_record_file_with_operation_state,
        NdjsonWriter,
    },
};
use serde_json::json;

#[test]
fn ndjson_writer_keeps_concurrent_records_parseable() {
    let result = ndjson_writer_keeps_concurrent_records_parseable_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn artifact_writer_is_idempotent_and_rejects_conflicts() {
    let result = artifact_writer_is_idempotent_and_rejects_conflicts_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn artifact_writer_is_race_safe_and_rejects_traversal() {
    let result = artifact_writer_is_race_safe_and_rejects_traversal_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn ndjson_writer_subprocess_worker() {
    let result = ndjson_writer_subprocess_worker_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn ndjson_writer_recovers_partial_tail_and_preserves_distinct_identical_records() {
    let result =
        ndjson_writer_recovers_partial_tail_and_preserves_distinct_identical_records_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn ndjson_writer_keeps_large_complete_tail_and_discards_large_partial_tail() {
    let result = ndjson_writer_keeps_large_complete_tail_and_discards_large_partial_tail_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn ndjson_writer_deduplicates_only_matching_operation_identity() {
    let result = ndjson_writer_deduplicates_only_matching_operation_identity_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn ndjson_writer_keeps_subprocess_records_exact_and_terminal() {
    let result = ndjson_writer_keeps_subprocess_records_exact_and_terminal_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn ndjson_operation_state_cleanup_waits_for_the_stream_lock() {
    let result = ndjson_operation_state_cleanup_waits_for_the_stream_lock_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

fn ndjson_operation_state_cleanup_waits_for_the_stream_lock_impl() -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    let writer = Arc::new(NdjsonWriter::new(&root));
    let path = writer.append_event_for_operation(
        "cleanup",
        "events",
        "cleanup-lock",
        &json!({"locked": true}),
    )?;
    let lock_path = path.with_file_name(format!(
        ".{}.operations.lock",
        path.file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| std::io::Error::other("stream path has no UTF-8 name"))?
    ));
    let lock_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(lock_path)?;
    lock_file.lock()?;

    let cleanup_path = path.clone();
    let (result_sender, result_receiver) = mpsc::sync_channel(1);
    let cleanup = thread::spawn(move || {
        let result = remove_record_file_with_operation_state(&cleanup_path);
        let _send_result = result_sender.send(result);
    });
    assert!(result_receiver
        .recv_timeout(Duration::from_millis(100))
        .is_err());
    assert!(path.exists());

    let retry_writer = Arc::clone(&writer);
    let (retry_sender, retry_receiver) = mpsc::sync_channel(1);
    let retry = thread::spawn(move || {
        let result = retry_writer.append_event_for_operation(
            "cleanup",
            "events",
            "cleanup-lock",
            &json!({"locked": true}),
        );
        let _send_result = retry_sender.send(result);
    });
    assert!(retry_receiver
        .recv_timeout(Duration::from_millis(100))
        .is_err());

    lock_file.unlock()?;
    result_receiver.recv_timeout(Duration::from_secs(5))??;
    cleanup
        .join()
        .map_err(|_panic| std::io::Error::other("operation cleanup worker panicked"))?;
    let retried_path = retry_receiver.recv_timeout(Duration::from_secs(5))??;
    retry
        .join()
        .map_err(|_panic| std::io::Error::other("routed retry worker panicked"))?;
    assert_eq!(retried_path, path);
    assert_eq!(fs::read_to_string(path)?, "{\"locked\":true}\n");
    Ok(())
}

fn ndjson_writer_keeps_concurrent_records_parseable_impl() -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    let writer = Arc::new(NdjsonWriter::new(&root));
    let workers = (0..16)
        .map(|worker| {
            let writer = Arc::clone(&writer);
            thread::spawn(move || append_worker_records(writer.as_ref(), worker))
        })
        .collect::<Vec<_>>();
    let mut path = None;
    for worker in workers {
        let result = worker
            .join()
            .map_err(|_error| std::io::Error::other("concurrent writer worker panicked"))?;
        path = Some(result?);
    }
    let path = path.ok_or_else(|| std::io::Error::other("no log path returned"))?;
    let payload = fs::read_to_string(path)?;
    let rows = payload
        .lines()
        .map(serde_json::from_str::<serde_json::Value>)
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(rows.len(), 256);
    Ok(())
}

fn append_worker_records(
    writer: &NdjsonWriter,
    worker: usize,
) -> std::io::Result<std::path::PathBuf> {
    let mut path = None;
    for record in 0..16 {
        path = Some(writer.append_event(
            "concurrency",
            "events",
            &json!({"worker": worker, "record": record}),
        )?);
    }
    path.ok_or_else(|| std::io::Error::other("worker wrote no records"))
}

fn ndjson_writer_subprocess_worker_impl() -> Result<(), Box<dyn Error>> {
    let Some(path) = env::var_os("OCENTRA_NDJSON_SUBPROCESS_PATH") else {
        return Ok(());
    };
    let record = env::var("OCENTRA_NDJSON_SUBPROCESS_RECORD")?;
    append_record(Path::new(&path), record.as_bytes())?;
    Ok(())
}

fn ndjson_writer_recovers_partial_tail_and_preserves_distinct_identical_records_impl(
) -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    fs::create_dir_all(&root)?;
    let path = root.join("recovery.ndjson");
    fs::write(&path, b"{\"partial\":true")?;
    let record = b"{\"id\":\"retry-safe\"}\n";
    append_record(&path, record)?;
    append_record(&path, record)?;
    let payload = fs::read(&path)?;
    assert_eq!(payload, [record.as_slice(), record.as_slice()].concat());
    Ok(())
}

fn ndjson_writer_keeps_large_complete_tail_and_discards_large_partial_tail_impl(
) -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    fs::create_dir_all(&root)?;
    let path = root.join("large-recovery.ndjson");
    let complete = [vec![b'x'; 2 * 1024 * 1024], vec![b'\n']].concat();
    fs::write(&path, &complete)?;
    let appended = b"{\"afterComplete\":true}\n";
    append_record(&path, appended)?;
    assert_eq!(
        fs::metadata(&path)?.len(),
        (complete.len() + appended.len()) as u64
    );

    let retained = b"{\"retained\":true}\n";
    let partial = vec![b'y'; 2 * 1024 * 1024];
    fs::write(&path, [retained.as_slice(), partial.as_slice()].concat())?;
    let recovered = b"{\"afterPartial\":true}\n";
    append_record(&path, recovered)?;
    assert_eq!(
        fs::read(&path)?,
        [retained.as_slice(), recovered.as_slice()].concat()
    );
    Ok(())
}

fn ndjson_writer_deduplicates_only_matching_operation_identity_impl() -> Result<(), Box<dyn Error>>
{
    let root = temp_dir!();
    fs::create_dir_all(&root)?;
    let path = root.join("operations.ndjson");
    let record = b"{\"samePayload\":true}\n";
    append_record_for_operation(&path, "retry-1", record)?;
    append_record_for_operation(&path, "retry-1", record)?;
    append_record_for_operation(&path, "distinct-2", record)?;
    assert_eq!(
        fs::read(&path)?,
        [record.as_slice(), record.as_slice()].concat()
    );
    Ok(())
}

fn ndjson_writer_keeps_subprocess_records_exact_and_terminal_impl() -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    let path = root.join("subprocess.ndjson");
    let executable = env::current_exe()?;
    let expected = (0..16)
        .map(|worker| format!("{{\"subprocessWorker\":{worker}}}\n"))
        .collect::<BTreeSet<_>>();
    let mut children = Vec::new();
    for record in &expected {
        children.push(
            Command::new(&executable)
                .args([
                    "--exact",
                    "concurrency_artifact::ndjson_writer_subprocess_worker",
                    "--nocapture",
                ])
                .env("OCENTRA_NDJSON_SUBPROCESS_PATH", &path)
                .env("OCENTRA_NDJSON_SUBPROCESS_RECORD", record)
                .spawn()?,
        );
    }
    for mut child in children {
        let status = child.wait()?;
        if !status.success() {
            return Err(std::io::Error::other("NDJSON subprocess worker failed").into());
        }
    }
    let payload = fs::read(&path)?;
    assert_eq!(payload.last(), Some(&b'\n'));
    let actual = String::from_utf8(payload)?
        .split_inclusive('\n')
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
    Ok(())
}

fn artifact_writer_is_idempotent_and_rejects_conflicts_impl() -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    let writer = ArtifactWriter::new(&root);
    let first =
        writer.write_text_artifact("scope", "run", "command", ArtifactKind::Stdout, "same")?;
    let replay =
        writer.write_text_artifact("scope", "run", "command", ArtifactKind::Stdout, "same")?;
    assert_eq!(first.sha256, replay.sha256);
    assert_eq!(first.created_at, replay.created_at);
    let conflict = match writer.write_text_artifact(
        "scope",
        "run",
        "command",
        ArtifactKind::Stdout,
        "different",
    ) {
        Ok(_) => return Err(std::io::Error::other("conflicting artifact was accepted").into()),
        Err(error) => error,
    };
    assert_eq!(conflict.kind(), std::io::ErrorKind::AlreadyExists);
    Ok(())
}

fn artifact_writer_is_race_safe_and_rejects_traversal_impl() -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    let writer = Arc::new(ArtifactWriter::new(&root));
    let workers = (0..16)
        .map(|_| {
            let writer = Arc::clone(&writer);
            thread::spawn(move || write_raced_artifact(writer.as_ref()))
        })
        .collect::<Vec<_>>();
    for worker in workers {
        let result = worker
            .join()
            .map_err(|_error| std::io::Error::other("artifact writer worker panicked"))?;
        result?;
    }

    let traversal = match writer.write_text_artifact(
        "../scope",
        "run",
        "command",
        ArtifactKind::Diagnostic,
        "value",
    ) {
        Ok(_) => return Err(std::io::Error::other("traversal path was accepted").into()),
        Err(error) => error,
    };
    assert_eq!(traversal.kind(), std::io::ErrorKind::InvalidInput);

    let temporary_files = fs::read_dir(
        root.join("scope")
            .join("artifacts")
            .join("run")
            .join("command"),
    )?
    .filter_map(Result::ok)
    .filter(|entry| entry.file_name().to_string_lossy().ends_with(".tmp"))
    .count();
    assert_eq!(temporary_files, 0);
    Ok(())
}

fn write_raced_artifact(writer: &ArtifactWriter) -> std::io::Result<()> {
    writer.write_text_artifact(
        "scope",
        "run",
        "command",
        ArtifactKind::Diagnostic,
        "race-safe content",
    )?;
    Ok(())
}
