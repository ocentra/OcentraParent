use super::{LanAiProviderRoutingState, LanPairingRuntime};

pub(super) fn routing_state(runtime: &LanPairingRuntime) -> LanAiProviderRoutingState {
    match runtime.lan_ai_provider_routing_state().0.as_str() {
        super::constants::value::LAN_AI_PROVIDER_ROUTING_AUTHORIZED_RESULT => {
            LanAiProviderRoutingState::AuthorizedResult
        }
        super::constants::value::LAN_AI_PROVIDER_ROUTING_BUSY => LanAiProviderRoutingState::Busy,
        super::constants::value::LAN_AI_PROVIDER_ROUTING_DEGRADED => {
            LanAiProviderRoutingState::Degraded
        }
        super::constants::value::LAN_AI_PROVIDER_ROUTING_UNSUPPORTED_CAPABILITY => {
            LanAiProviderRoutingState::UnsupportedCapability
        }
        _ => LanAiProviderRoutingState::Unavailable,
    }
}
