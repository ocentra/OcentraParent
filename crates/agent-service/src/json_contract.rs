#[path = "json_contract/string.rs"]
pub(crate) mod string;

use serde::Serialize;
use serde_json::Value;

pub(crate) fn serialize_json_string<T>(value: &T) -> string::JsonText
where
    T: Serialize + ?Sized,
{
    string::serialize_json_string(value)
}

pub(crate) fn serialize_json_value<T>(value: T) -> Value
where
    T: Serialize,
{
    serde_json::to_value(value).unwrap_or_else(|error| string::serialize_error_value(&error))
}
