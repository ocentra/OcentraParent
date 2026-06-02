use ocentra_parent_agent_protocol::{
    constants, BrowserBridgeKind, BrowserCapabilityStatus, BrowserChannel, BrowserCustodyLabel,
    BrowserFamily, BrowserManagedSessionStatus, BrowserManagedState, BrowserQueryVisibilityLabel,
    BROWSER_EVIDENCE_SCHEMA_VERSION,
};

pub fn missing_browser_status(checked_at: String) -> BrowserManagedSessionStatus {
    BrowserManagedSessionStatus {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        checked_at,
        managed_browser_session_id: None,
        browser_family: None,
        browser_channel: None,
        browser_version: None,
        profile_id: None,
        profile_path_ref: None,
        process_id: None,
        bridge_kind: None,
        bridge_endpoint_ref: None,
        managed_state: BrowserManagedState::NotInstalled,
        capability_status: BrowserCapabilityStatus::ManagedProfileMissing,
        degraded_reason: Some(constants::value::MANAGED_BROWSER_EXECUTABLE_MISSING.to_string()),
        started_at: None,
        custody_label: BrowserCustodyLabel::Unavailable,
        query_visibility: BrowserQueryVisibilityLabel::Unavailable,
    }
}

pub fn profile_missing_status(checked_at: String) -> BrowserManagedSessionStatus {
    let mut status = missing_browser_status(checked_at);
    status.managed_state = BrowserManagedState::InstalledSupported;
    status.degraded_reason =
        Some(constants::value::MANAGED_BROWSER_PROFILE_DIR_MISSING.to_string());
    status
}

pub fn unmanaged_browser_status(
    checked_at: String,
    process_id: u32,
    browser_family: BrowserFamily,
    browser_channel: BrowserChannel,
) -> BrowserManagedSessionStatus {
    let mut status = missing_browser_status(checked_at);
    status.process_id = Some(process_id);
    status.browser_family = Some(browser_family);
    status.browser_channel = Some(browser_channel);
    status.managed_state = BrowserManagedState::InstalledSupported;
    status.capability_status = BrowserCapabilityStatus::UnmanagedBrowser;
    status.degraded_reason = Some(constants::value::MANAGED_BROWSER_UNMANAGED_PROCESS.to_string());
    status.custody_label = BrowserCustodyLabel::ChildDeviceLocal;
    status
}

pub fn managed_profile_ready_status(
    checked_at: String,
    browser_family: BrowserFamily,
    browser_channel: BrowserChannel,
) -> BrowserManagedSessionStatus {
    let mut status = base_managed_status(checked_at);
    status.browser_family = Some(browser_family);
    status.browser_channel = Some(browser_channel);
    status
}

pub fn bridge_disconnected_status(
    checked_at: String,
    reason: &'static str,
) -> BrowserManagedSessionStatus {
    let mut status = base_managed_status(checked_at);
    status.managed_state = BrowserManagedState::BridgeDisconnected;
    status.capability_status = BrowserCapabilityStatus::BridgeMissing;
    status.degraded_reason = Some(reason.to_string());
    status
}

pub fn status_with_error(checked_at: String, reason: &'static str) -> BrowserManagedSessionStatus {
    let mut status = base_managed_status(checked_at);
    status.managed_state = BrowserManagedState::Error;
    status.capability_status = BrowserCapabilityStatus::AdapterError;
    status.degraded_reason = Some(reason.to_string());
    status
}

pub fn connected_status(
    checked_at: String,
    browser_version: Option<String>,
    capability_status: BrowserCapabilityStatus,
    degraded_reason: Option<String>,
) -> BrowserManagedSessionStatus {
    let mut status = base_managed_status(checked_at);
    status.browser_version = browser_version;
    status.managed_state = BrowserManagedState::BridgeConnected;
    status.capability_status = capability_status;
    status.degraded_reason = degraded_reason;
    status
}

fn base_managed_status(checked_at: String) -> BrowserManagedSessionStatus {
    BrowserManagedSessionStatus {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        checked_at,
        managed_browser_session_id: Some(constants::browser::SESSION_ID_DEV.to_string()),
        browser_family: Some(BrowserFamily::UnknownChromium),
        browser_channel: Some(BrowserChannel::Unknown),
        browser_version: None,
        profile_id: Some(constants::browser::PROFILE_ID_DEV.to_string()),
        profile_path_ref: Some(constants::browser::PROFILE_PATH_REF_MANAGED.to_string()),
        process_id: None,
        bridge_kind: Some(BrowserBridgeKind::ChromiumDevtoolsProtocol),
        bridge_endpoint_ref: Some(
            constants::browser::BRIDGE_ENDPOINT_REF_LOOPBACK_DEVTOOLS.to_string(),
        ),
        managed_state: BrowserManagedState::ManagedProfileReady,
        capability_status: BrowserCapabilityStatus::BridgeMissing,
        degraded_reason: None,
        started_at: None,
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
    }
}
