#[macro_use]
#[path = "../support/unit_root_basic_harness.rs"]
mod unit_root_basic_harness;
declare_agent_service_unit_root_basic_harness!();
#[path = "../../src/browser_evidence_payload.rs"]
mod browser_evidence_payload;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../support/test_invariants/require_json_decode.rs"]
mod test_require_json_decode;
#[path = "../support/test_invariants/require_log_string_field.rs"]
mod test_require_log_string_field;
#[path = "../support/test_invariants/require_ok.rs"]
mod test_require_ok;
#[path = "../support/test_invariants/require_some.rs"]
mod test_require_some;
#[path = "../support/test_invariants/serialize_test_json.rs"]
mod test_serialize_json;

use ocentra_parent_agent_protocol::browser::{
    BrowserActiveProofSource, BrowserActiveTabState, BrowserCapabilityStatus, BrowserChannel,
    BrowserCustodyLabel, BrowserFamily,
};
use ocentra_parent_agent_protocol::browser_managed::BrowserQueryVisibilityLabel;
use ocentra_parent_agent_protocol::browser_read_model::{
    BrowserEvidenceReadModel, BrowserTabEvidence,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::BROWSER_EVIDENCE_SCHEMA_VERSION;

#[test]
fn browser_evidence_payload_uses_degraded_reason_field() {
    let payload = browser_evidence_payload::browser_evidence_read_model_payload(&read_model());

    assert_eq!(
        payload.get(constants::field::DEGRADED_REASON),
        Some(&LogFieldValue::String(
            constants::value::BROWSER_BRIDGE_NO_PAGE_TARGETS.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::ACTIVE_PROOF_SOURCE),
        Some(&LogFieldValue::String(
            constants::browser::ACTIVE_PROOF_SOURCE_TARGET_LIST_ONLY.to_string()
        ))
    );
    assert_eq!(payload.get(constants::field::REASON), None);
}

#[test]
fn browser_evidence_payload_validates_invariant_helpers_with_real_values() {
    let ok_value = crate::test_require_ok::require_ok(
        Ok::<_, &'static str>("invariant-ok"),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let some_value = crate::test_require_some::require_some(
        Some("invariant-some"),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let serialized = crate::test_serialize_json::serialize_test_json(&serde_json::json!({
        "ok": ok_value,
        "some": some_value,
    }));
    let decoded: serde_json::Value = crate::test_require_json_decode::require_json_decode(
        serialized,
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let log_field = LogFieldValue::String("invariant-field".to_string());
    let string_field = crate::test_require_log_string_field::require_log_string_field(
        Some(&log_field),
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    let timestamp_now: String = crate::time::timestamp_now();
    let timestamp_epoch: String = crate::time::timestamp_from_epoch_seconds(0);
    let timestamp_after_epoch: String = crate::time::timestamp_after_epoch_seconds(0, 1);

    assert_eq!(decoded["ok"], "invariant-ok");
    assert_eq!(decoded["some"], "invariant-some");
    assert_eq!(string_field, "invariant-field");
    let parsed_now = crate::test_require_ok::require_ok(
        chrono::DateTime::parse_from_rfc3339(&timestamp_now),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    assert_eq!(
        parsed_now.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        timestamp_now
    );
    assert_eq!(timestamp_epoch, "1970-01-01T00:00:00.000Z");
    assert_eq!(timestamp_after_epoch, "1970-01-01T00:00:01.000Z");
}

fn read_model() -> BrowserEvidenceReadModel {
    BrowserEvidenceReadModel {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        returned: 1,
        latest_event_id: Some(constants::event_id::HEALTH_REPORTED.to_string()),
        latest_observed_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
        capability_status: Some(BrowserCapabilityStatus::TabListOnly),
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
        rows: vec![BrowserTabEvidence {
            schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
            browser_evidence_id: constants::browser::EVIDENCE_ID_PREFIX.to_string(),
            observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
            fresh_until: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
            source_id: constants::browser::SOURCE_ID_MANAGED_CHROMIUM_DEVTOOLS.to_string(),
            adapter_id: constants::browser::ADAPTER_ID_MANAGED_CHROMIUM_DEVTOOLS.to_string(),
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            browser_family: BrowserFamily::Chrome,
            browser_channel: BrowserChannel::Stable,
            managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
            profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
            process_id: constants::activity_store::TEST_BROWSER_PROCESS_ID,
            window_id: None,
            tab_id: Some(constants::activity_store::TEST_BROWSER_TAB_ID_FROM_TARGET.to_string()),
            target_id: Some(constants::activity_store::TEST_BROWSER_TARGET_ID.to_string()),
            active_state: BrowserActiveTabState::Unknown,
            active_proof_source: BrowserActiveProofSource::TargetListOnly,
            url: constants::activity_store::TEST_BROWSER_URL.to_string(),
            origin: constants::activity_store::TEST_BROWSER_ORIGIN.to_string(),
            domain: constants::activity_store::TEST_BROWSER_DOMAIN.to_string(),
            title: Some(constants::activity_store::TEST_BROWSER_TITLE.to_string()),
            capability_status: BrowserCapabilityStatus::TabListOnly,
            degraded_reason: Some(constants::value::BROWSER_BRIDGE_NO_PAGE_TARGETS.to_string()),
            stale_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
            custody_label: BrowserCustodyLabel::ChildDeviceLocal,
            query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
        }],
    }
}
