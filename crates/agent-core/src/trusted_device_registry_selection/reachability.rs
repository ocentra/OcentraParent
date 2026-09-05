use std::cmp::Ordering;

use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use super::{rfc3339_cmp, TrustedDeviceRegistry};

impl TrustedDeviceRegistry {
    pub fn clear_selected_route_reachability(&mut self) -> bool {
        if self.selected_pairing_id.is_none() {
            return false;
        }
        let changed =
            self.selected_route_stale_at.is_some() || self.selected_route_offline_at.is_some();
        self.selected_route_stale_at = None;
        self.selected_route_offline_at = None;
        changed
    }

    pub fn mark_selected_offline(&mut self, offline_at: &str) -> bool {
        if self.selected_entry().is_none()
            || chrono::DateTime::parse_from_rfc3339(offline_at).is_err()
        {
            return false;
        }
        self.selected_route_offline_at = Some(offline_at.to_string());
        true
    }

    pub fn mark_selected_stale(&mut self, stale_at: &str) -> bool {
        if self.selected_entry().is_none()
            || chrono::DateTime::parse_from_rfc3339(stale_at).is_err()
        {
            return false;
        }
        self.selected_route_stale_at = Some(stale_at.to_string());
        true
    }

    pub(crate) fn selected_reachability_at(
        &self,
        observed_at: &str,
    ) -> LanPairingDeviceReachability {
        if self.selected_route_offline_at.is_some() {
            return LanPairingDeviceReachability::Offline;
        }
        let Some(stale_at) = self.selected_route_stale_at.as_deref() else {
            return LanPairingDeviceReachability::Online;
        };
        match rfc3339_cmp(observed_at, stale_at) {
            Some(Ordering::Greater) => LanPairingDeviceReachability::Stale,
            Some(_) => LanPairingDeviceReachability::Online,
            None => LanPairingDeviceReachability::Offline,
        }
    }
}
