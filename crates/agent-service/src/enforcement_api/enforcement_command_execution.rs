#[path = "enforcement_command_execution/adapter_outcome.rs"]
mod adapter_outcome;
#[path = "enforcement_command_execution/provenance.rs"]
mod provenance;
#[path = "enforcement_command_execution/rejected_audit.rs"]
mod rejected_audit;

use ocentra_eventing::ids::CorrelationId;
use ocentra_eventing::journal::policy::JournalDispatchPhase;
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
use ocentra_parent_agent_protocol::enforcement::EnforcementAuditJournalEvent;
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
use self::provenance::{
    enforcement_audit_provenance, record_audit_provenance, EnforcementAuditProvenance,
};
use self::rejected_audit::record_rejected_enforcement_audit;
use super::enforcement_pre_action_journal::{
    eventing_journal::{
        append_enforcement_audit_journal_event_phase, EnforcementEventingJournalPath,
    },
    journal_before_action_outcome,
};
use super::enforcement_report_payload::{
    build_enforcement_report_payload, enforcement_journal_fields,
};
use super::EnforcementJournalPaths;

pub(crate) async fn build_enforcement_audit_report_with_paths(
    command: AgentCommandEnvelope,
    paths: EnforcementJournalPaths,
) -> AgentEventEnvelope {
    let target = command.source.clone();
    let correlation_id = command.message_id.clone();
    let provenance = enforcement_audit_provenance(&command.command);
    match execute_enforcement_command(command, paths, provenance).await {
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
    provenance: Option<EnforcementAuditProvenance>,
) -> Result<LogFields, EnforcementCommandExecutionError> {
    let command_correlation_id = EnforcementText(command.message_id.clone());
    let command_sent_at = EnforcementText(command.sent_at.clone());
    let observed_at = EnforcementText(timestamp_now());
    let request = parse_enforcement_command_payload(&command, &observed_at)
        .map_err(EnforcementCommandExecutionError::PayloadRejection)?;
    let authorization = match authorize_enforcement_boundary(request.input.clone()) {
        Ok(authorization) => authorization,
        Err(rejection) => {
            record_rejected_enforcement_audit(
                &command_correlation_id,
                &command_sent_at,
                &request,
                rejection,
                &observed_at,
                &paths,
            )
            .await?;
            return Err(EnforcementCommandExecutionError::BoundaryRejection(
                rejection,
            ));
        }
    };
    let before_action_outcome =
        journal_before_action_outcome(&request, &authorization.action, observed_at);
    record_eventing_enforcement_audit(
        &command_correlation_id,
        &command_sent_at,
        &request,
        &before_action_outcome,
        &paths,
        JournalDispatchPhase::BeforeDispatch,
    )
    .await?;
    record_enforcement_audit(&request, &before_action_outcome, &paths, None).await?;
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
    let final_journal_append = record_eventing_enforcement_audit(
        &command_correlation_id,
        &command_sent_at,
        &request,
        &outcome,
        &paths,
        JournalDispatchPhase::AfterDispatch,
    )
    .await?;
    outcome.audit_event.journal_sequence = Some(final_journal_append.sequence.to_string());
    let status = record_enforcement_audit(&request, &outcome, &paths, provenance).await?;
    let active_state = crate::enforcement_timer_state_file::store_active_timer_state_for_outcome(
        &outcome,
        &paths.timer_state_path,
        completed_at.0.as_str(),
    )
    .await
    .map_err(activity_capture_store_error)?;

    let mut payload = build_enforcement_report_payload(&outcome, &status, active_state.as_ref())
        .map_err(EnforcementCommandExecutionError::Journal)?;
    record_audit_provenance(&mut payload, provenance);
    Ok(payload)
}

async fn record_eventing_enforcement_audit(
    command_correlation_id: &EnforcementText,
    command_sent_at: &EnforcementText,
    request: &EnforcementCommandPayload,
    outcome: &EnforcementBoundaryOutcome,
    paths: &EnforcementJournalPaths,
    phase: JournalDispatchPhase,
) -> Result<ocentra_eventing::journal::JournalAppend, EnforcementJournalBuildError> {
    let mut eventing_journal_path = paths.journal_path.clone();
    eventing_journal_path.set_extension(constants::enforcement::EVENTING_JOURNAL_EXTENSION);
    append_enforcement_audit_journal_event_phase(
        EnforcementEventingJournalPath {
            path: eventing_journal_path,
        },
        eventing_audit_event(request, outcome, command_sent_at),
        CorrelationId::parse(command_correlation_id.0.clone()).map_err(eventing_journal_error)?,
        phase,
    )
    .await
    .map_err(eventing_journal_error)
}

fn eventing_journal_error(_: impl std::fmt::Debug) -> EnforcementJournalBuildError {
    EnforcementJournalBuildError::Store
}

fn eventing_audit_event(
    request: &EnforcementCommandPayload,
    outcome: &EnforcementBoundaryOutcome,
    command_sent_at: &EnforcementText,
) -> EnforcementAuditJournalEvent {
    let mut event = EnforcementAuditJournalEvent::from(&outcome.audit_event);
    event.device_id = Some(request.device_id.0.clone());
    event.source_peer_id = Some(request.source_peer_id.0.clone());
    event.target_route = Some(request.target_route.0.clone());
    event.observed_at = command_sent_at.0.clone();
    event
}

async fn record_enforcement_audit(
    request: &EnforcementCommandPayload,
    outcome: &EnforcementBoundaryOutcome,
    paths: &EnforcementJournalPaths,
    provenance: Option<EnforcementAuditProvenance>,
) -> Result<ActivityIngestStatus, EnforcementJournalBuildError> {
    let event = enforcement_activity_event(request, outcome, provenance)?;
    record_enforcement_activity_event(event, paths).await
}

async fn record_enforcement_activity_event(
    event: ActivityEvent,
    paths: &EnforcementJournalPaths,
) -> Result<ActivityIngestStatus, EnforcementJournalBuildError> {
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
    provenance: Option<EnforcementAuditProvenance>,
) -> Result<ActivityEvent, EnforcementJournalBuildError> {
    let mut fields = enforcement_journal_fields(outcome)?;
    record_audit_provenance(&mut fields, provenance);
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
        fields,
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
