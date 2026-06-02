use std::path::PathBuf;

use ocentra_parent_agent_core::{
    authorize_enforcement_boundary, evaluate_enforcement_boundary, terminate_owned_process,
    EnforcementAdapterOutcome, EnforcementBoundaryInput, EnforcementBoundaryOutcome,
    OwnedProcessTerminationTarget,
};
use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, ActivityIngestStatus, ActivityObserver,
    ActivitySource, ActivitySubject, ActivitySubjectKind, AgentCommandEnvelope, AgentEventEnvelope,
    AgentEventName, EnforcementAdapterKind, EnforcementMode, LogFieldValue, LogFields, LogLevel,
    ACTIVITY_SCHEMA_VERSION,
};

use crate::{
    activity_capture::record_activity_events_to_paths,
    activity_store_path::{activity_db_path, activity_journal_key_path, activity_journal_path},
    enforcement_os_adapter_product_proof_read_model::product_control_spine::v08_enforcement_product_control_spine_read_model,
    enforcement_payload::{parse_enforcement_command_payload, EnforcementCommandPayload},
    enforcement_policy_dispatch_read_model::v08_enforcement_policy_dispatch_read_model,
    enforcement_timer_state_file::store_active_timer_state_for_outcome,
    enforcement_timer_state_path::enforcement_timer_state_path,
    event_builder::build_event,
    fields::fields_from_pairs,
    time::timestamp_now,
};

mod enforcement_broad_adapter_proof_payload;
mod enforcement_broad_adapter_proof_read_model;
#[cfg(test)]
mod enforcement_broad_adapter_proof_read_model_tests;
mod enforcement_broad_adapter_proof_report;
mod enforcement_product_control_payload;

pub use self::enforcement_broad_adapter_proof_report::build_enforcement_broad_adapter_proof_report;
use self::enforcement_product_control_payload::{
    enforcement_policy_dispatch_payload, enforcement_product_control_spine_payload,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EnforcementJournalPaths {
    pub journal_path: PathBuf,
    pub key_path: PathBuf,
    pub store_path: PathBuf,
    pub timer_state_path: PathBuf,
}

impl EnforcementJournalPaths {
    pub(crate) fn from_environment() -> Self {
        Self {
            journal_path: activity_journal_path(),
            key_path: activity_journal_key_path(),
            store_path: activity_db_path(),
            timer_state_path: enforcement_timer_state_path(),
        }
    }
}

pub async fn build_enforcement_audit_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_enforcement_audit_report_with_paths(command, EnforcementJournalPaths::from_environment())
        .await
}

pub async fn build_enforcement_product_control_spine_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let generated_at = timestamp_now();
    let read_model = v08_enforcement_product_control_spine_read_model(&generated_at);
    build_event(
        constants::event_id::ENFORCEMENT_PRODUCT_CONTROL_SPINE_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentEnforcementProductControlSpineReported,
        LogLevel::Info,
        enforcement_product_control_spine_payload(&read_model),
        None,
    )
}

pub async fn build_enforcement_policy_dispatch_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let generated_at = timestamp_now();
    let read_model = v08_enforcement_policy_dispatch_read_model(&generated_at);
    build_event(
        constants::event_id::ENFORCEMENT_POLICY_DISPATCH_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentEnforcementPolicyDispatchReported,
        LogLevel::Info,
        enforcement_policy_dispatch_payload(&read_model),
        None,
    )
}

pub(crate) async fn build_enforcement_audit_report_with_paths(
    command: AgentCommandEnvelope,
    paths: EnforcementJournalPaths,
) -> AgentEventEnvelope {
    let target = command.source.clone();
    let correlation_id = command.message_id.clone();
    match execute_enforcement_command(command, paths).await {
        Ok(payload) => build_event(
            constants::event_id::ENFORCEMENT_AUDIT_REPORTED,
            &correlation_id,
            target,
            AgentEventName::AgentEnforcementAuditReported,
            LogLevel::Info,
            payload,
            None,
        ),
        Err(reason) => build_event(
            constants::event_id::COMMAND_REJECTED,
            &correlation_id,
            target,
            AgentEventName::AgentCommandRejected,
            LogLevel::Warn,
            fields_from_pairs(vec![(
                constants::field::REASON,
                LogFieldValue::String(reason.to_string()),
            )]),
            None,
        ),
    }
}

async fn execute_enforcement_command(
    command: AgentCommandEnvelope,
    paths: EnforcementJournalPaths,
) -> Result<LogFields, &'static str> {
    let observed_at = timestamp_now();
    let request = parse_enforcement_command_payload(&command, &observed_at)?;
    let authorization = authorize_enforcement_boundary(request.input.clone())
        .map_err(|error| error.as_protocol_str())?;
    let completed_at = timestamp_now();
    let adapter_outcome = match authorization.adapter_request {
        Some(adapter_request) => Some(adapter_outcome_for_request(
            &request,
            &authorization.action,
            adapter_request.adapter_kind,
            adapter_request.mode,
            &completed_at,
        )?),
        None => None,
    };
    let outcome_input = final_input(request.input.clone(), adapter_outcome, &completed_at);
    let mut outcome =
        evaluate_enforcement_boundary(outcome_input).map_err(|error| error.as_protocol_str())?;
    outcome.audit_event.journal_sequence = Some(outcome.audit_event.audit_event_id.clone());
    let status = record_enforcement_audit(&request, &outcome, &paths).await?;
    let active_state =
        store_active_timer_state_for_outcome(&outcome, &paths.timer_state_path, &completed_at)
            .await?;

    enforcement_report_payload(&outcome, &status, active_state.as_ref())
}

