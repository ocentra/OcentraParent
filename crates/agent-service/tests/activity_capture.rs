extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "support/activity_capture_test_support.rs"]
pub mod test_support;

#[path = "../src/activity_capture.rs"]
mod activity_capture;
#[path = "../src/activity_store_path.rs"]
mod activity_store_path;
mod event_builder {
    pub fn portal_peer() -> ocentra_parent_agent_protocol::transport::AgentPeer {
        ocentra_parent_agent_protocol::transport::AgentPeer {
            peer_id: ocentra_parent_agent_protocol::constants::peer::PORTAL_DEV.to_string(),
            role: ocentra_parent_agent_protocol::transport::AgentPeerRole::Portal,
        }
    }
}
#[path = "../src/fields.rs"]
mod fields;
#[path = "../src/json_contract.rs"]
mod json_contract;
#[path = "support/test_invariants.rs"]
mod test_invariants;
#[path = "../src/time.rs"]
mod time;

mod dev_log {
    pub fn write_agent_info(
        message: &str,
        fields: ocentra_parent_agent_protocol::logging::LogFields,
    ) -> std::io::Result<()> {
        agent_service_lib::dev_log::write_agent_info(message, fields)
    }
}

#[path = "unit/activity_capture_browser_tests.rs"]
mod activity_capture_browser_tests;
#[path = "unit/activity_capture_freshness_tests.rs"]
mod activity_capture_freshness_tests;
#[path = "unit/activity_capture_inventory_tests.rs"]
mod activity_capture_inventory_tests;
#[path = "unit/activity_capture_tests.rs"]
mod activity_capture_tests;

#[cfg(test)]
mod clippy_linkage {
    use crate::activity_capture::{
        spawn_startup_activity_capture, startup_activity_capture_enabled,
        startup_activity_capture_enabled_for_value,
    };
    use crate::test_invariants::{
        require_json_decode, require_log_string_field, require_ok, require_some,
        serialize_test_json,
    };
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::logging::LogFieldValue;
    use std::env;

    #[test]
    fn activity_capture_runtime_and_helpers_are_linked() {
        let encoded = serialize_test_json(&serde_json::json!({
            "activity_capture": true
        }));
        let decoded: serde_json::Value =
            require_json_decode(&encoded, "activity_capture linkage json");
        assert!(require_some(
            decoded
                .get("activity_capture")
                .and_then(|value| value.as_bool()),
            "activity_capture linkage bool",
        ));
        let field = LogFieldValue::String(encoded);
        let text = require_log_string_field(Some(&field), "activity_capture linkage field");
        let _: serde_json::Value = require_json_decode(text, "activity_capture linkage field json");
        let _: () = require_ok(Ok::<(), std::io::Error>(()), "activity_capture linkage ok");
        let _ = crate::activity_store_path::activity_journal_path();
        let _ = crate::activity_store_path::activity_journal_key_path();
        let _ = crate::event_builder::portal_peer();
        let _ = crate::json_contract::serialize_json_value(serde_json::json!({
            "activity_capture": true
        }));
        let _ = crate::time::timestamp_from_epoch_seconds(1);
        let _ = crate::time::timestamp_after_epoch_seconds(1, 1);

        let previous = env::var(constants::env_var::ACTIVITY_CAPTURE_STARTUP_DISABLED).ok();
        env::set_var(
            constants::env_var::ACTIVITY_CAPTURE_STARTUP_DISABLED,
            constants::value::TRUE,
        );

        assert!(!startup_activity_capture_enabled());
        assert!(!startup_activity_capture_enabled_for_value(Some(
            constants::value::TRUE
        )));
        spawn_startup_activity_capture();

        match previous {
            Some(value) => {
                env::set_var(constants::env_var::ACTIVITY_CAPTURE_STARTUP_DISABLED, value)
            }
            None => env::remove_var(constants::env_var::ACTIVITY_CAPTURE_STARTUP_DISABLED),
        }
    }
}
