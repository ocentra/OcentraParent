use std::{error::Error, fs};

use ocentra_parent_logging_core::{
    ndjson_interop_test_support::append_record_with_external_interleave,
    ndjson_test_support::{
        forget_operation_compaction_cache, operation_compaction_cache_counts,
        operation_compaction_scan_bytes, operation_state_entry_count,
        record_matches_with_short_reads, replace_operation_state_without_cache_notice,
        seed_operation_compaction_cache,
    },
    ndjson_writer::{append_record_for_operation, remove_record_file_with_operation_state},
};
use sha2::{Digest, Sha256};

#[test]
fn ndjson_operation_repairs_empty_and_partial_final_markers() {
    let result = ndjson_operation_repairs_empty_and_partial_final_markers_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn ndjson_operation_candidate_reader_handles_short_reads() {
    assert!(record_matches_with_short_reads(b"{\"shortRead\":true}\n").unwrap_or(false));
}

#[test]
fn ndjson_operation_compacts_commit_inodes_and_cleans_data_lifecycle_state() {
    let result = ndjson_operation_compacts_commit_inodes_and_cleans_data_lifecycle_state_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn ndjson_operation_indexes_compacted_commits_without_repeated_full_scans() {
    let result = ndjson_operation_indexes_compacted_commits_without_repeated_full_scans_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn ndjson_operation_append_mode_preserves_external_rows_and_exact_replay() {
    let result = ndjson_operation_append_mode_preserves_external_rows_and_exact_replay_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn ndjson_operation_compaction_cache_stays_bounded() {
    let result = ndjson_operation_compaction_cache_stays_bounded_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

fn ndjson_operation_repairs_empty_and_partial_final_markers_impl() -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    fs::create_dir_all(&root)?;
    for (name, extension, marker) in [
        ("empty-intent", "intent", Vec::new()),
        ("partial-commit", "commit", b"partial\nmarker".to_vec()),
    ] {
        let path = root.join(format!("{name}.ndjson"));
        let operation_id = format!("{name}-operation");
        let operation_directory = root.join(format!(".{name}.ndjson.operations"));
        fs::create_dir_all(&operation_directory)?;
        let key = format!("{:x}", Sha256::digest(operation_id.as_bytes()));
        fs::write(
            operation_directory.join(format!("{key}.{extension}")),
            marker,
        )?;
        let record = format!("{{\"repaired\":\"{name}\"}}\n");
        append_record_for_operation(&path, &operation_id, record.as_bytes())?;
        append_record_for_operation(&path, &operation_id, record.as_bytes())?;
        assert_eq!(fs::read(&path)?, record.as_bytes());
    }
    Ok(())
}

fn ndjson_operation_compacts_commit_inodes_and_cleans_data_lifecycle_state_impl(
) -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    fs::create_dir_all(&root)?;
    let path = root.join("bounded-operations.ndjson");
    let records = (0..128)
        .map(|index| format!("{{\"operation\":{index}}}\n"))
        .collect::<Vec<_>>();
    for (index, record) in records.iter().enumerate() {
        append_record_for_operation(&path, &format!("operation-{index}"), record.as_bytes())?;
    }
    assert_eq!(operation_state_entry_count(&path)?, 1);
    let length = fs::metadata(&path)?.len();
    append_record_for_operation(&path, "operation-0", records[0].as_bytes())?;
    append_record_for_operation(&path, "operation-127", records[127].as_bytes())?;
    assert_eq!(fs::metadata(&path)?.len(), length);
    assert_eq!(operation_state_entry_count(&path)?, 1);

    remove_record_file_with_operation_state(&path)?;
    assert!(!path.exists());
    assert_eq!(operation_state_entry_count(&path)?, 0);
    append_record_for_operation(&path, "operation-0", records[0].as_bytes())?;
    assert_eq!(fs::read(&path)?, records[0].as_bytes());
    assert_eq!(operation_state_entry_count(&path)?, 1);
    Ok(())
}

fn ndjson_operation_indexes_compacted_commits_without_repeated_full_scans_impl(
) -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    fs::create_dir_all(&root)?;
    let path = root.join("indexed-operations.ndjson");
    let records = (0..128)
        .map(|index| format!("{{\"indexed\":{index}}}\n"))
        .collect::<Vec<_>>();
    for (index, record) in records.iter().enumerate() {
        append_record_for_operation(&path, &format!("indexed-{index}"), record.as_bytes())?;
    }

    forget_operation_compaction_cache(&path)?;
    append_record_for_operation(&path, "indexed-0", records[0].as_bytes())?;
    let first_scan = operation_compaction_scan_bytes(&path)?;
    append_record_for_operation(&path, "indexed-127", records[127].as_bytes())?;
    assert_eq!(operation_compaction_scan_bytes(&path)?, first_scan);
    assert_eq!(
        first_scan,
        fs::metadata(root.join(".indexed-operations.ndjson.operations/commits.state"))?.len()
    );
    assert!(!root
        .join(".indexed-operations.ndjson.operations/commits.ndjson")
        .exists());

    replace_operation_state_without_cache_notice(&path)?;
    let replacement = b"{\"indexed\":\"replacement\"}\n";
    append_record_for_operation(&path, "indexed-0", replacement)?;
    assert_eq!(fs::read(&path)?, replacement);
    Ok(())
}

fn ndjson_operation_append_mode_preserves_external_rows_and_exact_replay_impl(
) -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    fs::create_dir_all(&root)?;
    let path = root.join("mixed-producers.ndjson");
    let external = b"{\"producer\":\"external\"}\n";
    let operation = b"{\"producer\":\"operation\"}\n";
    append_record_with_external_interleave(&path, "mixed-producer-operation", operation, external)?;
    assert_eq!(
        fs::read(&path)?,
        [external.as_slice(), operation.as_slice()].concat()
    );
    append_record_for_operation(&path, "mixed-producer-operation", operation)?;
    assert_eq!(
        fs::read(&path)?,
        [external.as_slice(), operation.as_slice()].concat()
    );
    Ok(())
}

fn ndjson_operation_compaction_cache_stays_bounded_impl() -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    fs::create_dir_all(&root)?;
    let mut latest = None;
    for path_index in 0..12 {
        let path = root.join(format!("bounded-cache-{path_index}.ndjson"));
        seed_operation_compaction_cache(&path, 5_000)?;
        latest = Some(path);
    }
    let latest = latest.ok_or_else(|| std::io::Error::other("missing cache proof path"))?;
    let (path_count, marker_count) = operation_compaction_cache_counts(&latest)?;
    assert!(path_count <= 8);
    assert!(marker_count <= 4_096);
    Ok(())
}
