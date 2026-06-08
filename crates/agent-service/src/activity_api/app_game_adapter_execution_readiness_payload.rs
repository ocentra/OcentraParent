use ocentra_parent_agent_protocol::{
    constants::{self, v08_supported_adapter_runtime_proof as proof},
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
    AppGameAdapterExecutionReadinessReadModel, AppGameAdapterExecutionReadinessRow, LogFieldValue,
    LogFields, LogLevel, APP_GAME_ADAPTER_EXECUTION_DECISION_ALLOWED,
    APP_GAME_ADAPTER_EXECUTION_DECISION_BLOCKED,
    APP_GAME_ADAPTER_EXECUTION_READINESS_CUSTODY_SUPPORTED_ADAPTER_RUNTIME_PROOF,
    APP_GAME_ADAPTER_EXECUTION_READINESS_READ_MODEL_ID,
    APP_GAME_ADAPTER_EXECUTION_READINESS_STATUS_PARTIAL, APP_GAME_ADAPTER_EXECUTION_ROW_ID_PREFIX,
    APP_GAME_ADAPTER_EXECUTION_STATE_DEGRADED, APP_GAME_ADAPTER_EXECUTION_STATE_MANUAL_REQUIRED,
    APP_GAME_ADAPTER_EXECUTION_STATE_PROVED_SCOPED, APP_GAME_ADAPTER_EXECUTION_STATE_UNAVAILABLE,
    APP_GAME_ADAPTER_EXECUTION_STATE_UNSUPPORTED, APP_GAME_ADAPTER_PRODUCT_NATIVE_APP,
    APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME, APP_GAME_PARENT_PLATFORM_ANDROID,
    APP_GAME_PARENT_PLATFORM_IOS, APP_GAME_PARENT_PLATFORM_LINUX, APP_GAME_PARENT_PLATFORM_MACOS,
    APP_GAME_PARENT_PLATFORM_WINDOWS, APP_GAME_SCHEMA_VERSION,
};

use crate::{event_builder::build_event, fields::fields_from_pairs, time::timestamp_now};

pub async fn build_activity_app_game_adapter_execution_readiness_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let generated_at = timestamp_now();
    let read_model = app_game_adapter_execution_readiness_read_model(&generated_at);
    build_event(
        constants::event_id::ACTIVITY_APP_GAME_ADAPTER_EXECUTION_READINESS_READ_MODEL_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentActivityAppGameAdapterExecutionReadinessReadModelReported,
        LogLevel::Info,
        app_game_adapter_execution_readiness_payload(&read_model),
        None,
    )
}

pub fn app_game_adapter_execution_readiness_read_model(
    generated_at: &str,
) -> AppGameAdapterExecutionReadinessReadModel {
    let rows = adapter_execution_rows(generated_at);
    let returned = rows.len() as u64;
    let execution_allowed_count =
        count_decision(&rows, APP_GAME_ADAPTER_EXECUTION_DECISION_ALLOWED);
    let blocked_before_execution_count =
        count_decision(&rows, APP_GAME_ADAPTER_EXECUTION_DECISION_BLOCKED);
    let adapter_execution_claimed_count = rows
        .iter()
        .filter(|row| row.adapter_execution_claimed)
        .count() as u64;

    AppGameAdapterExecutionReadinessReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        read_model_id: APP_GAME_ADAPTER_EXECUTION_READINESS_READ_MODEL_ID.to_string(),
        generated_at: generated_at.to_string(),
        source_read_model_ids: vec![proof::READ_MODEL_ID.to_string()],
        custody_label: APP_GAME_ADAPTER_EXECUTION_READINESS_CUSTODY_SUPPORTED_ADAPTER_RUNTIME_PROOF
            .to_string(),
        capability_status: APP_GAME_ADAPTER_EXECUTION_READINESS_STATUS_PARTIAL.to_string(),
        returned,
        execution_allowed_count,
        blocked_before_execution_count,
        adapter_execution_claimed_count,
        broad_installed_app_blocking_claimed: false,
        child_device_delivery_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        private_diagnostics_claimed: false,
        rows,
    }
}

