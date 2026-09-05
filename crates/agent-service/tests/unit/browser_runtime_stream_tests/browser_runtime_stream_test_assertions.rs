use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use std::primitive::str as TestStr;
use std::string::String as TestString;

use crate::browser_runtime_stream_payload::BrowserRuntimeServiceStreamReport;

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
    action_intent_id: &TestStr,
) {
    assert_eq!(report.action_intent_handoff_candidates, 1);
    assert_eq!(
        report.action_intent_handoff_outbox_refs,
        vec![expected_action_intent_ref(
            constants::browser::ACTION_INTENT_OUTBOX_REF_PREFIX,
            action_intent_id,
        )]
    );
    assert_eq!(
        report.action_intent_handoff_refs,
        vec![expected_action_intent_ref(
            constants::browser::ACTION_INTENT_HANDOFF_REF_PREFIX,
            action_intent_id,
        )]
    );
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_CANDIDATES),
        Some(&LogFieldValue::Number(1.0))
    );
}

pub(super) fn assert_action_intent_handoff_payload_refs(
    payload: &LogFields,
    action_intent_id: &TestStr,
) {
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_OUTBOX_REFS),
        Some(&LogFieldValue::String(serialize_test_json(&vec![
            expected_action_intent_ref(
                constants::browser::ACTION_INTENT_OUTBOX_REF_PREFIX,
                action_intent_id,
            )
        ])))
    );
    assert_eq!(
        payload.get(constants::field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REFS),
        Some(&LogFieldValue::String(serialize_test_json(&vec![
            expected_action_intent_ref(
                constants::browser::ACTION_INTENT_HANDOFF_REF_PREFIX,
                action_intent_id,
            )
        ])))
    );
}

fn serialize_test_json<T>(value: &T) -> TestString
where
    T: serde::Serialize + ?Sized,
{
    super::serialize_test_json(value)
}

fn expected_action_intent_ref(prefix: &TestStr, action_intent_id: &TestStr) -> TestString {
    let suffix = action_intent_id
        .strip_prefix(constants::browser::ACTION_INTENT_ID_PREFIX)
        .unwrap_or(action_intent_id);
    let mut value = TestString::from(prefix);
    value.push_str(suffix);
    value
}
