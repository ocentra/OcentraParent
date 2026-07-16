use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeAiProviderState;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;

use crate::lan_pairing::LanPairingRuntime;

#[path = "provider_routing/routing_state.rs"]
mod routing_state;

impl LanPairingRuntime {
    pub(crate) fn lan_ai_provider_busy(&self) -> bool {
        std::env::var(constants::lan_pairing::LAN_AI_PROVIDER_BUSY_ENV)
            .ok()
            .filter(|value| !value.is_empty())
            .as_deref()
            == Some(constants::value::TRUE)
    }

    pub(crate) fn lan_ai_provider_status_value(&self) -> LanPairingText {
        match self.lan_ai_provider_heartbeat_reachability() {
            Some(LanPairingDeviceReachability::Offline) => {
                return constants::value::LAN_AI_PROVIDER_STATUS_UNAVAILABLE
                    .to_string()
                    .into();
            }
            Some(LanPairingDeviceReachability::Stale) => {
                return constants::value::LAN_AI_PROVIDER_STATUS_DEGRADED
                    .to_string()
                    .into();
            }
            Some(LanPairingDeviceReachability::Online) | None => {}
        }

        if self.lan_ai_provider_busy()
            && self.device_role_read_model().lan_ai_provider_state
                == DeviceRuntimeAiProviderState::Available
        {
            return constants::value::LAN_AI_PROVIDER_STATUS_BUSY
                .to_string()
                .into();
        }

        match self.device_role_read_model().lan_ai_provider_state {
            DeviceRuntimeAiProviderState::Available
                if !self.lan_ai_provider_capabilities.is_empty() =>
            {
                constants::value::LAN_AI_PROVIDER_STATUS_AVAILABLE
                    .to_string()
                    .into()
            }
            DeviceRuntimeAiProviderState::Degraded => {
                constants::value::LAN_AI_PROVIDER_STATUS_DEGRADED
                    .to_string()
                    .into()
            }
            DeviceRuntimeAiProviderState::Available | DeviceRuntimeAiProviderState::Unavailable => {
                constants::value::LAN_AI_PROVIDER_STATUS_UNAVAILABLE
                    .to_string()
                    .into()
            }
        }
    }
}
