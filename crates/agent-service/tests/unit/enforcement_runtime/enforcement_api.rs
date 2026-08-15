use std::{env, path::PathBuf};

use ocentra_parent_agent_core::{
    enforcement_adapter::{
        terminate_owned_process, EnforcementAdapterOutcome, OwnedProcessTerminationTarget,
    },
    enforcement_boundary::{
        authorize_enforcement_boundary, evaluate_enforcement_boundary, EnforcementBoundaryInput,
        EnforcementBoundaryOutcome,
    },
};
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::activity::ActivityEventKind;
use ocentra_parent_agent_protocol::activity::ActivityObserver;
use ocentra_parent_agent_protocol::activity::ActivitySource;
use ocentra_parent_agent_protocol::activity::ActivitySubject;
use ocentra_parent_agent_protocol::activity::ActivitySubjectKind;
use ocentra_parent_agent_protocol::activity::ACTIVITY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::activity_query::ActivityIngestStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::enforcement::EnforcementAdapterKind;
use ocentra_parent_agent_protocol::enforcement::EnforcementMode;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::activity_capture::record_activity_events_to_paths;
use crate::enforcement_payload::{
    parse_enforcement_command_payload, EnforcementCommandPayload, EnforcementText,
};
use crate::enforcement_pre_action_journal::journal_before_action_outcome;
use crate::enforcement_timer_state_file::store_active_timer_state_for_outcome_with_app_game_session;
use crate::event_builder::build_event;
use crate::fields::fields_from_pairs;
use crate::test_text::TestText;
use crate::time::timestamp_now;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EnforcementJournalPaths {
    pub journal_path: PathBuf,
    pub key_path: PathBuf,
    pub store_path: PathBuf,
    pub timer_state_path: crate::enforcement_timer_state_path::EnforcementTimerStatePath,
}

impl EnforcementJournalPaths {
    pub(crate) fn from_environment() -> Self {
        Self {
            journal_path: crate::activity_store_path::activity_journal_path().into(),
            key_path: crate::activity_store_path::activity_journal_key_path().into(),
            store_path: crate::activity_store_path::activity_db_path().into(),
            timer_state_path: env::var(constants::env_var::AGENT_ENFORCEMENT_TIMER_STATE_PATH)
                .map(PathBuf::from)
                .map(crate::enforcement_timer_state_path::EnforcementTimerStatePath)
                .unwrap_or_else(|_| {
                    let mut path = env::temp_dir();
                    path.push(constants::enforcement::TIMER_STATE_FILE_NAME);
                    crate::enforcement_timer_state_path::EnforcementTimerStatePath(path)
                }),
        }
    }
}

pub(crate) async fn build_enforcement_audit_report_with_paths(
    command: AgentCommandEnvelope,
    paths: EnforcementJournalPaths,
) -> AgentEventEnvelope {
    let target = command.source.clone();
    let correlation_id = command.message_id.clone();
    let report = execute_enforcement_command(command, paths)
        .await
        .map(enforcement_success_report)
        .unwrap_or_else(enforcement_rejection_report);

    build_event(
        report.event_id,
        &correlation_id,
        target,
        report.event_name,
        report.level,
        report.payload,
        None,
    )
}

async fn execute_enforcement_command(
    command: AgentCommandEnvelope,
    paths: EnforcementJournalPaths,
) -> Result<LogFields, TestText> {
    let observed_at = TestText::from_display(timestamp_now::<String>());
    let observed_at_text = EnforcementText::from(observed_at.to_string());
    let request = parse_enforcement_command_payload(&command, &observed_at_text)
        .map_err(|error| TestText::from_display(format!("{error:?}")))?;
    let authorization = authorize_enforcement_boundary(request.input.clone())
        .map_err(|error| TestText::from_display(error.as_protocol_str()))?;
    let before_action_outcome =
        journal_before_action_outcome(&request, &authorization.action, observed_at.to_string());
    record_enforcement_audit(&request, &before_action_outcome, &paths).await?;
    let completed_at = TestText::from_display(timestamp_now::<String>());
    let adapter_outcome = authorization
        .adapter_request
        .as_ref()
        .map(|adapter_request| {
            adapter_outcome_for_request(
                &request,
                &authorization.action,
                adapter_request.adapter_kind,
                adapter_request.mode,
                &completed_at,
            )
        })
        .transpose()?;
    let outcome_input = final_input(request.input.clone(), adapter_outcome, &completed_at);
    let mut outcome = evaluate_enforcement_boundary(outcome_input)
        .map_err(|error| TestText::from_display(error.as_protocol_str()))?;
    outcome.audit_event.journal_sequence = Some(outcome.audit_event.audit_event_id.clone());
    let status = record_enforcement_audit(&request, &outcome, &paths).await?;
    let active_state = store_active_timer_state_for_outcome_with_app_game_session(
        &outcome,
        &paths.timer_state_path,
        &completed_at.to_string(),
        None,
    )
    .await
    .map_err(|error| TestText::from_display(format!("{error:?}")))?;

    enforcement_report_payload(&outcome, &status, active_state.as_ref())
}

