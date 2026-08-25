use std::time::{SystemTime, UNIX_EPOCH};

use ocentra_parent_agent_protocol::{
    browser::{BrowserChannel, BrowserFamily},
    constants,
};
use ocentra_schema::managed_browser_cdp_capture::MANAGED_BROWSER_CDP_AUTHORITY_TTL_MS;

use super::{authority::LaunchBinding, ManagedBrowserCdpCaptureError};
use crate::browser_bridge_poll::BrowserBridgePollError;

pub(super) fn validate(binding: &LaunchBinding) -> Result<(), ManagedBrowserCdpCaptureError> {
    let now = unix_epoch_millis()?;
    if now < binding.created_at_epoch_ms || now > binding.expires_at_epoch_ms {
        return Err(ManagedBrowserCdpCaptureError::AuthorityExpired);
    }
    if binding.created_at_epoch_ms > binding.expires_at_epoch_ms
        || binding.authority_started_at.elapsed().as_millis()
            > u128::from(MANAGED_BROWSER_CDP_AUTHORITY_TTL_MS)
    {
        return Err(ManagedBrowserCdpCaptureError::AuthorityExpired);
    }
    if binding
        .expires_at_epoch_ms
        .saturating_sub(binding.created_at_epoch_ms)
        > MANAGED_BROWSER_CDP_AUTHORITY_TTL_MS
    {
        return Err(ManagedBrowserCdpCaptureError::AuthorityExpired);
    }
    if !binding.endpoint.ip().is_loopback()
        || binding.endpoint.port() == constants::browser::DEVTOOLS_PORT_UNRESERVED
    {
        return Err(BrowserBridgePollError::NonLoopbackEndpoint.into());
    }
    if binding.process_id == constants::browser::PROCESS_ID_UNKNOWN {
        return Err(BrowserBridgePollError::UntrustedProcess.into());
    }
    if !managed_session_id_is_valid(binding)
        || !profile_id_is_valid(binding)
        || binding.executable_path.as_os_str().is_empty()
        || binding.profile_path.as_os_str().is_empty()
        || binding.generation == 0
    {
        return Err(BrowserBridgePollError::UntrustedSession.into());
    }
    if matches!(
        binding.browser_family,
        BrowserFamily::Unknown | BrowserFamily::UnknownChromium
    ) || matches!(binding.browser_channel, BrowserChannel::Unknown)
    {
        return Err(BrowserBridgePollError::UntrustedBrowserIdentity.into());
    }
    Ok(())
}

fn managed_session_id_is_valid(binding: &LaunchBinding) -> bool {
    binding
        .managed_browser_session_id
        .starts_with(constants::browser::SESSION_ID_PREFIX_MANAGED)
}

fn profile_id_is_valid(binding: &LaunchBinding) -> bool {
    binding
        .profile_id
        .starts_with(constants::browser::PROFILE_ID_PREFIX_MANAGED)
        && !binding.profile_id.trim().is_empty()
        && !binding
            .profile_id
            .chars()
            .any(|character| matches!(character, '/' | '\\' | ':'))
}

pub(super) fn unix_epoch_millis() -> Result<u64, ManagedBrowserCdpCaptureError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_error| ManagedBrowserCdpCaptureError::AuthorityExpired)
        .and_then(|duration| {
            u64::try_from(duration.as_millis())
                .map_err(|_error| ManagedBrowserCdpCaptureError::AuthorityExpired)
        })
}
