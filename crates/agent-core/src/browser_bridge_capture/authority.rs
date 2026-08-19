use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    time::Instant,
};

use ocentra_parent_agent_protocol::browser::{BrowserChannel, BrowserFamily};

use super::{ManagedBrowserCdpCaptureError, binding};
use crate::browser_managed_session::BrowserManagedLaunch;

#[derive(Clone)]
pub(super) struct LaunchBinding {
    pub(super) endpoint: SocketAddr,
    pub(super) managed_browser_session_id: String,
    pub(super) profile_id: String,
    pub(super) process_id: u32,
    pub(super) browser_family: BrowserFamily,
    pub(super) browser_channel: BrowserChannel,
    pub(super) executable_path: PathBuf,
    pub(super) profile_path: PathBuf,
    pub(super) generation: u64,
    pub(super) created_at_epoch_ms: u64,
    pub(super) expires_at_epoch_ms: u64,
    pub(super) authority_started_at: Instant,
    pub(super) authority_started_epoch_ms: u64,
}

pub(super) fn from_launch(
    launch: &BrowserManagedLaunch,
) -> Result<LaunchBinding, ManagedBrowserCdpCaptureError> {
    let authority = launch.cdp_authority();
    let authority_started_epoch_ms = binding::unix_epoch_millis()?;
    let binding = LaunchBinding {
        endpoint: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), authority.bridge_port()),
        managed_browser_session_id: authority.managed_browser_session_id().to_owned(),
        profile_id: authority.profile_id().to_owned(),
        process_id: authority.process_id(),
        browser_family: authority.browser_family(),
        browser_channel: authority.browser_channel(),
        executable_path: authority.executable_path().clone(),
        profile_path: authority.profile_path().clone(),
        generation: authority.generation(),
        created_at_epoch_ms: authority.created_at_epoch_ms(),
        expires_at_epoch_ms: authority.expires_at_epoch_ms(),
        authority_started_at: Instant::now(),
        authority_started_epoch_ms,
    };
    binding::validate(&binding)?;
    Ok(binding)
}

pub(super) fn from_launch_for_retirement(
    launch: &BrowserManagedLaunch,
) -> Result<LaunchBinding, ManagedBrowserCdpCaptureError> {
    let authority = launch.cdp_authority();
    let authority_started_epoch_ms = binding::unix_epoch_millis()?;
    Ok(LaunchBinding {
        endpoint: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), authority.bridge_port()),
        managed_browser_session_id: authority.managed_browser_session_id().to_owned(),
        profile_id: authority.profile_id().to_owned(),
        process_id: authority.process_id(),
        browser_family: authority.browser_family(),
        browser_channel: authority.browser_channel(),
        executable_path: authority.executable_path().clone(),
        profile_path: authority.profile_path().clone(),
        generation: authority.generation(),
        created_at_epoch_ms: authority.created_at_epoch_ms(),
        expires_at_epoch_ms: authority.expires_at_epoch_ms(),
        authority_started_at: Instant::now(),
        authority_started_epoch_ms,
    })
}
