use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use serde::Serialize;
use serde_json::Value;

pub(super) fn log_fields_from_json(value: &Value) -> Result<LogFields, String> {
    let payload = value.as_object().ok_or_else(|| {
        "parent Rust facade expected an object payload for agent command dispatch".to_string()
    })?;
    let mut fields = LogFields::new();
    for (key, value) in payload {
        fields.insert(
            key.clone(),
            log_field_value_from_json(value).map_err(|error| {
                format!("parent Rust facade rejected agent command payload field {key}: {error}")
            })?,
        );
    }
    Ok(fields)
}

pub(super) fn resolve_command_origin() -> String {
    read_allowed_origin_from_env().unwrap_or_else(|| {
        constants::bind::DEFAULT_ALLOWED_ORIGINS
            .first()
            .copied()
            .unwrap_or(constants::lan_pairing::ALLOWED_ORIGIN)
            .to_string()
    })
}

pub(super) fn log_field_string(value: &LogFieldValue) -> Option<&str> {
    match value {
        LogFieldValue::String(value) => Some(value.as_str()),
        _ => None,
    }
}

pub(super) fn serialized_enum_label<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|json| json.as_str().map(ToOwned::to_owned))
        .unwrap_or_default()
}

fn log_field_value_from_json(value: &Value) -> Result<LogFieldValue, &'static str> {
    match value {
        Value::String(value) => Ok(LogFieldValue::String(value.clone())),
        Value::Number(value) => value
            .as_f64()
            .map(LogFieldValue::Number)
            .ok_or("numbers must be finite f64-compatible values"),
        Value::Bool(value) => Ok(LogFieldValue::Boolean(*value)),
        Value::Null => Ok(LogFieldValue::Null(())),
        Value::Array(_) | Value::Object(_) => {
            Err("nested objects and arrays are not supported on the LAN bridge payload")
        }
    }
}

fn read_allowed_origin_from_env() -> Option<String> {
    std::env::var(constants::env_var::AGENT_ALLOWED_ORIGINS)
        .ok()
        .and_then(|value| {
            value
                .split(constants::delimiter::LIST)
                .map(str::trim)
                .find(|origin| !origin.is_empty())
                .map(ToOwned::to_owned)
        })
}
