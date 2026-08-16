use ocentra_parent_agent_protocol::browser::BrowserCapabilityStatus;

use super::BrowserBridgePollError;

#[path = "error/capability.rs"]
mod capability;
#[path = "error/non_response.rs"]
mod non_response;
#[path = "error/response.rs"]
mod response;

pub(crate) fn browser_bridge_poll_error_reason(error: &BrowserBridgePollError) -> &'static str {
    if is_response_error(error) {
        return response::browser_bridge_poll_response_reason(error);
    }
    non_response::browser_bridge_poll_non_response_reason(error)
}

pub(crate) fn browser_bridge_poll_error_capability_status(
    error: &BrowserBridgePollError,
) -> BrowserCapabilityStatus {
    capability::browser_bridge_poll_error_capability_status(error)
}

fn is_response_error(error: &BrowserBridgePollError) -> bool {
    matches!(
        error,
        BrowserBridgePollError::InvalidHttpResponse
            | BrowserBridgePollError::InvalidJson
            | BrowserBridgePollError::InvalidTargetPayload
            | BrowserBridgePollError::ResponseTooLarge
    )
}
