use ocentra_parent_agent_protocol::app_game::APP_GAME_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::app_game_adapter_execution_readiness::{
    AppGameAdapterExecutionReadinessReadModel, AppGameAdapterExecutionReadinessRow,
    APP_GAME_ADAPTER_EXECUTION_DECISION_ALLOWED, APP_GAME_ADAPTER_EXECUTION_DECISION_BLOCKED,
    APP_GAME_ADAPTER_EXECUTION_READINESS_CUSTODY_SUPPORTED_ADAPTER_RUNTIME_PROOF,
    APP_GAME_ADAPTER_EXECUTION_READINESS_READ_MODEL_ID,
    APP_GAME_ADAPTER_EXECUTION_READINESS_STATUS_PARTIAL, APP_GAME_ADAPTER_EXECUTION_ROW_ID_PREFIX,
    APP_GAME_ADAPTER_EXECUTION_STATE_DEGRADED, APP_GAME_ADAPTER_EXECUTION_STATE_MANUAL_REQUIRED,
    APP_GAME_ADAPTER_EXECUTION_STATE_PROVED_SCOPED, APP_GAME_ADAPTER_EXECUTION_STATE_UNAVAILABLE,
    APP_GAME_ADAPTER_EXECUTION_STATE_UNSUPPORTED, APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE,
    APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_APPLICABLE, APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED,
    APP_GAME_ADAPTER_PRODUCT_NATIVE_APP, APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME,
    APP_GAME_PARENT_PLATFORM_IOS, APP_GAME_PARENT_PLATFORM_MACOS,
};
use ocentra_parent_agent_protocol::app_game_authority_classifier::{
    APP_GAME_PARENT_PLATFORM_ANDROID, APP_GAME_PARENT_PLATFORM_LINUX,
    APP_GAME_PARENT_PLATFORM_WINDOWS,
};
use ocentra_parent_agent_protocol::constants::{
    self, v08_supported_adapter_runtime_proof as proof,
};
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};

use super::app_game_adapter_host_capabilities::HostCapabilitySignals;
use crate::{event_builder::build_event, fields::fields_from_pairs, time::timestamp_now};

