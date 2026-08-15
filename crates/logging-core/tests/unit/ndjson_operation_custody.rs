use std::{error::Error, fs, path::Path};

use ocentra_parent_logging_core::ndjson_writer::append_record_for_operation;

#[test]
fn ndjson_operation_rejects_line_breaks_in_public_operation_ids() {
    let result = ndjson_operation_rejects_line_breaks_in_public_operation_ids_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn ndjson_operation_rejects_missing_truncated_and_rotated_committed_records() {
    let result = ndjson_operation_rejects_missing_truncated_and_rotated_committed_records_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

fn ndjson_operation_rejects_line_breaks_in_public_operation_ids_impl() -> Result<(), Box<dyn Error>>
{
    let root = temp_dir!();
    fs::create_dir_all(&root)?;
    for (name, operation_id) in [("lf", "job\n1"), ("cr", "job\r1")] {
        let path = root.join(format!("{name}.ndjson"));
        let error = append_record_for_operation(&path, operation_id, b"{\"safe\":true}\n")
            .err()
            .ok_or_else(|| std::io::Error::other("line-break operation id was accepted"))?;
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
        assert!(!path.exists());
    }
    Ok(())
}

fn ndjson_operation_rejects_missing_truncated_and_rotated_committed_records_impl(
) -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    fs::create_dir_all(&root)?;
    let record = b"{\"custody\":true}\n";

    let deleted = root.join("deleted.ndjson");
    append_record_for_operation(&deleted, "deleted-operation", record)?;
    fs::remove_file(&deleted)?;
    assert_committed_replay_corruption(&deleted, "deleted-operation", record)?;

    let truncated = root.join("truncated.ndjson");
    append_record_for_operation(&truncated, "truncated-operation", record)?;
    fs::OpenOptions::new()
        .write(true)
        .open(&truncated)?
        .set_len(0)?;
    assert_committed_replay_corruption(&truncated, "truncated-operation", record)?;

    let rotated = root.join("rotated.ndjson");
    append_record_for_operation(&rotated, "rotated-operation", record)?;
    fs::rename(&rotated, root.join("rotated.ndjson.previous"))?;
    fs::write(&rotated, b"{\"replacement\":1}\n")?;
    assert_committed_replay_corruption(&rotated, "rotated-operation", record)?;
    Ok(())
}

fn assert_committed_replay_corruption(
    path: &Path,
    operation_id: &str,
    record: &[u8],
) -> Result<(), Box<dyn Error>> {
    let error = append_record_for_operation(path, operation_id, record)
        .err()
        .ok_or_else(|| std::io::Error::other("missing committed record was accepted"))?;
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    assert_eq!(
        error.to_string(),
        "committed operation record is missing or corrupted"
    );
    Ok(())
}
