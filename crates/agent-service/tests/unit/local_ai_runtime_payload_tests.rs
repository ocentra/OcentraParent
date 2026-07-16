use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::{
    local_ai_provider_scheduler::LocalAiProviderSchedulerRuntime,
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_payload::local_ai_runtime_status_payload,
    local_ai_runtime_provider_proof_read_model::local_ai_runtime_provider_proof_read_model,
    local_ai_runtime_status::{
        local_ai_runtime_status_from_config, unavailable_local_ai_runtime_status,
        unavailable_local_provider_adapter_probe,
    },
};

#[test]
fn local_ai_runtime_status_payload_exposes_runtime_status_without_model_execution() {
    let payload = unconfigured_status_payload();

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
    assert!(matches!(
        payload.get(constants::field::LOCAL_AI_RUNTIME_PROVIDER_PROOF_READ_MODEL),
        Some(LogFieldValue::String(value))
            if value.contains(constants::local_ai_runtime_provider_proof::READ_MODEL_ID)
    ));
}

#[test]
fn local_ai_runtime_status_payload_exposes_probe_fields_without_execution() {
    let payload = unconfigured_status_payload();

    assert_eq!(
        payload.get(constants::field::LOCAL_AI_ADAPTER_PROBE_STATE),
        Some(&LogFieldValue::String(
            constants::local_ai_runtime::ADAPTER_PROBE_STATE_UNAVAILABLE.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::LOCAL_AI_EXECUTION_ALLOWED),
        Some(&LogFieldValue::Boolean(false))
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

#[test]
fn local_ai_runtime_status_payload_exposes_model_cache_fields_without_paths() {
    let payload = unconfigured_status_payload();

    assert_eq!(
        payload.get(constants::field::LOCAL_AI_MODEL_ARTIFACT_REF),
        Some(&LogFieldValue::String(
            constants::local_ai_runtime::MODEL_REFERENCE_DEFAULT_GEMMA_4.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::LOCAL_AI_MODEL_CACHE_STATE),
        Some(&LogFieldValue::String(
            constants::local_ai_runtime::CACHE_STATE_UNAVAILABLE.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::LOCAL_AI_MODEL_DOWNLOAD_ENABLED),
        Some(&LogFieldValue::Boolean(false))
    );
    assert_eq!(
        payload.get(constants::field::LOCAL_AI_MODEL_CACHE_BYTE_SIZE),
        Some(&LogFieldValue::Number(0.0))
    );
}

fn unconfigured_status_payload() -> LogFields {
    let status = unavailable_local_ai_runtime_status(
        constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
    );
    let probe = unavailable_local_provider_adapter_probe(
        constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
    );
    let (_, _, cache) = local_ai_runtime_status_from_config(
        constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        &LocalAiRuntimeConfigSnapshot::unconfigured(),
    );
    let provider_proof = local_ai_runtime_provider_proof_read_model(
        constants::local_ai_runtime::TEST_CHECKED_AT,
        &LocalAiProviderSchedulerRuntime::new().status_snapshot(),
    );
    local_ai_runtime_status_payload(&status, &probe, &cache, &provider_proof)
}
