use std::fmt::Debug;

use ocentra_parent_agent_protocol::logging::LogFieldValue;
use serde::{de::DeserializeOwned, Serialize};

use crate::json_contract::serialize_json_string;

pub(crate) fn require_ok<T, E>(result: Result<T, E>, context: &str) -> T
where
    E: Debug,
{
    match result {
        Ok(value) => value,
        Err(error) => unreachable!("{context}: {error:?}"),
    }
}

pub(crate) fn require_some<T>(value: Option<T>, context: &str) -> T {
    match value {
        Some(value) => value,
        None => unreachable!("{context}"),
    }
}

pub(crate) fn require_json_decode<T>(text: &str, context: &str) -> T
where
    T: DeserializeOwned,
{
    match serde_json::from_str(text) {
        Ok(value) => value,
        Err(error) => unreachable!("{context}: {error:?}"),
    }
}

pub(crate) fn require_log_string_field<'a>(
    value: Option<&'a LogFieldValue>,
    context: &str,
) -> &'a str {
    match value {
        Some(LogFieldValue::String(text)) => text,
        other => unreachable!("{context}: {other:?}"),
    }
}

pub(crate) fn serialize_test_json<T>(value: &T) -> String
where
    T: Serialize + ?Sized,
{
    serialize_json_string(value)
}
