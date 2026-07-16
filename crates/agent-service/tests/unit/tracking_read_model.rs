#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../support/test_text.rs"]
mod test_text;

#[path = "../support/command_dispatch_test_support.rs"]
pub mod test_support;

#[path = "../support/activity_report_env_lock.rs"]
mod activity_report_env_lock;
#[path = "../../src/event_builder.rs"]
mod event_builder;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../support/test_invariants.rs"]
mod test_invariants;
#[path = "../../src/time.rs"]
mod time;
#[path = "../../src/websocket/tracking_retention_settings_write.rs"]
mod tracking_retention_settings_write;

#[cfg(test)]
mod clippy_linkage {
    use super::test_invariants::{
        require_json_decode, require_log_string_field, require_ok, require_some,
        serialize_test_json,
    };
    use ocentra_parent_agent_protocol::logging::LogFieldValue;

    #[test]
    fn tracking_read_model_harness_links_test_invariants() {
        let serialized = serialize_test_json(&Some("tracking"));
        let decoded: Option<String> = require_json_decode(&serialized, "tracking JSON");
        assert_eq!(require_some(decoded, "tracking value"), "tracking");
        assert_eq!(
            require_ok::<_, ()>(Ok("tracking"), "tracking result"),
            "tracking"
        );
        let field = LogFieldValue::String(serialized);
        assert_eq!(
            require_log_string_field(Some(&field), "tracking field"),
            "\"tracking\""
        );
    }
}

#[test]
fn tracking_read_model_harness_links_epoch_time_helpers() {
    let portal = event_builder::portal_peer();
    assert_eq!(
        portal.peer_id,
        ocentra_parent_agent_protocol::constants::peer::PORTAL_DEV
    );

    assert_eq!(
        time::timestamp_after_epoch_seconds::<String>(0, 0),
        "1970-01-01T00:00:00.000Z"
    );
    assert_eq!(
        time::timestamp_after_epoch_seconds::<String>(30, 3),
        "1970-01-01T00:00:33.000Z"
    );
}

#[path = "tracking_read_model_service_tests.rs"]
mod tracking_read_model_service_tests;
#[path = "tracking_retention_settings_write_tests.rs"]
mod tracking_retention_settings_write_tests;
