use ocentra_parent_agent_protocol::{
    activity_query::{ActivityIngestStatus, ActivityRecentSummary},
    constants,
    logging::LogFieldValue,
    transport::{
        AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
        AgentPeerRole, AgentRoute,
    },
    AGENT_PROTOCOL_SCHEMA_VERSION,
};

#[test]
fn activity_payload_serializers_project_app_game_read_model_statuses() {
    let ingest_payload = super::activity_payload::ingest_status_payload(&ActivityIngestStatus {
        schema_version: 1,
        database_ready: true,
        events_ingested: 3,
        events_stored: 2,
        duplicate_events: 1,
        last_event_id: Some("app-game-event-1".to_string()),
    });
    assert_eq!(
        ingest_payload.get(constants::field::DATABASE_READY),
        Some(&LogFieldValue::Boolean(true))
    );
    assert_eq!(
        ingest_payload.get(constants::field::LAST_EVENT_ID),
        Some(&LogFieldValue::String("app-game-event-1".to_string()))
    );

    let recent_payload = super::activity_payload::recent_summary_payload(&ActivityRecentSummary {
        schema_version: 1,
        limit: 10,
        returned: 2,
        first_observed_at: None,
        last_observed_at: None,
        last_event_id: None,
        most_recent_kind: None,
        most_recent_observer: None,
        most_recent_subject_kind: None,
        most_recent_subject_id: None,
        most_recent_subject_name: None,
    });
    assert_eq!(
        recent_payload.get(constants::field::LIMIT),
        Some(&LogFieldValue::Number(10.0))
    );
    assert_eq!(
        recent_payload.get(constants::field::RETURNED),
        Some(&LogFieldValue::Number(2.0))
    );
}

#[test]
fn activity_store_error_event_projects_the_app_game_read_model_failure() {
    let event = super::activity_store_error_event::activity_store_error_event(
        AgentCommandEnvelope {
            schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
            message_id: constants::event_id::ACTIVITY_APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL_REPORTED
                .to_string(),
            sent_at: "2026-07-13T00:00:00Z".to_string(),
            source: AgentPeer {
                peer_id: constants::peer::PORTAL_DEV.to_string(),
                role: AgentPeerRole::Portal,
            },
            target: AgentMessageTarget {
                device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
                platform: ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
                    .to_string(),
                route: AgentRoute::Localhost,
            },
            command: AgentCommandName::AgentActivityAppGameTimerParentSurfaceReadModelGet,
            payload: Default::default(),
        },
        super::activity_api::ActivityEventId(
            constants::event_id::ACTIVITY_APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL_REPORTED,
        ),
        AgentEventName::AgentActivityAppGameTimerParentSurfaceReadModelReported,
    );

    assert_eq!(
        event.event,
        AgentEventName::AgentActivityAppGameTimerParentSurfaceReadModelReported
    );
    assert_eq!(
        event.payload.get(constants::field::REASON),
        Some(&LogFieldValue::String(
            constants::value::ACTIVITY_STORE_UNAVAILABLE.to_string()
        ))
    );
}

#[tokio::test]
async fn app_game_timer_parent_surface_report_builder_is_linked_in_aggregate_target() {
    let event = super::app_game_timer_parent_surface_payload::build_activity_app_game_timer_parent_surface_report(
        AgentCommandEnvelope {
            schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
            message_id: constants::event_id::ACTIVITY_APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL_REPORTED
                .to_string(),
            sent_at: "2026-07-13T00:00:00Z".to_string(),
            source: AgentPeer {
                peer_id: constants::peer::PORTAL_DEV.to_string(),
                role: AgentPeerRole::Portal,
            },
            target: AgentMessageTarget {
                device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
                platform: ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
                    .to_string(),
                route: AgentRoute::Localhost,
            },
            command: AgentCommandName::AgentActivityAppGameTimerParentSurfaceReadModelGet,
            payload: Default::default(),
        },
    )
    .await;

    assert_eq!(
        event.event,
        AgentEventName::AgentActivityAppGameTimerParentSurfaceReadModelReported
    );
}
