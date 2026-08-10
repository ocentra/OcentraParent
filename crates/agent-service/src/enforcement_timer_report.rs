use ocentra_eventing::{
    ids::CorrelationId,
    journal::{policy::JournalDispatchPhase, JournalAppend},
};
use ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryOutcome;
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::activity::ActivityEventKind;
use ocentra_parent_agent_protocol::activity::ActivityObserver;
use ocentra_parent_agent_protocol::activity::ActivitySource;
use ocentra_parent_agent_protocol::activity::ActivitySubject;
use ocentra_parent_agent_protocol::activity::ActivitySubjectKind;
use ocentra_parent_agent_protocol::activity::ACTIVITY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::activity_query::ActivityIngestStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::enforcement::EnforcementActiveTimerState;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;

use crate::{
    activity_capture::record_activity_events_to_paths,
    enforcement_api::{
        enforcement_pre_action_journal::eventing_journal::{
            append_enforcement_audit_journal_event_phase, EnforcementEventingJournalPath,
        },
        EnforcementJournalPaths,
    },
    enforcement_timer_payload::EnforcementTimerCommandPayload,
    fields::fields_from_pairs,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TimerFieldName(&'static str);

#[derive(Clone, Debug, PartialEq, Eq)]
struct TimerFieldText(String);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TimerReportError {
    Serialize,
    Store,
}

pub(crate) async fn record_timer_activity(
    request: &EnforcementTimerCommandPayload,
    outcome: &EnforcementBoundaryOutcome,
    paths: &EnforcementJournalPaths,
) -> Result<ActivityIngestStatus, TimerReportError> {
    let event = timer_activity_event(request, outcome)?;
    let journal_path = paths.journal_path.clone();
    let key_path = paths.key_path.clone();
    let store_path = paths.store_path.clone();
    tokio::task::spawn_blocking(move || {
        record_activity_events_to_paths(&journal_path, &key_path, &store_path, &[event])
            .map_err(activity_capture_store_error)
    })
    .await
    .map_err(activity_capture_store_error)?
}

pub(crate) async fn record_timer_eventing_audit(
    request: &EnforcementTimerCommandPayload,
    outcome: &EnforcementBoundaryOutcome,
    paths: &EnforcementJournalPaths,
) -> Result<JournalAppend, TimerReportError> {
    record_timer_eventing_audit_phase(request, outcome, paths, JournalDispatchPhase::AfterDispatch)
        .await
}

pub(crate) async fn record_timer_eventing_audit_phase(
    request: &EnforcementTimerCommandPayload,
    outcome: &EnforcementBoundaryOutcome,
    paths: &EnforcementJournalPaths,
    phase: JournalDispatchPhase,
) -> Result<JournalAppend, TimerReportError> {
    let mut eventing_journal_path = paths.journal_path.clone();
    eventing_journal_path.set_extension(constants::enforcement::EVENTING_JOURNAL_EXTENSION);
    let mut event = ocentra_parent_agent_protocol::enforcement::EnforcementAuditJournalEvent::from(
        &outcome.audit_event,
    );
    event.observed_at = request.command_sent_at.0.clone();
    event.device_id = Some(request.device_id.0.clone());
    event.source_peer_id = Some(request.source_peer_id.0.clone());
    event.target_route = Some(request.target_route.0.clone());
    append_enforcement_audit_journal_event_phase(
        EnforcementEventingJournalPath {
            path: eventing_journal_path,
        },
        event,
        CorrelationId::parse(request.command_correlation_id.0.clone())
            .map_err(eventing_journal_store_error)?,
        phase,
    )
    .await
    .map_err(eventing_journal_store_error)
}

pub(crate) fn timer_report_payload(
    outcome: &EnforcementBoundaryOutcome,
    status: &ActivityIngestStatus,
    active_state: Option<&EnforcementActiveTimerState>,
) -> Result<LogFields, TimerReportError> {
    let mut payload = timer_journal_fields(outcome)?;
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
        match status.last_event_id.as_deref() {
            Some(value) => LogFieldValue::String(value.to_string()),
            None => LogFieldValue::Null(()),
        },
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
        serialized_active_state(active_state)?,
    );
    Ok(payload)
}

pub(crate) fn unavailable_timer_payload(reason: LogFieldValue) -> LogFields {
    fields_from_pairs(vec![
        (constants::field::AVAILABLE, LogFieldValue::Boolean(false)),
        (constants::field::REASON, reason),
        (
            constants::field::ENFORCEMENT_STATUS,
            LogFieldValue::String(constants::enforcement::RESULT_UNAVAILABLE.to_string()),
        ),
        (
            constants::field::ENFORCEMENT_TIMER_EVENT_KIND,
            LogFieldValue::String(constants::enforcement::TIMER_RECOVERY_NEEDED.to_string()),
        ),
    ])
}

fn timer_activity_event(
    request: &EnforcementTimerCommandPayload,
    outcome: &EnforcementBoundaryOutcome,
) -> Result<ActivityEvent, TimerReportError> {
    Ok(ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: outcome.audit_event.audit_event_id.clone(),
        observed_at: outcome.audit_event.observed_at.clone(),
        source: ActivitySource {
            device_id: request.device_id.0.clone(),
            platform: request.platform.0.clone(),
            observer: ActivityObserver::AgentService,
            source_id: constants::enforcement::SOURCE_ID_AGENT_SERVICE.to_string(),
        },
        kind: ActivityEventKind::EnforcementAuditRecorded,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Intervention,
            subject_id: outcome.action.action_id.clone(),
            display_name: Some(outcome.action.mode.as_protocol_str().to_string()),
        },
        fields: timer_journal_fields(outcome)?,
        evidence: Vec::new(),
    })
}

