use std::string::String as TestString;
use std::primitive::str as TestStr;
use ocentra_parent_agent_protocol::browser::BrowserRuntimePhase;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use serde_json::Value;

use crate::browser_runtime_stream_payload::BrowserRuntimeServiceStreamReport;
use crate::test_invariants::{require_json_decode, require_some};

const BROWSER_ACTION_INTENT_EXECUTION_FIELDS: [&TestStr; 4] = [
    constants::field::BROWSER_RUNTIME_ACTION_INTENT_DISPATCH_ATTEMPTS,
    constants::field::BROWSER_RUNTIME_ACTION_INTENT_ADAPTER_EXECUTIONS,
    constants::field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_INTERVENTION_EXECUTIONS,
    constants::field::BROWSER_RUNTIME_ACTION_INTENT_ENFORCEMENT_EXECUTIONS,
];

pub(super) fn assert_action_intent_execution_payload_zero(payload: &LogFields) {
    for field in BROWSER_ACTION_INTENT_EXECUTION_FIELDS {
        assert_eq!(payload.get(field), Some(&LogFieldValue::Number(0.0)));
    }
}

pub(super) fn assert_action_intent_handoff_report_ready(
    report: &BrowserRuntimeServiceStreamReport,
    payload: &LogFields,
) {
    assert_eq!(report.action_intent_handoff_candidates, 1);
    assert_eq!(
        report.action_intent_handoff_outbox_refs,
        vec![constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_OUTBOX_REF.to_string()]
    );
    assert_eq!(
        report.action_intent_handoff_refs,
        vec![constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REF.to_string()]
    );
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_CANDIDATES),
        Some(&LogFieldValue::Number(1.0))
    );
}

pub(super) fn assert_action_intent_handoff_payload_refs(payload: &LogFields) {
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_OUTBOX_REFS),
        Some(&LogFieldValue::String(serialize_test_json(&vec![
            constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_OUTBOX_REF
        ])))
    );
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REFS),
        Some(&LogFieldValue::String(serialize_test_json(&vec![
            constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REF
        ])))
    );
}

pub(super) fn assert_child_status_report_refs(
    report: &BrowserRuntimeServiceStreamReport,
    payload: &LogFields,
) {
    let child_command_refs = payload_string_refs(
        payload,
        constants::field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_COMMAND_REFS,
    );
    let child_accepted_event_refs = payload_string_refs(
        payload,
        constants::field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_ACCEPTED_EVENT_REFS,
    );
    let parent_read_model_refs = payload_string_refs(
        payload,
        constants::field::BROWSER_RUNTIME_ACTION_INTENT_PARENT_READ_MODEL_REFS,
    );

    assert_eq!(report.action_intent_child_accepted_rows, 1);
    assert_eq!(report.action_intent_child_command_refs, child_command_refs);
    assert_eq!(
        report.action_intent_child_accepted_event_refs,
        child_accepted_event_refs
    );
    assert_eq!(
        report.action_intent_parent_read_model_refs,
        parent_read_model_refs
    );
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_ACCEPTED_ROWS),
        Some(&LogFieldValue::Number(1.0))
    );
}

pub(super) fn assert_store_backed_stream_payload_header(event: &AgentEventEnvelope) {
    assert_eq!(
        event.event,
        AgentEventName::AgentBrowserRuntimeEventChainStreamReported
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::BROWSER_RUNTIME_STREAMED_EVENTS),
        Some(&LogFieldValue::Number(
            (BrowserRuntimePhase::ordered_chain().len() - 4) as f64
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_CANDIDATES),
        Some(&LogFieldValue::Number(1.0))
    );
}

pub(super) fn assert_store_backed_stream_first_entry(entries: &[Value]) {
    let last_entry = require_some(entries.last(), constants::error::AGENT_EVENT_SERIALIZES);
    assert_eq!(
        last_entry[constants::field::EVENT_TYPE],
        constants::browser::EVENT_BROWSER_READ_MODEL_PROJECTED
    );
    assert_eq!(
        entries[0][constants::field::PAYLOAD][constants::field::CAPABILITY_STATUS],
        constants::browser::CAPABILITY_STATUS_TAB_LIST_ONLY
    );
    assert_eq!(
        entries[0][constants::field::PAYLOAD][constants::field::QUERY_VISIBILITY],
        constants::browser::QUERY_VISIBILITY_LIVE_LOCAL
    );
    assert_eq!(
        entries[0][constants::field::PAYLOAD][constants::field::DEGRADED_REASON],
        constants::value::BROWSER_BRIDGE_NO_PAGE_TARGETS
    );
}

pub(super) fn assert_store_backed_stream_child_status_and_no_execution(payload: &LogFields) {
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_ACCEPTED_ROWS),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_DISPATCH_ATTEMPTS),
        Some(&LogFieldValue::Number(0.0))
    );
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_INTERVENTION_EXECUTIONS),
        Some(&LogFieldValue::Number(0.0))
    );
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_ENFORCEMENT_EXECUTIONS),
        Some(&LogFieldValue::Number(0.0))
    );
}

fn serialize_test_json<T>(value: &T) -> TestString
where
    T: serde::Serialize + ?Sized,
{
    crate::test_invariants::serialize_test_json(value)
}

fn payload_string_refs(payload: &LogFields, field_name: &TestStr) -> Vec<TestString> {
    let encoded_refs = require_some(
        payload.get(field_name).and_then(|value| match value {
            LogFieldValue::String(value) => Some(value.as_str()),
            _ => None,
        }),
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    require_json_decode(encoded_refs, constants::error::AGENT_EVENT_SERIALIZES)
}

