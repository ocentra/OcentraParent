use std::cmp::Ordering;

use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingTrustState,
};

use super::{
    signer_authority::signer_anchor_binding::binding_is_well_formed,
    signer_authority_types::LanSignedChildAuthorityBindingRef, TrustedDeviceRegistry,
};
use crate::trusted_device_registry_selection::rfc3339_cmp;

impl TrustedDeviceRegistry {
    pub(crate) fn registered_authority_binding_is_current(
        &self,
        binding: &LanSignedChildAuthorityBindingRef<'_>,
        observed_at: &str,
    ) -> bool {
        if !binding_is_well_formed(binding) {
            return false;
        }
        let Some(entry) = self
            .entries
            .iter()
            .find(|entry| entry.pairing_id == binding.pairing_id)
        else {
            return false;
        };
        let Some(anchor) = self.signer_anchors.get(binding.pairing_id) else {
            return false;
        };
        entry.trust_state == LanPairingTrustState::Paired
            && entry.revoked_at.is_none()
            && rfc3339_cmp(observed_at, entry.expires_at.as_str())
                .is_some_and(|ordering| ordering == Ordering::Less)
            && self.selected_pairing_id.as_deref() == Some(binding.pairing_id)
            && self.selected_reachability_at(observed_at) == LanPairingDeviceReachability::Online
            && entry.child_device.device_id == binding.child_device_id
            && entry.child_device.device_id == binding.target_device_id
            && entry.child_device.install_id.as_deref() == Some(binding.install_id)
            && entry.parent_device.device_id == binding.parent_device_id
            && entry.route_id == binding.route_id
            && entry.proof_digest == binding.registry_proof_digest
            && self.signer_anchor_generations.get(binding.pairing_id)
                == Some(&binding.authority_generation)
            && anchor.authority_generation == binding.authority_generation
            && anchor.install_id == binding.install_id
            && anchor.family_hash == binding.family_hash
            && anchor.parent_device_id == binding.parent_device_id
            && anchor.public_key_id == binding.public_key_id
            && anchor.public_key_sha256 == binding.public_key_sha256
    }
}
