#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "support/command_dispatch_test_support.rs"]
pub mod test_support;

#[path = "../src/activity_payload.rs"]
mod activity_payload;
#[path = "support/activity_report_env_lock.rs"]
mod activity_report_env_lock;
#[path = "../src/activity_store_path.rs"]
mod activity_store_path;
#[path = "../src/activity_surface_store.rs"]
mod activity_surface_store;
#[path = "../src/enforcement_timer_state_file.rs"]
mod enforcement_timer_state_file;
#[path = "../src/enforcement_timer_state_path.rs"]
mod enforcement_timer_state_path;
#[path = "../src/event_builder.rs"]
mod event_builder;
#[path = "../src/fields.rs"]
mod fields;
#[path = "../src/json_contract.rs"]
mod json_contract;
#[path = "support/test_invariants.rs"]
mod test_invariants;
#[path = "../src/time.rs"]
mod time;

#[path = "../src/activity_api/app_game_timer_parent_surface_action_results.rs"]
mod app_game_timer_parent_surface_action_results;
#[path = "../src/activity_api/app_game_timer_parent_surface_payload.rs"]
mod app_game_timer_parent_surface_payload;

#[path = "unit/app_game_timer_parent_surface_payload_tests.rs"]
mod app_game_timer_parent_surface_payload_tests;
#[path = "unit/app_game_timer_parent_surface_service_tests.rs"]
mod app_game_timer_parent_surface_service_tests;

use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};

fn activity_store_error_event(
    command: AgentCommandEnvelope,
    event_id_suffix: &str,
    event: AgentEventName,
) -> AgentEventEnvelope {
    event_builder::build_event(
        event_id_suffix,
        &command.message_id,
        command.source,
        event,
        LogLevel::Error,
        activity_payload::activity_store_error_payload(),
        None,
    )
}

#[cfg(test)]
mod clippy_linkage {
    use crate::app_game_timer_parent_surface_payload;
    use crate::test_invariants::{
        require_json_decode, require_log_string_field, require_ok, require_some,
        serialize_test_json,
    };
    use ocentra_parent_agent_protocol::activity::ActivityObserver;
    use ocentra_parent_agent_protocol::activity::ActivitySubjectKind;
    use ocentra_parent_agent_protocol::activity_query::{
        ActivityIngestStatus, ActivityRecentSummary,
    };
    use ocentra_parent_agent_protocol::logging::LogFieldValue;
    use ocentra_parent_agent_protocol::transport::{
        AgentCommandEnvelope, AgentEventName, AgentMessageTarget, AgentPeer, AgentPeerRole,
        AgentRoute,
    };

