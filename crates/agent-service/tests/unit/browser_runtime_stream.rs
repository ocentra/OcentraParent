#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[macro_use]
#[path = "../support/unit_root_basic_harness.rs"]
mod unit_root_basic_harness;
declare_agent_service_unit_root_basic_harness!();

use crate::browser_runtime_stream_payload::browser_runtime_event_chain_stream_payload;
use crate::browser_runtime_stream_tests::{
    managed_row, policy_preview_read_model_for_browser, read_model, stream_entries, TestResult,
};
use ocentra_parent_agent_protocol::constants;
use std::io::Error as IoError;

#[path = "../support/command_dispatch_test_support.rs"]
pub mod test_support;

#[path = "../support/activity_report_env_lock.rs"]
mod activity_report_env_lock;
#[path = "browser_runtime_stream_tests.rs"]
mod browser_runtime_stream_tests;

#[cfg(test)]
mod clippy_linkage {
    use crate::browser_runtime_stream_payload::browser_runtime_event_chain_stream_payload;
    use crate::browser_runtime_stream_tests::{
        managed_row, policy_preview_read_model_for_browser, read_model, stream_entries,
    };
    use crate::test_invariants::{
        require_json_decode, require_log_string_field, require_ok, serialize_test_json,
    };
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::logging::LogFieldValue;

    #[tokio::test]
    async fn browser_runtime_stream_helpers_are_linked() {
        let read_model = read_model(vec![managed_row()]);
        let policy_preview = match policy_preview_read_model_for_browser(&read_model) {
            Ok(policy_preview) => policy_preview,
            Err(_) => return,
        };
        let report = crate::browser_runtime_stream_payload::stream_browser_runtime_event_chain_for_read_model_with_policy_preview(
            &read_model,
            Some(&policy_preview),
        )
        .await;
        let payload = browser_runtime_event_chain_stream_payload(&report);
        let entries = stream_entries(&payload);
        let serialized = crate::json_contract::serialize_json_value(serde_json::json!({
            "entries": entries.len(),
        }));
        let encoded = serialize_test_json(&serialized);
        let _: serde_json::Value =
            require_json_decode(&encoded, constants::error::AGENT_EVENT_SERIALIZES);
        let _: () = require_ok(
            Ok::<(), std::io::Error>(()),
            constants::error::AGENT_EVENT_SERIALIZES,
        );
        let field = LogFieldValue::String(encoded);
        let _ = require_log_string_field(Some(&field), constants::error::AGENT_EVENT_SERIALIZES);
        assert_eq!(entries.len(), 6);
    }
}

#[tokio::test]
async fn browser_runtime_stream_smoke_uses_request_and_json_helpers() -> TestResult {
    let read_model = read_model(vec![managed_row()]);
    let policy_preview = policy_preview_read_model_for_browser(&read_model)?;
    let report = crate::browser_runtime_stream_payload::stream_browser_runtime_event_chain_for_read_model_with_policy_preview(
        &read_model,
        Some(&policy_preview),
    )
    .await;
    let payload = browser_runtime_event_chain_stream_payload(&report);
    let entries = stream_entries(&payload);
    let request_report =
        crate::browser_runtime_stream_request::request_browser_runtime_service_stream_report(
            read_model,
            Some(policy_preview),
        )
        .await
        .map_err(|error| IoError::other(format!("{error:?}")))?;
    let serialized = crate::json_contract::serialize_json_value(serde_json::json!({
        "entries": entries.len(),
        "streamed_events": request_report.streamed_events,
    }));
    let _: serde_json::Value = crate::test_invariants::require_json_decode(
        &crate::test_invariants::serialize_test_json(&serialized),
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    assert_eq!(request_report, report);
    assert_eq!(serialized["entries"], serde_json::json!(entries.len()));
    assert_eq!(
        serialized["streamed_events"],
        serde_json::json!(request_report.streamed_events)
    );
    Ok(())
}
