use std::{collections::BTreeMap, fs::read_to_string, io, path::Path};

use atomicwrites::{AllowOverwrite, AtomicFile};
use ocentra_parent_agent_protocol::{constants, lan_pairing::LanTrustedDeviceRegistryEntry};
use serde_json::{json, Value};

use super::{
    known_household_devices::{
        household_device_decisions_from_json, known_household_devices_from_json, optional_string,
    },
    signer_authority_types::LanTrustedDeviceSignerAnchor,
    TrustedDeviceRegistry,
};

const SIGNER_ANCHORS_KEY: &str = "signerAnchors";
const SIGNER_ANCHOR_GENERATIONS_KEY: &str = "signerAnchorGenerations";

impl TrustedDeviceRegistry {
    pub fn load_json(path: &Path) -> Self {
        read_to_string(path)
            .ok()
            .and_then(|content| Self::from_json_text(&content))
            .unwrap_or_default()
    }

    pub fn load_json_strict(path: &Path) -> io::Result<Self> {
        let content = read_to_string(path)?;
        let value = serde_json::from_str::<Value>(&content)
            .map_err(|_error| io::Error::from(io::ErrorKind::InvalidData))?;
        let mut registry = Self::from_json_text(&content)
            .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;
        reject_untrusted_signer_anchors(&value)?;
        registry.signer_anchors.clear();
        if let Some(generations) = value.get(SIGNER_ANCHOR_GENERATIONS_KEY) {
            registry.signer_anchor_generations = serde_json::from_value(generations.clone())
                .map_err(|_error| io::Error::from(io::ErrorKind::InvalidData))?;
        }
        registry
            .validate_persisted_authority_state()
            .map_err(|_error| io::Error::from(io::ErrorKind::InvalidData))?;
        Ok(registry)
    }

    pub fn save_json(&self, path: &Path) -> io::Result<()> {
        let content =
            serde_json::to_string_pretty(&self.to_json_value()).map_err(io::Error::other)?;
        AtomicFile::new(path, AllowOverwrite)
            .write(|file| {
                use std::io::Write;

                file.write_all(content.as_bytes())?;
                file.sync_all()
            })
            .map_err(|error| io::Error::other(error.to_string()))
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
        registry.signer_anchor_generations = value
            .get(SIGNER_ANCHOR_GENERATIONS_KEY)
            .and_then(|generations| serde_json::from_value(generations.clone()).ok())
            .unwrap_or_default();
        Some(registry)
    }

    pub(super) fn to_json_value(&self) -> Value {
        json!({
            constants::field::SCHEMA_VERSION: 1,
            constants::field::ENTRIES: self.entries,
            constants::lan_pairing::REGISTRY_KEY_HOUSEHOLD_DEVICE_DECISIONS: &self.household_device_decisions,
            constants::lan_pairing::REGISTRY_KEY_KNOWN_HOUSEHOLD_DEVICES: &self.known_household_devices,
            SIGNER_ANCHORS_KEY: &self.signer_anchors,
            SIGNER_ANCHOR_GENERATIONS_KEY: &self.signer_anchor_generations,
            constants::field::LAN_SELECTED_PAIRING_ID: self.selected_pairing_id,
            constants::field::LAN_SELECTED_ROUTE_STALE_AT: self.selected_route_stale_at,
            constants::field::LAN_SELECTED_ROUTE_OFFLINE_AT: self.selected_route_offline_at,
        })
    }
}

fn reject_untrusted_signer_anchors(value: &Value) -> io::Result<()> {
    let Some(anchors) = value.get(SIGNER_ANCHORS_KEY) else {
        return Ok(());
    };
    let persisted =
        serde_json::from_value::<BTreeMap<String, LanTrustedDeviceSignerAnchor>>(anchors.clone())
            .map_err(|_error| io::Error::from(io::ErrorKind::InvalidData))?;
    if persisted.is_empty() {
        Ok(())
    } else {
        Err(io::Error::from(io::ErrorKind::InvalidData))
    }
}
