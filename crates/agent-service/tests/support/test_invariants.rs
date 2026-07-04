use std::fmt::Debug;
use std::string::String as TestString;

use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use serde::{de::DeserializeOwned, Serialize};

use crate::json_contract::serialize_json_string;

pub(crate) fn require_ok<T, E>(result: Result<T, E>, context: impl std::fmt::Display) -> T
where
    E: Debug,
{
    result.expect(context.to_string().as_str())
}

pub(crate) fn require_some<T>(value: Option<T>, context: impl std::fmt::Display) -> T {
    value.expect(context.to_string().as_str())
}

pub(crate) fn require_json_decode<T>(text: impl AsRef<[u8]>, context: impl std::fmt::Display) -> T
where
    T: DeserializeOwned,
{
    serde_json::from_slice(text.as_ref()).expect(context.to_string().as_str())
}

pub(crate) fn require_log_string_field<'a>(
    value: Option<&'a LogFieldValue>,
    context: impl std::fmt::Display,
) -> &'a TestString {
    match value.expect(context.to_string().as_str()) {
        LogFieldValue::String(text) => text,
        _ => std::process::abort(),
    }
}

pub(crate) fn log_field(
    payload: &LogFields,
    field: impl std::fmt::Display,
    context: impl std::fmt::Display,
) -> LogFieldValue {
    let field_name = field.to_string();
    payload
        .get(field_name.as_str())
        .cloned()
        .expect(context.to_string().as_str())
}

pub(crate) fn serialize_test_json<T>(value: &T) -> TestString
where
    T: Serialize + ?Sized,
{
    serialize_json_string(value)
}