    async fn activity_surface_store_helpers_are_linked() {
        let _ = crate::activity_store_path::activity_db_path();
        let _ = crate::activity_store_path::activity_journal_path();
        let _ = crate::activity_store_path::activity_journal_key_path();
        let _ = crate::activity_payload::ingest_status_payload(&ActivityIngestStatus {
            schema_version: 1,
            database_ready: true,
            events_ingested: 1,
            events_stored: 1,
            duplicate_events: 0,
            last_event_id: Some("event-1".to_string()),
        });
        let _ = crate::activity_payload::recent_summary_payload(&ActivityRecentSummary {
            schema_version: 1,
            limit: 1,
            returned: 1,
            first_observed_at: Some("2026-06-29T00:00:00Z".to_string()),
            last_observed_at: Some("2026-06-29T00:00:00Z".to_string()),
            last_event_id: Some("event-1".to_string()),
            most_recent_kind: None,
            most_recent_observer: Some(ActivityObserver::AgentService),
            most_recent_subject_kind: Some(ActivitySubjectKind::Device),
            most_recent_subject_id: Some("device-1".to_string()),
            most_recent_subject_name: Some("device".to_string()),
        });
        let _ = crate::activity_payload::activity_store_error_payload();
        if let Some(snapshot) = crate::activity_surface_store::local_store_snapshot().await {
            let _ = (
                snapshot.device_id.as_str(),
                snapshot.recent_returned,
                snapshot.last_event_id.as_deref(),
                snapshot.last_observed_at.as_deref(),
                snapshot.browser_returned,
                snapshot.network_returned,
                snapshot.games_returned,
                snapshot.screen_returned,
            );
        }
        let _ = crate::activity_surface_store::local_store_snapshot_from_path(
            std::path::PathBuf::from("C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/tmp/app-game-surface-clippy.db"),
        )
        .await;
        let _ = crate::activity_surface_store::load_browser_model().await;
        let _ = crate::activity_surface_store::load_browser_model_from_path(
            std::path::PathBuf::from("C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/tmp/app-game-surface-clippy.db"),
        )
        .await;
        let _ = crate::activity_surface_store::load_network_model().await;
        let _ = crate::activity_surface_store::load_network_model_from_path(
            std::path::PathBuf::from("C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/tmp/app-game-surface-clippy.db"),
        )
        .await;
        let _ = crate::activity_surface_store::load_app_game_model().await;
        let _ = crate::activity_surface_store::load_app_game_model_from_path(
            std::path::PathBuf::from("C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/tmp/app-game-surface-clippy.db"),
        )
        .await;
        let _ = crate::activity_surface_store::load_screen_summary().await;
        let _ = crate::activity_surface_store::load_screen_summary_from_path(
            std::path::PathBuf::from("C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/tmp/app-game-surface-clippy.db"),
        )
        .await;
        let _ = crate::enforcement_timer_state_file::read_active_timer_state(
            std::path::Path::new(
                "C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/tmp/app-game-surface-clippy.db",
            ),
        )
        .await;
        let _ = crate::enforcement_timer_state_file::remove_active_timer_state(
            std::path::Path::new(
                "C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/tmp/app-game-surface-clippy.db",
            ),
        )
        .await;
        let _ = crate::event_builder::portal_peer();
        let _ = crate::json_contract::serialize_json_value(serde_json::json!({
            "app_game_timer_parent_surface": true
        }));
        let _ = crate::time::timestamp_from_epoch_seconds(1);
        let _ = crate::time::timestamp_after_epoch_seconds(1, 1);
    }

    #[tokio::test]
    async fn test_invariants_are_linked() {
        let encoded = serialize_test_json(&serde_json::json!({
            "app_game_timer_parent_surface": true
        }));
        let decoded: serde_json::Value =
            require_json_decode(&encoded, "app_game_timer_parent_surface linkage json");
        assert!(require_some(
            decoded
                .get("app_game_timer_parent_surface")
                .and_then(|value| value.as_bool()),
            "app_game_timer_parent_surface linkage bool",
        ));

        let field = LogFieldValue::String(encoded);
        let text =
            require_log_string_field(Some(&field), "app_game_timer_parent_surface linkage field");
        let _: serde_json::Value =
            require_json_decode(text, "app_game_timer_parent_surface linkage field json");
        let _: () = require_ok(
            Ok::<(), std::io::Error>(()),
            "app_game_timer_parent_surface linkage ok",
        );
        activity_surface_store_helpers_are_linked().await;

        let event = app_game_timer_parent_surface_payload::build_activity_app_game_timer_parent_surface_report(
            AgentCommandEnvelope {
                schema_version: ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION,
                message_id: "cmd-app-game-surface-clippy".to_string(),
                sent_at: "2026-06-29T00:00:00Z".to_string(),
                source: AgentPeer {
                    peer_id: ocentra_parent_agent_protocol::constants::peer::PORTAL_DEV
                        .to_string(),
                    role: AgentPeerRole::Portal,
                },
                target: AgentMessageTarget {
                    device_id:
                        ocentra_parent_agent_protocol::constants::peer::LOCAL_DEV_AGENT.to_string(),
                    platform: "windows".to_string(),
                    route: AgentRoute::Localhost,
                },
                command:
                    ocentra_parent_agent_protocol::transport::AgentCommandName::AgentActivityAppGameTimerParentSurfaceReadModelGet,
                payload: ocentra_parent_agent_protocol::logging::LogFields::new(),
            },
        )
        .await;
        assert_eq!(
            event.event,
            AgentEventName::AgentActivityAppGameTimerParentSurfaceReadModelReported
        );
    }
}
