use std::{
    collections::BTreeSet,
    fs::{read_to_string, write},
    io,
    path::Path,
};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingDeviceRef, LanPairingProof, LanPairingRejectionReason,
    LanPairingTrustState, LanParentIntentEnvelope, LanTrustedDeviceRegistryEntry,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanHouseholdDeviceDecision,
};
use serde_json::{json, Value};

mod helpers;
mod known_household_devices;
mod validation;
use self::helpers::{
    household_scan_truth_device, merge_known_household_device_by_canonical_id,
    push_unique_scan_truth_device,
};
use self::known_household_devices::{
    household_device_decisions_from_json, known_household_devices_from_json, optional_string,
    restore_known_household_device,
};

#[derive(Clone, Debug, Default)]
pub struct TrustedDeviceRegistry {
    pub(crate) entries: Vec<LanTrustedDeviceRegistryEntry>,
    pub(crate) household_device_decisions: Vec<LanHouseholdDeviceDecision>,
    pub(crate) known_household_devices: Vec<LanCanonicalHouseholdDevice>,
    accepted_intent_ids: BTreeSet<String>,
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
            selected_pairing_id: None,
            selected_route_stale_at: None,
            selected_route_offline_at: None,
        }
    }

    pub fn load_json(path: &Path) -> Self {
        read_to_string(path)
            .ok()
            .and_then(|content| Self::from_json_text(&content))
            .unwrap_or_default()
    }

    pub fn save_json(&self, path: &Path) -> io::Result<()> {
        let content =
            serde_json::to_string_pretty(&self.to_json_value()).map_err(io::Error::other)?;
        write(path, content)
    }

    pub fn entries(&self) -> &[LanTrustedDeviceRegistryEntry] {
        &self.entries
    }

    pub fn household_device_decisions(&self) -> &[LanHouseholdDeviceDecision] {
        &self.household_device_decisions
    }

    pub fn known_household_devices(&self) -> &[LanCanonicalHouseholdDevice] {
        &self.known_household_devices
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

    fn from_json_text(content: &str) -> Option<Self> {
        if let Ok(entries) = serde_json::from_str::<Vec<LanTrustedDeviceRegistryEntry>>(content) {
            return Some(Self::from_entries(entries));
        }

        let value = serde_json::from_str::<Value>(content).ok()?;
        let entries = serde_json::from_value::<Vec<LanTrustedDeviceRegistryEntry>>(
            value.get(constants::field::ENTRIES)?.clone(),
        )
        .ok()?;
        let mut registry = Self::from_entries(entries);
        registry.selected_pairing_id =
            optional_string(&value, constants::field::LAN_SELECTED_PAIRING_ID);
        registry.selected_route_stale_at =
            optional_string(&value, constants::field::LAN_SELECTED_ROUTE_STALE_AT);
        registry.selected_route_offline_at =
            optional_string(&value, constants::field::LAN_SELECTED_ROUTE_OFFLINE_AT);
        registry.household_device_decisions =
            household_device_decisions_from_json(&value).unwrap_or_default();
        registry.known_household_devices =
            known_household_devices_from_json(&value).unwrap_or_default();
        Some(registry)
    }

    fn to_json_value(&self) -> Value {
        json!({
            constants::field::SCHEMA_VERSION: 1,
            constants::field::ENTRIES: self.entries,
            constants::lan_pairing::REGISTRY_KEY_HOUSEHOLD_DEVICE_DECISIONS: &self.household_device_decisions,
            constants::lan_pairing::REGISTRY_KEY_KNOWN_HOUSEHOLD_DEVICES: &self.known_household_devices,
            constants::field::LAN_SELECTED_PAIRING_ID: self.selected_pairing_id,
            constants::field::LAN_SELECTED_ROUTE_STALE_AT: self.selected_route_stale_at,
            constants::field::LAN_SELECTED_ROUTE_OFFLINE_AT: self.selected_route_offline_at,
        })
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
