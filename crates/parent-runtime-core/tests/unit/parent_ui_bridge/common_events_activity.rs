use super::super::super::*;

use super::responses::{app_game_read_model_response_event, PayloadText};

pub(crate) fn app_use_read_model_response_event() -> AgentEventEnvelope {
    activity_surface_read_model_response_event(
        PayloadText("app-use-read-model-1".to_string()),
        PayloadText("app-use-read-model".to_string()),
        AgentEventName::AgentActivityAppUseReadModelReported,
        constants::activity_surface::READ_MODEL_APP_USE,
        json!({
            "schemaVersion": 1,
            "request": {
                "schemaVersion": 1,
                "scope": {
                    "scopeKind": "family",
                    "familyId": null,
                    "deviceId": null
                },
                "requestedAt": "2026-06-08T12:45:00Z",
                "rangeStart": "2026-06-07T00:00:00Z",
                "rangeEnd": "2026-06-08T12:45:00Z"
            },
            "state": "ready",
            "generatedAt": "2026-06-08T12:45:00Z",
            "summary": "App-use activity is ready.",
            "rows": []
        }),
    )
}

pub(crate) fn games_read_model_response_event() -> AgentEventEnvelope {
    activity_surface_read_model_response_event(
        PayloadText("games-read-model-1".to_string()),
        PayloadText("games-read-model".to_string()),
        AgentEventName::AgentActivityGamesReadModelReported,
        constants::activity_surface::READ_MODEL_GAMES,
        json!({
            "schemaVersion": 1,
            "request": {
                "schemaVersion": 1,
                "scope": {
                    "scopeKind": "family",
                    "familyId": null,
                    "deviceId": null
                },
                "requestedAt": "2026-06-08T12:45:00Z",
                "rangeStart": "2026-06-07T00:00:00Z",
                "rangeEnd": "2026-06-08T12:45:00Z"
            },
            "state": "ready",
            "generatedAt": "2026-06-08T12:45:00Z",
            "summary": "Games activity is ready.",
            "rows": []
        }),
    )
}

fn activity_surface_read_model_response_event(
    event_id: PayloadText,
    correlation_id: PayloadText,
    event: AgentEventName,
    read_model_kind: &str,
    read_model: Value,
) -> AgentEventEnvelope {
    let mut response = app_game_read_model_response_event(
        event_id,
        correlation_id,
        event,
        PayloadText(constants::field::ACTIVITY_READ_MODEL.to_string()),
        &read_model,
    );
    response.payload.insert(
        constants::field::ACTIVITY_READ_MODEL_KIND.to_string(),
        LogFieldValue::String(read_model_kind.to_string()),
    );
    response
}
