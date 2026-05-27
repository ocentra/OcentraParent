use ocentra_parent_agent_protocol::{constants, DeviceRuntimeAiProviderState};

use crate::lan_pairing::LanPairingRuntime;

impl LanPairingRuntime {
    pub(crate) fn lan_ai_provider_busy(&self) -> bool {
        std::env::var(constants::lan_pairing::LAN_AI_PROVIDER_BUSY_ENV)
            .ok()
            .filter(|value| !value.is_empty())
            .as_deref()
            == Some(constants::value::TRUE)
    }

    pub(crate) fn lan_ai_provider_status_value(&self) -> &'static str {
        if self.lan_ai_provider_busy()
            && self.device_role_read_model().lan_ai_provider_state
                == DeviceRuntimeAiProviderState::Available
        {
            return constants::value::LAN_AI_PROVIDER_STATUS_BUSY;
        }

        match self.device_role_read_model().lan_ai_provider_state {
            DeviceRuntimeAiProviderState::Available
                if !self.lan_ai_provider_capabilities.is_empty() =>
            {
                constants::value::LAN_AI_PROVIDER_STATUS_AVAILABLE
            }
            DeviceRuntimeAiProviderState::Degraded => {
                constants::value::LAN_AI_PROVIDER_STATUS_DEGRADED
            }
            DeviceRuntimeAiProviderState::Available | DeviceRuntimeAiProviderState::Unavailable => {
                constants::value::LAN_AI_PROVIDER_STATUS_UNAVAILABLE
            }
        }
    }

    pub(crate) fn lan_ai_provider_routing_state(&self) -> &'static str {
        match self.lan_ai_provider_status_value() {
            constants::value::LAN_AI_PROVIDER_STATUS_AVAILABLE => {
                constants::value::LAN_AI_PROVIDER_ROUTING_AUTHORIZED_RESULT
            }
            constants::value::LAN_AI_PROVIDER_STATUS_BUSY => {
                constants::value::LAN_AI_PROVIDER_ROUTING_BUSY
            }
            constants::value::LAN_AI_PROVIDER_STATUS_DEGRADED => {
                constants::value::LAN_AI_PROVIDER_ROUTING_DEGRADED
            }
            _ => constants::value::LAN_AI_PROVIDER_ROUTING_UNAVAILABLE,
        }
    }
}
