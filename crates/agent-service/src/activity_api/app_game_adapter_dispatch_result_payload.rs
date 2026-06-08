use ocentra_parent_agent_protocol::{
    constants::{self},
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, AppGameAdapterDispatchPreflightRow,
    AppGameAdapterDispatchResultReadModel, AppGameAdapterDispatchResultRow, LogFieldValue,
    LogFields, LogLevel, APP_GAME_ADAPTER_DISPATCH_COMMAND_AUDIT_OWNED_PROCESS,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_ACCEPTED,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_BLOCKED,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_ACCEPTED,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_DEGRADED,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_MANUAL_REQUIRED,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_UNAVAILABLE,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_UNSUPPORTED,
    APP_GAME_ADAPTER_DISPATCH_DECISION_ELIGIBLE,
    APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_DECISION_BLOCKED,
    APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_DECISION_RECORDED,
    APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_OWNED_PROCESS_ID,
    APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_OWNED_PROCESS_REF,
    APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_STATE_BLOCKED,
    APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_STATE_RECORDED,
    APP_GAME_ADAPTER_DISPATCH_OUTCOME_READY, APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL_ID,
    APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_DEGRADED,
    APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_ELIGIBLE,
    APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_UNAVAILABLE,
    APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_UNSUPPORTED,
    APP_GAME_ADAPTER_DISPATCH_RESULT_CLAIM_BLOCKED,
    APP_GAME_ADAPTER_DISPATCH_RESULT_CLAIM_SCOPED_TIMER,
    APP_GAME_ADAPTER_DISPATCH_RESULT_CUSTODY_PREFLIGHT_AND_COMMAND,
    APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_ACTION_MODE,
    APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_COMMAND,
    APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_EVENT,
    APP_GAME_ADAPTER_DISPATCH_RESULT_FALLBACK_BLOCKED,
    APP_GAME_ADAPTER_DISPATCH_RESULT_FALLBACK_SCOPED_TIMER,
    APP_GAME_ADAPTER_DISPATCH_RESULT_OWNED_PROCESS_ID,
    APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL_ID, APP_GAME_ADAPTER_DISPATCH_RESULT_ROW_ID_PREFIX,
    APP_GAME_ADAPTER_DISPATCH_RESULT_STATUS_PARTIAL, APP_GAME_ADAPTER_DISPATCH_TIMER_OWNED_PROCESS,
    APP_GAME_SCHEMA_VERSION,
};

use super::app_game_adapter_dispatch_preflight_payload::app_game_adapter_dispatch_preflight_read_model;
use crate::{event_builder::build_event, fields::fields_from_pairs, time::timestamp_now};

pub async fn build_activity_app_game_adapter_dispatch_result_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let generated_at = timestamp_now();
    let read_model = app_game_adapter_dispatch_result_read_model(&generated_at);
    build_event(
        constants::event_id::ACTIVITY_APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentActivityAppGameAdapterDispatchResultReadModelReported,
        LogLevel::Info,
        app_game_adapter_dispatch_result_payload(&read_model),
        None,
    )
}

