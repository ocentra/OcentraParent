use ocentra_parent_agent_protocol::constants;

use super::super::BrowserBridgePollError;

pub(crate) fn browser_bridge_poll_response_reason(error: &BrowserBridgePollError) -> &'static str {
    match error {
        BrowserBridgePollError::InvalidHttpResponse => {
            constants::value::BROWSER_BRIDGE_INVALID_RESPONSE
        }
        BrowserBridgePollError::InvalidJson => constants::value::BROWSER_BRIDGE_INVALID_JSON,
        BrowserBridgePollError::InvalidTargetPayload => {
            constants::value::BROWSER_BRIDGE_INVALID_TARGET_PAYLOAD
        }
        BrowserBridgePollError::ResponseTooLarge => {
            constants::value::BROWSER_BRIDGE_RESPONSE_TOO_LARGE
        }
        _ => constants::value::BROWSER_BRIDGE_INVALID_RESPONSE,
    }
}
