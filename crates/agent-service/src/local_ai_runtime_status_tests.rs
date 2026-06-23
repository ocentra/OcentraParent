use ocentra_parent_agent_protocol::constants;
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_status::{
        local_ai_runtime_status_from_config, unavailable_local_ai_runtime_status,
        unavailable_local_provider_adapter_probe,
    },
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
    assert!(status.capability_flags.is_empty());
}

#[test]
fn unavailable_local_provider_adapter_probe_reports_no_execution_boundary() {
    let probe = unavailable_local_provider_adapter_probe(
        constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
    );

    assert_eq!(
        probe.adapter_boundary.as_protocol_str(),
        constants::local_ai_runtime::ADAPTER_BOUNDARY_STATUS_ONLY
    );
    assert_eq!(
        probe.probe_state.as_protocol_str(),
        constants::local_ai_runtime::ADAPTER_PROBE_STATE_UNAVAILABLE
    );
    assert_eq!(
        probe.configuration_state.as_protocol_str(),
        constants::local_ai_runtime::PROVIDER_CONFIGURATION_UNCONFIGURED
    );
    assert_eq!(
        probe.readiness_state.as_protocol_str(),
        constants::local_ai_runtime::ADAPTER_READINESS_STATE_NOT_READY
    );
    assert!(!probe.execution_allowed);
}

#[test]
fn configured_local_ai_runtime_reports_local_binary_and_model_without_enabling_execution() {
    let binary = write_temp_file(constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI);
    let model = write_temp_file(constants::local_ai_runtime::MODEL_ID_LOCAL_GGUF_CONFIGURED);
    let config = LocalAiRuntimeConfigSnapshot::from_parts(
        Some(binary.clone()),
        Some(model.clone()),
        None,
        None,
    );

    let (status, probe, cache) = local_ai_runtime_status_from_config(
        constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        &config,
    );

    assert_eq!(
        status.provider_id,
        constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI
    );
    assert_eq!(
        status.execution_state.as_protocol_str(),
        constants::local_ai_runtime::EXECUTION_STATE_DISABLED
    );
    assert_eq!(
        probe.probe_state.as_protocol_str(),
        constants::local_ai_runtime::ADAPTER_PROBE_STATE_READY
    );
    assert!(!probe.execution_allowed);
    assert_eq!(
        cache.source_policy.as_protocol_str(),
        constants::local_ai_runtime::SOURCE_POLICY_PARENT_INSTALLED
    );
    assert_eq!(
        cache.cache_state.as_protocol_str(),
        constants::local_ai_runtime::CACHE_STATE_DEGRADED
    );

    remove_temp_file(binary);
    remove_temp_file(model);
}

#[test]
fn execution_enabled_local_ai_runtime_reports_explicit_ready_boundary() {
    let binary = write_temp_file(constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI);
    let model = write_temp_file(constants::local_ai_runtime::MODEL_ID_LOCAL_GGUF_CONFIGURED);
    let config = LocalAiRuntimeConfigSnapshot::from_parts_with_execution(
        Some(binary.clone()),
        Some(model.clone()),
        None,
        None,
        true,
        constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
        constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
    );

    let (status, probe, _cache) = local_ai_runtime_status_from_config(
        constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        &config,
    );

    assert_eq!(
        status.adapter_boundary.as_protocol_str(),
        constants::local_ai_runtime::ADAPTER_BOUNDARY_LOCAL_ADAPTER_READY
    );
    assert_eq!(
        status.execution_state.as_protocol_str(),
        constants::local_ai_runtime::EXECUTION_STATE_DRY_RUN_READY
    );
    assert_eq!(status.unavailable_reason, None);
    assert!(status
        .capability_flags
        .iter()
        .any(|flag| flag.as_protocol_str()
            == constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION));
    assert_eq!(
        probe.readiness_state.as_protocol_str(),
        constants::local_ai_runtime::ADAPTER_READINESS_STATE_READY
    );
    assert!(probe.execution_allowed);
    remove_temp_file(binary);
    remove_temp_file(model);
}

#[test]
fn missing_model_file_keeps_runtime_unavailable_without_leaking_configured_path() {
    let binary = write_temp_file(constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI);
    let model = unused_temp_path();
    let config =
        LocalAiRuntimeConfigSnapshot::from_parts(Some(binary.clone()), Some(model), None, None);

    let (status, probe, cache) = local_ai_runtime_status_from_config(
        constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        &config,
    );

    assert_eq!(
        status.unavailable_reason,
        Some(constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_FILE_MISSING.to_string())
    );
    assert_eq!(
        probe.unavailable_reason,
        Some(constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_FILE_MISSING.to_string())
    );
    assert_eq!(
        cache
            .unavailable_reason
            .as_ref()
            .map(|reason| reason.as_protocol_str()),
        Some(constants::local_ai_runtime::CACHE_UNAVAILABLE_ARTIFACT_NOT_INSTALLED)
    );
    assert_eq!(cache.cache_byte_size, 0);

    remove_temp_file(binary);
}

pub(crate) fn write_temp_file(prefix: &str) -> PathBuf {
    let path = unique_temp_path(prefix);
    fs::write(&path, constants::local_ai_runtime::TEST_CHECKED_AT)
        .unwrap_or_else(|_| panic!("{}", constants::error::LOCALHOST_BIND_SUCCEEDS));
    path
}

pub(crate) fn unused_temp_path() -> PathBuf {
    unique_temp_path(constants::local_ai_runtime::MODEL_ID_LOCAL_GGUF_CONFIGURED)
}

fn unique_temp_path(prefix: &str) -> PathBuf {
    let mut name = prefix.to_string();
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&nanos_now().to_string());
    let mut path = std::env::temp_dir();
    path.push(name);
    path
}

fn nanos_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}

pub(crate) fn remove_temp_file(path: PathBuf) {
    let _ = fs::remove_file(path);
}
