use std::{error::Error, fs, os::unix::fs::symlink};

use ocentra_parent_logging_core::artifact::{ArtifactKind, ArtifactWriter};

#[test]
fn artifact_writer_rejects_symlinked_artifact_and_metadata_leaves() {
    let result = artifact_writer_rejects_symlinked_artifact_and_metadata_leaves_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

fn artifact_writer_rejects_symlinked_artifact_and_metadata_leaves_impl(
) -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    let directory = root
        .join("scope")
        .join("artifacts")
        .join("run")
        .join("command");
    fs::create_dir_all(&directory)?;
    let external_artifact = root.join("external-artifact.log");
    fs::write(&external_artifact, b"same")?;
    let artifact_path = directory.join("diagnostic.log");
    symlink(&external_artifact, &artifact_path)?;
    let writer = ArtifactWriter::new(&root);
    let artifact_result =
        writer.write_text_artifact("scope", "run", "command", ArtifactKind::Diagnostic, "same");
    assert!(matches!(
        artifact_result,
        Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied
    ));
    assert_eq!(fs::read(&external_artifact)?, b"same");

    fs::remove_file(&artifact_path)?;
    writer.write_text_artifact("scope", "run", "command", ArtifactKind::Diagnostic, "same")?;
    let metadata_path = directory.join("diagnostic.log.metadata.json");
    let external_metadata = root.join("external-metadata.json");
    fs::write(&external_metadata, fs::read(&metadata_path)?)?;
    fs::remove_file(&metadata_path)?;
    symlink(&external_metadata, &metadata_path)?;
    let metadata_result =
        writer.write_text_artifact("scope", "run", "command", ArtifactKind::Diagnostic, "same");
    assert!(matches!(
        metadata_result,
        Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied
    ));
    Ok(())
}
