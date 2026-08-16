use std::{net::SocketAddr, path::PathBuf};

use ocentra_parent_agent_protocol::browser::{BrowserChannel, BrowserFamily};
use ocentra_parent_agent_protocol::browser_managed::{
    BrowserManagedProfileLifecycleState, BrowserManagedProfileStoreEntry,
};
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserManagedLaunch {
    pub process_id: u32,
    pub bridge_port: u16,
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub profile_path_ref: String,
    pub bridge_endpoint_ref: String,
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
