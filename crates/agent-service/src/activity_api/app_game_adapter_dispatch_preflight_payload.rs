use ocentra_parent_agent_protocol::app_game::APP_GAME_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::app_game_adapter_dispatch_preflight::{
    AppGameAdapterDispatchPreflightReadModel, AppGameAdapterDispatchPreflightRow,
    APP_GAME_ADAPTER_DISPATCH_AUDIT_OWNED_PROCESS, APP_GAME_ADAPTER_DISPATCH_CLAIM_BLOCKED,
    APP_GAME_ADAPTER_DISPATCH_CLAIM_SCOPED_TIMER, APP_GAME_ADAPTER_DISPATCH_DECISION_BLOCKED,
    APP_GAME_ADAPTER_DISPATCH_DECISION_ELIGIBLE, APP_GAME_ADAPTER_DISPATCH_EVIDENCE_OWNED_PROCESS,
    APP_GAME_ADAPTER_DISPATCH_FALLBACK_BLOCKED, APP_GAME_ADAPTER_DISPATCH_FALLBACK_SCOPED_TIMER,
    APP_GAME_ADAPTER_DISPATCH_INTENT_OWNED_PROCESS_TIME_LIMIT,
    APP_GAME_ADAPTER_DISPATCH_OUTCOME_DEGRADED, APP_GAME_ADAPTER_DISPATCH_OUTCOME_MANUAL_REQUIRED,
    APP_GAME_ADAPTER_DISPATCH_OUTCOME_READY, APP_GAME_ADAPTER_DISPATCH_OUTCOME_UNAVAILABLE,
    APP_GAME_ADAPTER_DISPATCH_OUTCOME_UNSUPPORTED,
    APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_CUSTODY_EXECUTION_AND_POLICY_DISPATCH,
    APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL_ID,
    APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_ROW_ID_PREFIX,
    APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_DEGRADED,
    APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_ELIGIBLE,
    APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_MANUAL_REQUIRED,
    APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_UNAVAILABLE,
    APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_UNSUPPORTED,
    APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATUS_PARTIAL,
    APP_GAME_ADAPTER_DISPATCH_TIMER_OWNED_PROCESS,
};
use ocentra_parent_agent_protocol::app_game_adapter_execution_readiness::{
    AppGameAdapterExecutionReadinessRow, APP_GAME_ADAPTER_EXECUTION_DECISION_ALLOWED,
    APP_GAME_ADAPTER_EXECUTION_READINESS_READ_MODEL_ID, APP_GAME_ADAPTER_EXECUTION_STATE_DEGRADED,
    APP_GAME_ADAPTER_EXECUTION_STATE_MANUAL_REQUIRED, APP_GAME_ADAPTER_EXECUTION_STATE_UNAVAILABLE,
    APP_GAME_ADAPTER_EXECUTION_STATE_UNSUPPORTED, APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE,
    APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_APPLICABLE, APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED,
};
use ocentra_parent_agent_protocol::constants::{
    self, v08_enforcement_policy_dispatch as dispatch, v08_supported_adapter_runtime_proof as proof,
};
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};

use super::app_game_adapter_execution_readiness_payload::{
    app_game_adapter_execution_readiness_read_model, GeneratedAtText,
};
use crate::{event_builder::build_event, fields::fields_from_pairs, time::timestamp_now};

#[derive(Clone, Debug, PartialEq, Eq)]
struct CapabilityStateText(String);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct StateText(&'static str);

pub async fn build_activity_app_game_adapter_dispatch_preflight_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let read_model =
        app_game_adapter_dispatch_preflight_read_model(GeneratedAtText(timestamp_now()));
    build_event(
        constants::event_id::ACTIVITY_APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentActivityAppGameAdapterDispatchPreflightReadModelReported,
        LogLevel::Info,
        app_game_adapter_dispatch_preflight_payload(&read_model),
        None,
    )
}

