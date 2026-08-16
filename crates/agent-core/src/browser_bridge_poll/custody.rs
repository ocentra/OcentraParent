use ocentra_parent_agent_protocol::browser::{BrowserChannel, BrowserFamily};
use ocentra_parent_agent_protocol::constants;

use super::{BrowserBridgePollConfig, BrowserBridgePollError};

pub(crate) fn validate_bridge_custody(
    config: &BrowserBridgePollConfig,
    observed_at: &str,
) -> Result<(), BrowserBridgePollError> {
    if config.expected_custody.bridge_port == constants::browser::DEVTOOLS_PORT_UNRESERVED
        || config.endpoint.port() != config.expected_custody.bridge_port
    {
        return Err(BrowserBridgePollError::UntrustedBridgePort);
    }
    if observed_at > config.expected_custody.session_fresh_until.as_str() {
        return Err(BrowserBridgePollError::StaleSession);
    }
    if config.managed_browser_session_id != config.expected_custody.managed_browser_session_id
        || !config
            .managed_browser_session_id
            .starts_with(constants::browser::SESSION_ID_PREFIX_MANAGED)
    {
        return Err(BrowserBridgePollError::UntrustedSession);
    }
    if config.profile_id != config.expected_custody.profile_id
        || !managed_profile_id_is_trusted(&config.profile_id)
    {
        return Err(BrowserBridgePollError::UntrustedProfile);
    }
    if config.process_id == constants::browser::PROCESS_ID_UNKNOWN
        || config.process_id != config.expected_custody.process_id
    {
        return Err(BrowserBridgePollError::UntrustedProcess);
    }
    if browser_identity_is_unknown(&config.browser_family, &config.browser_channel)
        || config.browser_family != config.expected_custody.browser_family
        || config.browser_channel != config.expected_custody.browser_channel
    {
        return Err(BrowserBridgePollError::UntrustedBrowserIdentity);
    }
    Ok(())
}

fn managed_profile_id_is_trusted(profile_id: &str) -> bool {
    profile_id.starts_with(constants::browser::PROFILE_ID_PREFIX_MANAGED)
        && !profile_id.contains(constants::browser::PATH_SEPARATOR_FORWARD)
        && !profile_id.contains(constants::browser::PATH_SEPARATOR_BACKSLASH)
        && !profile_id.contains(constants::browser::PATH_SEPARATOR_COLON)
        && profile_id != constants::browser::PATH_SEGMENT_DEFAULT
        && profile_id != constants::browser::PATH_SEGMENT_USER_DATA
}

fn browser_identity_is_unknown(family: &BrowserFamily, channel: &BrowserChannel) -> bool {
    matches!(
        family,
        BrowserFamily::Unknown | BrowserFamily::UnknownChromium
    ) || matches!(channel, BrowserChannel::Unknown)
}
