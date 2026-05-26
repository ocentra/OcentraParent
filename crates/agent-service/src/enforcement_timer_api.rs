use ocentra_parent_agent_core::{cancelled_timer_outcome, restart_recovered_timer_outcome};
use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
    EnforcementActiveTimerState, LogFieldValue, LogFields, LogLevel,
};

use crate::{
    enforcement_api::EnforcementJournalPaths,
    enforcement_timer_payload::{
        parse_parent_override_payload, parse_timer_recovery_payload, EnforcementTimerCommandPayload,
    },
    enforcement_timer_report::{
        record_timer_activity, timer_report_payload, unavailable_timer_payload,
    },
    enforcement_timer_state_file::{
        read_active_timer_state, remove_active_timer_state, store_active_timer_state_for_outcome,
    },
    event_builder::build_event,
    fields::fields_from_pairs,
    time::timestamp_now,
};

pub async fn build_enforcement_timer_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_enforcement_timer_report_with_paths(command, EnforcementJournalPaths::from_environment())
        .await
}

pub(crate) async fn build_enforcement_timer_report_with_paths(
    command: AgentCommandEnvelope,
    paths: EnforcementJournalPaths,
) -> AgentEventEnvelope {
    let target = command.source.clone();
    let correlation_id = command.message_id.clone();
    match execute_timer_command(command, paths).await {
        Ok(payload) => build_event(
            constants::event_id::ENFORCEMENT_TIMER_REPORTED,
            &correlation_id,
            target,
            AgentEventName::AgentEnforcementTimerReported,
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

async fn execute_timer_command(
    command: AgentCommandEnvelope,
    paths: EnforcementJournalPaths,
) -> Result<LogFields, &'static str> {
    let observed_at = timestamp_now();
    match command.command {
        ocentra_parent_agent_protocol::AgentCommandName::AgentEnforcementTimerRecover => {
            let request = parse_timer_recovery_payload(&command, &observed_at);
            recover_timer(request, paths).await
        }
        ocentra_parent_agent_protocol::AgentCommandName::AgentEnforcementOverrideCancel => {
            let request = parse_parent_override_payload(&command, &observed_at)?;
            cancel_timer(request, paths).await
        }
        _ => Err(constants::enforcement::REJECTION_COMMAND_PAYLOAD_INVALID),
    }
}

async fn recover_timer(
    request: EnforcementTimerCommandPayload,
    paths: EnforcementJournalPaths,
) -> Result<LogFields, &'static str> {
    let Some(state) = read_active_timer_state(&paths.timer_state_path).await? else {
        return Ok(unavailable_timer_payload(
            constants::enforcement::REJECTION_ACTIVE_TIMER_STATE_REQUIRED,
        ));
    };
    validate_expected_action(&request, &state)?;
    let mut outcome = restart_recovered_timer_outcome(&state, request.transition_ids.clone());
    outcome.audit_event.journal_sequence = Some(outcome.audit_event.audit_event_id.clone());
    let status = record_timer_activity(&request, &outcome, &paths).await?;
    let active_state = store_active_timer_state_for_outcome(
        &outcome,
        &paths.timer_state_path,
        &request.transition_ids.observed_at,
    )
    .await?;
    timer_report_payload(&outcome, &status, active_state.as_ref())
}

async fn cancel_timer(
    request: EnforcementTimerCommandPayload,
    paths: EnforcementJournalPaths,
) -> Result<LogFields, &'static str> {
    let Some(state) = read_active_timer_state(&paths.timer_state_path).await? else {
        return Ok(unavailable_timer_payload(
            constants::enforcement::REJECTION_ACTIVE_TIMER_STATE_REQUIRED,
        ));
    };
    validate_expected_action(&request, &state)?;
    let parent_override = request
        .parent_override
        .clone()
        .ok_or(constants::enforcement::REJECTION_PARENT_ACTION_REQUIRED)?;
    let mut outcome =
        cancelled_timer_outcome(&state, request.transition_ids.clone(), parent_override);
    outcome.audit_event.journal_sequence = Some(outcome.audit_event.audit_event_id.clone());
    let status = record_timer_activity(&request, &outcome, &paths).await?;
    remove_active_timer_state(&paths.timer_state_path).await?;
    timer_report_payload(&outcome, &status, None)
}

fn validate_expected_action(
    request: &EnforcementTimerCommandPayload,
    state: &EnforcementActiveTimerState,
) -> Result<(), &'static str> {
    match request.expected_action_id.as_deref() {
        Some(action_id) if action_id != state.action.action_id.as_str() => {
            Err(constants::enforcement::REJECTION_ACTIVE_TIMER_STATE_MISMATCH)
        }
        _ => Ok(()),
    }
}
