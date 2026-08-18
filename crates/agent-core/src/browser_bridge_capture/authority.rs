use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use ocentra_parent_agent_protocol::browser::{BrowserChannel, BrowserFamily};

use super::{binding, ManagedBrowserCdpCaptureError};
use crate::browser_managed_session::BrowserManagedLaunch;

pub(super) struct LaunchBinding {
    pub(super) endpoint: SocketAddr,
    pub(super) managed_browser_session_id: String,
    pub(super) profile_id: String,
    pub(super) process_id: u32,
    pub(super) browser_family: BrowserFamily,
    pub(super) browser_channel: BrowserChannel,
    pub(super) session_secret: [u8; 32],
    pub(super) generation: u64,
    pub(super) created_at_epoch_ms: u64,
    pub(super) expires_at_epoch_ms: u64,
    pub(super) executable_binding: String,
    pub(super) profile_binding: String,
}

pub(super) fn from_launch(
    launch: &BrowserManagedLaunch,
) -> Result<LaunchBinding, ManagedBrowserCdpCaptureError> {
    let authority = launch.cdp_authority();
    let binding = LaunchBinding {
        endpoint: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), launch.bridge_port),
        managed_browser_session_id: authority.managed_browser_session_id.clone(),
        profile_id: authority.profile_id.clone(),
        process_id: launch.process_id,
        browser_family: launch.browser_family,
        browser_channel: launch.browser_channel,
        session_secret: authority.session_secret,
        generation: authority.generation,
        created_at_epoch_ms: authority.created_at_epoch_ms,
        expires_at_epoch_ms: authority.expires_at_epoch_ms,
        executable_binding: authority.executable_binding.clone(),
        profile_binding: authority.profile_binding.clone(),
    };
    binding::validate(&binding)?;
    Ok(binding)
}
