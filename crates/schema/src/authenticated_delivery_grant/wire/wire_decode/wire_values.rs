use super::GrantWireValue;

pub(super) fn payload_length(value: &GrantWireValue, field: &str) -> Result<usize, String> {
    match value {
        GrantWireValue::PayloadLength(value) => Ok(*value),
        _ => Err(format!("field `{field}` has the wrong wire type")),
    }
}

pub(super) fn schema_version(value: &GrantWireValue, field: &str) -> Result<u16, String> {
    match value {
        GrantWireValue::SchemaVersion(value) => Ok(*value),
        _ => Err(format!("field `{field}` has the wrong wire type")),
    }
}

pub(super) fn dry_run(value: &GrantWireValue, field: &str) -> Result<bool, String> {
    match value {
        GrantWireValue::DryRun(value) => Ok(*value),
        _ => Err(format!("field `{field}` has the wrong wire type")),
    }
}