fn adapter_outcome_for_request(
    request: &EnforcementCommandPayload,
    action: &ocentra_parent_agent_protocol::enforcement::EnforcementAction,
    adapter_kind: EnforcementAdapterKind,
    mode: EnforcementMode,
    completed_at: &TestText,
) -> Result<EnforcementAdapterOutcome, TestText> {
    if adapter_kind != EnforcementAdapterKind::ProcessControl
        || mode != EnforcementMode::TerminateProcess
    {
        return Err(TestText::from_display(
            constants::enforcement::REJECTION_UNSUPPORTED_CAPABILITY,
        ));
    }

    let pid = request.process_id.ok_or_else(|| {
        TestText::from_display(constants::enforcement::REJECTION_PROCESS_ID_REQUIRED)
    })?;
    Ok(terminate_owned_process(
        OwnedProcessTerminationTarget {
            pid,
            expected_process_name: action.target.target_value.clone(),
        },
        completed_at.as_ref(),
    ))
}

fn final_input(
    mut input: EnforcementBoundaryInput,
    adapter_outcome: Option<EnforcementAdapterOutcome>,
    completed_at: &TestText,
) -> EnforcementBoundaryInput {
    input.completed_at = Some(completed_at.to_string());
    input.adapter_outcome = adapter_outcome;
    input
}

async fn record_enforcement_audit(
    request: &EnforcementCommandPayload,
    outcome: &EnforcementBoundaryOutcome,
    paths: &EnforcementJournalPaths,
) -> Result<ActivityIngestStatus, TestText> {
    let event = enforcement_activity_event(request, outcome)?;
    let journal_path = paths.journal_path.clone();
    let key_path = paths.key_path.clone();
    let store_path = paths.store_path.clone();
    tokio::task::spawn_blocking(move || {
        record_activity_events_to_paths(&journal_path, &key_path, &store_path, &[event])
    })
    .await
    .map_err(|_join_error| TestText::from_display(constants::value::ACTIVITY_CAPTURE_STORE_ERROR))?
    .map_err(|_store_error| TestText::from_display(constants::value::ACTIVITY_CAPTURE_STORE_ERROR))
}