pub fn app_game_adapter_dispatch_result_read_model(
    generated_at: &str,
) -> AppGameAdapterDispatchResultReadModel {
    let preflight = app_game_adapter_dispatch_preflight_read_model(generated_at);
    let rows = preflight
        .rows
        .iter()
        .map(|row| dispatch_result_row(row, generated_at))
        .collect::<Vec<_>>();
    let returned = rows.len() as u64;
    let command_accepted_count = rows
        .iter()
        .filter(|row| {
            row.dispatch_command_result_decision
                == APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_ACCEPTED
        })
        .count() as u64;
    let blocked_before_command_count = rows
        .iter()
        .filter(|row| {
            row.dispatch_command_result_decision
                == APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_BLOCKED
        })
        .count() as u64;
    let adapter_dispatch_command_result_claimed_count = rows
        .iter()
        .filter(|row| row.adapter_dispatch_command_result_claimed)
        .count() as u64;
    let execution_audit_recorded_count = rows
        .iter()
        .filter(|row| {
            row.dispatch_execution_audit_decision
                == APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_DECISION_RECORDED
        })
        .count() as u64;
    let blocked_before_execution_audit_count = rows
        .iter()
        .filter(|row| {
            row.dispatch_execution_audit_decision
                == APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_DECISION_BLOCKED
        })
        .count() as u64;
    let service_local_execution_audit_claimed_count = rows
        .iter()
        .filter(|row| row.service_local_execution_audit_claimed)
        .count() as u64;

    AppGameAdapterDispatchResultReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        read_model_id: APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL_ID.to_string(),
        generated_at: generated_at.to_string(),
        source_read_model_ids: vec![
            APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL_ID.to_string(),
            APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_COMMAND.to_string(),
        ],
        custody_label: APP_GAME_ADAPTER_DISPATCH_RESULT_CUSTODY_PREFLIGHT_AND_COMMAND.to_string(),
        capability_status: APP_GAME_ADAPTER_DISPATCH_RESULT_STATUS_PARTIAL.to_string(),
        returned,
        command_accepted_count,
        blocked_before_command_count,
        execution_audit_recorded_count,
        blocked_before_execution_audit_count,
        adapter_dispatch_command_result_claimed_count,
        service_local_execution_audit_claimed_count,
        adapter_dispatch_executed_claimed_count: 0,
        broad_installed_app_blocking_claimed: false,
        child_device_delivery_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        private_diagnostics_claimed: false,
        rows,
    }
}

pub fn app_game_adapter_dispatch_result_payload(
    read_model: &AppGameAdapterDispatchResultReadModel,
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
            constants::field::APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL,
            LogFieldValue::String(
                serde_json::to_string(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
    ])
}

fn dispatch_result_row(
    row: &AppGameAdapterDispatchPreflightRow,
    generated_at: &str,
) -> AppGameAdapterDispatchResultRow {
    let accepted = dispatch_command_result_accepted(row);
    let command = command_handoff_fields(accepted);
    let audit = execution_audit_fields(accepted);
    let mut row_id = String::from(APP_GAME_ADAPTER_DISPATCH_RESULT_ROW_ID_PREFIX);
    row_id.push_str(&row.source_proof_entry_id);

    AppGameAdapterDispatchResultRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id,
        source_dispatch_preflight_row_id: row.row_id.clone(),
        source_proof_entry_id: row.source_proof_entry_id.clone(),
        platform: row.platform.clone(),
        product_meanings: row.product_meanings.clone(),
        adapter_capability: row.adapter_capability.clone(),
        dispatch_preflight_state: row.dispatch_preflight_state.clone(),
        dispatch_decision: row.dispatch_decision.clone(),
        dispatch_intent_id: row.dispatch_intent_id.clone(),
        dispatch_outcome_state: row.dispatch_outcome_state.clone(),
        dispatch_command_result_state: dispatch_command_result_state(row, accepted).to_string(),
        dispatch_command_result_decision: if accepted {
            APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_ACCEPTED
        } else {
            APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_BLOCKED
        }
        .to_string(),
        enforcement_command_name: command.enforcement_command_name,
        enforcement_event_name: command.enforcement_event_name,
        enforcement_action_mode: command.enforcement_action_mode,
        dispatch_command_result_id: command.dispatch_command_result_id,
        dispatch_command_audit_refs: command.dispatch_command_audit_refs,
        dispatch_command_timer_refs: command.dispatch_command_timer_refs,
        dispatch_execution_audit_state: audit.state,
        dispatch_execution_audit_decision: audit.decision,
        dispatch_execution_audit_id: audit.id,
        dispatch_execution_audit_refs: audit.refs,
        manual_proof_requirements: if accepted {
            Vec::new()
        } else {
            row.manual_proof_requirements.clone()
        },
        claim_boundary: if accepted {
            APP_GAME_ADAPTER_DISPATCH_RESULT_CLAIM_SCOPED_TIMER
        } else {
            APP_GAME_ADAPTER_DISPATCH_RESULT_CLAIM_BLOCKED
        }
        .to_string(),
        fallback_behavior: if accepted {
            APP_GAME_ADAPTER_DISPATCH_RESULT_FALLBACK_SCOPED_TIMER
        } else {
            APP_GAME_ADAPTER_DISPATCH_RESULT_FALLBACK_BLOCKED
        }
        .to_string(),
        adapter_dispatch_command_result_claimed: accepted,
        adapter_dispatch_executed_claimed: false,
        service_local_execution_audit_claimed: accepted,
        broad_installed_app_blocking_claimed: false,
        child_device_delivery_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        private_diagnostics_claimed: false,
        last_checked_at: generated_at.to_string(),
    }
}

