#[path = "enforcement_command_execution/adapter_outcome.rs"]
mod adapter_outcome;

use ocentra_parent_agent_core::enforcement_boundary::{
    authorize_enforcement_boundary, evaluate_enforcement_boundary, EnforcementBoundaryOutcome,
    EnforcementBoundaryRejection,
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
use ocentra_parent_agent_protocol::enforcement::AppGameTimerSessionBinding;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::enforcement_payload::{
    parse_enforcement_command_payload, EnforcementCommandPayload, EnforcementPayloadError,
    EnforcementText,
};
use crate::{
    activity_capture::record_activity_events_to_paths, event_builder::build_event,
    fields::fields_from_pairs, time::timestamp_now,
};

use self::adapter_outcome::{adapter_outcome_for_request, final_input};
use super::enforcement_pre_action_journal::journal_before_action_outcome;
use super::enforcement_report_payload::{
    build_enforcement_report_payload, enforcement_journal_fields,
};
use super::EnforcementJournalPaths;

pub(super) async fn build_enforcement_audit_report_with_paths(
    command: AgentCommandEnvelope,
    paths: EnforcementJournalPaths,
) -> AgentEventEnvelope {
    build_enforcement_audit_report(command, paths, None).await
}

pub(super) async fn build_enforcement_audit_report_with_app_game_session(
    command: AgentCommandEnvelope,
    paths: EnforcementJournalPaths,
    app_game_session: AppGameTimerSessionBinding,
) -> AgentEventEnvelope {
    build_enforcement_audit_report(command, paths, Some(app_game_session)).await
}

async fn build_enforcement_audit_report(
    command: AgentCommandEnvelope,
    paths: EnforcementJournalPaths,
    app_game_session: Option<AppGameTimerSessionBinding>,
) -> AgentEventEnvelope {
    let target = command.source.clone();
    let correlation_id = command.message_id.clone();
    match execute_enforcement_command(command, paths, app_game_session).await {
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
            fields_from_pairs(vec![(constants::field::REASON, reason.log_field_value())]),
            None,
        ),
    }
}

async fn execute_enforcement_command(
    command: AgentCommandEnvelope,
    paths: EnforcementJournalPaths,
    app_game_session: Option<AppGameTimerSessionBinding>,
) -> Result<LogFields, EnforcementCommandExecutionError> {
    let observed_at = EnforcementText(timestamp_now());
    let request = parse_enforcement_command_payload(&command, &observed_at)
        .map_err(EnforcementCommandExecutionError::PayloadRejection)?;
    let authorization = authorize_enforcement_boundary(request.input.clone())
        .map_err(EnforcementCommandExecutionError::BoundaryRejection)?;
    let before_action_outcome =
        journal_before_action_outcome(&request, &authorization.action, observed_at);
    record_enforcement_audit(&request, &before_action_outcome, &paths).await?;
    let completed_at = EnforcementText(timestamp_now());
    let adapter_outcome = adapter_outcome_for_request(
        &request,
        &authorization.action,
        authorization.adapter_request.as_ref(),
        &completed_at,
    )?;
    let outcome_input = final_input(request.input.clone(), adapter_outcome, &completed_at);
    let mut outcome = evaluate_enforcement_boundary(outcome_input)
        .map_err(EnforcementCommandExecutionError::BoundaryRejection)?;
    outcome.audit_event.journal_sequence = Some(outcome.audit_event.audit_event_id.clone());
    let status = record_enforcement_audit(&request, &outcome, &paths).await?;
    let active_state = crate::enforcement_timer_state_file::store_active_timer_state_for_outcome_with_app_game_session(
        &outcome,
        &paths.timer_state_path,
        completed_at.0.as_str(),
        app_game_session,
    )
    .await
    .map_err(activity_capture_store_error)?;

    build_enforcement_report_payload(&outcome, &status, active_state.as_ref())
        .map_err(EnforcementCommandExecutionError::Journal)
}

async fn record_enforcement_audit(
    request: &EnforcementCommandPayload,
    outcome: &EnforcementBoundaryOutcome,
    paths: &EnforcementJournalPaths,
) -> Result<ActivityIngestStatus, EnforcementJournalBuildError> {
    let event = enforcement_activity_event(request, outcome)?;
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

fn enforcement_activity_event(
    request: &EnforcementCommandPayload,
    outcome: &EnforcementBoundaryOutcome,
) -> Result<ActivityEvent, EnforcementJournalBuildError> {
    Ok(ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: outcome.audit_event.audit_event_id.clone(),
        observed_at: outcome.audit_event.observed_at.clone(),
        source: ActivitySource {
            device_id: request.device_id.clone().0,
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

#[derive(Clone, Copy, Debug)]
pub(super) enum EnforcementJournalBuildError {
    Serialize,
    Store,
}

#[derive(Clone, Copy, Debug)]
enum EnforcementCommandExecutionError {
    PayloadRejection(EnforcementPayloadError),
    BoundaryRejection(EnforcementBoundaryRejection),
    Journal(EnforcementJournalBuildError),
}

impl EnforcementCommandExecutionError {
    fn log_field_value(self) -> LogFieldValue {
        let value = match self {
            Self::PayloadRejection(reason) => reason.to_string(),
            Self::BoundaryRejection(reason) => reason.as_protocol_str().to_string(),
            Self::Journal(EnforcementJournalBuildError::Serialize) => {
                constants::error::AGENT_EVENT_SERIALIZES.to_string()
            }
            Self::Journal(EnforcementJournalBuildError::Store) => {
                constants::value::ACTIVITY_CAPTURE_STORE_ERROR.to_string()
            }
        };
        LogFieldValue::String(value)
    }
}

impl From<EnforcementJournalBuildError> for EnforcementCommandExecutionError {
    fn from(error: EnforcementJournalBuildError) -> Self {
        Self::Journal(error)
    }
}

fn activity_capture_store_error(_: impl std::fmt::Debug) -> EnforcementJournalBuildError {
    EnforcementJournalBuildError::Store
}
