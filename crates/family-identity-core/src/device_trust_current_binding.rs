//! Current, family-owned trust binding for a child runtime.
//!
//! The binding is deliberately not serializable, cloneable, or constructible
//! from public identity strings.  A caller obtains one only by consuming a
//! `CurrentSignerAuthority` returned from the durable lifecycle/signer query.

use crate::device_trust_lifecycle::DeviceTrustLifecycleState;

#[derive(Debug, PartialEq, Eq)]
pub struct CurrentChildDeviceTrustBinding {
    family_id: String,
    trust_subject: String,
    parent_device_id: String,
    child_device_id: String,
    installation_id: String,
    signer_key_id: String,
    signer_key_sha256: String,
    lifecycle_generation: u64,
    installation_binding_generation: u64,
    authority_generation: u64,
    state: DeviceTrustLifecycleState,
}

impl CurrentChildDeviceTrustBinding {
    pub(crate) fn from_current_signer_authority(
        family_id: String,
        trust_subject: String,
        parent_device_id: String,
        child_device_id: String,
        installation_id: String,
        signer_key_id: String,
        signer_key_sha256: String,
        lifecycle_generation: u64,
        installation_binding_generation: u64,
        authority_generation: u64,
        state: DeviceTrustLifecycleState,
    ) -> Self {
        Self {
            family_id,
            trust_subject,
            parent_device_id,
            child_device_id,
            installation_id,
            signer_key_id,
            signer_key_sha256,
            lifecycle_generation,
            installation_binding_generation,
            authority_generation,
            state,
        }
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

    pub fn state(&self) -> DeviceTrustLifecycleState {
        self.state
    }
}
