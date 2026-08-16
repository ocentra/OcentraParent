use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use crate::lan_pairing::LanPairingRuntime;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LanAiProviderHeartbeatState {
    pub(crate) observed_at: String,
    pub(crate) reachability: LanPairingDeviceReachability,
}

impl LanPairingRuntime {
    pub(crate) fn lan_ai_provider_heartbeat_reachability(
        &self,
    ) -> Option<LanPairingDeviceReachability> {
        self.lan_ai_provider_heartbeat
            .lock()
            .ok()
            .and_then(|state| state.as_ref().map(|state| state.reachability.clone()))
    }

    pub(crate) fn lan_ai_provider_heartbeat_allows_routing(&self) -> bool {
        !matches!(
            self.lan_ai_provider_heartbeat_reachability(),
            Some(LanPairingDeviceReachability::Offline | LanPairingDeviceReachability::Stale)
        )
    }
}
