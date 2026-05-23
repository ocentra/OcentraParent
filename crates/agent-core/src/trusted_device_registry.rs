use std::{
    collections::BTreeSet,
    fs::{read_to_string, write},
    io,
    path::Path,
};

use ocentra_parent_agent_protocol::{
    LanPairingDeviceRef, LanPairingProof, LanPairingRejectionReason, LanPairingTrustState,
    LanParentIntentEnvelope, LanTrustedDeviceRegistryEntry,
};

#[derive(Clone, Debug, Default)]
pub struct TrustedDeviceRegistry {
    entries: Vec<LanTrustedDeviceRegistryEntry>,
    accepted_intent_ids: BTreeSet<String>,
}

impl TrustedDeviceRegistry {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn from_entries(entries: Vec<LanTrustedDeviceRegistryEntry>) -> Self {
        Self {
            entries,
            accepted_intent_ids: BTreeSet::new(),
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
        if intent.target_child_device_id != entry.child_device.device_id {
            return Err(LanPairingRejectionReason::WrongDevice);
        }
        if intent.route_id != entry.route_id {
            return Err(LanPairingRejectionReason::UnsupportedRoute);
        }
        if intent.proof_digest != entry.proof_digest {
            return Err(LanPairingRejectionReason::Malformed);
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
