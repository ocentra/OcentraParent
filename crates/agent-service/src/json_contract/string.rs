use ocentra_parent_agent_protocol::constants;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct JsonText(pub(crate) String);

impl AsRef<[u8]> for JsonText {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

#[derive(Serialize)]
struct JsonSerializationError {
    error: String,
    detail: String,
}

pub(crate) fn serialize_json_string<T>(value: &T) -> JsonText
where
    T: Serialize + ?Sized,
{
    JsonText(
        serde_json::to_string(value)
            .unwrap_or_else(|error| serialize_error_value(&error).to_string()),
    )
}

pub(super) fn serialize_error_value(error: &serde_json::Error) -> Value {
    serde_json::to_value(JsonSerializationError {
        error: constants::error::AGENT_EVENT_SERIALIZES.to_string(),
        detail: error.to_string(),
    })
    .unwrap_or_else(|_| Value::String(constants::error::AGENT_EVENT_SERIALIZES.to_string()))
}
