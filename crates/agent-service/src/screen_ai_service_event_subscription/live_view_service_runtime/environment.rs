use std::env;

use ocentra_parent_agent_protocol::constants;
use ocentra_screen_live_view_core::live_view_runtime::{
    ScreenLiveViewRuntimeMode, ScreenLiveViewRuntimePermission, ScreenLiveViewRuntimeTransport,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct EnvVarName(pub(super) &'static str);

pub(super) fn live_view_mode_from_environment() -> ScreenLiveViewRuntimeMode {
    match env::var(constants::screen_flow::SCREEN_LIVE_VIEW_MODE_ENV).ok() {
        Some(value) if value == constants::screen_flow::SCREEN_LIVE_VIEW_MODE_LAN_ONLY => {
            ScreenLiveViewRuntimeMode::LanOnlyView
        }
        Some(value) if value == constants::screen_flow::SCREEN_LIVE_VIEW_MODE_RELAY_BACKED => {
            ScreenLiveViewRuntimeMode::RelayBackedView
        }
        _ => ScreenLiveViewRuntimeMode::Disabled,
    }
}

pub(super) fn live_view_transport_from_environment() -> ScreenLiveViewRuntimeTransport {
    match env::var(constants::screen_flow::SCREEN_LIVE_VIEW_TRANSPORT_ENV).ok() {
        Some(value)
            if value == constants::screen_flow::SCREEN_LIVE_VIEW_TRANSPORT_LAN_MUTUAL_AUTH =>
        {
            ScreenLiveViewRuntimeTransport::LanMutualAuth
        }
        Some(value) if value == constants::screen_flow::SCREEN_LIVE_VIEW_TRANSPORT_RELAY_E2EE => {
            ScreenLiveViewRuntimeTransport::RelayEndToEndEncrypted
        }
        _ => ScreenLiveViewRuntimeTransport::None,
    }
}

pub(super) fn live_view_permission_from_environment() -> ScreenLiveViewRuntimePermission {
    match env::var(constants::screen_flow::SCREEN_LIVE_VIEW_PERMISSION_ENV).ok() {
        Some(value)
            if value == constants::screen_flow::SCREEN_LIVE_VIEW_PERMISSION_CAPTURE_ONLY =>
        {
            ScreenLiveViewRuntimePermission::ScreenCaptureOnly
        }
        Some(value) if value == constants::screen_flow::SCREEN_LIVE_VIEW_PERMISSION_LIVE_VIEW => {
            ScreenLiveViewRuntimePermission::LiveViewPermission
        }
        _ => ScreenLiveViewRuntimePermission::Missing,
    }
}

pub(super) fn env_flag(env_var_name: EnvVarName) -> bool {
    env::var(env_var_name.0).is_ok_and(|value| value == constants::screen_flow::ENV_TRUE)
}
