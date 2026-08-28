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

#[test]
fn app_game_activity_read_models_clippy_linkage() {
    let _ = super::activity_capture::startup_activity_capture_enabled;
    let _ = super::activity_capture::startup_activity_capture_enabled_for_value;
    let _ = super::activity_capture::record_activity_capture_once;
    let _ = super::activity_capture::record_activity_capture_to_paths;
    let _ = super::activity_capture::record_activity_capture_to_paths_at;
    let _ = super::activity_store_path::activity_journal_path;
    let _ = super::activity_store_path::activity_journal_key_path;
    let _ = super::activity_surface_store::local_store_snapshot;
    let _ = super::activity_surface_store::local_store_snapshot_from_path;
    let _ = super::activity_surface_store::load_browser_model;
    let _ = super::activity_surface_store::load_browser_model_from_path;
    let _ = super::activity_surface_store::load_network_model;
    let _ = super::activity_surface_store::load_network_model_from_path;
    let _ = super::activity_surface_store::load_app_game_model;
    let _ = super::activity_surface_store::load_app_game_model_from_path;
    let _ = super::activity_surface_store::load_screen_summary;
    let _ = super::activity_surface_store::load_screen_summary_from_path;
    let snapshot = super::activity_surface_store::ActivitySurfaceStoreSnapshot {
        device_id: super::activity_surface_store::ActivitySurfaceDeviceRefText(String::new()),
        recent_returned: 0,
        last_event_id: None,
        last_observed_at: None,
        browser_returned: 0,
        network_returned: 0,
        games_returned: 0,
        screen_returned: 0,
    };
    let _ = (
        snapshot.device_id,
        snapshot.recent_returned,
        snapshot.last_event_id,
        snapshot.last_observed_at,
        snapshot.browser_returned,
        snapshot.network_returned,
        snapshot.games_returned,
        snapshot.screen_returned,
    );
    let _ = super::dev_log::write_agent_info_ref;
    let _ = super::dev_log::write_agent_warn_ref;
    let _ = super::dev_log::write_agent_error_ref;
    let _ = super::dev_log::write_agent_debug_ref;
    let _ = super::enforcement_api::build_enforcement_audit_report;
    let _ = super::enforcement_api::build_enforcement_product_control_spine_report;
    let _ = super::enforcement_api::build_enforcement_policy_dispatch_report;
    let _ = super::enforcement_api::enforcement_broad_adapter_proof_report::build_enforcement_broad_adapter_proof_report;
    let _ = super::enforcement_api::enforcement_supported_adapter_runtime_proof_report::build_enforcement_supported_adapter_runtime_proof_report;
    let _ =
        super::enforcement_browser_domain_adapter_app_control_proof_states::app_control_state_specs;
    let _ = super::enforcement_browser_domain_adapter_proof_read_model::v08_browser_domain_adapter_proof_read_model(
        super::enforcement_os_adapter_product_proof_read_model::product_control_spine::GeneratedAtText(
            String::new(),
        ),
    );
    let _ = super::enforcement_cross_platform_capability_proof_read_model::v08_cross_platform_enforcement_capability_proof_read_model(
        super::enforcement_cross_platform_capability_proof_read_model::GeneratedAtTextRef(
            "",
        ),
    );
    let _ = super::enforcement_os_adapter_product_proof_read_model::v08_os_adapter_product_proof_read_model(
        super::enforcement_os_adapter_product_proof_read_model::GeneratedAtTextRef(""),
    );
    let _ = super::enforcement_payload::parse_enforcement_command_payload;
    let _ =
        super::enforcement_policy_dispatch_read_model::v08_enforcement_policy_dispatch_read_model(
            super::enforcement_policy_dispatch_read_model::DispatchText(String::new()),
        );
    let _ = super::enforcement_timer_state_file::read_active_timer_state;
    let _ = super::enforcement_timer_state_file::remove_active_timer_state;
    let _ = super::enforcement_timer_state_path::enforcement_timer_state_path;
    let _: fn(
        super::event_builder::EventIdSuffixText,
        super::event_builder::EventCorrelationIdText,
        ocentra_parent_agent_protocol::transport::AgentPeer,
        ocentra_parent_agent_protocol::transport::AgentEventName,
        ocentra_parent_agent_protocol::logging::LogLevel,
        ocentra_parent_agent_protocol::logging::LogFields,
        Option<ocentra_parent_agent_protocol::logging::AgentLogSnapshot>,
    ) -> ocentra_parent_agent_protocol::transport::AgentEventEnvelope =
        super::event_builder::build_event;
    let _ = super::event_builder::portal_peer;
    let _ = super::host_identity_read_model::host_identity_read_model;
    let _ = super::json_contract::serialize_json_string::<serde_json::Value>;
    let _ = super::json_contract::serialize_json_value::<serde_json::Value>;
    let _: fn(u64, u64) -> String = super::time::timestamp_after_epoch_seconds;
    let _: fn(u64, u64) -> String = super::time::timestamp_after_epoch_seconds;
    let _ = super::windows_adapter_artifact_gate_read_model::evaluate_windows_adapter_artifact_gate;
    let _ = super::windows_adapter_artifact_gate_read_model::windows_adapter_artifact_gate_proof;
    let _ = super::windows_adapter_capability_read_model::windows_adapter_capability_proof;
    let _ = super::app_game_adapter_dispatch_execute_payload::build_activity_app_game_adapter_dispatch_execute_report;
    let _ = super::app_game_dispatch_evidence::AppGameDispatchEvidenceRejection::log_value;
    let _ = super::app_game_dispatch_evidence::validate_app_game_dispatch_evidence;
    let _ = super::app_game_dispatch_evidence::validate_app_game_timer_session;
    let _ = super::app_game_adapter_dispatch_preflight_payload::build_activity_app_game_adapter_dispatch_preflight_report;
    let _ = super::app_game_adapter_execution_readiness_payload::build_activity_app_game_adapter_execution_readiness_report;
    let _ = super::app_game_child_runtime_transport_receipt_payload::build_activity_app_game_child_runtime_transport_receipt_report;
    let _ =
        super::app_game_platform_proof_status_payload::app_game_platform_proof_status_read_model;
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
