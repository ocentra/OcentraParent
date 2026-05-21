use ocentra_parent_agent_protocol::{constants, LogFieldValue};

use crate::{
    local_ai_runtime_payload::local_ai_runtime_status_payload,
    local_ai_runtime_status::unavailable_local_ai_runtime_status,
};

#[test]
fn unavailable_local_ai_runtime_status_reports_safe_unconfigured_state() {
    let status = unavailable_local_ai_runtime_status(
        constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
    );

    assert_eq!(
        status.load_state.as_protocol_str(),
        constants::local_ai_runtime::LOAD_STATE_UNAVAILABLE
    );
    assert_eq!(
        status.degraded_state.as_protocol_str(),
        constants::local_ai_runtime::DEGRADED_PROVIDER_UNAVAILABLE
    );
    assert_eq!(
        status.privacy_mode.as_protocol_str(),
        constants::local_ai_runtime::PRIVACY_MODE_LOCAL_ONLY
    );
    assert_eq!(
        status.adapter_boundary.as_protocol_str(),
        constants::local_ai_runtime::ADAPTER_BOUNDARY_LOCAL_ADAPTER_UNAVAILABLE
    );
    assert_eq!(
        status.execution_state.as_protocol_str(),
        constants::local_ai_runtime::EXECUTION_STATE_DISABLED
    );
    assert_eq!(
        status.provider_source.as_protocol_str(),
        constants::local_ai_runtime::PROVIDER_SOURCE_UNAVAILABLE
    );
    assert_eq!(
        status.unavailable_reason,
        Some(constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED.to_string())
    );
    assert!(status.capability_flags.is_empty());
}

#[test]
fn local_ai_runtime_status_payload_exposes_status_without_model_execution() {
    let status = unavailable_local_ai_runtime_status(
        constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
    );
    let payload = local_ai_runtime_status_payload(&status);

    assert_eq!(
        payload.get(constants::field::LOCAL_AI_RUNTIME_REFERENCE_ID),
        Some(&LogFieldValue::String(
            constants::local_ai_runtime::RUNTIME_REFERENCE_DEV_UNCONFIGURED.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::LOAD_STATE),
        Some(&LogFieldValue::String(
            constants::local_ai_runtime::LOAD_STATE_UNAVAILABLE.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::LOCAL_AI_PRIVACY_MODE),
        Some(&LogFieldValue::String(
            constants::local_ai_runtime::PRIVACY_MODE_LOCAL_ONLY.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::LOCAL_AI_ADAPTER_BOUNDARY),
        Some(&LogFieldValue::String(
            constants::local_ai_runtime::ADAPTER_BOUNDARY_LOCAL_ADAPTER_UNAVAILABLE.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::LOCAL_AI_EXECUTION_STATE),
        Some(&LogFieldValue::String(
            constants::local_ai_runtime::EXECUTION_STATE_DISABLED.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::LOCAL_AI_PROVIDER_SOURCE),
        Some(&LogFieldValue::String(
            constants::local_ai_runtime::PROVIDER_SOURCE_UNAVAILABLE.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::LOCAL_AI_CAPABILITY_FLAGS),
        Some(&LogFieldValue::String(
            constants::local_ai_runtime::CAPABILITY_FLAGS_NONE.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::LOCAL_AI_UNAVAILABLE_REASON),
        Some(&LogFieldValue::String(
            constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED.to_string()
        ))
    );
}