pub fn app_game_adapter_dispatch_preflight_read_model(
    generated_at: GeneratedAtText,
) -> AppGameAdapterDispatchPreflightReadModel {
    let readiness = app_game_adapter_execution_readiness_read_model(generated_at.clone());
    let rows = readiness
        .rows
        .iter()
        .map(|row| dispatch_preflight_row(row, generated_at.clone()))
        .collect::<Vec<_>>();
    let returned = rows.len() as u64;
    let dispatch_eligible_count = rows
        .iter()
        .filter(|row| row.dispatch_decision == APP_GAME_ADAPTER_DISPATCH_DECISION_ELIGIBLE)
        .count() as u64;
    let blocked_before_dispatch_count = rows
        .iter()
        .filter(|row| row.dispatch_decision == APP_GAME_ADAPTER_DISPATCH_DECISION_BLOCKED)
        .count() as u64;
    let adapter_dispatch_eligible_count = rows
        .iter()
        .filter(|row| row.adapter_dispatch_eligible)
        .count() as u64;

    AppGameAdapterDispatchPreflightReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        read_model_id: APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL_ID.to_string(),
        generated_at: generated_at.0,
        source_read_model_ids: vec![
            APP_GAME_ADAPTER_EXECUTION_READINESS_READ_MODEL_ID.to_string(),
            dispatch::READ_MODEL_ID.to_string(),
        ],
        custody_label: APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_CUSTODY_EXECUTION_AND_POLICY_DISPATCH
            .to_string(),
        capability_status: APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATUS_PARTIAL.to_string(),
        returned,
        dispatch_eligible_count,
        blocked_before_dispatch_count,
        adapter_dispatch_eligible_count,
        adapter_dispatch_executed_claimed_count: 0,
        host_capability_available_count: count_host_capability_state(
            &rows,
            &CapabilityStateText(APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE.to_string()),
        ),
        host_capability_not_detected_count: count_host_capability_state(
            &rows,
            &CapabilityStateText(APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED.to_string()),
        ),
        host_capability_not_applicable_count: count_host_capability_state(
            &rows,
            &CapabilityStateText(APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_APPLICABLE.to_string()),
        ),
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

pub fn app_game_adapter_dispatch_preflight_payload(
    read_model: &AppGameAdapterDispatchPreflightReadModel,
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
            constants::field::APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL,
            LogFieldValue::String(serde_json::to_string(read_model).unwrap_or_default()),
        ),
    ])
}

fn dispatch_preflight_row(
    row: &AppGameAdapterExecutionReadinessRow,
    generated_at: GeneratedAtText,
) -> AppGameAdapterDispatchPreflightRow {
    let eligible = row.execution_decision == APP_GAME_ADAPTER_EXECUTION_DECISION_ALLOWED
        && row.source_proof_entry_id == proof::ENTRY_ID_APP_GAME_TIMER;
    let mut row_id = String::from(APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_ROW_ID_PREFIX);
    row_id.push_str(&row.source_proof_entry_id);
    let route = dispatch_preflight_route(row, eligible);

    AppGameAdapterDispatchPreflightRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id,
        source_execution_readiness_row_id: row.row_id.clone(),
        source_proof_entry_id: row.source_proof_entry_id.clone(),
        platform: row.platform.clone(),
        product_meanings: row.product_meanings.clone(),
        adapter_capability: row.adapter_capability.clone(),
        adapter_execution_state: row.adapter_execution_state.clone(),
        execution_decision: row.execution_decision.clone(),
        dispatch_preflight_state: route.dispatch_preflight_state.to_string(),
        dispatch_decision: route.dispatch_decision.to_string(),
        dispatch_intent_id: route.dispatch_intent_id.map(|value| value.to_string()),
        dispatch_outcome_state: route.dispatch_outcome_state.to_string(),
        dispatch_evidence_refs: route
            .dispatch_evidence_refs
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        host_capability_state: row.host_capability_state.clone(),
        host_capability_evidence_refs: row.host_capability_evidence_refs.clone(),
        host_capability_probe_refs: row.host_capability_probe_refs.clone(),
        dispatch_audit_refs: route
            .dispatch_audit_refs
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        dispatch_timer_refs: route
            .dispatch_timer_refs
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        manual_proof_requirements: route
            .manual_proof_requirements
            .unwrap_or_else(|| row.manual_proof_requirements.clone()),
        claim_boundary: route.claim_boundary.to_string(),
        fallback_behavior: route.fallback_behavior.to_string(),
        adapter_dispatch_eligible: eligible,
        adapter_dispatch_executed_claimed: false,
        broad_installed_app_blocking_claimed: false,
        child_device_delivery_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        private_diagnostics_claimed: false,
        last_checked_at: generated_at.0,
    }
}

