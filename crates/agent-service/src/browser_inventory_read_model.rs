use ocentra_parent_agent_core::browser_windows_inventory::BrowserWindowsInventoryObservation;
use ocentra_parent_agent_protocol::{
    constants, BrowserActiveTabCapability, BrowserCapabilityStatus, BrowserChannel,
    BrowserCustodyLabel, BrowserExactUrlCapability, BrowserFamily, BrowserInventoryInstallState,
    BrowserInventoryReadModel, BrowserInventoryRow, BrowserInventoryRunningState,
    BrowserManagedProfileState, BrowserManagedSessionStatus, BrowserManagedState,
    BrowserManagementTier, BrowserQueryVisibilityLabel, BrowserSupportTier,
    BrowserUnmanagedFallbackCapability, BROWSER_EVIDENCE_SCHEMA_VERSION,
};

pub fn browser_inventory_read_model_from_status(
    status: &BrowserManagedSessionStatus,
) -> BrowserInventoryReadModel {
    let row = browser_inventory_row_from_status(status);
    BrowserInventoryReadModel {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        generated_at: status.checked_at.clone(),
        limit: 1,
        returned: 1,
        latest_observed_at: Some(status.checked_at.clone()),
        capability_status: Some(status.capability_status.clone()),
        custody_label: status.custody_label.clone(),
        query_visibility: status.query_visibility.clone(),
        rows: vec![row],
    }
}

pub fn browser_inventory_read_model_from_windows_inventory(
    scanned_at: String,
    observations: &[BrowserWindowsInventoryObservation],
) -> BrowserInventoryReadModel {
    let rows = observations
        .iter()
        .enumerate()
        .map(|(index, observation)| {
            browser_inventory_row_from_windows_observation(&scanned_at, index, observation)
        })
        .collect::<Vec<_>>();
    let returned = rows.len() as u64;

    BrowserInventoryReadModel {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        generated_at: scanned_at.clone(),
        limit: returned,
        returned,
        latest_observed_at: latest_observed_at(&scanned_at, returned),
        capability_status: None,
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
        rows,
    }
}

fn browser_inventory_row_from_status(status: &BrowserManagedSessionStatus) -> BrowserInventoryRow {
    let browser_family = status
        .browser_family
        .clone()
        .unwrap_or(BrowserFamily::Unknown);
    let browser_channel = status
        .browser_channel
        .clone()
        .unwrap_or(BrowserChannel::Unknown);
    let inventory_state = inventory_state_from_status(status);
    BrowserInventoryRow {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        inventory_row_id: inventory_row_id(&browser_family, &status.capability_status),
        scanned_at: status.checked_at.clone(),
        device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
        product_name: product_name(&browser_family),
        browser_family,
        browser_channel,
        browser_version: status.browser_version.clone(),
        install_state: inventory_state.install_state,
        running_state: inventory_state.running_state,
        management_tier: inventory_state.management_tier,
        support_tier: inventory_state.support_tier,
        exact_url_capability: inventory_state.exact_url_capability,
        active_tab_capability: inventory_state.active_tab_capability,
        managed_profile_state: inventory_state.managed_profile_state,
        unmanaged_fallback_capability: inventory_state.unmanaged_fallback_capability,
        executable_path_ref: executable_path_ref(status),
        publisher_signature_ref: status.unmanaged_signature_ref.clone(),
        file_hash_ref: status.unmanaged_process_hash_ref.clone(),
        profile_id: status.profile_id.clone(),
        process_id: status.process_id,
        capability_status: status.capability_status.clone(),
        reason_code: reason_code(status),
        custody_label: status.custody_label.clone(),
        query_visibility: status.query_visibility.clone(),
    }
}

fn browser_inventory_row_from_windows_observation(
    scanned_at: &str,
    row_index: usize,
    observation: &BrowserWindowsInventoryObservation,
) -> BrowserInventoryRow {
    BrowserInventoryRow {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        inventory_row_id: windows_inventory_row_id(observation, row_index),
        scanned_at: scanned_at.to_string(),
        device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
        product_name: observation.product_name.clone(),
        browser_family: observation.browser_family.clone(),
        browser_channel: observation.browser_channel.clone(),
        browser_version: None,
        install_state: observation.install_state.clone(),
        running_state: observation.running_state.clone(),
        management_tier: observation.management_tier.clone(),
        support_tier: observation.support_tier.clone(),
        exact_url_capability: observation.exact_url_capability.clone(),
        active_tab_capability: observation.active_tab_capability.clone(),
        managed_profile_state: observation.managed_profile_state.clone(),
        unmanaged_fallback_capability: observation.unmanaged_fallback_capability.clone(),
        executable_path_ref: observation.executable_path.as_ref().map(|_| {
            constants::browser::INVENTORY_EXECUTABLE_PATH_REF_WINDOWS_REDACTED.to_string()
        }),
        publisher_signature_ref: None,
        file_hash_ref: None,
        profile_id: None,
        process_id: observation.process_id,
        capability_status: observation.capability_status.clone(),
        reason_code: observation.reason_code.to_string(),
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
    }
}

