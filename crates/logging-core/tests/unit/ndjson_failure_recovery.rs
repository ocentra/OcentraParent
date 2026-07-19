use std::{error::Error, fs};

use ocentra_parent_logging_core::{
    ndjson_test_support::{
        append_record_with_fault, publish_artifact_with_forced_fallback, AppendFault,
    },
    ndjson_writer::append_record_for_operation,
};

#[test]
fn ndjson_writer_recovers_from_injected_write_and_sync_failures() {
    let result = ndjson_writer_recovers_from_injected_write_and_sync_failures_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn artifact_fallback_publishes_immutably_and_cleans_temporary_file() {
    let result = artifact_fallback_publishes_immutably_and_cleans_temporary_file_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

fn artifact_fallback_publishes_immutably_and_cleans_temporary_file_impl(
) -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    fs::create_dir_all(&root)?;
    let path = root.join("artifact.log");
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
