use std::env;

use ocentra_parent_agent_protocol::constants;

use super::BrowserRuntimeErrorText;

pub(super) fn configured_bridge_port() -> Result<Option<u16>, BrowserRuntimeErrorText> {
    match env::var(constants::env_var::MANAGED_BROWSER_BRIDGE_PORT) {
        Ok(port) => port.parse::<u16>().map(Some).map_err(|error| {
            let _ = error;
            BrowserRuntimeErrorText(constants::value::MANAGED_BROWSER_INVALID_BRIDGE_PORT)
        }),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(_) => Err(BrowserRuntimeErrorText(
            constants::value::MANAGED_BROWSER_INVALID_BRIDGE_PORT,
        )),
    }
}

pub(super) fn launch_on_status_enabled() -> bool {
    env::var(constants::env_var::MANAGED_BROWSER_LAUNCH_ON_STATUS)
        .map(|value| value == constants::value::TRUE)
        .unwrap_or(false)
}