fn adapter_outcome_for_request(
    request: &EnforcementCommandPayload,
    action: &ocentra_parent_agent_protocol::EnforcementAction,
    adapter_kind: EnforcementAdapterKind,
    mode: EnforcementMode,
    completed_at: &str,
) -> Result<EnforcementAdapterOutcome, &'static str> {
    match (adapter_kind, mode) {
        (EnforcementAdapterKind::ProcessControl, EnforcementMode::TerminateProcess) => {
            let pid = request
                .process_id
                .ok_or(constants::enforcement::REJECTION_PROCESS_ID_REQUIRED)?;
            Ok(terminate_owned_process(
                OwnedProcessTerminationTarget {
                    pid,
                    expected_process_name: action.target.target_value.clone(),
                },
                completed_at,
            ))
        }
        _ => Err(constants::enforcement::REJECTION_UNSUPPORTED_CAPABILITY),
    }
}

fn final_input(
    mut input: EnforcementBoundaryInput,
    adapter_outcome: Option<EnforcementAdapterOutcome>,
    completed_at: &str,
) -> EnforcementBoundaryInput {
    input.completed_at = Some(completed_at.to_string());
    input.adapter_outcome = adapter_outcome;
    input
}

async fn record_enforcement_audit(
    request: &EnforcementCommandPayload,
    outcome: &EnforcementBoundaryOutcome,
    paths: &EnforcementJournalPaths,
) -> Result<ActivityIngestStatus, &'static str> {
    let event = enforcement_activity_event(request, outcome)?;
    let journal_path = paths.journal_path.clone();
    let key_path = paths.key_path.clone();
    let store_path = paths.store_path.clone();
    tokio::task::spawn_blocking(move || {
        record_activity_events_to_paths(&journal_path, &key_path, &store_path, &[event])
    })
    .await
    .map_err(|_| constants::value::ACTIVITY_CAPTURE_STORE_ERROR)?
    .map_err(|_| constants::value::ACTIVITY_CAPTURE_STORE_ERROR)
}

fn enforcement_activity_event(
    request: &EnforcementCommandPayload,
    outcome: &EnforcementBoundaryOutcome,
) -> Result<ActivityEvent, &'static str> {
    Ok(ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: outcome.audit_event.audit_event_id.clone(),
        observed_at: outcome.audit_event.observed_at.clone(),
        source: ActivitySource {
            device_id: request.device_id.clone(),
            platform: request.platform.clone(),
            observer: ActivityObserver::AgentService,
            source_id: constants::enforcement::SOURCE_ID_AGENT_SERVICE.to_string(),
        },
        kind: ActivityEventKind::EnforcementAuditRecorded,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Intervention,
            subject_id: outcome.action.action_id.clone(),
            display_name: Some(outcome.action.mode.as_protocol_str().to_string()),
        },
        fields: enforcement_journal_fields(outcome)?,
        evidence: Vec::new(),
    })
}

fn enforcement_journal_fields(
    outcome: &EnforcementBoundaryOutcome,
) -> Result<LogFields, &'static str> {
    let mut fields = base_enforcement_field_pairs(outcome);
    fields.extend(serialized_enforcement_field_pairs(outcome)?);
    Ok(fields_from_pairs(fields))
}

fn base_enforcement_field_pairs(
    outcome: &EnforcementBoundaryOutcome,
) -> Vec<(&'static str, LogFieldValue)> {
    vec![
        (
            constants::field::POLICY_DECISION_ID,
            LogFieldValue::String(outcome.action.policy_decision_id.clone()),
        ),
        (
            constants::field::POLICY_ACTION,
            LogFieldValue::String(outcome.action.policy_action.as_protocol_str().to_string()),
        ),
        (
            constants::field::POLICY_TARGET_TYPE,
            LogFieldValue::String(
                outcome
                    .action
                    .target
                    .target_type
                    .as_protocol_str()
                    .to_string(),
            ),
        ),
        (
            constants::field::POLICY_TARGET_VALUE,
            LogFieldValue::String(outcome.action.target.target_value.clone()),
        ),
        (
            constants::field::ENFORCEMENT_ACTION_ID,
            LogFieldValue::String(outcome.action.action_id.clone()),
        ),
        (
            constants::field::ENFORCEMENT_RESULT_ID,
            LogFieldValue::String(outcome.result.result_id.clone()),
        ),
        (
            constants::field::ENFORCEMENT_AUDIT_EVENT_ID,
            LogFieldValue::String(outcome.audit_event.audit_event_id.clone()),
        ),
        (
            constants::field::ENFORCEMENT_STATUS,
            LogFieldValue::String(outcome.result.status.as_protocol_str().to_string()),
        ),
        (
            constants::field::ENFORCEMENT_ADAPTER_RESULT_CODE,
            LogFieldValue::String(
                outcome
                    .result
                    .adapter_result_code
                    .as_protocol_str()
                    .to_string(),
            ),
        ),
        (
            constants::field::ENFORCEMENT_ROLLBACK_STATE,
            LogFieldValue::String(outcome.result.rollback_state.as_protocol_str().to_string()),
        ),
        (
            constants::field::ENFORCEMENT_CAPABILITY_STATE,
            LogFieldValue::String(
                outcome
                    .result
                    .capability
                    .capability_state
                    .as_protocol_str()
                    .to_string(),
            ),
        ),
        (
            constants::field::EVIDENCE_REFERENCE_IDS,
            LogFieldValue::String(evidence_reference_ids(outcome)),
        ),
    ]
}

