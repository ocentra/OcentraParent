use std::{error::Error, fs, sync::Arc, thread};

use ocentra_parent_logging_core::{
    artifact::{ArtifactKind, ArtifactWriter},
    ndjson_writer::NdjsonWriter,
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

fn artifact_writer_is_idempotent_and_rejects_conflicts_impl() -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    let writer = ArtifactWriter::new(&root);
    let first =
        writer.write_text_artifact("scope", "run", "command", ArtifactKind::Stdout, "same")?;
    let replay =
        writer.write_text_artifact("scope", "run", "command", ArtifactKind::Stdout, "same")?;
    assert_eq!(first.sha256, replay.sha256);
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
