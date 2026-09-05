use ocentra_parent_agent_protocol::constants;

use super::super::BrowserBridgePollError;

pub(crate) fn browser_bridge_poll_non_response_reason(
    error: &BrowserBridgePollError,
) -> &'static str {
    match error {
        BrowserBridgePollError::NonLoopbackEndpoint => {
            constants::value::BROWSER_BRIDGE_NON_LOOPBACK_ENDPOINT
        }
        BrowserBridgePollError::StaleSession => constants::value::BROWSER_BRIDGE_STALE_SESSION,
        BrowserBridgePollError::Timeout => constants::value::BROWSER_BRIDGE_TIMEOUT,
        BrowserBridgePollError::UntrustedBridgePort => {
            constants::value::BROWSER_BRIDGE_UNTRUSTED_PORT
        }
        BrowserBridgePollError::UntrustedBrowserIdentity => {
            constants::value::BROWSER_BRIDGE_UNTRUSTED_BROWSER_IDENTITY
        }
        BrowserBridgePollError::UntrustedProcess => {
            constants::value::BROWSER_BRIDGE_UNTRUSTED_PROCESS
        }
        BrowserBridgePollError::UntrustedProfile => {
            constants::value::BROWSER_BRIDGE_UNTRUSTED_PROFILE
        }
        BrowserBridgePollError::UntrustedSession => {
            constants::value::BROWSER_BRIDGE_UNTRUSTED_SESSION
        }
        BrowserBridgePollError::ManualRequired => {
            constants::value::MANAGED_BROWSER_BRIDGE_ENDPOINT_MANUAL_REQUIRED
        }
        BrowserBridgePollError::Io => constants::value::BROWSER_BRIDGE_IO_ERROR,
        _ => constants::value::BROWSER_BRIDGE_IO_ERROR,
    }
}
