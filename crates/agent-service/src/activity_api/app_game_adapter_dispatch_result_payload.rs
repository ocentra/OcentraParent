use ocentra_parent_agent_protocol::app_game::APP_GAME_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::app_game_adapter_dispatch_preflight::{
    AppGameAdapterDispatchPreflightRow, APP_GAME_ADAPTER_DISPATCH_DECISION_ELIGIBLE,
    APP_GAME_ADAPTER_DISPATCH_OUTCOME_READY, APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL_ID,
    APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_DEGRADED,
    APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_ELIGIBLE,
    APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_UNAVAILABLE,
    APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_UNSUPPORTED,
    APP_GAME_ADAPTER_DISPATCH_TIMER_OWNED_PROCESS,
};
use ocentra_parent_agent_protocol::app_game_adapter_dispatch_result::{
    AppGameAdapterDispatchResultReadModel, AppGameAdapterDispatchResultRow,
    APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_BLOCKED,
    APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_MISSING,
    APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_REPORTED,
    APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_REF_PREFIX,
    APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_STATE_BLOCKED,
    APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_STATE_MISSING,
    APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_STATE_REPORTED,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_AUDIT_OWNED_PROCESS,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_ACCEPTED,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_BLOCKED,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_ACCEPTED,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_DEGRADED,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_MANUAL_REQUIRED,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_UNAVAILABLE,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_UNSUPPORTED,
    APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_DECISION_BLOCKED,
    APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_DECISION_RECORDED,
    APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_OWNED_PROCESS_ID,
    APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_OWNED_PROCESS_REF,
    APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_STATE_BLOCKED,
    APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_STATE_RECORDED,
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
    APP_GAME_ADAPTER_DISPATCH_RESULT_STATUS_PARTIAL,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};

use super::app_game_adapter_dispatch_preflight_payload::app_game_adapter_dispatch_preflight_read_model;
use crate::activity_store_path::activity_db_path;
use crate::{event_builder::build_event, fields::fields_from_pairs, time::timestamp_now};
use ocentra_parent_agent_core::activity_store::ActivityStore;
use std::path::PathBuf;

pub async fn build_activity_app_game_adapter_dispatch_result_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    build_activity_app_game_adapter_dispatch_result_report_with_store_path(
        command,
        activity_db_path(),
    )
    .await
}

pub(crate) async fn build_activity_app_game_adapter_dispatch_result_report_with_store_path(
    command: AgentCommandEnvelope,
    store_path: PathBuf,
) -> AgentEventEnvelope {
    let generated_at = timestamp_now();
    let execution_evidence = tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(store_path).ok()?;
        let fields = store.latest_enforcement_audit_fields().ok()??;
        app_game_adapter_dispatch_execution_evidence_from_payload(&fields).ok()
    })
    .await
    .ok()
    .flatten();
    let read_model = app_game_adapter_dispatch_result_read_model_with_execution(
        &generated_at,
        execution_evidence.as_ref(),
    );
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

pub fn app_game_adapter_dispatch_result_read_model_with_execution(
    generated_at: &str,
    execution_evidence: Option<&AppGameAdapterDispatchExecutionEvidence>,
) -> AppGameAdapterDispatchResultReadModel {
    let preflight = app_game_adapter_dispatch_preflight_read_model(generated_at);
    let rows = preflight
        .rows
        .iter()
        .map(|row| dispatch_result_row(row, generated_at, execution_evidence))
        .collect::<Vec<_>>();
    let counts = dispatch_result_counts(&rows);

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
        returned: counts.returned,
        command_accepted_count: counts.command_accepted,
        blocked_before_command_count: counts.blocked_before_command,
        execution_audit_recorded_count: counts.execution_audit_recorded,
        blocked_before_execution_audit_count: counts.blocked_before_execution_audit,
        adapter_execution_reported_count: counts.adapter_execution_reported,
        adapter_execution_evidence_missing_count: counts.adapter_execution_evidence_missing,
        blocked_before_adapter_execution_count: counts.blocked_before_adapter_execution,
        adapter_dispatch_command_result_claimed_count: counts.command_result_claimed,
        service_local_execution_audit_claimed_count: counts.service_local_audit_claimed,
        adapter_dispatch_executed_claimed_count: counts.adapter_dispatch_executed_claimed,
        broad_installed_app_blocking_claimed: false,
        child_device_delivery_claimed: false,
        platform_enforcement_claimed: rows.iter().any(|row| row.platform_enforcement_claimed),
        provider_delivery_claimed: false,
        private_diagnostics_claimed: false,
        rows,
    }
}

