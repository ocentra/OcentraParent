use ocentra_eventing::journal::policy::JournalDispatchPhase;
use ocentra_parent_agent_core::enforcement_app_time_limit::{
    app_time_limit_target_from_action, expire_app_time_limit_for_owned_process,
    AppTimeLimitTargetRejection,
};
use ocentra_parent_agent_core::enforcement_timer_state::{
    cancelled_timer_outcome, expired_timer_outcome, expiring_timer_before_dispatch_outcome,
    restart_recovered_timer_outcome,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentCommandName;

use crate::activity_api::GeneratedAtText;
use crate::app_game_dispatch_evidence::{
    validate_app_game_timer_session, AppGameDispatchEvidenceRejection, AppGameDispatchStorePath,
};
use crate::enforcement_api::EnforcementJournalPaths;
use crate::enforcement_timer_payload::{
    parse_parent_override_payload, parse_timer_expiry_payload, parse_timer_recovery_payload,
    EnforcementTimerCommandPayload, EnforcementTimerPayloadError,
};
use crate::enforcement_timer_report::{
    record_timer_activity, record_timer_eventing_audit, record_timer_eventing_audit_phase,
    timer_report_payload, unavailable_timer_payload, TimerReportError,
};
use crate::enforcement_timer_state_file::{
    read_active_timer_state, remove_active_timer_state,
    store_active_timer_state_for_outcome_with_app_game_session, EnforcementTimerStoredAtTextRef,
};
use crate::time::timestamp_now;

use super::state_error::timer_state_file_error;
use super::validation::validate_expected_action;

#[derive(Debug)]
pub(crate) enum EnforcementTimerCommandError {
    CommandPayloadInvalid,
    ActiveTimerStateMismatch,
    ParentActionRequired,
    ActiveTimerStateRequired,
    Payload(EnforcementTimerPayloadError),
    Report(TimerReportError),
    AppTimeLimitTarget(AppTimeLimitTargetRejection),
    AppGameSessionEvidence(AppGameDispatchEvidenceRejection),
}

impl From<EnforcementTimerPayloadError> for EnforcementTimerCommandError {
    fn from(error: EnforcementTimerPayloadError) -> Self {
        Self::Payload(error)
    }
}

impl From<TimerReportError> for EnforcementTimerCommandError {
    fn from(error: TimerReportError) -> Self {
        Self::Report(error)
    }
}

impl From<AppTimeLimitTargetRejection> for EnforcementTimerCommandError {
    fn from(error: AppTimeLimitTargetRejection) -> Self {
        Self::AppTimeLimitTarget(error)
    }
}

pub(crate) async fn execute_timer_command(
    command: AgentCommandEnvelope,
    paths: EnforcementJournalPaths,
) -> Result<LogFields, EnforcementTimerCommandError> {
    let observed_at = GeneratedAtText(timestamp_now());
    match command.command {
        AgentCommandName::AgentEnforcementTimerRecover => {
            let request = parse_timer_recovery_payload(&command, &observed_at);
            recover_timer(request, paths).await
        }
        AgentCommandName::AgentEnforcementTimerExpire => {
            let request = parse_timer_expiry_payload(&command, &observed_at)?;
            expire_timer(request, paths).await
        }
        AgentCommandName::AgentEnforcementOverrideCancel => {
            let request = parse_parent_override_payload(&command, &observed_at)?;
            cancel_timer(request, paths).await
        }
        _ => Err(EnforcementTimerCommandError::CommandPayloadInvalid),
    }
}

async fn recover_timer(
    request: EnforcementTimerCommandPayload,
    paths: EnforcementJournalPaths,
) -> Result<LogFields, EnforcementTimerCommandError> {
    let timer_state_path = paths.timer_state_path.clone();
    let Some(state) = read_active_timer_state(&timer_state_path)
        .await
        .map_err(timer_state_file_error)?
    else {
        return Ok(active_timer_state_required_payload());
    };
    validate_expected_action(&request, &state)?;
    let mut outcome = restart_recovered_timer_outcome(&state, request.transition_ids.clone());
    let journal_append = record_timer_eventing_audit(&request, &outcome, &paths).await?;
    outcome.audit_event.journal_sequence = Some(journal_append.sequence.to_string());
    let status = record_timer_activity(&request, &outcome, &paths).await?;
    let active_state = store_active_timer_state_for_outcome_with_app_game_session(
        &outcome,
        &timer_state_path,
        EnforcementTimerStoredAtTextRef(&request.transition_ids.observed_at),
        state.app_game_session.clone(),
    )
    .await
    .map_err(timer_state_file_error)?;
    timer_report_payload(&outcome, &status, active_state.as_ref()).map_err(Into::into)
}

async fn expire_timer(
    request: EnforcementTimerCommandPayload,
    paths: EnforcementJournalPaths,
) -> Result<LogFields, EnforcementTimerCommandError> {
    let timer_state_path = paths.timer_state_path.clone();
    let Some(state) = read_active_timer_state(&timer_state_path)
        .await
        .map_err(timer_state_file_error)?
    else {
        return Ok(active_timer_state_required_payload());
    };
    validate_expected_action(&request, &state)?;
    if let Some(binding) = state.app_game_session.as_ref() {
        validate_app_game_timer_session(
            binding,
            AppGameDispatchStorePath(paths.store_path.clone()),
        )
        .await
        .map_err(EnforcementTimerCommandError::AppGameSessionEvidence)?;
    }
    let target = app_time_limit_target_from_action(&state.action, request.process_id)?;
    let before_dispatch_outcome =
        expiring_timer_before_dispatch_outcome(&state, request.transition_ids.clone());
    record_timer_eventing_audit_phase(
        &request,
        &before_dispatch_outcome,
        &paths,
        JournalDispatchPhase::BeforeDispatch,
    )
    .await?;
    let adapter_outcome =
        expire_app_time_limit_for_owned_process(target, &request.transition_ids.observed_at);
    let mut outcome =
        expired_timer_outcome(&state, request.transition_ids.clone(), adapter_outcome);
    let journal_append = record_timer_eventing_audit_phase(
        &request,
        &outcome,
        &paths,
        JournalDispatchPhase::AfterDispatch,
    )
    .await?;
    outcome.audit_event.journal_sequence = Some(journal_append.sequence.to_string());
    let status = record_timer_activity(&request, &outcome, &paths).await?;
    remove_active_timer_state(&timer_state_path)
        .await
        .map_err(timer_state_file_error)?;
    timer_report_payload(&outcome, &status, None).map_err(Into::into)
}

async fn cancel_timer(
    request: EnforcementTimerCommandPayload,
    paths: EnforcementJournalPaths,
) -> Result<LogFields, EnforcementTimerCommandError> {
    let timer_state_path = paths.timer_state_path.clone();
    let Some(state) = read_active_timer_state(&timer_state_path)
        .await
        .map_err(timer_state_file_error)?
    else {
        return Ok(active_timer_state_required_payload());
    };
    validate_expected_action(&request, &state)?;
    let parent_override = request
        .parent_override
        .clone()
        .ok_or(EnforcementTimerCommandError::ParentActionRequired)?;
    let mut outcome =
        cancelled_timer_outcome(&state, request.transition_ids.clone(), parent_override);
    let journal_append = record_timer_eventing_audit(&request, &outcome, &paths).await?;
    outcome.audit_event.journal_sequence = Some(journal_append.sequence.to_string());
    let status = record_timer_activity(&request, &outcome, &paths).await?;
    remove_active_timer_state(&timer_state_path)
        .await
        .map_err(timer_state_file_error)?;
    timer_report_payload(&outcome, &status, None).map_err(Into::into)
}

fn active_timer_state_required_payload() -> LogFields {
    unavailable_timer_payload(LogFieldValue::String(
        constants::enforcement::REJECTION_ACTIVE_TIMER_STATE_REQUIRED.to_string(),
    ))
}