struct BrowserInventoryDerivedState {
    install_state: BrowserInventoryInstallState,
    running_state: BrowserInventoryRunningState,
    management_tier: BrowserManagementTier,
    support_tier: BrowserSupportTier,
    exact_url_capability: BrowserExactUrlCapability,
    active_tab_capability: BrowserActiveTabCapability,
    managed_profile_state: BrowserManagedProfileState,
    unmanaged_fallback_capability: BrowserUnmanagedFallbackCapability,
}

fn inventory_state_from_status(
    status: &BrowserManagedSessionStatus,
) -> BrowserInventoryDerivedState {
    match status.capability_status {
        BrowserCapabilityStatus::UnmanagedBrowser => unmanaged_inventory_state(),
        BrowserCapabilityStatus::TabListOnly | BrowserCapabilityStatus::Available => {
            managed_target_list_inventory_state(status)
        }
        BrowserCapabilityStatus::ManagedProfileMissing => missing_inventory_state(),
        BrowserCapabilityStatus::BridgeMissing => managed_profile_missing_bridge_state(),
        BrowserCapabilityStatus::UnsupportedBrowser => unsupported_inventory_state(),
        BrowserCapabilityStatus::PermissionLimited
        | BrowserCapabilityStatus::Stale
        | BrowserCapabilityStatus::AdapterError
        | BrowserCapabilityStatus::DisabledByParent => degraded_inventory_state(),
    }
}

fn managed_target_list_inventory_state(
    status: &BrowserManagedSessionStatus,
) -> BrowserInventoryDerivedState {
    BrowserInventoryDerivedState {
        install_state: BrowserInventoryInstallState::Installed,
        running_state: match status.managed_state {
            BrowserManagedState::BridgeConnected | BrowserManagedState::RunningManaged => {
                BrowserInventoryRunningState::RunningManaged
            }
            _ => BrowserInventoryRunningState::NotRunning,
        },
        management_tier: BrowserManagementTier::Managed,
        support_tier: BrowserSupportTier::ManagedTargetList,
        exact_url_capability: BrowserExactUrlCapability::ManagedTargetListOnly,
        active_tab_capability: BrowserActiveTabCapability::TargetListOnly,
        managed_profile_state: BrowserManagedProfileState::Ready,
        unmanaged_fallback_capability: BrowserUnmanagedFallbackCapability::OsBlockManualRequired,
    }
}

fn unmanaged_inventory_state() -> BrowserInventoryDerivedState {
    BrowserInventoryDerivedState {
        install_state: BrowserInventoryInstallState::CandidateRunning,
        running_state: BrowserInventoryRunningState::RunningUnmanaged,
        management_tier: BrowserManagementTier::Unmanaged,
        support_tier: BrowserSupportTier::UnmanagedProcessOnly,
        exact_url_capability: BrowserExactUrlCapability::NotClaimed,
        active_tab_capability: BrowserActiveTabCapability::NotClaimed,
        managed_profile_state: BrowserManagedProfileState::NotApplicable,
        unmanaged_fallback_capability: BrowserUnmanagedFallbackCapability::ReportOnly,
    }
}

fn missing_inventory_state() -> BrowserInventoryDerivedState {
    BrowserInventoryDerivedState {
        install_state: BrowserInventoryInstallState::NotInstalled,
        running_state: BrowserInventoryRunningState::Unknown,
        management_tier: BrowserManagementTier::Unknown,
        support_tier: BrowserSupportTier::Unknown,
        exact_url_capability: BrowserExactUrlCapability::Unavailable,
        active_tab_capability: BrowserActiveTabCapability::Unavailable,
        managed_profile_state: BrowserManagedProfileState::Unavailable,
        unmanaged_fallback_capability: BrowserUnmanagedFallbackCapability::Unavailable,
    }
}

fn managed_profile_missing_bridge_state() -> BrowserInventoryDerivedState {
    BrowserInventoryDerivedState {
        install_state: BrowserInventoryInstallState::Installed,
        running_state: BrowserInventoryRunningState::NotRunning,
        management_tier: BrowserManagementTier::Managed,
        support_tier: BrowserSupportTier::Candidate,
        exact_url_capability: BrowserExactUrlCapability::Unavailable,
        active_tab_capability: BrowserActiveTabCapability::Unavailable,
        managed_profile_state: BrowserManagedProfileState::Missing,
        unmanaged_fallback_capability: BrowserUnmanagedFallbackCapability::OsBlockManualRequired,
    }
}

