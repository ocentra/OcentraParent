use std::collections::{BTreeMap, BTreeSet};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingDeviceRef, LanPairingProof, LanPairingRejectionReason,
    LanPairingTrustState, LanParentIntentEnvelope, LanTrustedDeviceRegistryEntry,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanHouseholdDeviceDecision,
};
mod current_authority_validation;
mod helpers;
mod json_persistence;
mod known_household_devices;
mod persistence;
mod signer_authority;
pub mod signer_authority_types;
mod validation;
use self::helpers::{
    household_scan_truth_device, merge_known_household_device_by_canonical_id,
    push_unique_scan_truth_device,
};
use self::known_household_devices::restore_known_household_device;
use self::signer_authority_types::LanTrustedDeviceSignerAnchor;

#[derive(Debug, Default)]
pub struct TrustedDeviceRegistry {
    pub(crate) entries: Vec<LanTrustedDeviceRegistryEntry>,
    pub(crate) household_device_decisions: Vec<LanHouseholdDeviceDecision>,
    pub(crate) known_household_devices: Vec<LanCanonicalHouseholdDevice>,
    accepted_intent_ids: BTreeSet<String>,
    accepted_challenge_ids: BTreeSet<String>,
    signer_anchors: BTreeMap<String, LanTrustedDeviceSignerAnchor>,
    signer_anchor_generations: BTreeMap<String, u64>,
    pub(crate) selected_pairing_id: Option<String>,
    pub(crate) selected_route_stale_at: Option<String>,
    pub(crate) selected_route_offline_at: Option<String>,
}

impl TrustedDeviceRegistry {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn from_entries(entries: Vec<LanTrustedDeviceRegistryEntry>) -> Self {
        Self {
            entries,
            household_device_decisions: Vec::new(),
            known_household_devices: Vec::new(),
            accepted_intent_ids: BTreeSet::new(),
            accepted_challenge_ids: BTreeSet::new(),
            signer_anchors: BTreeMap::new(),
            signer_anchor_generations: BTreeMap::new(),
            selected_pairing_id: None,
            selected_route_stale_at: None,
            selected_route_offline_at: None,
        }
    }

    pub fn entries(&self) -> &[LanTrustedDeviceRegistryEntry] {
        &self.entries
    }

    pub fn household_device_decisions(&self) -> &[LanHouseholdDeviceDecision] {
        &self.household_device_decisions
    }

    pub fn has_household_device_decision(&self, action_id: &str) -> bool {
        !action_id.is_empty()
            && self
                .household_device_decisions
                .iter()
                .any(|decision| decision.action_id == action_id)
    }

    pub fn known_household_devices(&self) -> &[LanCanonicalHouseholdDevice] {
        &self.known_household_devices
    }

    pub fn record_challenge_request(&mut self, challenge_id: &str) -> bool {
        if challenge_id.trim().is_empty() || self.accepted_challenge_ids.contains(challenge_id) {
            return false;
        }
        if self.accepted_challenge_ids.len()
            >= constants::lan_pairing::LAN_PAIRING_MAX_ACCEPTED_INTENT_HISTORY
        {
            if let Some(oldest) = self.accepted_challenge_ids.iter().next().cloned() {
                self.accepted_challenge_ids.remove(&oldest);
            }
        }
        self.accepted_challenge_ids.insert(challenge_id.to_string());
        true
    }

    pub fn scan_truth_devices(&self) -> Vec<LanPairingDeviceRef> {
        let mut devices = self
            .entries
            .iter()
            .filter(|entry| {
                entry.trust_state == LanPairingTrustState::Paired && entry.revoked_at.is_none()
            })
            .map(|entry| entry.child_device.clone())
            .collect::<Vec<_>>();

        for device in &self.known_household_devices {
            if let Some(truth_device) = household_scan_truth_device(device) {
                push_unique_scan_truth_device(&mut devices, truth_device);
            }
        }

        devices
    }

    pub fn apply_household_device_decision(
        &mut self,
        decision: LanHouseholdDeviceDecision,
    ) -> bool {
        self.household_device_decisions
            .retain(|candidate| candidate.action_id != decision.action_id);
        if self.household_device_decisions.len()
            >= constants::lan_pairing::LAN_PAIRING_MAX_HOUSEHOLD_DECISION_HISTORY
        {
            let remove_count = self.household_device_decisions.len()
                - constants::lan_pairing::LAN_PAIRING_MAX_HOUSEHOLD_DECISION_HISTORY
                + 1;
            self.household_device_decisions.drain(..remove_count);
        }
        self.household_device_decisions.push(decision);
        true
    }

    pub fn merge_known_household_devices(
        &mut self,
        devices: Vec<LanCanonicalHouseholdDevice>,
    ) -> bool {
        let mut changed = false;
        for device in devices {
            changed |= merge_known_household_device_by_canonical_id(
                &mut self.known_household_devices,
                device,
            );
        }
        changed
    }

    pub fn known_household_devices_for_read_model(
        &self,
        current_devices: &[LanCanonicalHouseholdDevice],
        observed_at: &str,
    ) -> Vec<LanCanonicalHouseholdDevice> {
        let mut merged = self
            .known_household_devices
            .iter()
            .cloned()
            .map(|device| restore_known_household_device(device, observed_at))
            .collect::<Vec<_>>();
        for device in &mut merged {
            if device.trust_state != LanPairingTrustState::Paired
                && device.trust_state != LanPairingTrustState::Revoked
                && device.network_identity.reachability != LanPairingDeviceReachability::Offline
            {
                device.network_identity.stale_at = Some(observed_at.to_string());
            }
        }
        for device in current_devices {
            let _ = merge_known_household_device_by_canonical_id(&mut merged, device.clone());
        }
        merged
    }

    pub fn accept_pairing_proof(
        &mut self,
        proof: &LanPairingProof,
        child_device: LanPairingDeviceRef,
        parent_device: LanPairingDeviceRef,
        trusted_at: &str,
    ) -> LanTrustedDeviceRegistryEntry {
        self.signer_anchors.remove(&proof.pairing_id);
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
                self.selected_route_stale_at = None;
                self.selected_route_offline_at = None;
            }
            self.signer_anchors.remove(pairing_id);
            return true;
        }
        false
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
}
