use std::{error::Error, fs};

use ocentra_parent_logging_core::{
    ndjson_test_support::{
        forget_operation_compaction_cache, operation_compaction_scan_bytes,
        replace_operation_state_without_cache_notice,
    },
    ndjson_writer::append_record_for_operation,
};

#[macro_use]
#[path = "support/mod.rs"]
mod support;

#[test]
fn ndjson_operation_indexes_compacted_commits_without_repeated_full_scans() {
    let result = ndjson_operation_indexes_compacted_commits_without_repeated_full_scans_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
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
