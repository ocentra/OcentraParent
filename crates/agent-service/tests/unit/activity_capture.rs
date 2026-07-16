extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../../src/activity_capture.rs"]
mod activity_capture;
#[path = "../../src/activity_store_path.rs"]
mod activity_store_path;
#[path = "../support/dev_log.rs"]
mod dev_log;
#[path = "../../src/event_builder.rs"]
mod event_builder;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../../src/json_contract.rs"]
mod json_contract;
#[path = "../support/test_invariants.rs"]
mod test_invariants;
#[path = "../support/activity_capture_test_support.rs"]
mod test_support;
#[path = "../support/test_text.rs"]
mod test_text;
#[path = "../../src/time.rs"]
mod time;

#[path = "activity_capture_browser_tests.rs"]
mod activity_capture_browser_tests;
#[path = "activity_capture_freshness_tests.rs"]
mod activity_capture_freshness_tests;
#[cfg(windows)]
#[path = "activity_capture_inventory_tests.rs"]
mod activity_capture_inventory_tests;
#[path = "activity_capture_tests.rs"]
mod activity_capture_tests;

#[cfg(test)]
mod clippy_linkage {
    use crate::activity_capture::{
        spawn_startup_activity_capture, startup_activity_capture_enabled,
        startup_activity_capture_enabled_for_value, StartupActivityCaptureDisabledValue,
    };
    use crate::test_invariants::{
        require_json_decode, require_log_string_field, require_ok, require_some,
        serialize_test_json,
    };
    use crate::test_text::TestText;
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::logging::LogFieldValue;
    use std::env;

    #[test]
    fn activity_capture_runtime_and_helpers_are_linked() -> Result<(), TestText> {
        let encoded = serde_json::json!({
            "activity_capture": true
        })
        .to_string();
        let decoded: serde_json::Value = serde_json::from_str(&encoded).map_err(|error| {
            TestText::from_display(format!("activity_capture linkage json: {error}"))
        })?;
        assert_eq!(decoded["activity_capture"], true);
        let serialized = serialize_test_json(&decoded);
        let _: serde_json::Value =
            require_json_decode(&serialized, constants::error::AGENT_EVENT_SERIALIZES);
        let _: () = require_ok(
            Ok::<(), TestText>(()),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let LogFieldValue::String(text) = LogFieldValue::String(encoded) else {
            return Err(TestText::from_display(
                "activity_capture linkage field must be text",
            ));
        };
        assert_eq!(
            require_log_string_field(
                Some(&LogFieldValue::String(text.clone())),
                constants::error::AGENT_EVENT_SERIALIZES,
            ),
            text.as_str()
        );
        assert!(require_some(
            Some(true),
            constants::error::AGENT_EVENT_SERIALIZES
        ));
        let _: serde_json::Value = serde_json::from_str(&text).map_err(|error| {
            TestText::from_display(format!("activity_capture linkage field json: {error}"))
        })?;
        let _ = crate::activity_store_path::activity_journal_path();
        let _ = crate::activity_store_path::activity_journal_key_path();
        let event = crate::event_builder::build_event(
            "activity-capture",
            "activity-capture-correlation",
            crate::event_builder::portal_peer(),
            ocentra_parent_agent_protocol::transport::AgentEventName::AgentHealthReported,
            ocentra_parent_agent_protocol::logging::LogLevel::Info,
            ocentra_parent_agent_protocol::logging::LogFields::new(),
            None,
        );
        assert_eq!(event.correlation_id, "activity-capture-correlation");
        let _ = crate::json_contract::serialize_json_value(serde_json::json!({
            "activity_capture": true
        }));
        let json_text = crate::json_contract::serialize_json_string(&decoded);
        assert_eq!(json_text.0, decoded.to_string());
        let _: String = crate::time::timestamp_from_epoch_seconds(1);
        let _: String = crate::time::timestamp_after_epoch_seconds(1, 1);
        let _ = crate::dev_log::write_agent_info_ref;
        let _ = crate::dev_log::write_agent_warn_ref;
        let _ = crate::dev_log::write_agent_error_ref;
        let _ = crate::dev_log::write_agent_debug_ref;

        let previous = env::var(constants::env_var::ACTIVITY_CAPTURE_STARTUP_DISABLED).ok();
        env::set_var(
            constants::env_var::ACTIVITY_CAPTURE_STARTUP_DISABLED,
            constants::value::TRUE,
        );

        assert!(!startup_activity_capture_enabled());
        assert!(!startup_activity_capture_enabled_for_value(
            &StartupActivityCaptureDisabledValue(Some(constants::value::TRUE))
        ));
        spawn_startup_activity_capture();

        match previous {
            Some(value) => {
                env::set_var(constants::env_var::ACTIVITY_CAPTURE_STARTUP_DISABLED, value)
            }
            None => env::remove_var(constants::env_var::ACTIVITY_CAPTURE_STARTUP_DISABLED),
        }
        Ok(())
    }
}