fn enforcement_activity_event(
    request: &EnforcementCommandPayload,
    outcome: &EnforcementBoundaryOutcome,
) -> Result<ActivityEvent, TestText> {
    Ok(ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: outcome.audit_event.audit_event_id.clone(),
        observed_at: outcome.audit_event.observed_at.clone(),
        source: ActivitySource {
            device_id: request.device_id.0.clone(),
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

fn enforcement_journal_fields(outcome: &EnforcementBoundaryOutcome) -> Result<LogFields, TestText> {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::POLICY_DECISION_ID.to_string(),
        LogFieldValue::String(outcome.action.policy_decision_id.clone()),
    );
    fields.insert(
        constants::field::POLICY_ACTION.to_string(),
        LogFieldValue::String(outcome.action.policy_action.as_protocol_str().to_string()),
    );
    fields.insert(
        constants::field::POLICY_TARGET_TYPE.to_string(),
        LogFieldValue::String(
            outcome
                .action
                .target
                .target_type
                .as_protocol_str()
                .to_string(),
        ),
    );
    fields.insert(
        constants::field::POLICY_TARGET_VALUE.to_string(),
        LogFieldValue::String(outcome.action.target.target_value.clone()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_ACTION_ID.to_string(),
        LogFieldValue::String(outcome.action.action_id.clone()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_RESULT_ID.to_string(),
        LogFieldValue::String(outcome.result.result_id.clone()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_AUDIT_EVENT_ID.to_string(),
        LogFieldValue::String(outcome.audit_event.audit_event_id.clone()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_STATUS.to_string(),
        LogFieldValue::String(outcome.result.status.as_protocol_str().to_string()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_ADAPTER_RESULT_CODE.to_string(),
        LogFieldValue::String(
            outcome
                .result
                .adapter_result_code
                .as_protocol_str()
                .to_string(),
        ),
    );
    fields.insert(
        constants::field::ENFORCEMENT_ROLLBACK_STATE.to_string(),
        LogFieldValue::String(outcome.result.rollback_state.as_protocol_str().to_string()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_CAPABILITY_STATE.to_string(),
        LogFieldValue::String(
            outcome
                .result
                .capability
                .capability_state
                .as_protocol_str()
                .to_string(),
        ),
    );
    fields.insert(
        constants::field::EVIDENCE_REFERENCE_IDS.to_string(),
        LogFieldValue::String(evidence_reference_ids(outcome).to_string()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_ACTION.to_string(),
        serialize_json_log_value(&outcome.action)?,
    );
    fields.insert(
        constants::field::ENFORCEMENT_RESULT.to_string(),
        serialize_json_log_value(&outcome.result)?,
    );
    fields.insert(
        constants::field::ENFORCEMENT_AUDIT_EVENT.to_string(),
        serialize_json_log_value(&outcome.audit_event)?,
    );
    fields.insert(
        constants::field::ENFORCEMENT_TIMER_EVENT.to_string(),
        optional_timer_event(outcome)?,
    );
    Ok(fields)
}

fn enforcement_report_payload(
    outcome: &EnforcementBoundaryOutcome,
    status: &ActivityIngestStatus,
    active_state: Option<&ocentra_parent_agent_protocol::enforcement::EnforcementActiveTimerState>,
) -> Result<LogFields, TestText> {
    let mut payload = enforcement_journal_fields(outcome)?;
    let last_event_id = status.last_event_id.as_ref().map(TestText::from_display);
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
        optional_string_value(last_event_id.as_ref()),
    );
    insert_timer_event_fields(&mut payload, &outcome.timer_event);
    payload.insert(
        constants::field::ENFORCEMENT_TIMER_STATE.to_string(),
        optional_timer_state(active_state)?,
    );
    Ok(payload)
}

fn optional_timer_event(outcome: &EnforcementBoundaryOutcome) -> Result<LogFieldValue, TestText> {
    serialize_optional_json(outcome.timer_event.as_ref())
}

fn optional_string_value(value: Option<&TestText>) -> LogFieldValue {
    value
        .map(|item| LogFieldValue::String(item.to_string()))
        .unwrap_or(LogFieldValue::Null(()))
}

fn optional_timer_state(
    active_state: Option<&ocentra_parent_agent_protocol::enforcement::EnforcementActiveTimerState>,
) -> Result<LogFieldValue, TestText> {
    serialize_optional_json(active_state)
}

fn enforcement_success_report(payload: LogFields) -> EnforcementAuditReport {
    EnforcementAuditReport {
        event_id: constants::event_id::ENFORCEMENT_AUDIT_REPORTED,
        event_name: AgentEventName::AgentEnforcementAuditReported,
        level: LogLevel::Info,
        payload,
    }
}

fn enforcement_rejection_report(reason: TestText) -> EnforcementAuditReport {
    let TestText(reason_text) = reason;
    EnforcementAuditReport {
        event_id: constants::event_id::COMMAND_REJECTED,
        event_name: AgentEventName::AgentCommandRejected,
        level: LogLevel::Warn,
        payload: fields_from_pairs(vec![(
            constants::field::REASON,
            LogFieldValue::String(reason_text),
        )]),
    }
}

fn insert_timer_event_fields(
    payload: &mut LogFields,
    timer_event: &Option<ocentra_parent_agent_protocol::enforcement::EnforcementTimerEvent>,
) {
    let Some(timer) = timer_event.as_ref() else {
        return;
    };
    payload.insert(
        constants::field::ENFORCEMENT_TIMER_EVENT_ID.to_string(),
        LogFieldValue::String(timer.timer_event_id.clone()),
    );
    payload.insert(
        constants::field::ENFORCEMENT_TIMER_EVENT_KIND.to_string(),
        LogFieldValue::String(timer.timer_event_kind.as_protocol_str().to_string()),
    );
}

fn serialize_json_log_value<T>(value: &T) -> Result<LogFieldValue, TestText>
where
    T: serde::Serialize,
{
    serde_json::to_string(value)
        .map(LogFieldValue::String)
        .map_err(|_serialize_error| {
            TestText::from_display(constants::error::AGENT_EVENT_SERIALIZES)
        })
}

fn serialize_optional_json<T>(value: Option<&T>) -> Result<LogFieldValue, TestText>
where
    T: serde::Serialize,
{
    value
        .map(serialize_json_log_value)
        .transpose()
        .map(|maybe_value| maybe_value.unwrap_or(LogFieldValue::Null(())))
}

fn evidence_reference_ids(outcome: &EnforcementBoundaryOutcome) -> TestText {
    let mut separator = [0; 4];
    TestText::from_display(
        outcome
            .action
            .evidence_references
            .iter()
            .map(|reference| reference.evidence_reference_id.as_str().to_string())
            .collect::<Vec<_>>()
            .join(constants::delimiter::LIST.encode_utf8(&mut separator)),
    )
}

struct EnforcementAuditReport {
    event_id: &'static str,
    event_name: AgentEventName,
    level: LogLevel,
    payload: LogFields,
}
