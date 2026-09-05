#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../../src/browser_runtime_delivery.rs"]
mod browser_runtime_delivery;
#[path = "../../src/browser_runtime_stream_api/child_status.rs"]
mod browser_runtime_stream_api;
#[path = "../../src/browser_runtime_stream_events.rs"]
mod browser_runtime_stream_events;
#[path = "../../src/browser_runtime_stream_payload.rs"]
mod browser_runtime_stream_payload;
#[path = "../../src/browser_runtime_stream_request.rs"]
mod browser_runtime_stream_request;
#[path = "browser_runtime_stream_tests.rs"]
mod browser_runtime_stream_tests;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../../src/json_contract.rs"]
mod json_contract;

use crate::browser_runtime_stream_payload::browser_runtime_event_chain_stream_payload;
use browser_runtime_stream_tests::{
    managed_row, policy_preview_read_model_for_browser, read_model, stream_entries, TestResult,
};
use std::io::Error as IoError;

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
    assert_eq!(request_report, report);
    assert_eq!(serialized["entries"], serde_json::json!(entries.len()));
    assert_eq!(
        serialized["streamed_events"],
        serde_json::json!(request_report.streamed_events)
    );
    Ok(())
}
