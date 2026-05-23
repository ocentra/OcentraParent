use std::{
    collections::BTreeSet,
    fs::{read_to_string, write},
    io,
    path::Path,
};

use ocentra_parent_agent_protocol::{
    constants, LanPairingAuthenticationState, LanPairingDeviceReachability, LanPairingDeviceRef,
    LanPairingNetworkMode, LanPairingProof, LanPairingRejectionReason, LanPairingTrustState,
    LanParentIntentEnvelope, LanSelectedRouteTarget, LanTrustedDeviceRegistryEntry,
};

#[derive(Clone, Debug, Default)]
pub struct TrustedDeviceRegistry {
    entries: Vec<LanTrustedDeviceRegistryEntry>,
    accepted_intent_ids: BTreeSet<String>,
    selected_pairing_id: Option<String>,
}

impl TrustedDeviceRegistry {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn from_entries(entries: Vec<LanTrustedDeviceRegistryEntry>) -> Self {
        Self {
            entries,
            accepted_intent_ids: BTreeSet::new(),
            selected_pairing_id: None,
        }
    }

    pub fn load_json(path: &Path) -> Self {
        read_to_string(path)
            .ok()
            .and_then(|content| {
                serde_json::from_str::<Vec<LanTrustedDeviceRegistryEntry>>(&content).ok()
            })
            .map(Self::from_entries)
            .unwrap_or_default()
    }

    pub fn save_json(&self, path: &Path) -> io::Result<()> {
        let content = serde_json::to_string_pretty(&self.entries).map_err(io::Error::other)?;
        write(path, content)
    }

    pub fn entries(&self) -> &[LanTrustedDeviceRegistryEntry] {
        &self.entries
    }

    pub fn accept_pairing_proof(
        &mut self,
        proof: &LanPairingProof,
        child_device: LanPairingDeviceRef,
        parent_device: LanPairingDeviceRef,
        trusted_at: &str,
    ) -> LanTrustedDeviceRegistryEntry {
        let entry = LanTrustedDeviceRegistryEntry {
            schema_version: proof.schema_version,
            pairing_id: proof.pairing_id.clone(),
            child_device,
            parent_device,
            route_id: proof.route_id.clone(),
            origin: proof.origin.clone(),
            proof_digest: proof.proof_digest.clone(),
            trust_state: LanPairingTrustState::Paired,
            trusted_at: trusted_at.to_string(),
            expires_at: proof.expires_at.clone(),
            revoked_at: None,
        };
        self.entries
            .retain(|candidate| candidate.pairing_id != entry.pairing_id);
        self.entries.push(entry.clone());
        entry
    }

    pub fn revoke_pairing(&mut self, pairing_id: &str, revoked_at: &str) -> bool {
        if let Some(entry) = self
            .entries
            .iter_mut()
            .find(|candidate| candidate.pairing_id == pairing_id)
        {
            entry.trust_state = LanPairingTrustState::Revoked;
            entry.revoked_at = Some(revoked_at.to_string());
            if self.selected_pairing_id.as_deref() == Some(pairing_id) {
                self.selected_pairing_id = None;
            }
            return true;
        }
        false
    }

    pub fn select_pairing(
        &mut self,
        pairing_id: &str,
        target_child_device_id: &str,
        route_id: &str,
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
        self.selected_target()
            .ok_or(LanPairingRejectionReason::UnselectedDevice)
    }

    pub fn selected_target(&self) -> Option<LanSelectedRouteTarget> {
        self.selected_entry().map(|entry| LanSelectedRouteTarget {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            selected_child_device_id: entry.child_device.device_id.clone(),
            route_id: entry.route_id.clone(),
            pairing_id: Some(entry.pairing_id.clone()),
            network_mode: LanPairingNetworkMode::LocalNetwork,
            reachability: LanPairingDeviceReachability::Online,
            stale_at: None,
        })
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

    pub fn validate_intent(
        &mut self,
        intent: &LanParentIntentEnvelope,
        origin: Option<&str>,
        observed_at: &str,
    ) -> Result<(), LanPairingRejectionReason> {
        self.validate_intent_with_selection_requirement(intent, origin, observed_at, true)
    }

    pub fn validate_selection_intent(
        &mut self,
        intent: &LanParentIntentEnvelope,
        origin: Option<&str>,
        observed_at: &str,
    ) -> Result<(), LanPairingRejectionReason> {
        self.validate_intent_with_selection_requirement(intent, origin, observed_at, false)
    }

    fn validate_intent_with_selection_requirement(
        &mut self,
        intent: &LanParentIntentEnvelope,
        origin: Option<&str>,
        observed_at: &str,
        require_selected_pairing: bool,
    ) -> Result<(), LanPairingRejectionReason> {
        if intent.pairing_id.is_empty() || intent.proof_digest.is_empty() {
            return Err(LanPairingRejectionReason::Anonymous);
        }
        if intent.intent_id.is_empty() || intent.route_id.is_empty() {
            return Err(LanPairingRejectionReason::Malformed);
        }
        if self.accepted_intent_ids.contains(&intent.intent_id) {
            return Err(LanPairingRejectionReason::Replayed);
        }

        let entry = self
            .entries
            .iter()
            .find(|candidate| candidate.pairing_id == intent.pairing_id)
            .ok_or(LanPairingRejectionReason::Anonymous)?;

        if entry.trust_state == LanPairingTrustState::Revoked || entry.revoked_at.is_some() {
            return Err(LanPairingRejectionReason::Revoked);
        }
        if origin != Some(entry.origin.as_str()) {
            return Err(LanPairingRejectionReason::WrongOrigin);
        }
        if intent.target_child_device_id.as_str() != entry.child_device.device_id.as_str() {
            return Err(LanPairingRejectionReason::WrongDevice);
        }
        if intent.route_id.as_str() != entry.route_id.as_str() {
            return Err(LanPairingRejectionReason::UnsupportedRoute);
        }
        if intent.proof_digest.as_str() != entry.proof_digest.as_str() {
            return Err(LanPairingRejectionReason::Malformed);
        }
        if require_selected_pairing
            && self.selected_pairing_id.as_deref() != Some(entry.pairing_id.as_str())
        {
            return Err(LanPairingRejectionReason::UnselectedDevice);
        }
        if observed_at > entry.expires_at.as_str() {
            return Err(LanPairingRejectionReason::Expired);
        }
        if observed_at > intent.expires_at.as_str() {
            return Err(LanPairingRejectionReason::Stale);
        }

        self.accepted_intent_ids.insert(intent.intent_id.clone());
        Ok(())
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
