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
    LanCanonicalHouseholdDevice, LanCanonicalHouseholdDeviceClassification,
    LanCanonicalHouseholdDeviceSource, LanHouseholdDeviceDecision,
};
use serde_json::{json, Value};

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
            changed |= upsert_known_household_device(&mut self.known_household_devices, device);
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
        for device in current_devices {
            let _ = upsert_known_household_device(&mut merged, device.clone());
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

fn known_household_devices_from_json(value: &Value) -> Option<Vec<LanCanonicalHouseholdDevice>> {
    value
        .get(constants::lan_pairing::REGISTRY_KEY_KNOWN_HOUSEHOLD_DEVICES)
        .and_then(|devices| serde_json::from_value(devices.clone()).ok())
}

fn optional_string(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn upsert_known_household_device(
    devices: &mut Vec<LanCanonicalHouseholdDevice>,
    incoming: LanCanonicalHouseholdDevice,
) -> bool {
    if let Some(existing) = devices
        .iter_mut()
        .find(|device| same_known_household_device(device, &incoming))
    {
        let before = existing.clone();
        merge_known_household_device(existing, incoming);
        return before != *existing;
    }

    devices.push(incoming);
    true
}

fn same_known_household_device(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
) -> bool {
    existing.canonical_device_id == incoming.canonical_device_id
        || existing
            .network_identity
            .mac_address
            .as_deref()
            .zip(incoming.network_identity.mac_address.as_deref())
            .map(|(left, right)| left.eq_ignore_ascii_case(right))
            .unwrap_or(false)
}

fn merge_known_household_device(
    existing: &mut LanCanonicalHouseholdDevice,
    incoming: LanCanonicalHouseholdDevice,
) {
    existing.display_name = preferred_display_name(&existing.display_name, &incoming.display_name);
    existing.classification = preferred_classification(
        existing.classification.clone(),
        incoming.classification.clone(),
    );
    existing.enrollable = existing.enrollable || incoming.enrollable;
    existing.discovery_state = incoming.discovery_state;
    existing.trust_state = incoming.trust_state;
    existing.route_id = incoming.route_id.or(existing.route_id.clone());
    existing.route_state = incoming.route_state;
    existing.network_mode = incoming.network_mode;
    merge_string_values(
        &mut existing.network_identity.ip_addresses,
        incoming.network_identity.ip_addresses,
    );
    merge_string_values(
        &mut existing.network_identity.network_interfaces,
        incoming.network_identity.network_interfaces,
    );
    existing.network_identity.hostname = incoming
        .network_identity
        .hostname
        .or(existing.network_identity.hostname.clone());
    existing.network_identity.mac_address = incoming
        .network_identity
        .mac_address
        .or(existing.network_identity.mac_address.clone());
    existing.network_identity.mac_vendor = incoming
        .network_identity
        .mac_vendor
        .or(existing.network_identity.mac_vendor.clone());
    existing.network_identity.reachability = incoming.network_identity.reachability;
    existing.network_identity.confidence = incoming.network_identity.confidence;
    existing.network_identity.stale_at = incoming
        .network_identity
        .stale_at
        .or(existing.network_identity.stale_at.clone());
    existing.network_identity.offline_at = incoming
        .network_identity
        .offline_at
        .or(existing.network_identity.offline_at.clone());
    merge_evidence_records(
        &mut existing.network_identity.evidence_records,
        incoming.network_identity.evidence_records,
    );
    merge_source_labels(&mut existing.source_labels, incoming.source_labels);
    merge_surfaces(
        &mut existing.policy_target_surfaces,
        incoming.policy_target_surfaces,
    );
    merge_roles(&mut existing.role_badges, incoming.role_badges);
    if incoming.child_agent_inventory.is_some() {
        existing.child_agent_inventory = incoming.child_agent_inventory;
    }
}

fn preferred_display_name(existing: &str, incoming: &str) -> String {
    if existing.starts_with(constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX)
        && !incoming.starts_with(constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX)
    {
        incoming.to_string()
    } else {
        existing.to_string()
    }
}

fn preferred_classification(
    existing: LanCanonicalHouseholdDeviceClassification,
    incoming: LanCanonicalHouseholdDeviceClassification,
) -> LanCanonicalHouseholdDeviceClassification {
    use LanCanonicalHouseholdDeviceClassification::{
        ChildAgent, NetworkInfrastructure, UnknownLanDevice, UnsupportedLanDevice,
    };

    match (existing, incoming) {
        (ChildAgent, _) | (_, ChildAgent) => ChildAgent,
        (NetworkInfrastructure, _) | (_, NetworkInfrastructure) => NetworkInfrastructure,
        (UnknownLanDevice, other) => other,
        (existing, UnsupportedLanDevice) => existing,
        (_, incoming) => incoming,
    }
}

fn merge_string_values(existing: &mut Vec<String>, incoming: Vec<String>) {
    for value in incoming {
        if !existing
            .iter()
            .any(|entry| entry.eq_ignore_ascii_case(&value))
        {
            existing.push(value);
        }
    }
}

fn merge_source_labels(
    existing: &mut Vec<LanCanonicalHouseholdDeviceSource>,
    incoming: Vec<LanCanonicalHouseholdDeviceSource>,
) {
    for source in incoming {
        if !existing.contains(&source) {
            existing.push(source);
        }
    }
}

fn merge_surfaces(
    existing: &mut Vec<ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdSurface>,
    incoming: Vec<ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdSurface>,
) {
    for surface in incoming {
        if !existing.contains(&surface) {
            existing.push(surface);
        }
    }
}

fn merge_roles(
    existing: &mut Vec<ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceRole>,
    incoming: Vec<ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceRole>,
) {
    for role in incoming {
        if !existing.contains(&role) {
            existing.push(role);
        }
    }
}

fn merge_evidence_records(
    existing: &mut Vec<ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceRecord>,
    incoming: Vec<ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceRecord>,
) {
    for record in incoming {
        if let Some(existing_record) = existing
            .iter_mut()
            .find(|entry| entry.merge_key.eq_ignore_ascii_case(&record.merge_key))
        {
            if record.first_seen_at < existing_record.first_seen_at {
                existing_record.first_seen_at = record.first_seen_at.clone();
            }
            if record.last_seen_at > existing_record.last_seen_at {
                existing_record.last_seen_at = record.last_seen_at.clone();
            }
            if record.note.is_some() {
                existing_record.note = record.note.clone();
            }
            existing_record.confidence = record.confidence.clone();
            if record.expires_at.is_some() {
                existing_record.expires_at = record.expires_at.clone();
            }
            continue;
        }
        existing.push(record);
    }
}

fn restore_known_household_device(
    mut device: LanCanonicalHouseholdDevice,
    observed_at: &str,
) -> LanCanonicalHouseholdDevice {
    if device.trust_state != LanPairingTrustState::Paired
        && device.trust_state != LanPairingTrustState::Revoked
    {
        device.discovery_state = match device.discovery_state {
            ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState::Revoked => {
                ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState::Revoked
            }
            _ => ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState::Stale,
        };
        if device.network_identity.reachability != LanPairingDeviceReachability::Offline {
            device.network_identity.reachability = LanPairingDeviceReachability::Stale;
            if device.network_identity.stale_at.is_none() {
                device.network_identity.stale_at = Some(observed_at.to_string());
            }
        }
    }
    device
}
