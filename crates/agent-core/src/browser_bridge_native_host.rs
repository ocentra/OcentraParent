use ocentra_parent_agent_protocol::constants;
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserNativeHostFrame<'a> {
    pub origin: &'a str,
    pub managed_browser_session_id: &'a str,
    pub profile_id: &'a str,
    pub length_bytes: usize,
    pub payload_json: &'a str,
    pub heartbeat_age_ms: u64,
    pub heartbeat_stale_after_ms: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrowserNativeHostFrameError {
    OriginMismatch,
    MissingManagedProfileBinding,
    DefaultProfileBinding,
    MessageLengthInvalid,
    InvalidJson,
    SchemaInvalid,
    HeartbeatStale,
}

pub fn validate_browser_native_host_frame(
    frame: &BrowserNativeHostFrame<'_>,
) -> Result<(), BrowserNativeHostFrameError> {
    validate_origin(frame)?;
    validate_managed_profile_binding(frame)?;
    validate_message_length(frame)?;
    validate_heartbeat(frame)?;
    validate_payload_schema(frame)
}

fn validate_origin(frame: &BrowserNativeHostFrame<'_>) -> Result<(), BrowserNativeHostFrameError> {
    if frame.origin == constants::browser::NATIVE_HOST_ALLOWED_ORIGIN {
        Ok(())
    } else {
        Err(BrowserNativeHostFrameError::OriginMismatch)
    }
}

fn validate_managed_profile_binding(
    frame: &BrowserNativeHostFrame<'_>,
) -> Result<(), BrowserNativeHostFrameError> {
    if frame.managed_browser_session_id.is_empty() || frame.profile_id.is_empty() {
        return Err(BrowserNativeHostFrameError::MissingManagedProfileBinding);
    }

    if frame.profile_id == constants::browser::PATH_SEGMENT_DEFAULT {
        return Err(BrowserNativeHostFrameError::DefaultProfileBinding);
    }

    Ok(())
}

fn validate_message_length(
    frame: &BrowserNativeHostFrame<'_>,
) -> Result<(), BrowserNativeHostFrameError> {
    if frame.length_bytes == 0
        || frame.length_bytes > constants::browser::NATIVE_HOST_MAX_MESSAGE_BYTES
        || frame.length_bytes != frame.payload_json.len()
    {
        return Err(BrowserNativeHostFrameError::MessageLengthInvalid);
    }

    Ok(())
}

fn validate_heartbeat(
    frame: &BrowserNativeHostFrame<'_>,
) -> Result<(), BrowserNativeHostFrameError> {
    if frame.heartbeat_age_ms <= frame.heartbeat_stale_after_ms {
        Ok(())
    } else {
        Err(BrowserNativeHostFrameError::HeartbeatStale)
    }
}

fn validate_payload_schema(
    frame: &BrowserNativeHostFrame<'_>,
) -> Result<(), BrowserNativeHostFrameError> {
    let payload = serde_json::from_str::<Value>(frame.payload_json)
        .map_err(|_error| BrowserNativeHostFrameError::InvalidJson)?;

    if payload_field_matches_u64(
        &payload,
        constants::field::SCHEMA_VERSION,
        constants::browser::NATIVE_HOST_SCHEMA_VERSION,
    ) && payload_field_matches_str(
        &payload,
        constants::field::MESSAGE_TYPE,
        constants::browser::NATIVE_HOST_MESSAGE_TYPE_TAB_STATE,
    ) && payload_field_matches_str(
        &payload,
        constants::field::MANAGED_BROWSER_SESSION_ID,
        frame.managed_browser_session_id,
    ) && payload_field_matches_str(&payload, constants::field::PROFILE_ID, frame.profile_id)
    {
        Ok(())
    } else {
        Err(BrowserNativeHostFrameError::SchemaInvalid)
    }
}

fn payload_field_matches_u64(payload: &Value, field: &str, expected: u64) -> bool {
    payload.get(field).and_then(Value::as_u64) == Some(expected)
}

fn payload_field_matches_str(payload: &Value, field: &str, expected: &str) -> bool {
    payload.get(field).and_then(Value::as_str) == Some(expected)
}
