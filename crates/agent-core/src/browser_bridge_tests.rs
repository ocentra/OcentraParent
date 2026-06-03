use ocentra_parent_agent_protocol::{
    constants, ActivityEventKind, ActivityObserver, ActivitySubjectKind, BrowserActiveProofSource,
    BrowserActiveTabState, BrowserCapabilityStatus, BrowserChannel, BrowserCustodyLabel,
    BrowserFamily, BrowserQueryVisibilityLabel, LogFieldValue,
};

use crate::{
    browser_tab_observation_event, BrowserBridgeEventError, BrowserBridgeTargetObservation,
};

#[test]
fn browser_target_observation_maps_to_managed_url_activity_event() {
    let event = mapped_event(browser_observation());

    assert_eq!(event.kind, ActivityEventKind::UrlObserved);
    assert_eq!(
        event.source.observer,
        ActivityObserver::ManagedBrowserBridge
    );
    assert_eq!(event.subject.kind, ActivitySubjectKind::Url);
    assert_eq!(
        event.fields.get(constants::field::URL),
        Some(&LogFieldValue::String(
            constants::activity_store::TEST_BROWSER_URL.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::DOMAIN),
        Some(&LogFieldValue::String(
            constants::activity_store::TEST_BROWSER_DOMAIN.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::ACTIVE_STATE),
        Some(&LogFieldValue::String(
            constants::browser::ACTIVE_STATE_UNKNOWN.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::CAPABILITY_STATUS),
        Some(&LogFieldValue::String(
            constants::browser::CAPABILITY_STATUS_TAB_LIST_ONLY.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::QUERY_VISIBILITY),
        Some(&LogFieldValue::String(
            constants::browser::QUERY_VISIBILITY_LIVE_LOCAL.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::FRESH_UNTIL),
        Some(&LogFieldValue::String(
            constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::STALE_AT),
        Some(&LogFieldValue::String(
            constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()
        ))
    );
}

#[test]
fn browser_target_observation_maps_stable_identity_and_custody_fields() {
    let event = mapped_event(browser_observation());

    assert_eq!(
        event.fields.get(constants::field::BROWSER_EVIDENCE_ID),
        Some(&LogFieldValue::String(expected_bridge_id(
            constants::browser::EVIDENCE_ID_PREFIX
        )))
    );
    assert_eq!(
        event.fields.get(constants::field::SOURCE_ID),
        Some(&LogFieldValue::String(
            constants::browser::SOURCE_ID_MANAGED_CHROMIUM_DEVTOOLS.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::ADAPTER_ID),
        Some(&LogFieldValue::String(
            constants::browser::ADAPTER_ID_MANAGED_CHROMIUM_DEVTOOLS.to_string()
        ))
    );
    assert_eq!(
        event
            .fields
            .get(constants::field::MANAGED_BROWSER_SESSION_ID),
        Some(&LogFieldValue::String(
            constants::browser::SESSION_ID_DEV.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::PROFILE_ID),
        Some(&LogFieldValue::String(
            constants::browser::PROFILE_ID_DEV.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::PROCESS_ID),
        Some(&LogFieldValue::Number(f64::from(
            constants::activity_store::TEST_BROWSER_PROCESS_ID
        )))
    );
    assert_eq!(
        event.fields.get(constants::field::TARGET_ID),
        Some(&LogFieldValue::String(
            constants::activity_store::TEST_BROWSER_TARGET_ID.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::TAB_ID),
        Some(&LogFieldValue::String(
            constants::activity_store::TEST_BROWSER_TAB_ID.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::ACTIVE_PROOF_SOURCE),
        Some(&LogFieldValue::String(
            constants::browser::ACTIVE_PROOF_SOURCE_TARGET_LIST_ONLY.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::CUSTODY_LABEL),
        Some(&LogFieldValue::String(
            constants::browser::CUSTODY_CHILD_DEVICE_LOCAL.to_string()
        ))
    );
}

#[test]
fn browser_target_observation_strips_credentials_from_origin() {
    let mut observation = browser_observation();
    observation.url = constants::activity_store::TEST_BROWSER_CREDENTIAL_URL.to_string();
    let event = mapped_event(observation);

    assert_eq!(
        event.fields.get(constants::field::URL),
        Some(&LogFieldValue::String(
            constants::activity_store::TEST_BROWSER_URL.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::ORIGIN),
        Some(&LogFieldValue::String(
            constants::activity_store::TEST_BROWSER_ORIGIN.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::DOMAIN),
        Some(&LogFieldValue::String(
            constants::activity_store::TEST_BROWSER_DOMAIN.to_string()
        ))
    );
}

#[test]
fn browser_target_observation_derives_tab_id_from_target_id() {
    let mut observation = browser_observation();
    observation.tab_id = None;
    let event = mapped_event(observation);

    assert_eq!(
        event.fields.get(constants::field::TAB_ID),
        Some(&LogFieldValue::String(
            constants::activity_store::TEST_BROWSER_TAB_ID_FROM_TARGET.to_string()
        ))
    );
}

#[test]
fn browser_target_observation_normalizes_case_port_and_credentials() {
    let mut observation = browser_observation();
    observation.url = constants::activity_store::TEST_BROWSER_URL_WITH_CREDENTIALS.to_string();
    observation.tab_id = None;
    let event = mapped_event(observation);

    assert_eq!(
        event.fields.get(constants::field::URL),
        Some(&LogFieldValue::String(
            constants::activity_store::TEST_BROWSER_NORMALIZED_URL_WITH_PORT.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::ORIGIN),
        Some(&LogFieldValue::String(
            constants::activity_store::TEST_BROWSER_ORIGIN_WITH_PORT.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::DOMAIN),
        Some(&LogFieldValue::String(
            constants::activity_store::TEST_BROWSER_DOMAIN.to_string()
        ))
    );
}

#[test]
fn browser_target_observation_rejects_invalid_url_before_journal_write() {
    let mut observation = browser_observation();
    observation.browser_family = BrowserFamily::Chrome;
    observation.url = constants::activity_store::TEST_INVALID_BROWSER_URL.to_string();
    observation.tab_id = None;
    observation.title = None;
    observation.capability_status = BrowserCapabilityStatus::AdapterError;

    let error = browser_tab_observation_event(
        observation,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
        0,
    )
    .expect_err(constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL);

    assert_eq!(error, BrowserBridgeEventError::InvalidUrl);
}

#[test]
fn browser_target_observation_rejects_empty_target_id() {
    let mut observation = browser_observation();
    observation.target_id = constants::value::EMPTY.to_string();

    let error = browser_tab_observation_event(
        observation,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
        0,
    )
    .expect_err(constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL);

    assert_eq!(error, BrowserBridgeEventError::InvalidTargetId);
}

fn mapped_event(
    observation: BrowserBridgeTargetObservation,
) -> ocentra_parent_agent_protocol::ActivityEvent {
    browser_tab_observation_event(
        observation,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
        0,
    )
    .expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET)
}

fn browser_observation() -> BrowserBridgeTargetObservation {
    BrowserBridgeTargetObservation {
        browser_family: BrowserFamily::Edge,
        browser_channel: BrowserChannel::Stable,
        managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
        profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
        process_id: constants::activity_store::TEST_BROWSER_PROCESS_ID,
        target_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
        tab_id: Some(constants::activity_store::TEST_BROWSER_TAB_ID.to_string()),
        window_id: None,
        active_state: BrowserActiveTabState::Unknown,
        active_proof_source: BrowserActiveProofSource::TargetListOnly,
        url: constants::activity_store::TEST_BROWSER_URL.to_string(),
        title: Some(constants::activity_store::TEST_BROWSER_TITLE.to_string()),
        capability_status: BrowserCapabilityStatus::TabListOnly,
        degraded_reason: None,
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
    }
}

fn expected_bridge_id(prefix: &str) -> String {
    let mut value = String::from(prefix);
    value.push_str(&0.to_string());
    value.push(constants::delimiter::HYPHEN);
    value.push_str(constants::activity_store::TEST_BROWSER_TARGET_ID);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(constants::activity_store::TEST_FIRST_OBSERVED_AT);
    value
}
