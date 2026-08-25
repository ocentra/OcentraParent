use chrono::DateTime;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;

use super::super::TrustedDeviceRegistry;
use super::signer_anchor_binding::validate_registry_authority_identifier;

impl TrustedDeviceRegistry {
    pub(crate) fn validate_persisted_authority_state(&self) -> Result<(), ()> {
        if !self.signer_anchors.is_empty() {
            return Err(());
        }
        for (pairing_id, generation) in &self.signer_anchor_generations {
            if !validate_registry_authority_identifier(pairing_id)
                || *generation == 0
                || !self
                    .entries
                    .iter()
                    .any(|entry| entry.pairing_id.as_str() == pairing_id.as_str())
            {
                return Err(());
            }
        }
        self.validate_persisted_selected_route()
    }

    fn validate_persisted_selected_route(&self) -> Result<(), ()> {
        let Some(pairing_id) = self.selected_pairing_id.as_deref() else {
            return if self.selected_route_stale_at.is_none()
                && self.selected_route_offline_at.is_none()
            {
                Ok(())
            } else {
                Err(())
            };
        };
        if !validate_registry_authority_identifier(pairing_id) {
            return Err(());
        }
        let entry = self
            .entries
            .iter()
            .find(|candidate| candidate.pairing_id == pairing_id)
            .ok_or(())?;
        if entry.trust_state != LanPairingTrustState::Paired
            || entry.revoked_at.is_some()
            || DateTime::parse_from_rfc3339(entry.expires_at.as_str()).is_err()
            || !valid_optional_timestamp(self.selected_route_stale_at.as_deref())
            || !valid_optional_timestamp(self.selected_route_offline_at.as_deref())
        {
            return Err(());
        }
        Ok(())
    }
}

fn valid_optional_timestamp(value: Option<&str>) -> bool {
    value.is_none_or(|timestamp| DateTime::parse_from_rfc3339(timestamp).is_ok())
}
