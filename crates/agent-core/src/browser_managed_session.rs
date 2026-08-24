use std::{
    fmt,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use chrono::{SecondsFormat, Utc};

use ocentra_parent_agent_protocol::browser::{BrowserChannel, BrowserFamily};
use ocentra_parent_agent_protocol::browser_managed::{
    BrowserManagedProfileLifecycleState, BrowserManagedProfileStoreEntry,
};
#[path = "browser_managed_session/accessors.rs"]
mod accessors;
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

/// Opaque configuration issued by the browser owner.
///
/// The profile-store boundary deliberately exposes no constructor or public
/// fields.  A path and its identity binding must come from the owner that
/// derives the Ocentra-managed root; callers cannot select a deletion root or
/// mint a binding by assembling this value at a public API boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BrowserManagedProfileStoreConfig {
    profile_root_dir: PathBuf,
    profile_id: String,
    profile_scope_id: String,
    device_id: String,
    browser_family: BrowserFamily,
    browser_channel: BrowserChannel,
    policy_revision: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BrowserManagedProfileStoreRecord {
    profile_dir: PathBuf,
    metadata_path: PathBuf,
    entry: BrowserManagedProfileStoreEntry,
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
    process_id: u32,
    bridge_port: u16,
    browser_family: BrowserFamily,
    browser_channel: BrowserChannel,
    profile_path_ref: String,
    bridge_endpoint_ref: String,
    pub(crate) cdp_authority: ManagedBrowserLaunchAuthority,
}

/// Private launch evidence carried only by a real managed-browser launch.
///
/// The public launch fields are status data and are intentionally insufficient
/// to mint a CDP capture authority. Endpoint, process, executable, and profile
/// identity are kept here so the capture owner can revalidate them without
/// accepting caller-assembled authority.
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct ManagedBrowserLaunchAuthority {
    managed_browser_session_id: String,
    profile_id: String,
    process_id: u32,
    bridge_port: u16,
    browser_family: BrowserFamily,
    browser_channel: BrowserChannel,
    executable_path: PathBuf,
    profile_path: PathBuf,
    generation: u64,
    created_at_epoch_ms: u64,
    expires_at_epoch_ms: u64,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrowserManagedLaunchError {
    DefaultProfileRejected,
    UnownedProfileRejected,
    BridgePortUnavailable,
    UnsupportedBrowser,
    ManualRequired,
    Io,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrowserManagedProfileStoreError {
    DefaultProfileRejected,
    UnownedProfileRejected,
    BindingMismatch,
    MetadataCorrupt,
    StoreBusy,
    UnsafePath,
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

pub(crate) fn load_managed_browser_profile_store(
    config: &BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    store::load_managed_browser_profile_store(config)
}

pub(crate) fn create_or_repair_managed_browser_profile_store(
    config: &BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    store::create_or_repair_managed_browser_profile_store(config)
}

pub(crate) fn delete_managed_browser_profile_store(
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

impl BrowserManagedLaunch {
    /// Builds bridge custody only from the private launch authority. The
    /// service may retain and pass this opaque launch, but cannot construct a
    /// trusted bridge config from process labels or environment values.
    fn bridge_poll_config(
        &self,
        session_fresh_until: impl Into<String>,
    ) -> crate::browser_bridge_poll::BrowserBridgePollConfig {
        let authority = &self.cdp_authority;
        crate::browser_bridge_poll::BrowserBridgePollConfig {
            endpoint: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), authority.bridge_port),
            managed_browser_session_id: authority.managed_browser_session_id.clone(),
            profile_id: authority.profile_id.clone(),
            process_id: authority.process_id,
            browser_family: authority.browser_family,
            browser_channel: authority.browser_channel,
            expected_custody: crate::browser_bridge_poll::BrowserBridgeExpectedCustody {
                bridge_port: authority.bridge_port,
                managed_browser_session_id: authority.managed_browser_session_id.clone(),
                profile_id: authority.profile_id.clone(),
                process_id: authority.process_id,
                browser_family: authority.browser_family,
                browser_channel: authority.browser_channel,
                session_fresh_until: session_fresh_until.into(),
            },
        }
    }

    pub fn poll_bridge(
        &self,
    ) -> Result<
        crate::browser_bridge_poll::BrowserBridgePollSnapshot,
        crate::browser_bridge_poll::BrowserBridgePollError,
    > {
        crate::browser_bridge_capture::revalidate_managed_browser_launch(self).map_err(
            |error| match error {
                crate::browser_bridge_capture::ManagedBrowserCdpCaptureError::Bridge(error) => {
                    error
                }
                _ => crate::browser_bridge_poll::BrowserBridgePollError::UntrustedProcess,
            },
        )?;
        let observed_at = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let config = self.bridge_poll_config(self.session_fresh_until());
        crate::browser_bridge_poll::poll_chromium_bridge(
            &config,
            &observed_at,
            &self.session_fresh_until(),
        )
    }

    pub fn expires_at_epoch_ms(&self) -> u64 {
        self.cdp_authority.expires_at_epoch_ms
    }

    fn session_fresh_until(&self) -> String {
        chrono::DateTime::<Utc>::from_timestamp_millis(
            i64::try_from(self.cdp_authority.expires_at_epoch_ms).unwrap_or(i64::MAX),
        )
        .unwrap_or_else(Utc::now)
        .to_rfc3339_opts(SecondsFormat::Millis, true)
    }

    pub fn managed_browser_session_id(&self) -> &str {
        &self.cdp_authority.managed_browser_session_id
    }

    pub fn retire(&self) -> bool {
        crate::browser_bridge_capture::retire_managed_browser_launch(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct BrowserManagedProfileStorePaths {
    profile_dir: PathBuf,
    metadata_path: PathBuf,
    deletion_path: PathBuf,
    lock_path: PathBuf,
}

struct ProfileStoreRecordInput {
    created_at: String,
    updated_at: String,
    lifecycle_state: BrowserManagedProfileLifecycleState,
    missing_since: Option<String>,
    repaired_at: Option<String>,
    deleted_at: Option<String>,
    repair_reason: Option<String>,
}