fn serialized_enforcement_field_pairs(
    outcome: &EnforcementBoundaryOutcome,
) -> Result<Vec<(&'static str, LogFieldValue)>, &'static str> {
    Ok(vec![
        (
            constants::field::ENFORCEMENT_ACTION,
            LogFieldValue::String(
                serde_json::to_string(&outcome.action)
                    .map_err(|_| constants::error::AGENT_EVENT_SERIALIZES)?,
            ),
        ),
        (
            constants::field::ENFORCEMENT_RESULT,
            LogFieldValue::String(
                serde_json::to_string(&outcome.result)
                    .map_err(|_| constants::error::AGENT_EVENT_SERIALIZES)?,
            ),
        ),
        (
            constants::field::ENFORCEMENT_AUDIT_EVENT,
            LogFieldValue::String(
                serde_json::to_string(&outcome.audit_event)
                    .map_err(|_| constants::error::AGENT_EVENT_SERIALIZES)?,
            ),
        ),
        (
            constants::field::ENFORCEMENT_TIMER_EVENT,
            optional_timer_event(outcome)?,
        ),
    ])
}

fn enforcement_report_payload(
    outcome: &EnforcementBoundaryOutcome,
    status: &ActivityIngestStatus,
    active_state: Option<&ocentra_parent_agent_protocol::EnforcementActiveTimerState>,
) -> Result<LogFields, &'static str> {
    let mut payload = enforcement_journal_fields(outcome)?;
    payload.insert(
        constants::field::DATABASE_READY.to_string(),
        LogFieldValue::Boolean(status.database_ready),
    );
    payload.insert(
        constants::field::EVENTS_INGESTED.to_string(),
        LogFieldValue::Number(status.events_ingested as f64),
    );
    payload.insert(
        constants::field::EVENTS_STORED.to_string(),
        LogFieldValue::Number(status.events_stored as f64),
    );
    payload.insert(
        constants::field::ENFORCEMENT_JOURNAL_EVENT_ID.to_string(),
        optional_string_value(status.last_event_id.as_deref()),
    );
    if let Some(timer) = &outcome.timer_event {
        payload.insert(
            constants::field::ENFORCEMENT_TIMER_EVENT_ID.to_string(),
            LogFieldValue::String(timer.timer_event_id.clone()),
        );
        payload.insert(
            constants::field::ENFORCEMENT_TIMER_EVENT_KIND.to_string(),
            LogFieldValue::String(timer.timer_event_kind.as_protocol_str().to_string()),
        );
    }
    payload.insert(
        constants::field::ENFORCEMENT_TIMER_STATE.to_string(),
        optional_timer_state(active_state)?,
    );
    Ok(payload)
}

fn optional_timer_event(
    outcome: &EnforcementBoundaryOutcome,
) -> Result<LogFieldValue, &'static str> {
    match &outcome.timer_event {
        Some(timer) => Ok(LogFieldValue::String(
            serde_json::to_string(timer).map_err(|_| constants::error::AGENT_EVENT_SERIALIZES)?,
        )),
        None => Ok(LogFieldValue::Null(())),
    }
}

fn optional_string_value(value: Option<&str>) -> LogFieldValue {
    value
        .map(|item| LogFieldValue::String(item.to_string()))
        .unwrap_or(LogFieldValue::Null(()))
}

fn optional_timer_state(
    active_state: Option<&ocentra_parent_agent_protocol::EnforcementActiveTimerState>,
) -> Result<LogFieldValue, &'static str> {
    match active_state {
        Some(state) => Ok(LogFieldValue::String(
            serde_json::to_string(state).map_err(|_| constants::error::AGENT_EVENT_SERIALIZES)?,
        )),
        None => Ok(LogFieldValue::Null(())),
    }
}

fn evidence_reference_ids(outcome: &EnforcementBoundaryOutcome) -> String {
    outcome
        .action
        .evidence_references
        .iter()
        .map(|reference| reference.evidence_reference_id.as_str())
        .collect::<Vec<_>>()
        .join(&constants::delimiter::LIST.to_string())
}