pub fn app_game_adapter_execution_readiness_payload(
    read_model: &AppGameAdapterExecutionReadinessReadModel,
) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        (
            constants::field::CUSTODY_LABEL,
            LogFieldValue::String(read_model.custody_label.clone()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(read_model.capability_status.clone()),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.returned as f64),
        ),
        (
            constants::field::APP_GAME_ADAPTER_EXECUTION_READINESS_READ_MODEL,
            LogFieldValue::String(
                serde_json::to_string(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
    ])
}

struct AdapterReadinessSpec {
    proof_entry_id: &'static str,
    platform: &'static str,
    adapter_capability: &'static str,
    adapter_execution_state: &'static str,
    execution_decision: &'static str,
    runtime_boundary: &'static str,
    target_identity_state: &'static str,
    rollback_reference_state: &'static str,
    audit_reference_state: &'static str,
    evidence_refs: &'static [&'static str],
    linked_proof_artifacts: &'static [&'static str],
    manual_proof_requirements: &'static [&'static str],
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
}

fn adapter_execution_rows(generated_at: &str) -> Vec<AppGameAdapterExecutionReadinessRow> {
    adapter_readiness_specs()
        .iter()
        .map(|spec| row_from_spec(spec, generated_at))
        .collect()
}

fn adapter_readiness_specs() -> Vec<AdapterReadinessSpec> {
    let mut specs = vec![scoped_timer_spec()];
    specs.extend(windows_blocked_specs());
    specs.extend(desktop_blocked_specs());
    specs.extend(mobile_blocked_specs());
    specs
}

fn scoped_timer_spec() -> AdapterReadinessSpec {
    AdapterReadinessSpec {
        proof_entry_id: proof::ENTRY_ID_APP_GAME_TIMER,
        platform: APP_GAME_PARENT_PLATFORM_WINDOWS,
        adapter_capability: proof::CAPABILITY_APP_GAME_TIMER,
        adapter_execution_state: APP_GAME_ADAPTER_EXECUTION_STATE_PROVED_SCOPED,
        execution_decision: APP_GAME_ADAPTER_EXECUTION_DECISION_ALLOWED,
        runtime_boundary: proof::ENTRY_ID_APP_GAME_TIMER,
        target_identity_state: proof::TARGET_PROCESS_SESSION_EVIDENCE,
        rollback_reference_state: proof::ROLLBACK_TIMER_RECOVERY,
        audit_reference_state: proof::AUDIT_BACKED,
        evidence_refs: &[
            proof::REF_APP_SESSION_EVIDENCE,
            proof::REF_OWNED_PROCESS_IDENTITY,
            proof::REF_TIMER_STATE,
        ],
        linked_proof_artifacts: &[
            proof::ARTIFACT_APP_TIME_LIMIT_PROOF,
            proof::ARTIFACT_ENFORCEMENT_TIMER_STATE,
        ],
        manual_proof_requirements: &[],
        claim_boundary: proof::CLAIM_APP_GAME_TIMER,
        fallback_behavior: proof::FALLBACK_APP_GAME_TIMER,
    }
}

fn windows_blocked_specs() -> [AdapterReadinessSpec; 3] {
    [
        blocked_spec(
            proof::ENTRY_ID_BROAD_APP_MANUAL,
            proof::CAPABILITY_BROAD_APP_BLOCKING,
            &[
                proof::REQUIREMENT_SAME_APP_IDENTITY,
                proof::REQUIREMENT_HOST_BLOCK_APPLY,
            ],
            proof::CLAIM_BROAD_APP_MANUAL,
            proof::FALLBACK_BROAD_APP_MANUAL,
        ),
        blocked_spec(
            proof::ENTRY_ID_BROAD_APP_ARTIFACT_STATUS,
            proof::CAPABILITY_BROAD_APP_ARTIFACT_STATUS,
            &[
                proof::REQUIREMENT_SAME_IDENTITY_APP_PACKAGE_EVIDENCE,
                proof::REQUIREMENT_ADAPTER_APPLY_RESULT,
                proof::REQUIREMENT_MANUAL_REVIEW_AFTER_ARTIFACT_GATE,
            ],
            proof::CLAIM_BROAD_APP_ARTIFACT_STATUS,
            proof::FALLBACK_BROAD_APP_ARTIFACT_STATUS,
        ),
        permission_degraded_spec(),
    ]
}

