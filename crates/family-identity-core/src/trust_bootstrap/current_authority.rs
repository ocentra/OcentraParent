use serde::Serialize;

/// Fresh authority presented at the local platform-key boundary.
///
/// The identity fields are deliberately separate from the persisted credential:
/// the caller must obtain them again from current authority state before each
/// unseal, and they must match the household/subject/device recorded when the key was
/// sealed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CurrentParentDeviceTrustAuthorityError {
    NotTrusted,
    DeviceBindingMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CurrentParentDeviceTrustAuthority {
    pub lifecycle_generation: u64,
    pub installation_binding_generation: u64,
}

/// Runtime-owned authority resolver.  Unsealing never accepts a deserializable
/// caller DTO as proof of current lifecycle state: the product runtime must
/// resolve the current household/device record at the moment of unseal.
pub trait CurrentParentDeviceTrustAuthoritySource {
    fn current_authorized_parent_device(
        &self,
        family_id: &str,
        trust_subject: &str,
        device_ref: &str,
    ) -> Result<CurrentParentDeviceTrustAuthority, CurrentParentDeviceTrustAuthorityError>;
}

pub fn require_current_parent_device_trust_authority(
    source: &impl CurrentParentDeviceTrustAuthoritySource,
    sealed_family_id: &str,
    sealed_trust_subject: &str,
    sealed_device_ref: &str,
    sealed_lifecycle_generation: u64,
    sealed_installation_binding_generation: u64,
) -> Result<(), CurrentParentDeviceTrustAuthorityError> {
    [
        non_empty_identity(sealed_family_id),
        non_empty_identity(sealed_trust_subject),
        non_empty_identity(sealed_device_ref),
    ]
    .into_iter()
    .all(std::convert::identity)
    .then_some(())
    .ok_or(CurrentParentDeviceTrustAuthorityError::DeviceBindingMismatch)?;
    let current = source.current_authorized_parent_device(
        sealed_family_id,
        sealed_trust_subject,
        sealed_device_ref,
    )?;
    (current.lifecycle_generation == sealed_lifecycle_generation
        && current.installation_binding_generation == sealed_installation_binding_generation)
        .then_some(())
        .ok_or(CurrentParentDeviceTrustAuthorityError::DeviceBindingMismatch)
}

fn non_empty_identity(value: &str) -> bool {
    !value.trim().is_empty()
}
