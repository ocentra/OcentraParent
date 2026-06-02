use std::{
    collections::BTreeSet,
    fs::{read_to_string, write},
    io,
    path::Path,
};

use ocentra_parent_agent_protocol::{
    constants, LanHouseholdDeviceDecision, LanPairingDeviceReachability, LanPairingDeviceRef,
    LanPairingProof, LanPairingRejectionReason, LanPairingTrustState, LanParentIntentEnvelope,
    LanTrustedDeviceRegistryEntry,
};
use serde_json::{json, Value};

#[derive(Clone, Debug, Default)]
pub struct TrustedDeviceRegistry {
    pub(crate) entries: Vec<LanTrustedDeviceRegistryEntry>,
    pub(crate) household_device_decisions: Vec<LanHouseholdDeviceDecision>,
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

    pub fn apply_household_device_decision(
        &mut self,
        decision: LanHouseholdDeviceDecision,
    ) -> bool {
        self.household_device_decisions
            .retain(|candidate| candidate.action_id != decision.action_id);
        self.household_device_decisions.push(decision);
        true
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
        Some(registry)
    }

    fn to_json_value(&self) -> Value {
        json!({
            constants::field::SCHEMA_VERSION: 1,
            constants::field::ENTRIES: self.entries,
            constants::lan_pairing::REGISTRY_KEY_HOUSEHOLD_DEVICE_DECISIONS: &self.household_device_decisions,
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
        if require_selected_pairing {
            match self.selected_reachability_at(observed_at) {
                LanPairingDeviceReachability::Offline => {
                    return Err(LanPairingRejectionReason::Offline);
                }
                LanPairingDeviceReachability::Stale => {
                    return Err(LanPairingRejectionReason::Stale);
                }
                LanPairingDeviceReachability::Online => {}
            }
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
}

fn household_device_decisions_from_json(value: &Value) -> Option<Vec<LanHouseholdDeviceDecision>> {
    value
        .get(constants::lan_pairing::REGISTRY_KEY_HOUSEHOLD_DEVICE_DECISIONS)
        .and_then(|decisions| serde_json::from_value(decisions.clone()).ok())
}

fn optional_string(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}
