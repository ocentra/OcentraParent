use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;

use crate::lan_pairing::LanPairingRuntime;

impl LanPairingRuntime {
    pub(crate) fn lan_ai_provider_routing_state(&self) -> LanPairingText {
        match self.lan_ai_provider_status_value().0.as_str() {
            constants::value::LAN_AI_PROVIDER_STATUS_AVAILABLE => {
                constants::value::LAN_AI_PROVIDER_ROUTING_AUTHORIZED_RESULT
                    .to_string()
                    .into()
            }
            constants::value::LAN_AI_PROVIDER_STATUS_BUSY => {
                constants::value::LAN_AI_PROVIDER_ROUTING_BUSY
                    .to_string()
                    .into()
            }
            constants::value::LAN_AI_PROVIDER_STATUS_DEGRADED => {
                constants::value::LAN_AI_PROVIDER_ROUTING_DEGRADED
                    .to_string()
                    .into()
            }
            _ => constants::value::LAN_AI_PROVIDER_ROUTING_UNAVAILABLE
                .to_string()
                .into(),
        }
    }
}
