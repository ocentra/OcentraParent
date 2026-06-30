use ocentra_parent_agent_core::browser_bridge_native_host::{
    validate_browser_native_host_frame, BrowserNativeHostFrame, BrowserNativeHostFrameError,
};
use ocentra_parent_agent_protocol::constants;
use serde_json::json;

#[test]
fn accepts_managed_profile_bound_payload() {
    assert_eq!(validate_browser_native_host_frame(&managed_frame()), Ok(()));
}

#[test]
fn rejects_untrusted_origin() {
    let frame = BrowserNativeHostFrame {
        origin: constants::browser::DEVTOOLS_TEST_RAW_DEBUGGER_URL,
        ..managed_frame()
    };

    assert_eq!(
        validate_browser_native_host_frame(&frame),
        Err(BrowserNativeHostFrameError::OriginMismatch)
    );
}

#[test]
fn rejects_missing_or_default_managed_profile_binding() {
    let missing = BrowserNativeHostFrame {
        managed_browser_session_id: constants::value::EMPTY,
        ..managed_frame()
    };
    let default_profile = BrowserNativeHostFrame {
        profile_id: constants::browser::PATH_SEGMENT_DEFAULT,
        ..managed_frame()
    };

    assert_eq!(
        validate_browser_native_host_frame(&missing),
        Err(BrowserNativeHostFrameError::MissingManagedProfileBinding)
    );
    assert_eq!(
        validate_browser_native_host_frame(&default_profile),
        Err(BrowserNativeHostFrameError::DefaultProfileBinding)
    );
}

#[test]
fn rejects_length_json_and_schema_drift() {
    let length = BrowserNativeHostFrame {
        length_bytes: constants::browser::NATIVE_HOST_MAX_MESSAGE_BYTES + 1,
        ..managed_frame()
    };
    let invalid_json = BrowserNativeHostFrame {
        payload_json: constants::browser::DEVTOOLS_TEST_INVALID_LIST_BODY,
        length_bytes: constants::browser::DEVTOOLS_TEST_INVALID_LIST_BODY.len(),
        ..managed_frame()
    };
    let schema_drift_payload = payload_for(
        constants::browser::NATIVE_HOST_SCHEMA_VERSION,
        constants::browser::NATIVE_HOST_MESSAGE_TYPE_TAB_STATE,
        constants::browser::SESSION_ID_DEV,
        constants::browser::PATH_SEGMENT_DEFAULT,
    );
    let schema = BrowserNativeHostFrame {
        length_bytes: schema_drift_payload.len(),
        payload_json: schema_drift_payload,
        ..managed_frame()
    };

    assert_eq!(
        validate_browser_native_host_frame(&length),
        Err(BrowserNativeHostFrameError::MessageLengthInvalid)
    );
    assert_eq!(
        validate_browser_native_host_frame(&invalid_json),
        Err(BrowserNativeHostFrameError::SchemaInvalid)
    );
    assert_eq!(
        validate_browser_native_host_frame(&schema),
        Err(BrowserNativeHostFrameError::SchemaInvalid)
    );
}

#[test]
fn rejects_stale_heartbeat() {
    let frame = BrowserNativeHostFrame {
        heartbeat_age_ms: constants::browser::NATIVE_HOST_STALE_HEARTBEAT_MS + 1,
        ..managed_frame()
    };

    assert_eq!(
        validate_browser_native_host_frame(&frame),
        Err(BrowserNativeHostFrameError::HeartbeatStale)
    );
}

fn managed_frame() -> BrowserNativeHostFrame<'static> {
    let payload_json = payload_for(
        constants::browser::NATIVE_HOST_SCHEMA_VERSION,
        constants::browser::NATIVE_HOST_MESSAGE_TYPE_TAB_STATE,
        constants::browser::SESSION_ID_DEV,
        constants::browser::PROFILE_ID_DEV,
    );

    BrowserNativeHostFrame {
        origin: constants::browser::NATIVE_HOST_ALLOWED_ORIGIN,
        managed_browser_session_id: constants::browser::SESSION_ID_DEV,
        profile_id: constants::browser::PROFILE_ID_DEV,
        length_bytes: payload_json.len(),
        payload_json,
        heartbeat_age_ms: 0,
        heartbeat_stale_after_ms: constants::browser::NATIVE_HOST_STALE_HEARTBEAT_MS,
    }
}

fn payload_for(
    schema_version: u64,
    message_type: &'static str,
    managed_browser_session_id: &'static str,
    profile_id: &'static str,
) -> &'static str {
    let payload_json = json!({
        constants::field::SCHEMA_VERSION: schema_version,
        constants::field::MESSAGE_TYPE: message_type,
        constants::field::MANAGED_BROWSER_SESSION_ID: managed_browser_session_id,
        constants::field::PROFILE_ID: profile_id
    })
    .to_string();

    Box::leak(payload_json.into_boxed_str())
}
