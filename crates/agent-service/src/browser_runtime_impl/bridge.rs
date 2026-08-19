use ocentra_parent_agent_core::browser_bridge_poll::{
    BrowserBridgePollError, BrowserBridgePollSnapshot,
};
use ocentra_parent_agent_core::browser_managed_session::BrowserManagedLaunch;
use ocentra_parent_agent_protocol::browser::BrowserCapabilityStatus;
use ocentra_parent_agent_protocol::browser_managed::{
    BrowserManagedProfileStoreEntry, BrowserManagedSessionStatus,
};
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
    launch: &BrowserManagedLaunch,
    profile_store_entry: &BrowserManagedProfileStoreEntry,
    started_at: BrowserRuntimeText,
) -> BridgePollResult {
    let checked_at = checked_at.0;
    match launch.poll_bridge(checked_at.clone()) {
        Ok(snapshot) => connected_bridge_status(
            BrowserRuntimeText(checked_at),
            BrowserRuntimeVersion(snapshot.browser_version.clone()),
            &snapshot,
            launch,
            profile_store_entry,
            started_at,
        ),
        Err(error) => BridgePollResult {
            status: bridge_disconnected_status(
                checked_at,
                error.reason(),
                started_at,
                launch,
                profile_store_entry,
            ),
            retire: matches!(
                error,
                BrowserBridgePollError::UntrustedProcess
                    | BrowserBridgePollError::StaleSession
                    | BrowserBridgePollError::UntrustedProfile
                    | BrowserBridgePollError::UntrustedSession
                    | BrowserBridgePollError::UntrustedBrowserIdentity
            ),
        },
    }
}

pub(super) struct BridgePollResult {
    pub(super) status: BrowserManagedSessionStatus,
    pub(super) retire: bool,
}

fn connected_bridge_status(
    checked_at: BrowserRuntimeText,
    browser_version: BrowserRuntimeVersion,
    snapshot: &BrowserBridgePollSnapshot,
    launch: &BrowserManagedLaunch,
    profile_store_entry: &BrowserManagedProfileStoreEntry,
    started_at: BrowserRuntimeText,
) -> BridgePollResult {
    let checked_at = checked_at.0;
    if let Err(error) = record_activity_events_to_paths(
        activity_journal_path().as_ref(),
        activity_journal_key_path().as_ref(),
        activity_db_path().as_ref(),
        &snapshot.events,
    ) {
        return BridgePollResult {
            status: connected_status(
                checked_at,
                browser_version.0,
                BrowserCapabilityStatus::AdapterError,
                Some(error.reason().to_string()),
                started_at,
                launch,
                profile_store_entry,
            ),
            retire: false,
        };
    }

    BridgePollResult {
        status: connected_status(
            checked_at,
            browser_version.0,
            BrowserCapabilityStatus::TabListOnly,
            browser_target_degraded_reason(snapshot.page_target_count).map(|reason| reason.0),
            started_at,
            launch,
            profile_store_entry,
        ),
        retire: false,
    }
}

fn browser_target_degraded_reason(page_target_count: usize) -> Option<BrowserRuntimeText> {
    if page_target_count == 0 {
        return Some(BrowserRuntimeText(
            constants::value::BROWSER_BRIDGE_NO_PAGE_TARGETS.to_string(),
        ));
    }
    None
}
