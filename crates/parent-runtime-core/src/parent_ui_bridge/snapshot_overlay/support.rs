use std::borrow::ToOwned;

use serde::Serialize;

use super::*;

pub(super) fn expect_agent_event(
    actual: &AgentEventName,
    expected: &AgentEventName,
) -> Result<(), String> {
    if actual == expected {
        return Ok(());
    }
    Err(format!(
        "agent-service expected {}, received {}",
        serialized_label(&expected),
        serialized_label(actual)
    ))
}

pub(super) fn serialized_label<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|json| json.as_str().map(ToOwned::to_owned))
        .unwrap_or_default()
}