fn desktop_blocked_specs() -> [AdapterReadinessSpec; 2] {
    [
        platform_unavailable_spec(
            proof::ENTRY_ID_LINUX_UNAVAILABLE,
            APP_GAME_PARENT_PLATFORM_LINUX,
            APP_GAME_ADAPTER_EXECUTION_STATE_UNAVAILABLE,
            &[
                proof::REQUIREMENT_LINUX_SERVICE,
                proof::REQUIREMENT_LINUX_PERMISSION,
            ],
            proof::CLAIM_LINUX_UNAVAILABLE,
            proof::FALLBACK_LINUX_UNAVAILABLE,
        ),
        platform_unavailable_spec(
            proof::ENTRY_ID_MACOS_UNSUPPORTED,
            APP_GAME_PARENT_PLATFORM_MACOS,
            APP_GAME_ADAPTER_EXECUTION_STATE_UNSUPPORTED,
            &[
                proof::REQUIREMENT_MACOS_PERMISSION,
                proof::REQUIREMENT_MACOS_PACKAGE_IDENTITY,
            ],
            proof::CLAIM_MACOS_UNSUPPORTED,
            proof::FALLBACK_MACOS_UNSUPPORTED,
        ),
    ]
}

fn mobile_blocked_specs() -> [AdapterReadinessSpec; 2] {
    [
        mobile_manual_spec(
            proof::ENTRY_ID_ANDROID_MANUAL,
            APP_GAME_PARENT_PLATFORM_ANDROID,
            &[
                proof::REQUIREMENT_ANDROID_DEVICE_OWNER,
                proof::REQUIREMENT_ANDROID_USAGE_STATS,
            ],
        ),
        mobile_manual_spec(
            proof::ENTRY_ID_IOS_MANUAL,
            APP_GAME_PARENT_PLATFORM_IOS,
            &[
                proof::REQUIREMENT_IOS_FAMILY_CONTROLS,
                proof::REQUIREMENT_IOS_DEVICE_ACTIVITY,
            ],
        ),
    ]
}

fn blocked_spec(
    proof_entry_id: &'static str,
    adapter_capability: &'static str,
    manual_proof_requirements: &'static [&'static str],
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
) -> AdapterReadinessSpec {
    AdapterReadinessSpec {
        proof_entry_id,
        platform: APP_GAME_PARENT_PLATFORM_WINDOWS,
        adapter_capability,
        adapter_execution_state: APP_GAME_ADAPTER_EXECUTION_STATE_MANUAL_REQUIRED,
        execution_decision: APP_GAME_ADAPTER_EXECUTION_DECISION_BLOCKED,
        runtime_boundary: proof_entry_id,
        target_identity_state: proof::TARGET_INSUFFICIENT_BROAD,
        rollback_reference_state: proof::ROLLBACK_MANUAL_REQUIRED,
        audit_reference_state: proof::AUDIT_MANUAL_REQUIRED,
        evidence_refs: &[],
        linked_proof_artifacts: &[],
        manual_proof_requirements,
        claim_boundary,
        fallback_behavior,
    }
}

fn permission_degraded_spec() -> AdapterReadinessSpec {
    AdapterReadinessSpec {
        proof_entry_id: proof::ENTRY_ID_PERMISSION_DEGRADED,
        platform: APP_GAME_PARENT_PLATFORM_WINDOWS,
        adapter_capability: proof::CAPABILITY_PERMISSION_DEPENDENCY,
        adapter_execution_state: APP_GAME_ADAPTER_EXECUTION_STATE_DEGRADED,
        execution_decision: APP_GAME_ADAPTER_EXECUTION_DECISION_BLOCKED,
        runtime_boundary: proof::ENTRY_ID_PERMISSION_DEGRADED,
        target_identity_state: proof::TARGET_NOT_APPLICABLE,
        rollback_reference_state: proof::ROLLBACK_UNAVAILABLE,
        audit_reference_state: proof::AUDIT_UNAVAILABLE,
        evidence_refs: &[proof::REF_ADAPTER_CAPABILITY_STATE],
        linked_proof_artifacts: &[proof::ARTIFACT_WINDOWS_ADAPTER_CAPABILITY_PROOF],
        manual_proof_requirements: &[proof::REQUIREMENT_PERMISSION_RESTORE],
        claim_boundary: proof::CLAIM_PERMISSION_DEGRADED,
        fallback_behavior: proof::FALLBACK_PERMISSION_DEGRADED,
    }
}