fn unsupported_inventory_state() -> BrowserInventoryDerivedState {
    BrowserInventoryDerivedState {
        install_state: BrowserInventoryInstallState::Installed,
        running_state: BrowserInventoryRunningState::Unknown,
        management_tier: BrowserManagementTier::Unsupported,
        support_tier: BrowserSupportTier::Unsupported,
        exact_url_capability: BrowserExactUrlCapability::Unsupported,
        active_tab_capability: BrowserActiveTabCapability::Unsupported,
        managed_profile_state: BrowserManagedProfileState::NotApplicable,
        unmanaged_fallback_capability: BrowserUnmanagedFallbackCapability::Unsupported,
    }
}

fn degraded_inventory_state() -> BrowserInventoryDerivedState {
    BrowserInventoryDerivedState {
        install_state: BrowserInventoryInstallState::Installed,
        running_state: BrowserInventoryRunningState::RunningUnknown,
        management_tier: BrowserManagementTier::ManualRequired,
        support_tier: BrowserSupportTier::ManualRequired,
        exact_url_capability: BrowserExactUrlCapability::ManualRequired,
        active_tab_capability: BrowserActiveTabCapability::ManualRequired,
        managed_profile_state: BrowserManagedProfileState::ManualRequired,
        unmanaged_fallback_capability: BrowserUnmanagedFallbackCapability::Unavailable,
    }
}

fn inventory_row_id(
    browser_family: &BrowserFamily,
    capability_status: &BrowserCapabilityStatus,
) -> String {
    match capability_status {
        BrowserCapabilityStatus::UnmanagedBrowser => {
            constants::browser::INVENTORY_ROW_ID_UNMANAGED_CHROME.to_string()
        }
        _ => match browser_family {
            BrowserFamily::Edge => constants::browser::INVENTORY_ROW_ID_EDGE_STABLE.to_string(),
            BrowserFamily::Chrome => {
                constants::browser::INVENTORY_ROW_ID_MANAGED_CHROME.to_string()
            }
            _ => constants::browser::INVENTORY_ROW_ID_UNKNOWN_BROWSER.to_string(),
        },
    }
}

fn product_name(browser_family: &BrowserFamily) -> String {
    match browser_family {
        BrowserFamily::Edge => constants::browser::PRODUCT_NAME_MICROSOFT_EDGE.to_string(),
        BrowserFamily::Chrome => constants::browser::PRODUCT_NAME_GOOGLE_CHROME.to_string(),
        BrowserFamily::Brave => constants::browser::FAMILY_BRAVE.to_string(),
        BrowserFamily::Firefox => constants::browser::FAMILY_FIREFOX.to_string(),
        BrowserFamily::Opera => constants::browser::FAMILY_OPERA.to_string(),
        BrowserFamily::UnknownChromium => constants::browser::FAMILY_UNKNOWN_CHROMIUM.to_string(),
        BrowserFamily::Unknown => constants::browser::FAMILY_UNKNOWN.to_string(),
    }
}

fn executable_path_ref(status: &BrowserManagedSessionStatus) -> Option<String> {
    status.unmanaged_executable_path_ref.clone().or_else(|| {
        status
            .process_id
            .map(|_| constants::browser::DEVTOOLS_TEST_EXECUTABLE_PATH.to_string())
    })
}

fn reason_code(status: &BrowserManagedSessionStatus) -> String {
    status.degraded_reason.clone().unwrap_or_else(|| {
        constants::browser::INVENTORY_REASON_MANAGED_TARGET_LIST_ACTIVE_TAB_UNPROVED.to_string()
    })
}

fn latest_observed_at(scanned_at: &str, returned: u64) -> Option<String> {
    if returned == 0 {
        return None;
    }
    Some(scanned_at.to_string())
}

fn windows_inventory_row_id(
    observation: &BrowserWindowsInventoryObservation,
    row_index: usize,
) -> String {
    let mut row_id = String::from(constants::browser::INVENTORY_ROW_ID_PREFIX_WINDOWS);
    row_id.push(constants::delimiter::HYPHEN);
    row_id.push_str(observation.browser_family.as_protocol_str());
    row_id.push(constants::delimiter::HYPHEN);
    row_id.push_str(observation.browser_channel.as_protocol_str());
    row_id.push(constants::delimiter::HYPHEN);
    row_id.push_str(observation.install_state.as_protocol_str());
    row_id.push(constants::delimiter::HYPHEN);
    match observation.process_id {
        Some(process_id) => row_id.push_str(&process_id.to_string()),
        None => row_id.push_str(&row_index.to_string()),
    }
    row_id
}
