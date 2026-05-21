use super::{
    constants, BrowserBridgeKind, BrowserCapabilityStatus, BrowserChannel, BrowserCustodyLabel,
    BrowserFamily, BrowserManagedSessionStatus, BrowserManagedState, BrowserQueryVisibilityLabel,
    BROWSER_EVIDENCE_SCHEMA_VERSION,
};

#[test]
fn browser_managed_status_serializes_to_contract_shape() {
    let status = BrowserManagedSessionStatus {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        checked_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        managed_browser_session_id: Some(constants::browser::SESSION_ID_DEV.to_string()),
        browser_family: Some(BrowserFamily::UnknownChromium),
        browser_channel: Some(BrowserChannel::Unknown),
        browser_version: Some(constants::browser::DEVTOOLS_TEST_BROWSER_VERSION.to_string()),
        profile_id: Some(constants::browser::PROFILE_ID_DEV.to_string()),
        profile_path_ref: Some(constants::browser::PROFILE_PATH_REF_MANAGED.to_string()),
        process_id: Some(constants::browser::PROCESS_ID_UNKNOWN),
        bridge_kind: Some(BrowserBridgeKind::ChromiumDevtoolsProtocol),
        bridge_endpoint_ref: Some(
            constants::browser::BRIDGE_ENDPOINT_REF_LOOPBACK_DEVTOOLS.to_string(),
        ),
        managed_state: BrowserManagedState::BridgeConnected,
        capability_status: BrowserCapabilityStatus::TabListOnly,
        degraded_reason: None,
        started_at: None,
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
    };

    let serialized = serde_json::to_value(status).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized[constants::field::MANAGED_STATE],
        constants::browser::MANAGED_STATE_BRIDGE_CONNECTED
    );
    assert_eq!(
        serialized[constants::field::BRIDGE_KIND],
        constants::browser::BRIDGE_KIND_CHROMIUM_DEVTOOLS_PROTOCOL
    );
    assert_eq!(
        serialized[constants::field::BRIDGE_ENDPOINT_REF],
        constants::browser::BRIDGE_ENDPOINT_REF_LOOPBACK_DEVTOOLS
    );
}
