use std::{error::Error, fs};

use ocentra_parent_logging_core::ndjson_writer::{
    append_record, append_record_for_operation, NdjsonWriter,
};
use serde::{ser::SerializeMap, Serialize, Serializer};
use serde_json::json;

#[test]
fn ndjson_writer_rejects_non_json_and_multi_record_raw_appends() {
    let result = ndjson_writer_rejects_non_json_and_multi_record_raw_appends_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn ndjson_writer_preserves_operation_identity_across_daily_stream_rollover() {
    let result = ndjson_writer_preserves_operation_identity_across_daily_stream_rollover_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

#[test]
fn ndjson_writer_canonicalizes_equivalent_operation_events_before_deduplication() {
    let result =
        ndjson_writer_canonicalizes_equivalent_operation_events_before_deduplication_impl();
    assert!(matches!(result, Ok(())), "{result:?}");
}

fn ndjson_writer_canonicalizes_equivalent_operation_events_before_deduplication_impl(
) -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    let writer = NdjsonWriter::new(&root);
    let first = OrderedFields::new([("alpha", 1), ("beta", 2)]);
    let second = OrderedFields::new([("beta", 2), ("alpha", 1)]);

    let path =
        writer.append_event_for_operation("canonical", "events", "same-operation", &first)?;
    let replay =
        writer.append_event_for_operation("canonical", "events", "same-operation", &second)?;

    assert_eq!(replay, path);
    assert_eq!(fs::read_to_string(path)?, "{\"alpha\":1,\"beta\":2}\n");
    Ok(())
}

struct OrderedFields(Vec<(&'static str, u8)>);

impl OrderedFields {
    fn new<const N: usize>(fields: [(&'static str, u8); N]) -> Self {
        Self(fields.into())
    }
}

impl Serialize for OrderedFields {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (key, value) in &self.0 {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}

fn ndjson_writer_rejects_non_json_and_multi_record_raw_appends_impl() -> Result<(), Box<dyn Error>>
{
    let root = temp_dir!();
    fs::create_dir_all(&root)?;
    for (index, record) in [
        b"not-json\n".as_slice(),
        b"{}\n{}\n".as_slice(),
        b"\n".as_slice(),
        b"{\"windows\":true}\r\n".as_slice(),
    ]
    .into_iter()
    .enumerate()
    {
        assert_rejected_raw_record(&root, index, record)?;
    }
    Ok(())
}

fn assert_rejected_raw_record(
    root: &std::path::Path,
    index: usize,
    record: &[u8],
) -> Result<(), Box<dyn Error>> {
    let plain_path = root.join(format!("invalid-plain-{index}.ndjson"));
    let plain = append_record(&plain_path, record);
    assert!(
        matches!(plain, Err(error) if error.kind() == std::io::ErrorKind::InvalidInput),
        "record={record:?}"
    );
    assert!(!plain_path.exists());

    let operation_path = root.join(format!("invalid-operation-{index}.ndjson"));
    let operation =
        append_record_for_operation(&operation_path, &format!("invalid-{index}"), record);
    assert!(
        matches!(operation, Err(error) if error.kind() == std::io::ErrorKind::InvalidInput),
        "record={record:?}"
    );
    assert!(!operation_path.exists());
    Ok(())
}

fn ndjson_writer_preserves_operation_identity_across_daily_stream_rollover_impl(
) -> Result<(), Box<dyn Error>> {
    let root = temp_dir!();
    let directory = root.join("rollover").join("ndjson").join("events");
    fs::create_dir_all(&directory)?;
    let prior_path = directory.join("2000-01-01.ndjson");
    let record = b"{\"rollover\":true}\n";
    append_record_for_operation(&prior_path, "rollover-operation", record)?;

    let writer = NdjsonWriter::new(&root);
    let returned = writer.append_event_for_operation(
        "rollover",
        "events",
        "rollover-operation",
        &json!({"rollover": true}),
    )?;
    assert_eq!(returned, prior_path);
    assert_eq!(fs::read(&returned)?, record);
    assert_eq!(daily_stream_count(&directory)?, 1);
    Ok(())
}

fn daily_stream_count(directory: &std::path::Path) -> std::io::Result<usize> {
    Ok(fs::read_dir(directory)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().and_then(|value| value.to_str()) == Some("ndjson"))
        .count())
}