fn platform_unavailable_spec(
    proof_entry_id: &'static str,
    platform: &'static str,
    adapter_execution_state: &'static str,
    manual_proof_requirements: &'static [&'static str],
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
) -> AdapterReadinessSpec {
    AdapterReadinessSpec {
        proof_entry_id,
        platform,
        adapter_capability: proof::CAPABILITY_DESKTOP_HOST,
        adapter_execution_state,
        execution_decision: APP_GAME_ADAPTER_EXECUTION_DECISION_BLOCKED,
        runtime_boundary: proof_entry_id,
        target_identity_state: proof::TARGET_UNSUPPORTED_PLATFORM,
        rollback_reference_state: proof::ROLLBACK_UNAVAILABLE,
        audit_reference_state: proof::AUDIT_UNAVAILABLE,
        evidence_refs: &[],
        linked_proof_artifacts: &[],
        manual_proof_requirements,
        claim_boundary,
        fallback_behavior,
    }
}

fn mobile_manual_spec(
    proof_entry_id: &'static str,
    platform: &'static str,
    manual_proof_requirements: &'static [&'static str],
) -> AdapterReadinessSpec {
    AdapterReadinessSpec {
        proof_entry_id,
        platform,
        adapter_capability: proof::CAPABILITY_MOBILE_CHILD_CONTROL,
        adapter_execution_state: APP_GAME_ADAPTER_EXECUTION_STATE_MANUAL_REQUIRED,
        execution_decision: APP_GAME_ADAPTER_EXECUTION_DECISION_BLOCKED,
        runtime_boundary: proof_entry_id,
        target_identity_state: proof::TARGET_NOT_APPLICABLE,
        rollback_reference_state: proof::ROLLBACK_MANUAL_REQUIRED,
        audit_reference_state: proof::AUDIT_MANUAL_REQUIRED,
        evidence_refs: &[],
        linked_proof_artifacts: &[],
        manual_proof_requirements,
        claim_boundary: proof::CLAIM_MOBILE_MANUAL,
        fallback_behavior: proof::FALLBACK_MOBILE_MANUAL,
    }
}

fn row_from_spec(
    spec: &AdapterReadinessSpec,
    generated_at: &str,
) -> AppGameAdapterExecutionReadinessRow {
    let mut row_id = String::from(APP_GAME_ADAPTER_EXECUTION_ROW_ID_PREFIX);
    row_id.push_str(spec.proof_entry_id);

    AppGameAdapterExecutionReadinessRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id,
        source_proof_entry_id: spec.proof_entry_id.to_string(),
        platform: spec.platform.to_string(),
        product_meanings: vec![
            APP_GAME_ADAPTER_PRODUCT_NATIVE_APP.to_string(),
            APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME.to_string(),
        ],
        adapter_capability: spec.adapter_capability.to_string(),
        adapter_execution_state: spec.adapter_execution_state.to_string(),
        execution_decision: spec.execution_decision.to_string(),
        runtime_boundary: spec.runtime_boundary.to_string(),
        target_identity_state: spec.target_identity_state.to_string(),
        rollback_reference_state: spec.rollback_reference_state.to_string(),
        audit_reference_state: spec.audit_reference_state.to_string(),
        evidence_refs: strings(spec.evidence_refs),
        linked_proof_artifacts: strings(spec.linked_proof_artifacts),
        manual_proof_requirements: strings(spec.manual_proof_requirements),
        claim_boundary: spec.claim_boundary.to_string(),
        fallback_behavior: spec.fallback_behavior.to_string(),
        adapter_execution_claimed: spec.execution_decision
            == APP_GAME_ADAPTER_EXECUTION_DECISION_ALLOWED,
        broad_installed_app_blocking_claimed: false,
        child_device_delivery_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        private_diagnostics_claimed: false,
        last_checked_at: generated_at.to_string(),
    }
}

fn count_decision(rows: &[AppGameAdapterExecutionReadinessRow], decision: &str) -> u64 {
    rows.iter()
        .filter(|row| row.execution_decision == decision)
        .count() as u64
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}
