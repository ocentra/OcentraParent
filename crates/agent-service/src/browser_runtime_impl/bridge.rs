use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use ocentra_parent_agent_core::browser_bridge_poll::{
    poll_chromium_bridge, BrowserBridgeExpectedCustody, BrowserBridgePollConfig,
    BrowserBridgePollSnapshot,
};
use ocentra_parent_agent_protocol::browser::BrowserCapabilityStatus;
use ocentra_parent_agent_protocol::browser::BrowserChannel;
use ocentra_parent_agent_protocol::browser::BrowserFamily;
use ocentra_parent_agent_protocol::browser_managed::BrowserManagedSessionStatus;
use ocentra_parent_agent_protocol::constants;

use crate::{
    activity_capture::record_activity_events_to_paths,
    activity_store_path::{activity_db_path, activity_journal_key_path, activity_journal_path},
    browser_runtime_status::{bridge_disconnected_status, connected_status},
};

use super::BrowserRuntimeText;

struct BrowserRuntimeVersion(Option<String>);

pub(super) fn bridge_poll_status(
    checked_at: BrowserRuntimeText,
    port: u16,
) -> BrowserManagedSessionStatus {
    let checked_at = checked_at.0;
    let config = BrowserBridgePollConfig {
        endpoint: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port),
        managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
        profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
        process_id: constants::browser::PROCESS_ID_UNKNOWN,
        browser_family: BrowserFamily::UnknownChromium,
        browser_channel: BrowserChannel::Unknown,
        expected_custody: BrowserBridgeExpectedCustody {
            bridge_port: port,
            managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
            profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
            process_id: constants::browser::PROCESS_ID_UNKNOWN,
            browser_family: BrowserFamily::UnknownChromium,
            browser_channel: BrowserChannel::Unknown,
            session_fresh_until: checked_at.clone(),
        },
    };

    match poll_chromium_bridge(&config, &checked_at, &checked_at) {
        Ok(snapshot) => connected_bridge_status(
            BrowserRuntimeText(checked_at),
            BrowserRuntimeVersion(snapshot.browser_version.clone()),
            &snapshot,
        ),
        Err(error) => bridge_disconnected_status(checked_at, error.reason()),
    }
}

fn connected_bridge_status(
    checked_at: BrowserRuntimeText,
    browser_version: BrowserRuntimeVersion,
    snapshot: &BrowserBridgePollSnapshot,
) -> BrowserManagedSessionStatus {
    let checked_at = checked_at.0;
    if let Err(error) = record_activity_events_to_paths(
        activity_journal_path().as_ref(),
        activity_journal_key_path().as_ref(),
        activity_db_path().as_ref(),
        &snapshot.events,
    ) {
        return connected_status(
            checked_at,
            browser_version.0,
            BrowserCapabilityStatus::AdapterError,
            Some(error.reason().to_string()),
        );
    }

    connected_status(
        checked_at,
        browser_version.0,
        BrowserCapabilityStatus::TabListOnly,
        browser_target_degraded_reason(snapshot.page_target_count).map(|reason| reason.0),
    )
}

fn browser_target_degraded_reason(page_target_count: usize) -> Option<BrowserRuntimeText> {
    if page_target_count == 0 {
        return Some(BrowserRuntimeText(
            constants::value::BROWSER_BRIDGE_NO_PAGE_TARGETS.to_string(),
        ));
    }
    None
}
