use ocentra_parent_agent_protocol::app_game::APP_GAME_TIMER_PARENT_SURFACE_SERIALIZATION_ERROR;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};

use crate::{
    activity_surface_store::app_game::load_app_game_model,
    enforcement_timer_state_file::read_active_timer_state,
    enforcement_timer_state_path::enforcement_timer_state_path, event_builder::build_event,
    fields::fields_from_pairs,
};

use super::{
    activity_store_error_event::activity_store_error_event,
    app_game_timer_parent_surface_payload::{
        app_game_timer_parent_surface_from_service_model_with_timer_state,
        app_game_timer_parent_surface_payload,
    },
};

pub async fn build_activity_app_game_timer_parent_surface_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match load_app_game_model().await {
        Some(model) => {
            let timer_state = read_active_timer_state(&enforcement_timer_state_path())
                .await
                .ok()
                .flatten();
            let read_model = app_game_timer_parent_surface_from_service_model_with_timer_state(
                &model,
                timer_state.as_ref(),
            );
            match app_game_timer_parent_surface_payload(&read_model) {
                Ok(payload) => build_event(
                    constants::event_id::ACTIVITY_APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL_REPORTED,
                    &command.message_id,
                    command.source,
                    AgentEventName::AgentActivityAppGameTimerParentSurfaceReadModelReported,
                    LogLevel::Info,
                    payload,
                    None,
                ),
                Err(_error) => timer_parent_surface_serialization_error_event(command),
            }
        }
        None => activity_store_error_event(
            command,
            crate::activity_api::ActivityEventId(
                constants::event_id::ACTIVITY_APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL_REPORTED,
            ),
            AgentEventName::AgentActivityAppGameTimerParentSurfaceReadModelReported,
        ),
    }
}

fn timer_parent_surface_serialization_error_event(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    build_event(
        constants::event_id::ACTIVITY_APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentActivityAppGameTimerParentSurfaceReadModelReported,
        LogLevel::Error,
        fields_from_pairs(vec![(
            constants::field::REASON,
            LogFieldValue::String(APP_GAME_TIMER_PARENT_SURFACE_SERIALIZATION_ERROR.to_string()),
        )]),
        None,
    )
}