struct DispatchResultCounts {
    returned: u64,
    command_accepted: u64,
    blocked_before_command: u64,
    execution_audit_recorded: u64,
    blocked_before_execution_audit: u64,
    adapter_execution_reported: u64,
    adapter_execution_evidence_missing: u64,
    blocked_before_adapter_execution: u64,
    command_result_claimed: u64,
    service_local_audit_claimed: u64,
    adapter_dispatch_executed_claimed: u64,
}

fn dispatch_result_counts(rows: &[AppGameAdapterDispatchResultRow]) -> DispatchResultCounts {
    DispatchResultCounts {
        returned: rows.len() as u64,
        command_accepted: count_rows(rows, |row| {
            row.dispatch_command_result_decision
                == APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_ACCEPTED
        }),
        blocked_before_command: count_rows(rows, |row| {
            row.dispatch_command_result_decision
                == APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_BLOCKED
        }),
        execution_audit_recorded: count_rows(rows, |row| {
            row.dispatch_execution_audit_decision
                == APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_DECISION_RECORDED
        }),
        blocked_before_execution_audit: count_rows(rows, |row| {
            row.dispatch_execution_audit_decision
                == APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_DECISION_BLOCKED
        }),
        adapter_execution_reported: count_rows(rows, |row| {
            row.dispatch_adapter_execution_decision
                == APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_REPORTED
        }),
        adapter_execution_evidence_missing: count_rows(rows, |row| {
            row.dispatch_adapter_execution_decision
                == APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_MISSING
        }),
        blocked_before_adapter_execution: count_rows(rows, |row| {
            row.dispatch_adapter_execution_decision
                == APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_BLOCKED
        }),
        command_result_claimed: count_rows(rows, |row| row.adapter_dispatch_command_result_claimed),
        service_local_audit_claimed: count_rows(rows, |row| {
            row.service_local_execution_audit_claimed
        }),
        adapter_dispatch_executed_claimed: count_rows(rows, |row| {
            row.adapter_dispatch_executed_claimed
        }),
    }
}

fn count_rows<F>(rows: &[AppGameAdapterDispatchResultRow], predicate: F) -> u64
where
    F: Fn(&AppGameAdapterDispatchResultRow) -> bool,
{
    rows.iter().filter(|row| predicate(row)).count() as u64
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
            LogFieldValue::String(serde_json::to_string(read_model).unwrap_or_default()),
        ),
    ])
}

