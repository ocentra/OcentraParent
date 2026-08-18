use std::{
    fmt,
    net::SocketAddr,
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::browser::{BrowserChannel, BrowserFamily};
use ocentra_parent_agent_protocol::browser_managed::{
    BrowserManagedProfileLifecycleState, BrowserManagedProfileStoreEntry,
};
#[path = "browser_managed_session/capability.rs"]
mod capability;
#[path = "browser_managed_session/launch.rs"]
mod launch;
#[path = "browser_managed_session/store.rs"]
mod store;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserManagedLaunchConfig {
    pub executable_path: PathBuf,
    pub profile_dir: PathBuf,
    pub bridge_port: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserManagedBridgePortReservation {
    pub endpoint: SocketAddr,
    pub bridge_port: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserManagedProfileStoreConfig {
    pub profile_root_dir: PathBuf,
    pub profile_id: String,
    pub profile_scope_id: String,
    pub device_id: String,
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub policy_revision: String,
    pub now: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserManagedProfileStoreRecord {
    pub profile_dir: PathBuf,
    pub metadata_path: PathBuf,
    pub entry: BrowserManagedProfileStoreEntry,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserManagedLaunchPlan {
    pub executable_path: PathBuf,
    pub args: Vec<String>,
    pub bridge_port: u16,
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub profile_path_ref: String,
    pub bridge_endpoint_ref: String,
}

#[derive(Clone, PartialEq, Eq)]
pub struct BrowserManagedLaunch {
    pub process_id: u32,
    pub bridge_port: u16,
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub profile_path_ref: String,
    pub bridge_endpoint_ref: String,
    pub(crate) cdp_authority: ManagedBrowserLaunchAuthority,
}

/// Private launch evidence carried only by a real managed-browser launch.
///
/// The public launch fields are status data and are intentionally insufficient
/// to mint a CDP capture authority. The random secret and binding digests are
/// created after the owned process is spawned and are never serialized or
/// exposed through Debug.
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct ManagedBrowserLaunchAuthority {
    pub(crate) managed_browser_session_id: String,
    pub(crate) profile_id: String,
    pub(crate) session_secret: [u8; 32],
    pub(crate) generation: u64,
    pub(crate) created_at_epoch_ms: u64,
    pub(crate) expires_at_epoch_ms: u64,
    pub(crate) executable_binding: String,
    pub(crate) profile_binding: String,
}

impl fmt::Debug for BrowserManagedLaunch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BrowserManagedLaunch")
            .field("process_id", &self.process_id)
            .field("bridge_port", &self.bridge_port)
            .field("browser_family", &self.browser_family)
            .field("browser_channel", &self.browser_channel)
            .field("profile_path_ref", &self.profile_path_ref)
            .field("bridge_endpoint_ref", &self.bridge_endpoint_ref)
            .field("cdp_authority", &"opaque")
            .finish()
    }
}

impl BrowserManagedLaunch {
    pub(crate) fn cdp_authority(&self) -> &ManagedBrowserLaunchAuthority {
        &self.cdp_authority
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrowserManagedLaunchError {
    DefaultProfileRejected,
    UnownedProfileRejected,
    BridgePortUnavailable,
    UnsupportedBrowser,
    Io,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrowserManagedProfileStoreError {
    DefaultProfileRejected,
    UnownedProfileRejected,
    MetadataCorrupt,
    Io,
}

impl BrowserManagedProfileStoreError {
    pub fn reason(&self) -> &'static str {
        store::profile_store_error_reason(self)
    }
}

impl BrowserManagedLaunchError {
    pub fn reason(&self) -> &'static str {
        launch::launch_error_reason(self)
    }
}

pub fn load_managed_browser_profile_store(
    config: &BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    store::load_managed_browser_profile_store(config)
}

pub fn create_or_repair_managed_browser_profile_store(
    config: &BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    store::create_or_repair_managed_browser_profile_store(config)
}

pub fn delete_managed_browser_profile_store(
    config: &BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    store::delete_managed_browser_profile_store(config)
}

pub fn reserve_managed_browser_bridge_port(
) -> Result<BrowserManagedBridgePortReservation, BrowserManagedLaunchError> {
    launch::reserve_managed_browser_bridge_port()
}

pub fn managed_browser_launch_plan(
    config: BrowserManagedLaunchConfig,
) -> Result<BrowserManagedLaunchPlan, BrowserManagedLaunchError> {
    launch::managed_browser_launch_plan(config)
}

pub fn launch_managed_browser(
    config: BrowserManagedLaunchConfig,
) -> Result<BrowserManagedLaunch, BrowserManagedLaunchError> {
    launch::launch_managed_browser(config)
}

pub(crate) fn managed_path_binding(path: &Path) -> String {
    capability::managed_path_binding(path)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct BrowserManagedProfileStorePaths {
    profile_dir: PathBuf,
    metadata_path: PathBuf,
}

struct ProfileStoreRecordInput {
    created_at: String,
    lifecycle_state: BrowserManagedProfileLifecycleState,
    missing_since: Option<String>,
    repaired_at: Option<String>,
    deleted_at: Option<String>,
    repair_reason: Option<String>,
}
