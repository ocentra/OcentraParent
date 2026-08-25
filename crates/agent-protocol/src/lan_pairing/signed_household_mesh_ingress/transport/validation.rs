use ocentra_eventing::error::EventingError;
use serde::Deserialize;

use super::{
    LanHouseholdMeshIngressSchemaVersionDto, LanHouseholdMeshSequenceDto,
    LAN_SIGNED_HOUSEHOLD_MESH_PROTOCOL_VALUES, MAX_SIGNED_TRANSPORT_TEXT_BYTES,
};

// BOUNDARY-INVARIANT: every scalar decoded by the signed transport boundary is
// rejected unless it satisfies its protocol-owned size, alphabet, and value
// constraints before cryptographic verification observes it.

pub(super) fn validate_transport_text(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, EventingError> {
    let value = value.into();
    if value.is_empty() {
        return Err(EventingError::EmptyValue { field });
    }
    if value.len() > MAX_SIGNED_TRANSPORT_TEXT_BYTES
        || !value.is_ascii()
        || value.chars().any(char::is_whitespace)
    {
        return Err(invalid_transport_value(field));
    }
    Ok(value)
}

pub(super) fn deserialize_schema_version<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = u16::deserialize(deserializer)?;
    LanHouseholdMeshIngressSchemaVersionDto::try_new(value)
        .map(LanHouseholdMeshIngressSchemaVersionDto::value)
        .map_err(serde::de::Error::custom)
}

pub(super) fn deserialize_sequence<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = u64::deserialize(deserializer)?;
    LanHouseholdMeshSequenceDto::try_new(value)
        .map(LanHouseholdMeshSequenceDto::value)
        .map_err(serde::de::Error::custom)
}

pub(super) fn validate_transport_sha256(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, EventingError> {
    let value = validate_transport_text(value, field)?;
    if value.len() != 64 || !value.chars().all(|character| character.is_ascii_hexdigit()) {
        return Err(invalid_transport_value(field));
    }
    Ok(value)
}

pub(super) fn validate_transport_message_type(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, EventingError> {
    let value = validate_transport_text(value, field)?;
    if !LAN_SIGNED_HOUSEHOLD_MESH_PROTOCOL_VALUES
        .iter()
        .any(|protocol| protocol.message_type == value)
    {
        return Err(invalid_transport_value(field));
    }
    Ok(value)
}

pub(super) fn invalid_transport_value(field: &'static str) -> EventingError {
    // ALLOC-JUSTIFICATION: EventingError owns its safely redacted display value.
    EventingError::InvalidValue {
        field,
        value: "[redacted]".to_owned(),
    }
}
