use std::string::String as TestString;

use ocentra_parent_agent_protocol::logging::LogFieldValue;

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