struct CommandHandoffFields {
    enforcement_command_name: Option<String>,
    enforcement_event_name: Option<String>,
    enforcement_action_mode: Option<String>,
    dispatch_command_result_id: Option<String>,
    dispatch_command_audit_refs: Vec<String>,
    dispatch_command_timer_refs: Vec<String>,
}

struct ExecutionAuditFields {
    state: String,
    decision: String,
    id: Option<String>,
    refs: Vec<String>,
}

fn dispatch_command_result_accepted(row: &AppGameAdapterDispatchPreflightRow) -> bool {
    row.dispatch_preflight_state == APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_ELIGIBLE
        && row.dispatch_decision == APP_GAME_ADAPTER_DISPATCH_DECISION_ELIGIBLE
        && row.dispatch_outcome_state == APP_GAME_ADAPTER_DISPATCH_OUTCOME_READY
        && row.adapter_dispatch_eligible
}

fn command_handoff_fields(accepted: bool) -> CommandHandoffFields {
    CommandHandoffFields {
        enforcement_command_name: accepted_field(
            accepted,
            APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_COMMAND,
        ),
        enforcement_event_name: accepted_field(
            accepted,
            APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_EVENT,
        ),
        enforcement_action_mode: accepted_field(
            accepted,
            APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_ACTION_MODE,
        ),
        dispatch_command_result_id: accepted_field(
            accepted,
            APP_GAME_ADAPTER_DISPATCH_RESULT_OWNED_PROCESS_ID,
        ),
        dispatch_command_audit_refs: accepted_refs(
            accepted,
            APP_GAME_ADAPTER_DISPATCH_COMMAND_AUDIT_OWNED_PROCESS,
        ),
        dispatch_command_timer_refs: accepted_refs(
            accepted,
            APP_GAME_ADAPTER_DISPATCH_TIMER_OWNED_PROCESS,
        ),
    }
}

fn execution_audit_fields(accepted: bool) -> ExecutionAuditFields {
    if accepted {
        return ExecutionAuditFields {
            state: APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_STATE_RECORDED.to_string(),
            decision: APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_DECISION_RECORDED.to_string(),
            id: Some(APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_OWNED_PROCESS_ID.to_string()),
            refs: vec![APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_OWNED_PROCESS_REF.to_string()],
        };
    }
    ExecutionAuditFields {
        state: APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_STATE_BLOCKED.to_string(),
        decision: APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_DECISION_BLOCKED.to_string(),
        id: None,
        refs: Vec::new(),
    }
}

fn accepted_field(accepted: bool, value: &str) -> Option<String> {
    accepted.then(|| value.to_string())
}

fn accepted_refs(accepted: bool, value: &str) -> Vec<String> {
    if accepted {
        vec![value.to_string()]
    } else {
        Vec::new()
    }
}

fn dispatch_command_result_state(
    row: &AppGameAdapterDispatchPreflightRow,
    accepted: bool,
) -> &'static str {
    if accepted {
        return APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_ACCEPTED;
    }
    match row.dispatch_preflight_state.as_str() {
        APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_DEGRADED => {
            APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_DEGRADED
        }
        APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_UNAVAILABLE => {
            APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_UNAVAILABLE
        }
        APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_UNSUPPORTED => {
            APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_UNSUPPORTED
        }
        _ => APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_MANUAL_REQUIRED,
    }
}
