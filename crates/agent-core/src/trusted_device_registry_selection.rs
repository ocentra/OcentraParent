use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingAuthenticationState, LanPairingDeviceReachability, LanPairingNetworkMode,
    LanPairingRejectionReason, LanPairingTrustState, LanSelectedRouteTarget,
    LanTrustedDeviceRegistryEntry,
};

use crate::TrustedDeviceRegistry;

impl TrustedDeviceRegistry {
    pub fn select_pairing(
        &mut self,
        pairing_id: &str,
        target_child_device_id: &str,
        route_id: &str,
        stale_at: &str,
    ) -> Result<LanSelectedRouteTarget, LanPairingRejectionReason> {
        let entry = self
            .entries
            .iter()
            .find(|candidate| candidate.pairing_id == pairing_id)
            .ok_or(LanPairingRejectionReason::Anonymous)?;

        if entry.trust_state == LanPairingTrustState::Revoked || entry.revoked_at.is_some() {
            return Err(LanPairingRejectionReason::Revoked);
        }
        if target_child_device_id != entry.child_device.device_id.as_str() {
            return Err(LanPairingRejectionReason::WrongDevice);
        }
        if route_id != entry.route_id.as_str() {
            return Err(LanPairingRejectionReason::UnsupportedRoute);
        }

        self.selected_pairing_id = Some(entry.pairing_id.clone());
        self.selected_route_stale_at = Some(stale_at.to_string());
        self.selected_route_offline_at = None;
        self.selected_target_at(stale_at)
            .ok_or(LanPairingRejectionReason::UnselectedDevice)
    }

    pub fn selected_target(&self) -> Option<LanSelectedRouteTarget> {
        self.selected_target_with_reachability(LanPairingDeviceReachability::Online)
    }

    pub fn selected_target_at(&self, observed_at: &str) -> Option<LanSelectedRouteTarget> {
        self.selected_target_with_reachability(self.selected_reachability_at(observed_at))
    }

    pub fn mark_selected_offline(&mut self, offline_at: &str) -> bool {
        if self.selected_entry().is_none() {
            return false;
        }
        self.selected_route_offline_at = Some(offline_at.to_string());
        true
    }

    pub fn mark_selected_stale(&mut self, stale_at: &str) -> bool {
        if self.selected_entry().is_none() {
            return false;
        }
        self.selected_route_stale_at = Some(stale_at.to_string());
        true
    }

    pub fn trusted_device_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.trust_state == LanPairingTrustState::Paired)
            .count()
    }

    pub fn has_revoked_pairing(&self) -> bool {
        self.entries.iter().any(is_revoked_entry)
    }

    pub fn revoked_device_ids(&self) -> Vec<String> {
        self.entries
            .iter()
            .filter(|entry| is_revoked_entry(entry))
            .map(|entry| entry.child_device.device_id.clone())
            .collect()
    }

    pub fn authentication_state(&self) -> LanPairingAuthenticationState {
        if self.selected_entry().is_some() {
            LanPairingAuthenticationState::Paired
        } else {
            LanPairingAuthenticationState::Unpaired
        }
    }

    pub fn trusted_device_ids(&self) -> Vec<String> {
        self.entries
            .iter()
            .filter(|entry| entry.trust_state == LanPairingTrustState::Paired)
            .map(|entry| entry.child_device.device_id.clone())
            .collect()
    }

    pub(crate) fn selected_reachability_at(
        &self,
        observed_at: &str,
    ) -> LanPairingDeviceReachability {
        if self.selected_route_offline_at.is_some() {
            return LanPairingDeviceReachability::Offline;
        }
        if self
            .selected_route_stale_at
            .as_deref()
            .is_some_and(|stale_at| observed_at > stale_at)
        {
            return LanPairingDeviceReachability::Stale;
        }
        LanPairingDeviceReachability::Online
    }

    fn selected_target_with_reachability(
        &self,
        reachability: LanPairingDeviceReachability,
    ) -> Option<LanSelectedRouteTarget> {
        self.selected_entry().map(|entry| LanSelectedRouteTarget {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            selected_child_device_id: entry.child_device.device_id.clone(),
            route_id: entry.route_id.clone(),
            pairing_id: Some(entry.pairing_id.clone()),
            trust_state: entry.trust_state,
            network_mode: LanPairingNetworkMode::LocalNetwork,
            reachability,
            stale_at: self.selected_route_stale_at.clone(),
            offline_at: self.selected_route_offline_at.clone(),
        })
    }

    fn selected_entry(&self) -> Option<&LanTrustedDeviceRegistryEntry> {
        self.selected_pairing_id.as_deref().and_then(|pairing_id| {
            self.entries.iter().find(|candidate| {
                candidate.pairing_id == pairing_id
                    && candidate.trust_state == LanPairingTrustState::Paired
                    && candidate.revoked_at.is_none()
            })
        })
    }
}

fn is_revoked_entry(entry: &LanTrustedDeviceRegistryEntry) -> bool {
    entry.trust_state == LanPairingTrustState::Revoked || entry.revoked_at.is_some()
}
