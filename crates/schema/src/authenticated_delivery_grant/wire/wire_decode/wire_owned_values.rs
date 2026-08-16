use super::GrantWireValue;

pub(super) fn string(value: GrantWireValue, field: &str) -> Result<String, String> {
    match value {
        GrantWireValue::String(value) => Ok(value),
        _ => Err(format!("field `{field}` has the wrong wire type")),
    }
}

pub(super) fn signature(value: GrantWireValue, field: &str) -> Result<Vec<u8>, String> {
    match value {
        GrantWireValue::Signature(value) => Ok(value),
        _ => Err(format!("field `{field}` has the wrong wire type")),
    }
}
