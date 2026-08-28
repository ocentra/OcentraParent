#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

use std::path::PathBuf as TestPathBuf;
use std::primitive::str as TestStr;
use std::string::String as TestString;

#[path = "../support/test_text.rs"]
mod test_text;

#[path = "../support/activity_report_env_lock.rs"]
mod activity_report_env_lock;
#[path = "../../src/activity_store_path.rs"]
mod activity_store_path;
#[path = "../../src/event_builder.rs"]
mod event_builder;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../../src/json_contract.rs"]
mod json_contract;
#[path = "../../src/websocket/policy_request_confirm.rs"]
mod policy_request_confirm;
#[path = "../../src/websocket/policy_request_resolution/persistence.rs"]
mod policy_request_resolution_persistence;
#[path = "../support/test_invariants.rs"]
mod test_invariants;
#[path = "../../src/time.rs"]
mod time;

#[path = "policy_request_confirm_tests.rs"]
mod policy_request_confirm_tests;

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

    struct ActivityDbPathRestore(Option<String>);

    impl Drop for ActivityDbPathRestore {
        fn drop(&mut self) {
            match self.0.take() {
                Some(value) => env::set_var(constants::env_var::ACTIVITY_DB_PATH, value),
                None => env::remove_var(constants::env_var::ACTIVITY_DB_PATH),
            }
        }
    }

    #[tokio::test]
    async fn public_wrapper_and_helpers_are_linked() {
        let _guard = activity_report_env_lock::REPORT_ENV_LOCK.lock().await;
        let encoded = serialize_test_json(&serde_json::json!({
            "policy_request_confirm": true
        }));
        let decoded: serde_json::Value =
            require_json_decode(&encoded, "policy_request_confirm linkage json");
        assert!(require_some(
            decoded
                .get("policy_request_confirm")
                .and_then(|value| value.as_bool()),
            "policy_request_confirm linkage bool",
        ));
        let field = LogFieldValue::String(encoded);
        let text = require_log_string_field(Some(&field), "policy_request_confirm linkage field");
        let _: serde_json::Value =
            require_json_decode(text, "policy_request_confirm linkage field json");
        let _: () = require_ok(
            Ok::<(), std::io::Error>(()),
            "policy_request_confirm linkage ok",
        );
        let _ = crate::activity_store_path::activity_journal_path();
        let _ = crate::activity_store_path::activity_journal_key_path();
        let _ = crate::event_builder::portal_peer();
        let _ = crate::json_contract::serialize_json_value(serde_json::json!({
            "policy_request_confirm": true
        }));
        let _: String = crate::time::timestamp_after_epoch_seconds(1, 0);
        let _: String = crate::time::timestamp_after_epoch_seconds(1, 1);

        let store_path = temp_store_path("policy-request-confirm-clippy");
        cleanup_path(&store_path);
        let _restore_activity_db_path =
            ActivityDbPathRestore(env::var(constants::env_var::ACTIVITY_DB_PATH).ok());
        env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);

        let event = policy_request_confirm::build_policy_request_assistant_preview_confirm_report(
            command_envelope(),
        )
        .await;

        assert_eq!(
            event.event,
            AgentEventName::AgentPolicyRequestAssistantPreviewConfirmReported
        );

        cleanup_path(&store_path);
    }

    fn command_envelope() -> AgentCommandEnvelope {
        AgentCommandEnvelope {
            schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
            message_id: "cmd-policy-request-confirm-clippy".to_string(),
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
            command: AgentCommandName::AgentPolicyRequestAssistantPreviewConfirm,
            payload: LogFields::new(),
        }
    }

    fn temp_store_path(suffix: &TestStr) -> TestPathBuf {
        let mut name = TestString::from(constants::activity_store::TEST_FILE_PREFIX);
        name.push_str(&std::process::id().to_string());
        name.push(constants::delimiter::HYPHEN);
        name.push_str(suffix);

        let mut path = std::env::temp_dir();
        path.push(name);
        path.set_extension(constants::activity_store::FILE_EXTENSION);
        path
    }

    fn cleanup_path(path: &TestPathBuf) {
        let _ = remove_file(path);
        let mut wal_path = path.clone();
        wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
        let _ = remove_file(wal_path);
        let mut shm_path = path.clone();
        shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
        let _ = remove_file(shm_path);
    }
}
