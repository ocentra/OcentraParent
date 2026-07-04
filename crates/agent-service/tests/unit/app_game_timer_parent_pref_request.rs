#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../support/test_text.rs"]
mod test_text;

#[path = "../support/command_dispatch_test_support.rs"]
pub mod test_support;

#[path = "../../src/activity_store_path.rs"]
mod activity_store_path;
#[path = "../../src/activity_surface_store.rs"]
mod activity_surface_store;
#[path = "../../src/event_builder.rs"]
mod event_builder;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../../src/json_contract.rs"]
mod json_contract;
#[path = "../support/test_invariants.rs"]
mod test_invariants;
#[path = "../../src/time.rs"]
mod time;

#[path = "../../src/activity_api/app_game_child_runtime_transport_receipt_payload.rs"]
mod app_game_child_runtime_transport_receipt_payload;
#[path = "../../src/activity_api/app_game_timer_parent_preference_setup_request.rs"]
mod app_game_timer_parent_preference_setup_request;
#[path = "../../src/activity_api/app_game_timer_parent_preference_setup_request_outbox.rs"]
mod app_game_timer_parent_preference_setup_request_outbox;
#[path = "../../src/activity_api/app_game_timer_parent_preference_setup_request_persistence.rs"]
mod app_game_timer_parent_preference_setup_request_persistence;
#[path = "../../src/activity_api/app_game_timer_parent_preference_setup_request_status.rs"]
mod app_game_timer_parent_preference_setup_request_status;

#[cfg(test)]
mod clippy_linkage {
    use super::*;
    use crate::test_invariants::{
        require_json_decode, require_log_string_field, require_ok, require_some,
        serialize_test_json,
    };
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
    use ocentra_parent_agent_protocol::transport::{
        AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
        AgentPeerRole, AgentRoute,
    };
    use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
    use std::{env, fs::remove_file};

    #[tokio::test]
    async fn public_wrapper_and_outbox_helper_are_linked() {
        let encoded = serialize_test_json(&serde_json::json!({
            "app_game_pref_request": true
        }));
        let decoded: serde_json::Value =
            require_json_decode(&encoded, "app_game_timer_parent_pref_request linkage json");
        assert!(require_some(
            decoded
                .get("app_game_pref_request")
                .and_then(|value| value.as_bool()),
            "app_game_timer_parent_pref_request linkage bool",
        ));
        let field = LogFieldValue::String(encoded);
        let text = require_log_string_field(
            Some(&field),
            "app_game_timer_parent_pref_request linkage field",
        );
        let _: serde_json::Value = require_json_decode(
            text,
            "app_game_timer_parent_pref_request linkage field json",
        );
        let _: () = require_ok(
            Ok::<(), std::io::Error>(()),
            "app_game_timer_parent_pref_request linkage ok",
        );
        link_shared_pref_request_helpers();

        let mut store_name = String::from(constants::activity_store::TEST_FILE_PREFIX);
        store_name.push_str(&std::process::id().to_string());
        store_name.push(constants::delimiter::HYPHEN);
        store_name.push_str("app-game-pref-request-clippy");

        let mut store_path = std::env::temp_dir();
        store_path.push(store_name);
        store_path.set_extension(constants::activity_store::FILE_EXTENSION);
        cleanup_path(&store_path);
        let previous_store_path = env::var(constants::env_var::ACTIVITY_DB_PATH).ok();
        env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);

        let event = app_game_timer_parent_preference_setup_request::build_activity_app_game_timer_parent_preference_setup_request_report(
            command_envelope(),
        )
        .await;
        link_activity_surface_store_helpers(&store_path).await;
        let _ =
            app_game_child_runtime_transport_receipt_payload::build_activity_app_game_child_runtime_transport_receipt_report(
                command_envelope(),
            )
            .await;
        let _ = app_game_timer_parent_preference_setup_request_outbox::setup_outbox_has_records(
            &store_path,
        );

        assert_eq!(
            event.event,
            AgentEventName::AgentActivityAppGameTimerParentPreferenceSetupRequested
        );

        match previous_store_path {
            Some(value) => env::set_var(constants::env_var::ACTIVITY_DB_PATH, value),
            None => env::remove_var(constants::env_var::ACTIVITY_DB_PATH),
        }
        cleanup_path(&store_path);
    }

    fn link_shared_pref_request_helpers() {
        let _ = crate::activity_store_path::activity_journal_path();
        let _ = crate::activity_store_path::activity_journal_key_path();
        let _ = crate::event_builder::portal_peer();
        let _ = crate::json_contract::serialize_json_value(serde_json::json!({
            "app_game_pref_request": true
        }));
        let _ = crate::time::timestamp_from_epoch_seconds(1);
        let _ = crate::time::timestamp_after_epoch_seconds(1, 1);
    }

    async fn link_activity_surface_store_helpers(store_path: &std::path::Path) {
        if let Some(snapshot) = activity_surface_store::local_store_snapshot().await {
            touch_activity_surface_snapshot(&snapshot);
        }
        if let Some(snapshot) =
            activity_surface_store::local_store_snapshot_from_path(store_path.to_path_buf()).await
        {
            touch_activity_surface_snapshot(&snapshot);
        }
        let _ = activity_surface_store::load_browser_model().await;
        let _ =
            activity_surface_store::load_browser_model_from_path(store_path.to_path_buf()).await;
        let _ = activity_surface_store::load_network_model().await;
        let _ =
            activity_surface_store::load_network_model_from_path(store_path.to_path_buf()).await;
        let _ = activity_surface_store::load_app_game_model().await;
        let _ =
            activity_surface_store::load_app_game_model_from_path(store_path.to_path_buf()).await;
        let _ = activity_surface_store::load_screen_summary().await;
        let _ =
            activity_surface_store::load_screen_summary_from_path(store_path.to_path_buf()).await;
    }

    fn touch_activity_surface_snapshot(
        snapshot: &activity_surface_store::ActivitySurfaceStoreSnapshot,
    ) {
        let _ = (
            snapshot.device_id.as_str(),
            snapshot.last_event_id.as_deref(),
            snapshot.last_observed_at.as_deref(),
            snapshot.recent_returned,
            snapshot.browser_returned,
            snapshot.network_returned,
            snapshot.games_returned,
            snapshot.screen_returned,
        );
    }

    fn command_envelope() -> AgentCommandEnvelope {
        AgentCommandEnvelope {
            schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
            message_id: "cmd-app-game-pref-linkage".to_string(),
            sent_at: "2026-06-29T00:00:00Z".to_string(),
            source: AgentPeer {
                peer_id: constants::peer::PORTAL_DEV.to_string(),
                role: AgentPeerRole::Portal,
            },
            target: AgentMessageTarget {
                device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
                platform: "windows".to_string(),
                route: AgentRoute::Localhost,
            },
            command: AgentCommandName::AgentActivityAppGameTimerParentPreferenceSetupRequest,
            payload: LogFields::new(),
        }
    }

    fn cleanup_path(path: &std::path::Path) {
        let _ = remove_file(path);
        let mut wal_path = path.to_path_buf();
        wal_path.set_file_name(format!(
            "{}.{}",
            path.file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or_default(),
            constants::activity_store::WAL_FILE_EXTENSION
        ));
        let _ = remove_file(wal_path);
        let mut shm_path = path.to_path_buf();
        shm_path.set_file_name(format!(
            "{}.{}",
            path.file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or_default(),
            constants::activity_store::SHM_FILE_EXTENSION
        ));
        let _ = remove_file(shm_path);
    }
}