pub async fn build_activity_app_game_adapter_execution_readiness_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let read_model =
        app_game_adapter_execution_readiness_read_model(GeneratedAtText(timestamp_now()));
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
    generated_at: GeneratedAtText,
) -> AppGameAdapterExecutionReadinessReadModel {
    let host_capabilities = HostCapabilitySignals::detect();
    let rows = adapter_execution_rows(&generated_at, &host_capabilities);
    let returned = rows.len() as u64;
    let execution_allowed_count = count_rows(&rows, |row| {
        row.execution_decision == APP_GAME_ADAPTER_EXECUTION_DECISION_ALLOWED
    });
    let blocked_before_execution_count = count_rows(&rows, |row| {
        row.execution_decision == APP_GAME_ADAPTER_EXECUTION_DECISION_BLOCKED
    });
    let adapter_execution_claimed_count = count_rows(&rows, |row| row.adapter_execution_claimed);

    AppGameAdapterExecutionReadinessReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        read_model_id: APP_GAME_ADAPTER_EXECUTION_READINESS_READ_MODEL_ID.to_string(),
        generated_at: generated_at.0,
        source_read_model_ids: vec![proof::READ_MODEL_ID.to_string()],
        custody_label: APP_GAME_ADAPTER_EXECUTION_READINESS_CUSTODY_SUPPORTED_ADAPTER_RUNTIME_PROOF
            .to_string(),
        capability_status: APP_GAME_ADAPTER_EXECUTION_READINESS_STATUS_PARTIAL.to_string(),
        returned,
        execution_allowed_count,
        blocked_before_execution_count,
        adapter_execution_claimed_count,
        host_capability_available_count: count_rows(&rows, |row| {
            row.host_capability_state == APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE
        }),
        host_capability_not_detected_count: count_rows(&rows, |row| {
            row.host_capability_state == APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED
        }),
        host_capability_not_applicable_count: count_rows(&rows, |row| {
            row.host_capability_state == APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_APPLICABLE
        }),
        host_capability_probe_ref_count: rows
            .iter()
            .map(|row| row.host_capability_probe_refs.len() as u64)
            .sum(),
        broad_installed_app_blocking_claimed: false,
        child_device_delivery_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        private_diagnostics_claimed: false,
        rows,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct GeneratedAtText(pub(crate) String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FieldList(pub(crate) Vec<String>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FieldText(pub(crate) String);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct StaticText(pub(crate) &'static str);

impl StaticText {
    fn is_empty(self) -> bool {
        self.0.is_empty()
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
            LogFieldValue::String(serialized_read_model(read_model).0),
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
    host_capability_state: StaticText,
    host_capability_evidence_refs: FieldList,
    host_capability_probe_refs: FieldList,
    linked_proof_artifacts: &'static [&'static str],
    manual_proof_requirements: &'static [&'static str],
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
}

fn adapter_execution_rows(
    generated_at: &GeneratedAtText,
    host_capabilities: &HostCapabilitySignals,
) -> Vec<AppGameAdapterExecutionReadinessRow> {
    adapter_readiness_specs(host_capabilities)
        .iter()
        .map(|spec| row_from_spec(spec, generated_at))
        .collect()
}

fn adapter_readiness_specs(host_capabilities: &HostCapabilitySignals) -> Vec<AdapterReadinessSpec> {
    let mut specs = vec![scoped_timer_spec()];
    specs.extend(windows_blocked_specs());
    specs.extend(desktop_blocked_specs(host_capabilities));
    specs.extend(mobile_blocked_specs(host_capabilities));
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
        host_capability_state: StaticText(APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE),
        host_capability_evidence_refs: FieldList(vec![
            proof::REF_ADAPTER_CAPABILITY_STATE.to_string()
        ]),
        host_capability_probe_refs: FieldList(
            vec![proof::REF_WINDOWS_HOST_LOCAL_PROBE.to_string()],
        ),
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
        AdapterReadinessSpec {
            proof_entry_id: proof::ENTRY_ID_BROAD_APP_MANUAL,
            platform: APP_GAME_PARENT_PLATFORM_WINDOWS,
            adapter_capability: proof::CAPABILITY_BROAD_APP_BLOCKING,
            adapter_execution_state: APP_GAME_ADAPTER_EXECUTION_STATE_MANUAL_REQUIRED,
            execution_decision: APP_GAME_ADAPTER_EXECUTION_DECISION_BLOCKED,
            runtime_boundary: proof::ENTRY_ID_BROAD_APP_MANUAL,
            target_identity_state: proof::TARGET_INSUFFICIENT_BROAD,
            rollback_reference_state: proof::ROLLBACK_MANUAL_REQUIRED,
            audit_reference_state: proof::AUDIT_MANUAL_REQUIRED,
            evidence_refs: &[],
            host_capability_state: StaticText(constants::value::EMPTY),
            host_capability_evidence_refs: FieldList(vec![
                proof::REF_ADAPTER_CAPABILITY_STATE.to_string()
            ]),
            host_capability_probe_refs: FieldList(vec![
                proof::REF_WINDOWS_HOST_LOCAL_PROBE.to_string()
            ]),
            linked_proof_artifacts: &[],
            manual_proof_requirements: &[
                proof::REQUIREMENT_SAME_APP_IDENTITY,
                proof::REQUIREMENT_HOST_BLOCK_APPLY,
            ],
            claim_boundary: proof::CLAIM_BROAD_APP_MANUAL,
            fallback_behavior: proof::FALLBACK_BROAD_APP_MANUAL,
        },
        broad_app_artifact_status_spec(),
        permission_degraded_spec(),
    ]
}

fn broad_app_artifact_status_spec() -> AdapterReadinessSpec {
    AdapterReadinessSpec {
        proof_entry_id: proof::ENTRY_ID_BROAD_APP_ARTIFACT_STATUS,
        platform: APP_GAME_PARENT_PLATFORM_WINDOWS,
        adapter_capability: proof::CAPABILITY_BROAD_APP_ARTIFACT_STATUS,
        adapter_execution_state: APP_GAME_ADAPTER_EXECUTION_STATE_MANUAL_REQUIRED,
        execution_decision: APP_GAME_ADAPTER_EXECUTION_DECISION_BLOCKED,
        runtime_boundary: proof::ENTRY_ID_BROAD_APP_ARTIFACT_STATUS,
        target_identity_state: proof::TARGET_INSUFFICIENT_BROAD,
        rollback_reference_state: proof::ROLLBACK_MANUAL_REQUIRED,
        audit_reference_state: proof::AUDIT_MANUAL_REQUIRED,
        evidence_refs: &[
            proof::REF_WINDOWS_ADAPTER_ARTIFACT_GATE,
            proof::REF_WINDOWS_ADAPTER_ARTIFACT_INGESTION,
        ],
        host_capability_state: StaticText(APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE),
        host_capability_evidence_refs: FieldList(vec![
            proof::REF_ADAPTER_CAPABILITY_STATE.to_string()
        ]),
        host_capability_probe_refs: FieldList(
            vec![proof::REF_WINDOWS_HOST_LOCAL_PROBE.to_string()],
        ),
        linked_proof_artifacts: &[
            proof::ARTIFACT_WINDOWS_ADAPTER_CAPABILITY_PROOF,
            proof::ARTIFACT_WINDOWS_ADAPTER_ARTIFACT_GATE,
            proof::ARTIFACT_WINDOWS_ADAPTER_ARTIFACT_INGESTION_PROOF,
        ],
        manual_proof_requirements: &[
            proof::REQUIREMENT_SAME_IDENTITY_APP_PACKAGE_EVIDENCE,
            proof::REQUIREMENT_ADAPTER_APPLY_RESULT,
            proof::REQUIREMENT_ADAPTER_ROLLBACK_RESULT,
            proof::REQUIREMENT_AUDIT_CUSTODY_EVENT,
            proof::REQUIREMENT_MANUAL_REVIEW_AFTER_ARTIFACT_GATE,
        ],
        claim_boundary: proof::CLAIM_BROAD_APP_ARTIFACT_STATUS,
        fallback_behavior: proof::FALLBACK_BROAD_APP_ARTIFACT_STATUS,
    }
}

fn desktop_blocked_specs(host_capabilities: &HostCapabilitySignals) -> [AdapterReadinessSpec; 2] {
    [
        AdapterReadinessSpec {
            proof_entry_id: proof::ENTRY_ID_LINUX_UNAVAILABLE,
            platform: APP_GAME_PARENT_PLATFORM_LINUX,
            adapter_capability: proof::CAPABILITY_DESKTOP_HOST,
            adapter_execution_state: APP_GAME_ADAPTER_EXECUTION_STATE_UNAVAILABLE,
            execution_decision: APP_GAME_ADAPTER_EXECUTION_DECISION_BLOCKED,
            runtime_boundary: proof::ENTRY_ID_LINUX_UNAVAILABLE,
            target_identity_state: proof::TARGET_UNSUPPORTED_PLATFORM,
            rollback_reference_state: proof::ROLLBACK_UNAVAILABLE,
            audit_reference_state: proof::AUDIT_UNAVAILABLE,
            evidence_refs: &[],
            host_capability_state: StaticText(host_capabilities.linux_state().0),
            host_capability_evidence_refs: FieldList(host_capabilities.linux_evidence_refs().0),
            host_capability_probe_refs: FieldList(host_capabilities.linux_probe_refs().0),
            linked_proof_artifacts: &[],
            manual_proof_requirements: &[
                proof::REQUIREMENT_LINUX_SERVICE,
                proof::REQUIREMENT_LINUX_PERMISSION,
            ],
            claim_boundary: proof::CLAIM_LINUX_UNAVAILABLE,
            fallback_behavior: proof::FALLBACK_LINUX_UNAVAILABLE,
        },
        AdapterReadinessSpec {
            proof_entry_id: proof::ENTRY_ID_MACOS_UNSUPPORTED,
            platform: APP_GAME_PARENT_PLATFORM_MACOS,
            adapter_capability: proof::CAPABILITY_DESKTOP_HOST,
            adapter_execution_state: APP_GAME_ADAPTER_EXECUTION_STATE_UNSUPPORTED,
            execution_decision: APP_GAME_ADAPTER_EXECUTION_DECISION_BLOCKED,
            runtime_boundary: proof::ENTRY_ID_MACOS_UNSUPPORTED,
            target_identity_state: proof::TARGET_UNSUPPORTED_PLATFORM,
            rollback_reference_state: proof::ROLLBACK_UNAVAILABLE,
            audit_reference_state: proof::AUDIT_UNAVAILABLE,
            evidence_refs: &[],
            host_capability_state: StaticText(APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_APPLICABLE),
            host_capability_evidence_refs: FieldList(Vec::new()),
            host_capability_probe_refs: FieldList(Vec::new()),
            linked_proof_artifacts: &[],
            manual_proof_requirements: &[
                proof::REQUIREMENT_MACOS_PERMISSION,
                proof::REQUIREMENT_MACOS_PACKAGE_IDENTITY,
            ],
            claim_boundary: proof::CLAIM_MACOS_UNSUPPORTED,
            fallback_behavior: proof::FALLBACK_MACOS_UNSUPPORTED,
        },
    ]
}

fn mobile_blocked_specs(host_capabilities: &HostCapabilitySignals) -> [AdapterReadinessSpec; 2] {
    [
        AdapterReadinessSpec {
            proof_entry_id: proof::ENTRY_ID_ANDROID_MANUAL,
            platform: APP_GAME_PARENT_PLATFORM_ANDROID,
            adapter_capability: proof::CAPABILITY_MOBILE_CHILD_CONTROL,
            adapter_execution_state: APP_GAME_ADAPTER_EXECUTION_STATE_MANUAL_REQUIRED,
            execution_decision: APP_GAME_ADAPTER_EXECUTION_DECISION_BLOCKED,
            runtime_boundary: proof::ENTRY_ID_ANDROID_MANUAL,
            target_identity_state: proof::TARGET_NOT_APPLICABLE,
            rollback_reference_state: proof::ROLLBACK_MANUAL_REQUIRED,
            audit_reference_state: proof::AUDIT_MANUAL_REQUIRED,
            evidence_refs: &[],
            host_capability_state: StaticText(host_capabilities.android_state().0),
            host_capability_evidence_refs: FieldList(host_capabilities.android_evidence_refs().0),
            host_capability_probe_refs: FieldList(host_capabilities.android_probe_refs().0),
            linked_proof_artifacts: &[],
            manual_proof_requirements: &[
                proof::REQUIREMENT_ANDROID_DEVICE_OWNER,
                proof::REQUIREMENT_ANDROID_USAGE_STATS,
            ],
            claim_boundary: proof::CLAIM_MOBILE_MANUAL,
            fallback_behavior: proof::FALLBACK_MOBILE_MANUAL,
        },
        AdapterReadinessSpec {
            proof_entry_id: proof::ENTRY_ID_IOS_MANUAL,
            platform: APP_GAME_PARENT_PLATFORM_IOS,
            adapter_capability: proof::CAPABILITY_MOBILE_CHILD_CONTROL,
            adapter_execution_state: APP_GAME_ADAPTER_EXECUTION_STATE_MANUAL_REQUIRED,
            execution_decision: APP_GAME_ADAPTER_EXECUTION_DECISION_BLOCKED,
            runtime_boundary: proof::ENTRY_ID_IOS_MANUAL,
            target_identity_state: proof::TARGET_NOT_APPLICABLE,
            rollback_reference_state: proof::ROLLBACK_MANUAL_REQUIRED,
            audit_reference_state: proof::AUDIT_MANUAL_REQUIRED,
            evidence_refs: &[],
            host_capability_state: StaticText(APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_APPLICABLE),
            host_capability_evidence_refs: FieldList(Vec::new()),
            host_capability_probe_refs: FieldList(Vec::new()),
            linked_proof_artifacts: &[],
            manual_proof_requirements: &[
                proof::REQUIREMENT_IOS_FAMILY_CONTROLS,
                proof::REQUIREMENT_IOS_DEVICE_ACTIVITY,
            ],
            claim_boundary: proof::CLAIM_MOBILE_MANUAL,
            fallback_behavior: proof::FALLBACK_MOBILE_MANUAL,
        },
    ]
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
        host_capability_state: StaticText(APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE),
        host_capability_evidence_refs: FieldList(vec![
            proof::REF_ADAPTER_CAPABILITY_STATE.to_string()
        ]),
        host_capability_probe_refs: FieldList(
            vec![proof::REF_WINDOWS_HOST_LOCAL_PROBE.to_string()],
        ),
        linked_proof_artifacts: &[proof::ARTIFACT_WINDOWS_ADAPTER_CAPABILITY_PROOF],
        manual_proof_requirements: &[proof::REQUIREMENT_PERMISSION_RESTORE],
        claim_boundary: proof::CLAIM_PERMISSION_DEGRADED,
        fallback_behavior: proof::FALLBACK_PERMISSION_DEGRADED,
    }
}

fn row_from_spec(
    spec: &AdapterReadinessSpec,
    generated_at: &GeneratedAtText,
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
        evidence_refs: spec
            .evidence_refs
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        host_capability_state: if spec.host_capability_state.is_empty()
            && spec.platform == APP_GAME_PARENT_PLATFORM_WINDOWS
        {
            APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE.to_string()
        } else {
            spec.host_capability_state.0.to_string()
        },
        host_capability_evidence_refs: spec.host_capability_evidence_refs.0.clone(),
        host_capability_probe_refs: spec.host_capability_probe_refs.0.clone(),
        linked_proof_artifacts: spec
            .linked_proof_artifacts
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        manual_proof_requirements: spec
            .manual_proof_requirements
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        claim_boundary: spec.claim_boundary.to_string(),
        fallback_behavior: spec.fallback_behavior.to_string(),
        adapter_execution_claimed: spec.execution_decision
            == APP_GAME_ADAPTER_EXECUTION_DECISION_ALLOWED,
        broad_installed_app_blocking_claimed: false,
        child_device_delivery_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        private_diagnostics_claimed: false,
        last_checked_at: generated_at.0.clone(),
    }
}

fn serialized_read_model(read_model: &AppGameAdapterExecutionReadinessReadModel) -> FieldText {
    match serde_json::to_string(read_model) {
        Ok(json) => FieldText(json),
        Err(_error) => FieldText(constants::value::EMPTY.to_string()),
    }
}

fn count_rows(
    rows: &[AppGameAdapterExecutionReadinessRow],
    predicate: impl Fn(&AppGameAdapterExecutionReadinessRow) -> bool,
) -> u64 {
    rows.iter().filter(|row| predicate(row)).count() as u64
}
