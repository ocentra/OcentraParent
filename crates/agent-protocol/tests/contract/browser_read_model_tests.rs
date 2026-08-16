use super::{
    constants, BrowserActiveProofSource, BrowserActiveTabState, BrowserCapabilityStatus,
    BrowserChannel, BrowserCustodyLabel, BrowserEvidenceReadModel, BrowserFamily,
    BrowserQueryVisibilityLabel, BrowserTabEvidence, BROWSER_EVIDENCE_SCHEMA_VERSION,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn browser_evidence_read_model_serializes_tab_list_only_rows() {
    let read_model = BrowserEvidenceReadModel {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        returned: 1,
        latest_event_id: Some(constants::browser::EVENT_ID_PREFIX.to_string()),
        latest_observed_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
        capability_status: Some(BrowserCapabilityStatus::TabListOnly),
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
        rows: vec![browser_tab_evidence()],
    };

    let serialized =
        serde_json::to_value(read_model).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], BROWSER_EVIDENCE_SCHEMA_VERSION);
    assert_eq!(serialized["returned"], 1);
    assert_eq!(
        serialized["rows"][0]["activeState"],
        constants::browser::ACTIVE_STATE_UNKNOWN
    );
    assert_eq!(
        serialized["rows"][0]["activeProofSource"],
        constants::browser::ACTIVE_PROOF_SOURCE_TARGET_LIST_ONLY
    );
    assert_eq!(
        serialized["rows"][0]["capabilityStatus"],
        constants::browser::CAPABILITY_STATUS_TAB_LIST_ONLY
    );
}

fn browser_tab_evidence() -> BrowserTabEvidence {
    BrowserTabEvidence {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        browser_evidence_id: constants::browser::EVIDENCE_ID_PREFIX.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        fresh_until: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        source_id: constants::browser::SOURCE_ID_MANAGED_CHROMIUM_DEVTOOLS.to_string(),
        adapter_id: constants::browser::ADAPTER_ID_MANAGED_CHROMIUM_DEVTOOLS.to_string(),
        device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
        browser_family: BrowserFamily::Edge,
        browser_channel: BrowserChannel::Stable,
        managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
        profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
        process_id: constants::browser::PROCESS_ID_UNKNOWN,
        window_id: None,
        tab_id: None,
        target_id: Some(constants::browser::DEVTOOLS_TEST_TARGET_ID.to_string()),
        active_state: BrowserActiveTabState::Unknown,
        active_proof_source: BrowserActiveProofSource::TargetListOnly,
        url: constants::activity_store::TEST_BROWSER_URL.to_string(),
        origin: constants::activity_store::TEST_BROWSER_ORIGIN.to_string(),
        domain: constants::activity_store::TEST_BROWSER_DOMAIN.to_string(),
        title: Some(constants::activity_store::TEST_BROWSER_TITLE.to_string()),
        capability_status: BrowserCapabilityStatus::TabListOnly,
        degraded_reason: None,
        stale_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
    }
}
