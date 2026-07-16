use ocentra_parent_agent_protocol::browser::BrowserCapabilityStatus;

use super::super::BrowserBridgePollError;

pub(crate) fn browser_bridge_poll_error_capability_status(
    error: &BrowserBridgePollError,
) -> BrowserCapabilityStatus {
    if matches!(
        error,
        BrowserBridgePollError::InvalidHttpResponse
            | BrowserBridgePollError::InvalidJson
            | BrowserBridgePollError::InvalidTargetPayload
            | BrowserBridgePollError::ResponseTooLarge
    ) {
        return BrowserCapabilityStatus::AdapterError;
    }
    BrowserCapabilityStatus::BridgeMissing
}
