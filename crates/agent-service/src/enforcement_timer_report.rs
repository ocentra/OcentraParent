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
    activity_capture::record_activity_events_to_paths, enforcement_api::EnforcementJournalPaths,
    enforcement_timer_payload::EnforcementTimerCommandPayload, fields::fields_from_pairs,
};

pub(crate) async fn record_timer_activity(
    request: &EnforcementTimerCommandPayload,
    outcome: &EnforcementBoundaryOutcome,
    paths: &EnforcementJournalPaths,
) -> Result<ActivityIngestStatus, &'static str> {
    let event = timer_activity_event(request, outcome)?;
    let journal_path = paths.journal_path.clone();
    let key_path = paths.key_path.clone();
    let store_path = paths.store_path.clone();
    tokio::task::spawn_blocking(move || {
        record_activity_events_to_paths(&journal_path, &key_path, &store_path, &[event])
    })
    .await
    .map_err(activity_capture_store_error)?
    .map_err(activity_capture_store_error)
}

pub(crate) fn timer_report_payload(
    outcome: &EnforcementBoundaryOutcome,
    status: &ActivityIngestStatus,
    active_state: Option<&EnforcementActiveTimerState>,
) -> Result<LogFields, &'static str> {
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
        serialized_active_state(active_state)?,
    );
    Ok(payload)
}

pub(crate) fn unavailable_timer_payload(reason: &str) -> LogFields {
    fields_from_pairs(vec![
        (constants::field::AVAILABLE, LogFieldValue::Boolean(false)),
        (
            constants::field::REASON,
            LogFieldValue::String(reason.to_string()),
        ),
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
        fields: timer_journal_fields(outcome)?,
        evidence: Vec::new(),
    })
}

fn timer_journal_fields(outcome: &EnforcementBoundaryOutcome) -> Result<LogFields, &'static str> {
    let mut fields = base_timer_field_pairs(outcome);
    fields.extend(serialized_timer_field_pairs(outcome)?);
    Ok(fields_from_pairs(fields))
}

fn base_timer_field_pairs(
    outcome: &EnforcementBoundaryOutcome,
) -> Vec<(&'static str, LogFieldValue)> {
    vec![
        (
            constants::field::POLICY_DECISION_ID,
            LogFieldValue::String(outcome.action.policy_decision_id.clone()),
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
            constants::field::EVIDENCE_REFERENCE_IDS,
            LogFieldValue::String(evidence_reference_ids(outcome)),
        ),
    ]
}

fn serialized_timer_field_pairs(
    outcome: &EnforcementBoundaryOutcome,
) -> Result<Vec<(&'static str, LogFieldValue)>, &'static str> {
    Ok(vec![
        (
            constants::field::ENFORCEMENT_ACTION,
            LogFieldValue::String(
                serde_json::to_string(&outcome.action).map_err(agent_event_serializes_error)?,
            ),
        ),
        (
            constants::field::ENFORCEMENT_RESULT,
            LogFieldValue::String(
                serde_json::to_string(&outcome.result).map_err(agent_event_serializes_error)?,
            ),
        ),
        (
            constants::field::ENFORCEMENT_AUDIT_EVENT,
            LogFieldValue::String(
                serde_json::to_string(&outcome.audit_event)
                    .map_err(agent_event_serializes_error)?,
            ),
        ),
        (
            constants::field::ENFORCEMENT_TIMER_EVENT,
            serialized_timer_event(outcome)?,
        ),
    ])
}

fn serialized_timer_event(
    outcome: &EnforcementBoundaryOutcome,
) -> Result<LogFieldValue, &'static str> {
    match &outcome.timer_event {
        Some(timer) => Ok(LogFieldValue::String(
            serde_json::to_string(timer).map_err(agent_event_serializes_error)?,
        )),
        None => Ok(LogFieldValue::Null(())),
    }
}

fn serialized_active_state(
    active_state: Option<&EnforcementActiveTimerState>,
) -> Result<LogFieldValue, &'static str> {
    match active_state {
        Some(state) => Ok(LogFieldValue::String(
            serde_json::to_string(state).map_err(agent_event_serializes_error)?,
        )),
        None => Ok(LogFieldValue::Null(())),
    }
}

fn optional_string_value(value: Option<&str>) -> LogFieldValue {
    value
        .map(|item| LogFieldValue::String(item.to_string()))
        .unwrap_or(LogFieldValue::Null(()))
}

fn evidence_reference_ids(outcome: &EnforcementBoundaryOutcome) -> String {
    let mut separator = [0; 4];
    outcome
        .action
        .evidence_references
        .iter()
        .map(|reference| reference.evidence_reference_id.as_str())
        .collect::<Vec<_>>()
        .join(constants::delimiter::LIST.encode_utf8(&mut separator))
}

fn activity_capture_store_error(_: impl std::fmt::Debug) -> &'static str {
    constants::value::ACTIVITY_CAPTURE_STORE_ERROR
}

fn agent_event_serializes_error(_: serde_json::Error) -> &'static str {
    constants::error::AGENT_EVENT_SERIALIZES
}
