#[path = "enforcement_command_execution/adapter_outcome.rs"]
mod adapter_outcome;
#[path = "enforcement_command_execution/eventing_audit.rs"]
mod eventing_audit;
#[path = "enforcement_command_execution/provenance.rs"]
mod provenance;
#[path = "enforcement_command_execution/rejected_audit.rs"]
mod rejected_audit;
#[path = "enforcement_command_execution/retry.rs"]
mod retry;
#[path = "enforcement_command_execution/run.rs"]
mod run;

use ocentra_parent_agent_core::enforcement_boundary::{
    EnforcementBoundaryOutcome, EnforcementBoundaryRejection,
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
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::enforcement_payload::{EnforcementCommandPayload, EnforcementPayloadError};
use crate::{
    activity_capture::record_activity_events_to_paths, event_builder::build_event,
    fields::fields_from_pairs,
};

use self::provenance::{
    enforcement_audit_provenance, record_audit_provenance, EnforcementAuditProvenance,
};
use self::retry::EnforcementRetryRecoveryError;
use self::run::execute_enforcement_command;
use super::enforcement_report_payload::enforcement_journal_fields;
use super::EnforcementJournalPaths;

pub(crate) async fn build_enforcement_audit_report_with_paths(
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
    let provenance = enforcement_audit_provenance(&command.command);
    match execute_enforcement_command(command, paths, provenance, app_game_session).await {
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
    RetryRecovery(EnforcementRetryRecoveryError),
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
            Self::RetryRecovery(EnforcementRetryRecoveryError::IdentityMismatch) => {
                constants::enforcement::REJECTION_RETRY_IDENTITY_MISMATCH.to_string()
            }
            Self::RetryRecovery(EnforcementRetryRecoveryError::ReconciliationRequired) => {
                constants::enforcement::REJECTION_RETRY_RECONCILIATION_REQUIRED.to_string()
            }
            Self::RetryRecovery(EnforcementRetryRecoveryError::Store) => {
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
