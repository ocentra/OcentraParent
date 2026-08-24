use std::{
    fs::{create_dir_all, read_to_string, OpenOptions},
    io,
    path::{Path, PathBuf},
};

use atomicwrites::{AllowOverwrite, AtomicFile};
use fs2::FileExt;
use ocentra_parent_agent_protocol::constants;
use serde_json::{json, Value};

use super::TrustedDeviceRegistry;

mod load;

const SIGNER_ANCHORS_KEY: &str = "signerAnchors";
const SIGNER_ANCHOR_GENERATIONS_KEY: &str = "signerAnchorGenerations";
const CONTROLLER_LEASE_KEY: &str = "controllerLease";
pub(super) const ACCEPTED_INTENT_IDS_KEY: &str = "acceptedIntentIds";
pub(super) const ACCEPTED_CHALLENGE_IDS_KEY: &str = "acceptedChallengeIds";

impl TrustedDeviceRegistry {
    pub fn load_json(path: &Path) -> Self {
        read_to_string(path)
            .ok()
            .and_then(|content| load::from_json_text(&content))
            .unwrap_or_default()
    }

    pub fn load_json_strict(path: &Path) -> io::Result<Self> {
        let content = read_to_string(path)?;
        let value = serde_json::from_str::<Value>(&content)
            .map_err(|_error| io::Error::from(io::ErrorKind::InvalidData))?;
        let mut registry = load::from_json_text(&content)
            .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;
        load::reject_untrusted_signer_anchors(&value)?;
        registry.signer_anchors.clear();
        if let Some(generations) = value.get(SIGNER_ANCHOR_GENERATIONS_KEY) {
            registry.signer_anchor_generations = serde_json::from_value(generations.clone())
                .map_err(|_error| io::Error::from(io::ErrorKind::InvalidData))?;
        }
        registry
            .validate_persisted_authority_state()
            .map_err(|_error| io::Error::from(io::ErrorKind::InvalidData))?;
        registry
            .validate_controller_lease_state()
            .map_err(|_error| io::Error::from(io::ErrorKind::InvalidData))?;
        load::reject_untrusted_paired_entries(&registry)?;
        Ok(registry)
    }

    /// Load a strict registry, creating the canonical empty file only for a
    /// genuinely missing first-run path. A lock is held across the existence
    /// check and atomic write so cooperating runtimes cannot both initialize
    /// the same registry. Malformed or unreadable files are never reset.
    pub fn load_or_initialize_json_strict(path: &Path) -> io::Result<Self> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                create_dir_all(parent)?;
            }
        }
        let lock_file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(registry_lock_path(path))?;
        FileExt::lock_exclusive(&lock_file)?;

        match Self::load_json_strict(path) {
            Ok(registry) => Ok(registry),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                Self::empty().save_json(path)?;
                Self::load_json_strict(path)
            }
            Err(error) => Err(error),
        }
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

    pub(super) fn to_json_value(&self) -> Value {
        json!({
            constants::field::SCHEMA_VERSION: 1,
            constants::field::ENTRIES: self.entries,
            constants::lan_pairing::REGISTRY_KEY_HOUSEHOLD_DEVICE_DECISIONS: &self.household_device_decisions,
            constants::lan_pairing::REGISTRY_KEY_KNOWN_HOUSEHOLD_DEVICES: &self.known_household_devices,
            SIGNER_ANCHORS_KEY: &self.signer_anchors,
            SIGNER_ANCHOR_GENERATIONS_KEY: &self.signer_anchor_generations,
            CONTROLLER_LEASE_KEY: &self.controller_lease,
            ACCEPTED_INTENT_IDS_KEY: &self.accepted_intent_ids,
            ACCEPTED_CHALLENGE_IDS_KEY: &self.accepted_challenge_ids,
            constants::field::LAN_SELECTED_PAIRING_ID: self.selected_pairing_id,
            constants::field::LAN_SELECTED_ROUTE_STALE_AT: self.selected_route_stale_at,
            constants::field::LAN_SELECTED_ROUTE_OFFLINE_AT: self.selected_route_offline_at,
        })
    }
}

fn registry_lock_path(registry_path: &Path) -> PathBuf {
    let mut lock_path = registry_path.as_os_str().to_os_string();
    lock_path.push(".lock");
    PathBuf::from(lock_path)
}
