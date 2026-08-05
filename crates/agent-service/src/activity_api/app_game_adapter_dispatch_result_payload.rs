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
use super::app_game_adapter_dispatch_result_fields::required_string;
use super::app_game_adapter_execution_readiness_payload::GeneratedAtText;
use crate::activity_store_path::activity_db_path;
use crate::{event_builder::build_event, fields::fields_from_pairs, time::timestamp_now};
use ocentra_parent_agent_core::activity_store::ActivityStore;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ActivityStorePath(pub(crate) PathBuf);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct StaticText(pub(crate) &'static str);

pub async fn build_activity_app_game_adapter_dispatch_result_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    build_activity_app_game_adapter_dispatch_result_report_with_store_path(
        command,
        ActivityStorePath(activity_db_path().into()),
    )
    .await
}

pub(crate) async fn build_activity_app_game_adapter_dispatch_result_report_with_store_path(
    command: AgentCommandEnvelope,
    store_path: ActivityStorePath,
) -> AgentEventEnvelope {
    let generated_at: String = timestamp_now();
    let execution_evidence = tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(store_path.0).ok()?;
        let fields = store
            .latest_matching_enforcement_audit_fields(|fields| {
                app_game_adapter_dispatch_execution_evidence(fields).is_some()
            })
            .ok()??;
        app_game_adapter_dispatch_execution_evidence(&fields)
    })
    .await
    .ok()
    .flatten();
    let read_model = app_game_adapter_dispatch_result_read_model(
        GeneratedAtText(generated_at),
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

pub(crate) fn app_game_adapter_dispatch_execution_evidence(
    fields: &LogFields,
) -> Option<AppGameAdapterDispatchExecutionEvidence> {
    let source_read_model_id =
        required_string(fields, StaticText(constants::field::SOURCE_READ_MODEL_ID)).ok()?;
    if source_read_model_id.0 != APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL_ID {
        return None;
    }
    required_string(
        fields,
        StaticText(constants::field::ENFORCEMENT_AUDIT_EVENT),
    )
    .ok()?;
    let status_text =
        required_string(fields, StaticText(constants::field::ENFORCEMENT_STATUS)).ok()?;

    Some(AppGameAdapterDispatchExecutionEvidence {
        result_id: required_string(fields, StaticText(constants::field::ENFORCEMENT_RESULT_ID))
            .ok()?
            .0,
        status_text,
        adapter_result_code: required_string(
            fields,
            StaticText(constants::field::ENFORCEMENT_ADAPTER_RESULT_CODE),
        )
        .ok()?
        .0,
        audit_event_id: required_string(
            fields,
            StaticText(constants::field::ENFORCEMENT_AUDIT_EVENT_ID),
        )
        .ok()?
        .0,
    })
}

pub(crate) struct DispatchResultCounts {
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

pub(crate) fn dispatch_result_counts(
    rows: &[AppGameAdapterDispatchResultRow],
) -> DispatchResultCounts {
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

pub(crate) fn app_game_adapter_dispatch_result_read_model(
    generated_at: GeneratedAtText,
    execution_evidence: Option<&AppGameAdapterDispatchExecutionEvidence>,
) -> AppGameAdapterDispatchResultReadModel {
    let preflight = app_game_adapter_dispatch_preflight_read_model(generated_at.clone());
    let rows = preflight
        .rows
        .iter()
        .map(|row| {
            dispatch_result_row(
                row,
                GeneratedAtText(generated_at.0.clone()),
                execution_evidence,
            )
        })
        .collect::<Vec<_>>();
    let counts = dispatch_result_counts(&rows);

    AppGameAdapterDispatchResultReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        read_model_id: APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL_ID.to_string(),
        generated_at: generated_at.0,
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

pub(crate) fn dispatch_result_row(
    row: &AppGameAdapterDispatchPreflightRow,
    generated_at: GeneratedAtText,
    execution_evidence: Option<&AppGameAdapterDispatchExecutionEvidence>,
) -> AppGameAdapterDispatchResultRow {
    let accepted = dispatch_command_result_accepted(row);
    let command = command_handoff_fields(accepted);
    let audit = execution_audit_fields(accepted);
    let adapter_execution = adapter_execution_fields(accepted, execution_evidence);
    let mut row_id = String::from(APP_GAME_ADAPTER_DISPATCH_RESULT_ROW_ID_PREFIX);
    row_id.push_str(&row.source_proof_entry_id);
    let route = dispatch_result_route(row, accepted);

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
        dispatch_command_result_state: dispatch_command_result_state(row, accepted).0.to_string(),
        dispatch_command_result_decision: route.dispatch_command_result_decision.to_string(),
        enforcement_command_name: command.enforcement_command_name,
        enforcement_event_name: command.enforcement_event_name,
        enforcement_action_mode: command.enforcement_action_mode,
        dispatch_command_result_id: command.dispatch_command_result_id,
        dispatch_command_audit_refs: command.dispatch_command_audit_refs,
        dispatch_command_timer_refs: command.dispatch_command_timer_refs,
        dispatch_execution_audit_state: audit.state,
        dispatch_execution_audit_decision: audit.decision,
        dispatch_execution_audit_id: audit.audit_ref.0,
        dispatch_execution_audit_refs: audit.refs,
        dispatch_adapter_execution_state: adapter_execution.state,
        dispatch_adapter_execution_decision: adapter_execution.decision,
        dispatch_adapter_execution_result_id: adapter_execution.result_id,
        dispatch_adapter_execution_status: adapter_execution.status_text.0,
        dispatch_adapter_execution_adapter_result_code: adapter_execution.adapter_result_code,
        dispatch_adapter_execution_audit_event_id: adapter_execution.audit_event_id,
        dispatch_adapter_execution_refs: adapter_execution.refs,
        manual_proof_requirements: route
            .manual_proof_requirements
            .unwrap_or_else(|| row.manual_proof_requirements.clone()),
        claim_boundary: route.claim_boundary.to_string(),
        fallback_behavior: route.fallback_behavior.to_string(),
        adapter_dispatch_command_result_claimed: route.adapter_dispatch_command_result_claimed,
        adapter_dispatch_executed_claimed: adapter_execution.executed_claimed,
        service_local_execution_audit_claimed: route.service_local_execution_audit_claimed,
        broad_installed_app_blocking_claimed: false,
        child_device_delivery_claimed: false,
        platform_enforcement_claimed: adapter_execution.platform_enforcement_claimed,
        provider_delivery_claimed: false,
        private_diagnostics_claimed: false,
        last_checked_at: generated_at.0,
    }
}

struct DispatchResultRoute {
    dispatch_command_result_decision: &'static str,
    manual_proof_requirements: Option<Vec<String>>,
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
    adapter_dispatch_command_result_claimed: bool,
    service_local_execution_audit_claimed: bool,
}

fn dispatch_result_route(
    row: &AppGameAdapterDispatchPreflightRow,
    accepted: bool,
) -> DispatchResultRoute {
    if accepted {
        return DispatchResultRoute {
            dispatch_command_result_decision:
                APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_ACCEPTED,
            manual_proof_requirements: Some(Vec::new()),
            claim_boundary: APP_GAME_ADAPTER_DISPATCH_RESULT_CLAIM_SCOPED_TIMER,
            fallback_behavior: APP_GAME_ADAPTER_DISPATCH_RESULT_FALLBACK_SCOPED_TIMER,
            adapter_dispatch_command_result_claimed: true,
            service_local_execution_audit_claimed: true,
        };
    }

    DispatchResultRoute {
        dispatch_command_result_decision: APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_BLOCKED,
        manual_proof_requirements: Some(row.manual_proof_requirements.clone()),
        claim_boundary: APP_GAME_ADAPTER_DISPATCH_RESULT_CLAIM_BLOCKED,
        fallback_behavior: APP_GAME_ADAPTER_DISPATCH_RESULT_FALLBACK_BLOCKED,
        adapter_dispatch_command_result_claimed: false,
        service_local_execution_audit_claimed: false,
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct OptionalText(Option<String>);

#[derive(Clone, Debug, PartialEq, Eq)]
struct StringList(Vec<String>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DispatchExecutionStatusText(pub String);

struct ExecutionAuditFields {
    state: String,
    decision: String,
    audit_ref: OptionalText,
    refs: Vec<String>,
}

struct AdapterExecutionFields {
    state: String,
    decision: String,
    result_id: Option<String>,
    status_text: OptionalText,
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
            StaticText(APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_COMMAND),
        )
        .0,
        enforcement_event_name: accepted_field(
            accepted,
            StaticText(APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_EVENT),
        )
        .0,
        enforcement_action_mode: accepted_field(
            accepted,
            StaticText(APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_ACTION_MODE),
        )
        .0,
        dispatch_command_result_id: accepted_field(
            accepted,
            StaticText(APP_GAME_ADAPTER_DISPATCH_RESULT_OWNED_PROCESS_ID),
        )
        .0,
        dispatch_command_audit_refs: accepted_refs(
            accepted,
            StaticText(APP_GAME_ADAPTER_DISPATCH_COMMAND_AUDIT_OWNED_PROCESS),
        )
        .0,
        dispatch_command_timer_refs: accepted_refs(
            accepted,
            StaticText(APP_GAME_ADAPTER_DISPATCH_TIMER_OWNED_PROCESS),
        )
        .0,
    }
}

fn execution_audit_fields(accepted: bool) -> ExecutionAuditFields {
    if accepted {
        return ExecutionAuditFields {
            state: APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_STATE_RECORDED.to_string(),
            decision: APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_DECISION_RECORDED.to_string(),
            audit_ref: OptionalText(Some(
                APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_OWNED_PROCESS_ID.to_string(),
            )),
            refs: vec![APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_OWNED_PROCESS_REF.to_string()],
        };
    }
    ExecutionAuditFields {
        state: APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_STATE_BLOCKED.to_string(),
        decision: APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_DECISION_BLOCKED.to_string(),
        audit_ref: OptionalText(None),
        refs: Vec::new(),
    }
}

fn adapter_execution_fields(
    accepted: bool,
    evidence: Option<&AppGameAdapterDispatchExecutionEvidence>,
) -> AdapterExecutionFields {
    if !accepted {
        return empty_adapter_execution_fields(
            StaticText(APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_STATE_BLOCKED),
            StaticText(APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_BLOCKED),
        );
    }

    let Some(evidence) = evidence else {
        return empty_adapter_execution_fields(
            StaticText(APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_STATE_MISSING),
            StaticText(APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_MISSING),
        );
    };

    let mut execution_ref = String::from(APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_REF_PREFIX);
    execution_ref.push_str(&evidence.audit_event_id);

    AdapterExecutionFields {
        state: APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_STATE_REPORTED.to_string(),
        decision: APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_REPORTED.to_string(),
        result_id: Some(evidence.result_id.clone()),
        status_text: OptionalText(Some(evidence.status_text.0.clone())),
        adapter_result_code: Some(evidence.adapter_result_code.clone()),
        audit_event_id: Some(evidence.audit_event_id.clone()),
        refs: vec![execution_ref],
        executed_claimed: evidence.status_text.0
            == constants::enforcement::RESULT_ACTUALLY_ENFORCED,
        platform_enforcement_claimed: evidence.status_text.0
            == constants::enforcement::RESULT_ACTUALLY_ENFORCED,
    }
}

fn empty_adapter_execution_fields(
    state: StaticText,
    decision: StaticText,
) -> AdapterExecutionFields {
    AdapterExecutionFields {
        state: state.0.to_string(),
        decision: decision.0.to_string(),
        result_id: None,
        status_text: OptionalText(None),
        adapter_result_code: None,
        audit_event_id: None,
        refs: Vec::new(),
        executed_claimed: false,
        platform_enforcement_claimed: false,
    }
}

fn accepted_field(accepted: bool, value: StaticText) -> OptionalText {
    OptionalText(accepted.then(|| value.0.to_string()))
}

fn accepted_refs(accepted: bool, value: StaticText) -> StringList {
    if accepted {
        StringList(vec![value.0.to_string()])
    } else {
        StringList(Vec::new())
    }
}

fn dispatch_command_result_state(
    row: &AppGameAdapterDispatchPreflightRow,
    accepted: bool,
) -> StaticText {
    if accepted {
        return StaticText(APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_ACCEPTED);
    }
    match row.dispatch_preflight_state.as_str() {
        APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_DEGRADED => {
            StaticText(APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_DEGRADED)
        }
        APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_UNAVAILABLE => {
            StaticText(APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_UNAVAILABLE)
        }
        APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_UNSUPPORTED => {
            StaticText(APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_UNSUPPORTED)
        }
        _ => StaticText(APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_MANUAL_REQUIRED),
    }
}
