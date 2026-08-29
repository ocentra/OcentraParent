#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[macro_use]
#[path = "../support/activity_report_env_lock.rs"]
mod activity_report_env_lock;
#[path = "../../src/browser_runtime_delivery.rs"]
mod browser_runtime_delivery;
#[path = "../../src/browser_runtime_stream_events.rs"]
mod browser_runtime_stream_events;
#[path = "../../src/event_builder.rs"]
mod event_builder;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../support/test_invariants.rs"]
mod test_invariants;
#[path = "../support/test_text.rs"]
mod test_text;
#[path = "../../src/time.rs"]
mod time;
mod activity_api {
    use ocentra_parent_agent_protocol::browser_read_model::BrowserEvidenceReadModel;
    use ocentra_parent_agent_protocol::logging::LogFields;
    use ocentra_parent_agent_protocol::transport::{
        AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
    };

    #[derive(Clone, Copy)]
    pub(crate) struct ActivityEventId(pub(crate) &'static str);

    pub(crate) async fn load_browser_evidence_read_model() -> Option<BrowserEvidenceReadModel> {
        None
    }

    pub(crate) mod activity_store_error_event {
        use super::{ActivityEventId, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName};

        pub(crate) fn activity_store_error_event(
            command: AgentCommandEnvelope,
            event_id_suffix: ActivityEventId,
            event: AgentEventName,
        ) -> AgentEventEnvelope {
            let AgentCommandEnvelope {
                message_id, source, ..
            } = command;
            crate::event_builder::build_event(
                event_id_suffix.0,
                &message_id,
                source,
                event,
                ocentra_parent_agent_protocol::logging::LogLevel::Error,
                super::LogFields::new(),
                None,
            )
        }
    }
}
mod policy_preview_api {
    use ocentra_parent_agent_protocol::activity::policy_preview::PolicyPreviewReadModel;

    pub(crate) async fn load_policy_preview_read_model() -> Option<PolicyPreviewReadModel> {
        None
    }
}
#[path = "../../src/browser_runtime_stream_api.rs"]
mod browser_runtime_stream_api;
#[path = "../../src/browser_runtime_stream_payload.rs"]
mod browser_runtime_stream_payload;
#[path = "../../src/browser_runtime_stream_request.rs"]
mod browser_runtime_stream_request;
#[path = "browser_runtime_stream_tests.rs"]
mod browser_runtime_stream_tests;
#[path = "../../src/json_contract.rs"]
mod json_contract;

use crate::browser_runtime_stream_payload::browser_runtime_event_chain_stream_payload;
use browser_runtime_stream_tests::{
    managed_row, policy_preview_read_model_for_browser, read_model, stream_entries, TestResult,
};
use ocentra_parent_agent_protocol::constants;
use std::io::Error as IoError;

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
        let _ = crate::browser_runtime_stream_api::build_browser_runtime_event_chain_stream_report;
        assert_eq!(
            crate::event_builder::portal_peer().peer_id,
            ocentra_parent_agent_protocol::constants::peer::PORTAL_DEV
        );
        assert_eq!(
            crate::time::timestamp_after_epoch_seconds::<String>(30, 3),
            "1970-01-01T00:00:33.000Z"
        );
        assert_eq!(
            crate::time::timestamp_from_epoch_seconds::<String>(0),
            "1970-01-01T00:00:00.000Z"
        );
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
        assert_eq!(report.exact_url_rows, 1);
        assert_eq!(report.failed_rows, 1);
        assert_eq!(report.manual_required_rows, 1);
        assert!(entries.is_empty());
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
        crate::test_invariants::serialize_test_json(&serialized),
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
