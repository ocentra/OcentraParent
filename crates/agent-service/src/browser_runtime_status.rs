use ocentra_parent_agent_protocol::browser::BrowserCapabilityStatus;
use ocentra_parent_agent_protocol::browser::BrowserChannel;
use ocentra_parent_agent_protocol::browser::BrowserCustodyLabel;
use ocentra_parent_agent_protocol::browser::BrowserFamily;
use ocentra_parent_agent_protocol::browser::BROWSER_EVIDENCE_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::browser_managed::BrowserBridgeKind;
use ocentra_parent_agent_protocol::browser_managed::BrowserManagedProfileLifecycleState;
use ocentra_parent_agent_protocol::browser_managed::BrowserManagedSessionStatus;
use ocentra_parent_agent_protocol::browser_managed::BrowserManagedState;
use ocentra_parent_agent_protocol::browser_managed::BrowserQueryVisibilityLabel;
use ocentra_parent_agent_protocol::constants;

use ocentra_parent_agent_core::{
    browser_managed_discovery::BrowserUnmanagedProcessObservation,
    browser_managed_session::BrowserManagedLaunch,
};
use ocentra_parent_agent_protocol::browser_managed::BrowserManagedProfileStoreEntry;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BrowserRuntimeText(String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BrowserRuntimeOptionalText(Option<String>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BrowserManagedLaunchStatus {
    process_id: u32,
    browser_family: BrowserFamily,
    browser_channel: BrowserChannel,
    profile_path_ref: String,
    bridge_endpoint_ref: String,
}

impl BrowserManagedLaunchStatus {
    pub(crate) fn new(
        process_id: u32,
        browser_family: BrowserFamily,
        browser_channel: BrowserChannel,
        profile_path_ref: BrowserRuntimeText,
        bridge_endpoint_ref: BrowserRuntimeText,
    ) -> Self {
        Self {
            process_id,
            browser_family,
            browser_channel,
            profile_path_ref: profile_path_ref.0,
            bridge_endpoint_ref: bridge_endpoint_ref.0,
        }
    }
}

impl From<BrowserManagedLaunch> for BrowserManagedLaunchStatus {
    fn from(launch: BrowserManagedLaunch) -> Self {
        Self::new(
            launch.process_id(),
            launch.browser_family(),
            launch.browser_channel(),
            BrowserRuntimeText::from(launch.profile_path_ref()),
            BrowserRuntimeText::from(launch.bridge_endpoint_ref()),
        )
    }
}

impl<T> From<T> for BrowserRuntimeText
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<T> From<Option<T>> for BrowserRuntimeOptionalText
where
    T: Into<String>,
{
    fn from(value: Option<T>) -> Self {
        Self(value.map(Into::into))
    }
}

pub(crate) fn missing_browser_status(
    checked_at: impl Into<BrowserRuntimeText>,
) -> BrowserManagedSessionStatus {
    let checked_at = checked_at.into();
    BrowserManagedSessionStatus {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        checked_at: checked_at.0,
        managed_browser_session_id: None,
        browser_family: None,
        browser_channel: None,
        browser_version: None,
        profile_id: None,
        profile_path_ref: None,
        profile_root_ref: None,
        profile_scope_id: None,
        profile_lifecycle_state: None,
        policy_revision: None,
        process_id: None,
        bridge_kind: None,
        bridge_endpoint_ref: None,
        unmanaged_process_name: None,
        unmanaged_executable_path_ref: None,
        unmanaged_signature_ref: None,
        unmanaged_process_hash_ref: None,
        unmanaged_process_kind: None,
        unmanaged_detection_confidence: None,
        unmanaged_detection_reason: None,
        managed_state: BrowserManagedState::NotInstalled,
        capability_status: BrowserCapabilityStatus::ManagedProfileMissing,
        degraded_reason: Some(constants::value::MANAGED_BROWSER_EXECUTABLE_MISSING.to_string()),
        started_at: None,
        custody_label: BrowserCustodyLabel::Unavailable,
        query_visibility: BrowserQueryVisibilityLabel::Unavailable,
    }
}

pub(crate) fn profile_missing_status(
    checked_at: impl Into<BrowserRuntimeText>,
) -> BrowserManagedSessionStatus {
    let mut status = missing_browser_status(checked_at);
    status.managed_state = BrowserManagedState::InstalledSupported;
    status.degraded_reason =
        Some(constants::value::MANAGED_BROWSER_PROFILE_DIR_MISSING.to_string());
    status.profile_lifecycle_state = Some(BrowserManagedProfileLifecycleState::Missing);
    status
}

pub(crate) fn unmanaged_browser_status(
    checked_at: impl Into<BrowserRuntimeText>,
    process: BrowserUnmanagedProcessObservation,
) -> BrowserManagedSessionStatus {
    let mut status = missing_browser_status(checked_at);
    status.process_id = Some(process.process_id);
    status.browser_family = Some(process.browser_family);
    status.browser_channel = Some(process.browser_channel);
    status.unmanaged_process_name = Some(process.process_name);
    status.unmanaged_executable_path_ref = process.executable_path_ref;
    status.unmanaged_signature_ref = process.signature_ref;
    status.unmanaged_process_hash_ref = process.process_hash_ref;
    status.unmanaged_process_kind = Some(process.process_kind);
    status.unmanaged_detection_confidence = Some(process.detection_confidence);
    status.unmanaged_detection_reason = Some(process.detection_reason);
    status.managed_state = BrowserManagedState::InstalledSupported;
    status.capability_status = BrowserCapabilityStatus::UnmanagedBrowser;
    status.degraded_reason = Some(constants::value::MANAGED_BROWSER_UNMANAGED_PROCESS.to_string());
    status.custody_label = BrowserCustodyLabel::ChildDeviceLocal;
    status
}

pub(crate) fn managed_profile_ready_status(
    checked_at: impl Into<BrowserRuntimeText>,
    browser_family: BrowserFamily,
    browser_channel: BrowserChannel,
    profile_store_entry: BrowserManagedProfileStoreEntry,
) -> BrowserManagedSessionStatus {
    let mut status = base_managed_status(checked_at);
    status.browser_family = Some(browser_family);
    status.browser_channel = Some(browser_channel);
    apply_profile_store_entry(&mut status, profile_store_entry);
    status
}

pub(crate) fn running_managed_status(
    checked_at: impl Into<BrowserRuntimeText>,
    launch: impl Into<BrowserManagedLaunchStatus>,
    profile_store_entry: BrowserManagedProfileStoreEntry,
    started_at: impl Into<BrowserRuntimeText>,
) -> BrowserManagedSessionStatus {
    let launch = launch.into();
    let started_at = started_at.into();
    let mut status = base_managed_status(checked_at);
    status.browser_family = Some(launch.browser_family);
    status.browser_channel = Some(launch.browser_channel);
    status.profile_path_ref = Some(launch.profile_path_ref);
    status.process_id = Some(launch.process_id);
    status.managed_state = BrowserManagedState::RunningManaged;
    status.capability_status = BrowserCapabilityStatus::BridgeMissing;
    status.degraded_reason =
        Some(constants::value::MANAGED_BROWSER_BRIDGE_CONNECT_PENDING.to_string());
    status.started_at = Some(started_at.0);
    apply_profile_store_entry(&mut status, profile_store_entry);
    status.bridge_endpoint_ref = Some(launch.bridge_endpoint_ref);
    status
}

pub(crate) fn bridge_disconnected_status(
    checked_at: impl Into<BrowserRuntimeText>,
    reason: impl Into<BrowserRuntimeText>,
    started_at: impl Into<BrowserRuntimeText>,
    launch: &BrowserManagedLaunch,
    profile_store_entry: &BrowserManagedProfileStoreEntry,
) -> BrowserManagedSessionStatus {
    let reason = reason.into();
    let mut status =
        managed_launch_base_status(checked_at, launch, profile_store_entry, started_at);
    status.managed_state = BrowserManagedState::BridgeDisconnected;
    status.capability_status = BrowserCapabilityStatus::Stale;
    status.degraded_reason = Some(reason.0);
    status
}

pub(crate) fn stopped_managed_status(
    checked_at: impl Into<BrowserRuntimeText>,
    reason: impl Into<BrowserRuntimeText>,
    launch: &BrowserManagedLaunch,
    profile_store_entry: &BrowserManagedProfileStoreEntry,
    started_at: impl Into<BrowserRuntimeText>,
) -> BrowserManagedSessionStatus {
    let reason = reason.into();
    let mut status =
        managed_launch_base_status(checked_at, launch, profile_store_entry, started_at);
    status.managed_state = BrowserManagedState::Stopped;
    status.capability_status = BrowserCapabilityStatus::Stale;
    status.degraded_reason = Some(reason.0);
    status
}

pub(crate) fn status_with_error(
    checked_at: impl Into<BrowserRuntimeText>,
    reason: impl Into<BrowserRuntimeText>,
) -> BrowserManagedSessionStatus {
    let reason = reason.into();
    let mut status = base_managed_status(checked_at);
    status.managed_state = BrowserManagedState::Error;
    status.capability_status = BrowserCapabilityStatus::AdapterError;
    status.degraded_reason = Some(reason.0);
    status
}

pub(crate) fn connected_status(
    checked_at: impl Into<BrowserRuntimeText>,
    browser_version: impl Into<BrowserRuntimeOptionalText>,
    capability_status: BrowserCapabilityStatus,
    degraded_reason: impl Into<BrowserRuntimeOptionalText>,
    started_at: impl Into<BrowserRuntimeText>,
    launch: &BrowserManagedLaunch,
    profile_store_entry: &BrowserManagedProfileStoreEntry,
) -> BrowserManagedSessionStatus {
    let browser_version = browser_version.into();
    let degraded_reason = degraded_reason.into();
    let mut status =
        managed_launch_base_status(checked_at, launch, profile_store_entry, started_at);
    status.browser_version = browser_version.0;
    status.managed_state = BrowserManagedState::BridgeConnected;
    status.capability_status = capability_status;
    status.degraded_reason = degraded_reason.0;
    status
}

fn managed_launch_base_status(
    checked_at: impl Into<BrowserRuntimeText>,
    launch: &BrowserManagedLaunch,
    profile_store_entry: &BrowserManagedProfileStoreEntry,
    started_at: impl Into<BrowserRuntimeText>,
) -> BrowserManagedSessionStatus {
    let mut status = base_managed_status(checked_at);
    status.managed_browser_session_id = Some(launch.managed_browser_session_id().to_owned());
    status.browser_family = Some(launch.browser_family());
    status.browser_channel = Some(launch.browser_channel());
    status.process_id = Some(launch.process_id());
    status.profile_id = Some(profile_store_entry.profile_id.clone());
    status.profile_path_ref = Some(profile_store_entry.profile_path_ref.clone());
    status.profile_root_ref = Some(profile_store_entry.profile_root_ref.clone());
    status.profile_scope_id = Some(profile_store_entry.profile_scope_id.clone());
    status.profile_lifecycle_state = Some(profile_store_entry.lifecycle_state);
    status.policy_revision = Some(profile_store_entry.policy_revision.clone());
    status.started_at = Some(started_at.into().0);
    status
}

fn base_managed_status(checked_at: impl Into<BrowserRuntimeText>) -> BrowserManagedSessionStatus {
    let checked_at = checked_at.into();
    BrowserManagedSessionStatus {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        checked_at: checked_at.0,
        managed_browser_session_id: None,
        browser_family: None,
        browser_channel: None,
        browser_version: None,
        profile_id: None,
        profile_path_ref: None,
        profile_root_ref: None,
        profile_scope_id: None,
        profile_lifecycle_state: None,
        policy_revision: None,
        process_id: None,
        bridge_kind: None,
        bridge_endpoint_ref: None,
        unmanaged_process_name: None,
        unmanaged_executable_path_ref: None,
        unmanaged_signature_ref: None,
        unmanaged_process_hash_ref: None,
        unmanaged_process_kind: None,
        unmanaged_detection_confidence: None,
        unmanaged_detection_reason: None,
        managed_state: BrowserManagedState::Error,
        capability_status: BrowserCapabilityStatus::BridgeMissing,
        degraded_reason: None,
        started_at: None,
        custody_label: BrowserCustodyLabel::Unavailable,
        query_visibility: BrowserQueryVisibilityLabel::Unavailable,
    }
}

fn apply_profile_store_entry(
    status: &mut BrowserManagedSessionStatus,
    entry: BrowserManagedProfileStoreEntry,
) {
    status.profile_id = Some(entry.profile_id);
    status.profile_path_ref = Some(entry.profile_path_ref);
    status.profile_root_ref = Some(entry.profile_root_ref);
    status.profile_scope_id = Some(entry.profile_scope_id);
    status.profile_lifecycle_state = Some(entry.lifecycle_state);
    status.policy_revision = Some(entry.policy_revision);
}
