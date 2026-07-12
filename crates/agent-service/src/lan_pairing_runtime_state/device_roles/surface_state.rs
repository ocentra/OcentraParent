use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    DeviceRuntimeAiProviderState, DeviceRuntimeRouteState, DeviceRuntimeSurface, LanPairingText,
};

use super::non_empty_env;

pub(super) fn platform_for_surface(surface: &DeviceRuntimeSurface) -> LanPairingText {
    match surface {
        DeviceRuntimeSurface::ParentMobile | DeviceRuntimeSurface::ChildAndroid => {
            LanPairingText(constants::local_ai_runtime::PLATFORM_OS_ANDROID.to_string())
        }
        DeviceRuntimeSurface::ChildIos => {
            LanPairingText(constants::value::DEVICE_RUNTIME_PLATFORM_IOS.to_string())
        }
        DeviceRuntimeSurface::ParentDesktop | DeviceRuntimeSurface::ChildDesktop => {
            LanPairingText(constants::local_ai_runtime::PLATFORM_OS_WINDOWS.to_string())
        }
    }
}

pub(super) fn route_state_for_surface(surface: &DeviceRuntimeSurface) -> DeviceRuntimeRouteState {
    match surface {
        DeviceRuntimeSurface::ParentMobile
        | DeviceRuntimeSurface::ChildAndroid
        | DeviceRuntimeSurface::ChildIos => DeviceRuntimeRouteState::ManualRequired,
        DeviceRuntimeSurface::ParentDesktop | DeviceRuntimeSurface::ChildDesktop => {
            DeviceRuntimeRouteState::Localhost
        }
    }
}

pub(super) fn ai_provider_state(has_ai_provider: bool) -> DeviceRuntimeAiProviderState {
    match (
        has_ai_provider,
        non_empty_env(LanPairingText(
            constants::lan_pairing::LAN_AI_PROVIDER_OPT_IN_ENV.to_string(),
        ))
        .as_ref()
        .map(|value| value.0.as_str()),
    ) {
        (true, Some(constants::value::TRUE)) => DeviceRuntimeAiProviderState::Available,
        (true, _) => DeviceRuntimeAiProviderState::Degraded,
        (false, _) => DeviceRuntimeAiProviderState::Unavailable,
    }
}
