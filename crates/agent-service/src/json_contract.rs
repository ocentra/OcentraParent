use ocentra_parent_agent_protocol::constants;
use serde::Serialize;
use serde_json::Value;

pub(crate) fn serialize_json_string<T>(value: &T) -> String
where
    T: Serialize + ?Sized,
{
    match serde_json::to_string(value) {
        Ok(serialized) => serialized,
        Err(error) => unreachable!("{}: {error}", constants::error::AGENT_EVENT_SERIALIZES),
    }
}

pub(crate) fn serialize_json_value<T>(value: T) -> Value
where
    T: Serialize,
{
    match serde_json::to_value(value) {
        Ok(serialized) => serialized,
        Err(error) => unreachable!("{}: {error}", constants::error::AGENT_EVENT_SERIALIZES),
    }
}
