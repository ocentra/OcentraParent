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

    #[cfg(test)]
    pub fn mark_lan_ai_provider_heartbeat_stale_for_test(&self, observed_at: &str) {
        self.record_lan_ai_provider_heartbeat_state_for_test(
            observed_at,
            LanPairingDeviceReachability::Stale,
        );
    }

    #[cfg(test)]
    pub fn mark_lan_ai_provider_heartbeat_offline_for_test(&self, observed_at: &str) {
        self.record_lan_ai_provider_heartbeat_state_for_test(
            observed_at,
            LanPairingDeviceReachability::Offline,
        );
    }

    #[cfg(test)]
    fn record_lan_ai_provider_heartbeat_state_for_test(
        &self,
        observed_at: &str,
        reachability: LanPairingDeviceReachability,
    ) {
        if let Ok(mut state) = self.lan_ai_provider_heartbeat.lock() {
            *state = Some(LanAiProviderHeartbeatState {
                observed_at: observed_at.to_string(),
                reachability,
            });
        }
    }
}