struct DispatchPreflightRoute {
    dispatch_preflight_state: &'static str,
    dispatch_decision: &'static str,
    dispatch_intent_id: Option<&'static str>,
    dispatch_outcome_state: &'static str,
    dispatch_evidence_refs: &'static [&'static str],
    dispatch_audit_refs: &'static [&'static str],
    dispatch_timer_refs: &'static [&'static str],
    manual_proof_requirements: Option<Vec<String>>,
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
}

fn dispatch_preflight_route(
    row: &AppGameAdapterExecutionReadinessRow,
    eligible: bool,
) -> DispatchPreflightRoute {
    if eligible {
        return DispatchPreflightRoute {
            dispatch_preflight_state: APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_ELIGIBLE,
            dispatch_decision: APP_GAME_ADAPTER_DISPATCH_DECISION_ELIGIBLE,
            dispatch_intent_id: Some(APP_GAME_ADAPTER_DISPATCH_INTENT_OWNED_PROCESS_TIME_LIMIT),
            dispatch_outcome_state: APP_GAME_ADAPTER_DISPATCH_OUTCOME_READY,
            dispatch_evidence_refs: &[APP_GAME_ADAPTER_DISPATCH_EVIDENCE_OWNED_PROCESS],
            dispatch_audit_refs: &[APP_GAME_ADAPTER_DISPATCH_AUDIT_OWNED_PROCESS],
            dispatch_timer_refs: &[APP_GAME_ADAPTER_DISPATCH_TIMER_OWNED_PROCESS],
            manual_proof_requirements: Some(Vec::new()),
            claim_boundary: APP_GAME_ADAPTER_DISPATCH_CLAIM_SCOPED_TIMER,
            fallback_behavior: APP_GAME_ADAPTER_DISPATCH_FALLBACK_SCOPED_TIMER,
        };
    }

    DispatchPreflightRoute {
        dispatch_preflight_state: dispatch_preflight_state(row).0,
        dispatch_decision: APP_GAME_ADAPTER_DISPATCH_DECISION_BLOCKED,
        dispatch_intent_id: None,
        dispatch_outcome_state: dispatch_outcome_state(row).0,
        dispatch_evidence_refs: &[],
        dispatch_audit_refs: &[],
        dispatch_timer_refs: &[],
        manual_proof_requirements: Some(row.manual_proof_requirements.clone()),
        claim_boundary: APP_GAME_ADAPTER_DISPATCH_CLAIM_BLOCKED,
        fallback_behavior: APP_GAME_ADAPTER_DISPATCH_FALLBACK_BLOCKED,
    }
}

fn dispatch_preflight_state(row: &AppGameAdapterExecutionReadinessRow) -> StateText {
    match row.adapter_execution_state.as_str() {
        APP_GAME_ADAPTER_EXECUTION_STATE_DEGRADED => {
            StateText(APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_DEGRADED)
        }
        APP_GAME_ADAPTER_EXECUTION_STATE_UNAVAILABLE => {
            StateText(APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_UNAVAILABLE)
        }
        APP_GAME_ADAPTER_EXECUTION_STATE_UNSUPPORTED => {
            StateText(APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_UNSUPPORTED)
        }
        APP_GAME_ADAPTER_EXECUTION_STATE_MANUAL_REQUIRED => {
            StateText(APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_MANUAL_REQUIRED)
        }
        _ => StateText(APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_MANUAL_REQUIRED),
    }
}

fn dispatch_outcome_state(row: &AppGameAdapterExecutionReadinessRow) -> StateText {
    match row.adapter_execution_state.as_str() {
        APP_GAME_ADAPTER_EXECUTION_STATE_DEGRADED => {
            StateText(APP_GAME_ADAPTER_DISPATCH_OUTCOME_DEGRADED)
        }
        APP_GAME_ADAPTER_EXECUTION_STATE_UNAVAILABLE => {
            StateText(APP_GAME_ADAPTER_DISPATCH_OUTCOME_UNAVAILABLE)
        }
        APP_GAME_ADAPTER_EXECUTION_STATE_UNSUPPORTED => {
            StateText(APP_GAME_ADAPTER_DISPATCH_OUTCOME_UNSUPPORTED)
        }
        _ => StateText(APP_GAME_ADAPTER_DISPATCH_OUTCOME_MANUAL_REQUIRED),
    }
}

fn count_host_capability_state(
    rows: &[AppGameAdapterDispatchPreflightRow],
    state: &CapabilityStateText,
) -> u64 {
    rows.iter()
        .filter(|row| row.host_capability_state == state.0)
        .count() as u64
}
