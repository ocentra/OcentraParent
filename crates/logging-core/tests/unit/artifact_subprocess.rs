use std::{env, error::Error, fs, path::Path, process::Command};

use ocentra_parent_logging_core::artifact::{ArtifactKind, ArtifactRef, ArtifactWriter};

const ROOT_ENV: &str = "OCENTRA_ARTIFACT_SUBPROCESS_ROOT";
const CONTENT_ENV: &str = "OCENTRA_ARTIFACT_SUBPROCESS_CONTENT";
const EXPECT_CONFLICT_ENV: &str = "OCENTRA_ARTIFACT_EXPECT_CONFLICT";

#[test]
fn artifact_writer_subprocess_worker() {
    let result = artifact_writer_subprocess_worker_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn artifact_writer_recovers_across_subprocess_replay_and_rejects_conflicting_publisher() {
    let result =
        artifact_writer_recovers_across_subprocess_replay_and_rejects_conflicting_publisher_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn artifact_writer_rejects_each_corrupted_integrity_metadata_field() {
    let result = artifact_writer_rejects_each_corrupted_integrity_metadata_field_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

fn artifact_writer_subprocess_worker_impl() -> Result<(), Box<dyn Error>> {
    let Some(root) = env::var_os(ROOT_ENV) else {
        return Ok(());
    };
    let content = env::var(CONTENT_ENV)?;
    let expect_conflict = env::var_os(EXPECT_CONFLICT_ENV).is_some();
    let result = ArtifactWriter::new(root).write_text_artifact(
        "subprocess",
        "recovery-run",
        "publisher-command",
        ArtifactKind::Diagnostic,
        &content,
    );
    match (expect_conflict, result) {
        (false, Ok(_)) => Ok(()),
        (true, Err(error)) if error.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
        (true, Ok(_)) => {
            Err(std::io::Error::other("conflicting subprocess publisher was accepted").into())
        }
        (_, Err(error)) => Err(error.into()),
    }
}

fn artifact_writer_recovers_across_subprocess_replay_and_rejects_conflicting_publisher_impl(
) -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    let executable = env::current_exe()?;
    run_artifact_subprocess(&executable, &root, "publisher-a", false)?;

    let artifact_path = root
        .join("subprocess")
        .join("artifacts")
        .join("recovery-run")
        .join("publisher-command")
        .join("diagnostic.log");
    let metadata_path = artifact_path.with_file_name("diagnostic.log.metadata.json");
    let artifact_after_publish = fs::read(&artifact_path)?;
    let metadata_after_publish = fs::read(&metadata_path)?;
    let artifact: ArtifactRef = serde_json::from_slice(&metadata_after_publish)?;
    assert_eq!(artifact_after_publish, b"publisher-a");
    assert_eq!(artifact.run_id, "recovery-run");
    assert_eq!(artifact.command_id, "publisher-command");
    assert_eq!(artifact.kind, ArtifactKind::Diagnostic);
    assert_eq!(artifact.byte_length, artifact_after_publish.len() as u64);

    run_artifact_subprocess(&executable, &root, "publisher-a", false)?;
    assert_eq!(fs::read(&artifact_path)?, artifact_after_publish);
    assert_eq!(fs::read(&metadata_path)?, metadata_after_publish);

    run_artifact_subprocess(&executable, &root, "publisher-b", true)?;
    assert_eq!(fs::read(&artifact_path)?, artifact_after_publish);
    assert_eq!(fs::read(&metadata_path)?, metadata_after_publish);

    let temporary_files = fs::read_dir(artifact_path.parent().ok_or_else(|| {
        std::io::Error::other("published artifact path has no parent directory")
    })?)?
    .filter_map(Result::ok)
    .filter(|entry| entry.file_name().to_string_lossy().ends_with(".tmp"))
    .count();
    assert_eq!(temporary_files, 0);
    Ok(())
}

fn artifact_writer_rejects_each_corrupted_integrity_metadata_field_impl(
) -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    let writer = ArtifactWriter::new(&root);
    writer.write_text_artifact(
        "corruption",
        "run",
        "command",
        ArtifactKind::Stdout,
        "line one\nline two\n",
    )?;
    let metadata_path = root
        .join("corruption")
        .join("artifacts")
        .join("run")
        .join("command")
        .join("stdout.log.metadata.json");
    let original = fs::read(&metadata_path)?;
    let corruptions = [
        ("schemaVersion", serde_json::json!(3)),
        ("eventType", serde_json::json!("wrong-record")),
        ("artifactId", serde_json::json!("artifact-wrong")),
        ("runId", serde_json::json!("wrong-run")),
        ("commandId", serde_json::json!("wrong-command")),
        ("path", serde_json::json!("wrong/path")),
        ("kind", serde_json::json!("stderr")),
        ("sha256", serde_json::json!("wrong-sha")),
        ("byteLength", serde_json::json!(999)),
        ("lineCount", serde_json::json!(999)),
        ("createdAt", serde_json::json!("2020-01-01T00:00:00Z")),
        ("custodySha256", serde_json::json!("wrong-custody-digest")),
    ];
    for (field, replacement) in corruptions {
        let mut metadata: serde_json::Value = serde_json::from_slice(&original)?;
        metadata[field] = replacement;
        fs::write(&metadata_path, serde_json::to_vec(&metadata)?)?;
        let error = writer
            .write_text_artifact(
                "corruption",
                "run",
                "command",
                ArtifactKind::Stdout,
                "line one\nline two\n",
            )
            .err()
            .ok_or_else(|| std::io::Error::other(format!("corrupted {field} was accepted")))?;
        assert_eq!(
            error.kind(),
            std::io::ErrorKind::InvalidData,
            "field={field}"
        );
    }
    fs::write(&metadata_path, &original)?;
    writer.write_text_artifact(
        "corruption",
        "run",
        "command",
        ArtifactKind::Stdout,
        "line one\nline two\n",
    )?;
    Ok(())
}

fn run_artifact_subprocess(
    executable: &Path,
    root: &Path,
    content: &str,
    expect_conflict: bool,
) -> Result<(), Box<dyn Error>> {
    let mut command = Command::new(executable);
    command
        .args([
            "--exact",
            "artifact_subprocess::artifact_writer_subprocess_worker",
            "--nocapture",
        ])
        .env(ROOT_ENV, root)
        .env(CONTENT_ENV, content);
    if expect_conflict {
        command.env(EXPECT_CONFLICT_ENV, "1");
    }
    if !command.status()?.success() {
        return Err(std::io::Error::other("artifact subprocess worker failed").into());
    }
    Ok(())
}
