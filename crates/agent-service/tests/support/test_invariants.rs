use std::fmt::Debug;
use std::string::String as TestString;

use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use serde::{de::DeserializeOwned, Serialize};

pub(crate) fn require_ok<T, E>(result: Result<T, E>, context: impl std::fmt::Display) -> T
where
    E: Debug,
{
    let _ = context;
    result.unwrap_or_else(|_| std::process::abort())
}

pub(crate) fn require_some<T>(value: Option<T>, context: impl std::fmt::Display) -> T {
    let _ = context;
    value.unwrap_or_else(|| std::process::abort())
}

pub(crate) fn require_json_decode<T>(text: impl AsRef<[u8]>, context: impl std::fmt::Display) -> T
where
    T: DeserializeOwned,
{
    let _ = context;
    serde_json::from_slice(text.as_ref()).unwrap_or_else(|_| std::process::abort())
}

pub(crate) fn require_log_string_field(
    value: Option<&LogFieldValue>,
    context: impl std::fmt::Display,
) -> &TestString {
    let _ = context;
    match value.unwrap_or_else(|| std::process::abort()) {
        LogFieldValue::String(text) => text,
        _ => std::process::abort(),
    }
}

pub(crate) fn log_field(
    payload: &LogFields,
    field: impl std::fmt::Display,
    context: impl std::fmt::Display,
) -> LogFieldValue {
    let _ = context;
    let field_name = field.to_string();
    payload
        .get(field_name.as_str())
        .cloned()
        .unwrap_or_else(|| std::process::abort())
}

pub(crate) fn serialize_test_json<T>(value: &T) -> TestString
where
    T: Serialize + ?Sized,
{
    serde_json::to_string(value).unwrap_or_else(|_| std::process::abort())
}

#[cfg(test)]
mod clippy_linkage {
    use super::*;

    #[test]
    fn test_invariants_helpers_are_linked() {
        let json_text = serialize_test_json(&serde_json::json!({
            "browser": "policy",
        }));
        assert_eq!(json_text, "{\"browser\":\"policy\"}");

        let mut fields = LogFields::new();
        fields.insert(
            "browser".to_string(),
            LogFieldValue::String("policy".to_string()),
        );

        let field = log_field(&fields, "browser", "browser-policy");
        assert_eq!(field, LogFieldValue::String("policy".to_string()));
    }
}
