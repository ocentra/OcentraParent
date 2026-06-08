use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
    AppGameTimerParentPreferenceSetupRequest, AppGameTimerParentPreferenceSetupRequestResult,
    LogFieldValue, LogFields, LogLevel,
};

use crate::{event_builder::build_event, fields::fields_from_pairs, time::timestamp_now};

type FieldPair = (&'static str, LogFieldValue);

pub async fn build_activity_app_game_timer_parent_preference_setup_request_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let result = app_game_timer_parent_preference_setup_request_from_command(&command);
    build_event(
        constants::event_id::ACTIVITY_APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUESTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentActivityAppGameTimerParentPreferenceSetupRequested,
        LogLevel::Info,
        app_game_timer_parent_preference_setup_request_payload(&result),
        None,
    )
}

pub fn app_game_timer_parent_preference_setup_request_from_command(
    command: &AgentCommandEnvelope,
) -> AppGameTimerParentPreferenceSetupRequestResult {
    let request = request_from_command(command);
    AppGameTimerParentPreferenceSetupRequestResult {
        schema_version: constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_REQUEST_SCHEMA_VERSION
            .to_string(),
        request_id: request.request_id,
        requested_at: request.requested_at,
        accepted_at: timestamp_now(),
        request_status: constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_REQUEST_ACCEPTED
            .to_string(),
        parent_surface_intent_reference_id: request.parent_surface_intent_reference_id,
        parent_preference_setup_reference_id: request.parent_preference_setup_reference_id,
        request_reference_ids: request.request_reference_ids,
        command_boundary_claimed: true,
        parent_preference_mutation_claimed: false,
        notification_rule_mutation_claimed: false,
        provider_delivery_claimed: false,
        durable_outbox_claimed: false,
        adapter_dispatch_claimed: false,
        platform_enforcement_claimed: false,
    }
}

pub fn app_game_timer_parent_preference_setup_request_payload(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> LogFields {
    fields_from_pairs(result_pairs(result))
}

fn result_pairs(result: &AppGameTimerParentPreferenceSetupRequestResult) -> Vec<FieldPair> {
    vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(result.accepted_at.clone()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(result.request_status.clone()),
        ),
        (
            constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST,
            LogFieldValue::String(
                serde_json::to_string(result).expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
    ]
}

fn request_from_command(
    command: &AgentCommandEnvelope,
) -> AppGameTimerParentPreferenceSetupRequest {
    match command
        .payload
        .get(constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST)
    {
        Some(LogFieldValue::String(value)) if !value.is_empty() => {
            serde_json::from_str(value).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => AppGameTimerParentPreferenceSetupRequest {
            request_id: command.message_id.clone(),
            requested_at: command.sent_at.clone(),
            parent_surface_intent_reference_id: command.message_id.clone(),
            parent_preference_setup_reference_id: command.message_id.clone(),
            request_reference_ids: vec![command.message_id.clone()],
        },
    }
}
