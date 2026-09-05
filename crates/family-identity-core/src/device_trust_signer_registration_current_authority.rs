use crate::device_trust_current_binding::{
    CurrentChildDeviceTrustBinding, CurrentChildDeviceTrustBindingInput,
};

use super::{CurrentSignerAuthority, DeviceTrustLifecycleState};

impl CurrentSignerAuthority {
    /// Consume the current durable signer snapshot into the only child-runtime
    /// trust binding shape. The binding cannot be constructed from identity
    /// strings or deserialized wire data by downstream callers.
    pub fn into_current_child_device_trust_binding(self) -> CurrentChildDeviceTrustBinding {
        CurrentChildDeviceTrustBinding::from_current_signer_authority(
            CurrentChildDeviceTrustBindingInput {
                family_id: self.family_id,
                trust_subject: self.trust_subject,
                parent_device_id: self.parent_device_id,
                child_device_id: self.child_device_id,
                installation_id: self.installation_id,
                signer_key_id: self.signer_key_id,
                signer_key_sha256: self.signer_key_sha256,
                lifecycle_generation: self.lifecycle_generation,
                installation_binding_generation: self.installation_binding_generation,
                authority_generation: self.authority_generation,
                state: self.state,
            },
        )
    }
}

impl CurrentSignerAuthority {
    pub fn state(&self) -> DeviceTrustLifecycleState {
        self.state
    }

    pub fn family_id(&self) -> &str {
        &self.family_id
    }

    pub fn trust_subject(&self) -> &str {
        &self.trust_subject
    }

    pub fn parent_device_id(&self) -> &str {
        &self.parent_device_id
    }

    pub fn child_device_id(&self) -> &str {
        &self.child_device_id
    }

    pub fn installation_id(&self) -> &str {
        &self.installation_id
    }

    pub fn signer_public_key(&self) -> &[u8; 32] {
        &self.signer_public_key
    }

    pub fn signer_key_id(&self) -> &str {
        &self.signer_key_id
    }

    pub fn signer_key_sha256(&self) -> &str {
        &self.signer_key_sha256
    }

    pub fn lifecycle_generation(&self) -> u64 {
        self.lifecycle_generation
    }

    pub fn installation_binding_generation(&self) -> u64 {
        self.installation_binding_generation
    }

    pub fn authority_generation(&self) -> u64 {
        self.authority_generation
    }

    pub fn credential_id(&self) -> &str {
        &self.credential_id
    }

    pub fn credential_algorithm(&self) -> i32 {
        self.credential_algorithm
    }

    pub fn credential_sign_count(&self) -> u32 {
        self.credential_sign_count
    }
}