fn timer_journal_fields(
    outcome: &EnforcementBoundaryOutcome,
) -> Result<LogFields, TimerReportError> {
    let mut fields = base_timer_field_pairs(outcome)
        .into_iter()
        .map(|(field, value)| (field.0, value))
        .collect::<Vec<_>>();
    fields.extend(
        serialized_timer_field_pairs(outcome)?
            .into_iter()
            .map(|(field, value)| (field.0, value)),
    );
    Ok(fields_from_pairs(fields))
}

fn base_timer_field_pairs(
    outcome: &EnforcementBoundaryOutcome,
) -> Vec<(TimerFieldName, LogFieldValue)> {
    vec![
        (
            TimerFieldName(constants::field::POLICY_DECISION_ID),
            LogFieldValue::String(outcome.action.policy_decision_id.clone()),
        ),
        (
            TimerFieldName(constants::field::ENFORCEMENT_ACTION_ID),
            LogFieldValue::String(outcome.action.action_id.clone()),
        ),
        (
            TimerFieldName(constants::field::ENFORCEMENT_RESULT_ID),
            LogFieldValue::String(outcome.result.result_id.clone()),
        ),
        (
            TimerFieldName(constants::field::ENFORCEMENT_AUDIT_EVENT_ID),
            LogFieldValue::String(outcome.audit_event.audit_event_id.clone()),
        ),
        (
            TimerFieldName(constants::field::ENFORCEMENT_STATUS),
            LogFieldValue::String(outcome.result.status.as_protocol_str().to_string()),
        ),
        (
            TimerFieldName(constants::field::ENFORCEMENT_ADAPTER_RESULT_CODE),
            LogFieldValue::String(
                outcome
                    .result
                    .adapter_result_code
                    .as_protocol_str()
                    .to_string(),
            ),
        ),
        (
            TimerFieldName(constants::field::ENFORCEMENT_ROLLBACK_STATE),
            LogFieldValue::String(outcome.result.rollback_state.as_protocol_str().to_string()),
        ),
        (
            TimerFieldName(constants::field::EVIDENCE_REFERENCE_IDS),
            LogFieldValue::String(evidence_reference_ids(outcome).0),
        ),
    ]
}

fn serialized_timer_field_pairs(
    outcome: &EnforcementBoundaryOutcome,
) -> Result<Vec<(TimerFieldName, LogFieldValue)>, TimerReportError> {
    Ok(vec![
        (
            TimerFieldName(constants::field::ENFORCEMENT_ACTION),
            LogFieldValue::String(
                serde_json::to_string(&outcome.action).map_err(agent_event_serializes_error)?,
            ),
        ),
        (
            TimerFieldName(constants::field::ENFORCEMENT_RESULT),
            LogFieldValue::String(
                serde_json::to_string(&outcome.result).map_err(agent_event_serializes_error)?,
            ),
        ),
        (
            TimerFieldName(constants::field::ENFORCEMENT_AUDIT_EVENT),
            LogFieldValue::String(
                serde_json::to_string(&outcome.audit_event)
                    .map_err(agent_event_serializes_error)?,
            ),
        ),
        (
            TimerFieldName(constants::field::ENFORCEMENT_TIMER_EVENT),
            serialized_timer_event(outcome)?,
        ),
    ])
}

fn serialized_timer_event(
    outcome: &EnforcementBoundaryOutcome,
) -> Result<LogFieldValue, TimerReportError> {
    match &outcome.timer_event {
        Some(timer) => Ok(LogFieldValue::String(
            serde_json::to_string(timer).map_err(agent_event_serializes_error)?,
        )),
        None => Ok(LogFieldValue::Null(())),
    }
}

fn serialized_active_state(
    active_state: Option<&EnforcementActiveTimerState>,
) -> Result<LogFieldValue, TimerReportError> {
    match active_state {
        Some(state) => Ok(LogFieldValue::String(
            serde_json::to_string(state).map_err(agent_event_serializes_error)?,
        )),
        None => Ok(LogFieldValue::Null(())),
    }
}

fn evidence_reference_ids(outcome: &EnforcementBoundaryOutcome) -> TimerFieldText {
    let separator = constants::delimiter::LIST.to_string();
    TimerFieldText(
        outcome
            .action
            .evidence_references
            .iter()
            .map(|reference| reference.evidence_reference_id.clone())
            .collect::<Vec<_>>()
            .join(separator.as_str()),
    )
}

fn activity_capture_store_error(_: impl std::fmt::Debug) -> TimerReportError {
    TimerReportError::Store
}

fn eventing_journal_store_error(_: impl std::fmt::Debug) -> TimerReportError {
    TimerReportError::Store
}

fn agent_event_serializes_error(_: serde_json::Error) -> TimerReportError {
    TimerReportError::Serialize
}