fn dispatch_result_row(
    row: &AppGameAdapterDispatchPreflightRow,
    generated_at: &str,
    execution_evidence: Option<&AppGameAdapterDispatchExecutionEvidence>,
) -> AppGameAdapterDispatchResultRow {
    let accepted = dispatch_command_result_accepted(row);
    let command = command_handoff_fields(accepted);
    let audit = execution_audit_fields(accepted);
    let adapter_execution = adapter_execution_fields(accepted, execution_evidence);
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
        dispatch_execution_audit_id: audit.audit_ref,
        dispatch_execution_audit_refs: audit.refs,
        dispatch_adapter_execution_state: adapter_execution.state,
        dispatch_adapter_execution_decision: adapter_execution.decision,
        dispatch_adapter_execution_result_id: adapter_execution.result_id,
        dispatch_adapter_execution_status: adapter_execution.status_text,
        dispatch_adapter_execution_adapter_result_code: adapter_execution.adapter_result_code,
        dispatch_adapter_execution_audit_event_id: adapter_execution.audit_event_id,
        dispatch_adapter_execution_refs: adapter_execution.refs,
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
        adapter_dispatch_executed_claimed: adapter_execution.executed_claimed,
        service_local_execution_audit_claimed: accepted,
        broad_installed_app_blocking_claimed: false,
        child_device_delivery_claimed: false,
        platform_enforcement_claimed: adapter_execution.platform_enforcement_claimed,
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

type ExecutionAuditRefText = Option<String>;
type AdapterExecutionStatusText = Option<String>;
type DispatchExecutionStatusText = String;

struct ExecutionAuditFields {
    state: String,
    decision: String,
    audit_ref: ExecutionAuditRefText,
    refs: Vec<String>,
}

struct AdapterExecutionFields {
    state: String,
    decision: String,
    result_id: Option<String>,
    status_text: AdapterExecutionStatusText,
    adapter_result_code: Option<String>,
    audit_event_id: Option<String>,
    refs: Vec<String>,
    executed_claimed: bool,
    platform_enforcement_claimed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameAdapterDispatchExecutionEvidence {
    pub result_id: String,
    pub status_text: DispatchExecutionStatusText,
    pub adapter_result_code: String,
    pub audit_event_id: String,
}

pub fn app_game_adapter_dispatch_execution_evidence_from_payload(
    payload: &LogFields,
) -> Result<AppGameAdapterDispatchExecutionEvidence, &'static str> {
    Ok(AppGameAdapterDispatchExecutionEvidence {
        result_id: required_string(payload, constants::field::ENFORCEMENT_RESULT_ID)?.to_string(),
        status_text: required_string(payload, constants::field::ENFORCEMENT_STATUS)?.to_string(),
        adapter_result_code: required_string(
            payload,
            constants::field::ENFORCEMENT_ADAPTER_RESULT_CODE,
        )?
        .to_string(),
        audit_event_id: required_string(payload, constants::field::ENFORCEMENT_AUDIT_EVENT_ID)?
            .to_string(),
    })
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
            audit_ref: Some(APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_OWNED_PROCESS_ID.to_string()),
            refs: vec![APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_OWNED_PROCESS_REF.to_string()],
        };
    }
    ExecutionAuditFields {
        state: APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_STATE_BLOCKED.to_string(),
        decision: APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_DECISION_BLOCKED.to_string(),
        audit_ref: None,
        refs: Vec::new(),
    }
}

fn adapter_execution_fields(
    accepted: bool,
    evidence: Option<&AppGameAdapterDispatchExecutionEvidence>,
) -> AdapterExecutionFields {
    if !accepted {
        return empty_adapter_execution_fields(
            APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_STATE_BLOCKED,
            APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_BLOCKED,
        );
    }

    let Some(evidence) = evidence else {
        return empty_adapter_execution_fields(
            APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_STATE_MISSING,
            APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_MISSING,
        );
    };

    let mut execution_ref = String::from(APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_REF_PREFIX);
    execution_ref.push_str(&evidence.audit_event_id);

    AdapterExecutionFields {
        state: APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_STATE_REPORTED.to_string(),
        decision: APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_REPORTED.to_string(),
        result_id: Some(evidence.result_id.clone()),
        status_text: Some(evidence.status_text.clone()),
        adapter_result_code: Some(evidence.adapter_result_code.clone()),
        audit_event_id: Some(evidence.audit_event_id.clone()),
        refs: vec![execution_ref],
        executed_claimed: evidence.status_text == constants::enforcement::RESULT_ACTUALLY_ENFORCED,
        platform_enforcement_claimed: evidence.status_text
            == constants::enforcement::RESULT_ACTUALLY_ENFORCED,
    }
}

fn empty_adapter_execution_fields(state: &str, decision: &str) -> AdapterExecutionFields {
    AdapterExecutionFields {
        state: state.to_string(),
        decision: decision.to_string(),
        result_id: None,
        status_text: None,
        adapter_result_code: None,
        audit_event_id: None,
        refs: Vec::new(),
        executed_claimed: false,
        platform_enforcement_claimed: false,
    }
}

fn required_string<'a>(payload: &'a LogFields, field: &str) -> Result<&'a str, &'static str> {
    match payload.get(field) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => Ok(value.trim()),
        _ => Err(constants::enforcement::REJECTION_COMMAND_PAYLOAD_INVALID),
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
