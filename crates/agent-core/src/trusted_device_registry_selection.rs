use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingAuthenticationState, LanPairingDeviceReachability, LanPairingNetworkMode,
    LanPairingRejectionReason, LanPairingTrustState, LanSelectedRouteTarget,
    LanTrustedDeviceRegistryEntry,
};
use std::cmp::Ordering;

use crate::TrustedDeviceRegistry;

mod reachability;

impl TrustedDeviceRegistry {
    pub fn select_pairing(
        &mut self,
        pairing_id: &str,
        target_child_device_id: &str,
        route_id: &str,
        stale_at: &str,
    ) -> Result<LanSelectedRouteTarget, LanPairingRejectionReason> {
        let selected = self.validate_selection_candidate(
            pairing_id,
            target_child_device_id,
            route_id,
            stale_at,
        )?;

        self.selected_pairing_id = selected.pairing_id.clone();
        self.selected_route_stale_at = Some(stale_at.to_string());
        self.selected_route_offline_at = None;
        Ok(selected)
    }

    pub fn selected_target(&self) -> Option<LanSelectedRouteTarget> {
        let observed_at = chrono::Utc::now().to_rfc3339();
        self.selected_target_at(&observed_at)
    }

    pub fn selected_target_at(&self, observed_at: &str) -> Option<LanSelectedRouteTarget> {
        self.selected_entry_at(observed_at)?;
        Some(self.selected_target_with_reachability(self.selected_reachability_at(observed_at))?)
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

    fn selected_entry_at(&self, observed_at: &str) -> Option<&LanTrustedDeviceRegistryEntry> {
        self.selected_entry().filter(|entry| {
            rfc3339_cmp(observed_at, entry.expires_at.as_str())
                .is_some_and(|ordering| ordering != Ordering::Greater)
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

    fn validate_selection_candidate(
        &self,
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
        if entry.trust_state != LanPairingTrustState::Paired {
            return Err(LanPairingRejectionReason::UnselectedDevice);
        }
        if target_child_device_id != entry.child_device.device_id.as_str() {
            return Err(LanPairingRejectionReason::WrongDevice);
        }
        if route_id != entry.route_id.as_str() {
            return Err(LanPairingRejectionReason::UnsupportedRoute);
        }
        let observed_at = chrono::Utc::now().to_rfc3339();
        let current_expiry_order = rfc3339_cmp(&observed_at, entry.expires_at.as_str())
            .ok_or(LanPairingRejectionReason::Malformed)?;
        if current_expiry_order == Ordering::Greater {
            return Err(LanPairingRejectionReason::Expired);
        }
        let stale_expiry_order = rfc3339_cmp(stale_at, entry.expires_at.as_str())
            .ok_or(LanPairingRejectionReason::Malformed)?;
        if stale_expiry_order == Ordering::Greater {
            return Err(LanPairingRejectionReason::Stale);
        }

        Ok(LanSelectedRouteTarget {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            selected_child_device_id: entry.child_device.device_id.clone(),
            route_id: entry.route_id.clone(),
            pairing_id: Some(entry.pairing_id.clone()),
            trust_state: entry.trust_state,
            network_mode: LanPairingNetworkMode::LocalNetwork,
            reachability: LanPairingDeviceReachability::Online,
            stale_at: Some(stale_at.to_string()),
            offline_at: None,
        })
    }
}

pub(crate) fn rfc3339_cmp(left: &str, right: &str) -> Option<Ordering> {
    let left = chrono::DateTime::parse_from_rfc3339(left).ok()?;
    let right = chrono::DateTime::parse_from_rfc3339(right).ok()?;
    Some(
        left.with_timezone(&chrono::Utc)
            .cmp(&right.with_timezone(&chrono::Utc)),
    )
}

fn is_revoked_entry(entry: &LanTrustedDeviceRegistryEntry) -> bool {
    entry.trust_state == LanPairingTrustState::Revoked || entry.revoked_at.is_some()
}
