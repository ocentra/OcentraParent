use super::super::{
    LanHouseholdMeshIngressAuthorization, LanHouseholdMeshIngressAuthorizationScope,
    LanHouseholdMeshIngressCustodyError,
};
use crate::trusted_device_registry::{
    signer_authority_types::LanSignedChildAuthorityBindingRef, TrustedDeviceRegistry,
};

impl LanHouseholdMeshIngressAuthorization {
    pub fn consume_selected_event_republish(
        self,
        registry: &TrustedDeviceRegistry,
        observed_at: &str,
    ) -> Result<LanHouseholdMeshIngressAuthorizationScope, LanHouseholdMeshIngressCustodyError>
    {
        let observed_at = chrono::DateTime::parse_from_rfc3339(observed_at)
            .map_err(|_error| LanHouseholdMeshIngressCustodyError::AuthorityStale)?;
        let issued_at = chrono::DateTime::parse_from_rfc3339(self.issued_at.as_str())
            .map_err(|_error| LanHouseholdMeshIngressCustodyError::AuthorityStale)?;
        let expires_at = chrono::DateTime::parse_from_rfc3339(self.expires_at.as_str())
            .map_err(|_error| LanHouseholdMeshIngressCustodyError::AuthorityStale)?;
        let reserved_at = chrono::DateTime::parse_from_rfc3339(self.reserved_at.as_str())
            .map_err(|_error| LanHouseholdMeshIngressCustodyError::AuthorityStale)?;
        let binding = LanSignedChildAuthorityBindingRef {
            pairing_id: self.pairing_id.as_str(),
            child_device_id: self.child_device_id.as_str(),
            target_device_id: self.target_device_id.as_str(),
            install_id: self.install_id.as_str(),
            family_hash: self.family_hash.as_str(),
            parent_device_id: self.parent_device_id.as_str(),
            route_id: self.route_id.as_str(),
            registry_proof_digest: self.registry_proof_digest.as_str(),
            authority_generation: self.authority_generation,
            public_key_id: self.signer_public_key_id.as_str(),
            public_key_sha256: self.signer_public_key_sha256.as_str(),
        };
        if !registry.registered_authority_binding_is_current(&binding, &observed_at.to_rfc3339())
            || observed_at < issued_at
            || observed_at < reserved_at
            || reserved_at < issued_at
            || reserved_at >= expires_at
            || observed_at >= expires_at
        {
            return Err(LanHouseholdMeshIngressCustodyError::AuthorityStale);
        }
        Ok(self.scope)
    }
}
