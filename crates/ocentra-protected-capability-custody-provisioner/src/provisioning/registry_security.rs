use ocentra_protected_capability_custody_windows_ffi::{
    AceObservation, RegistryAncestorObservation, SecurityDescriptorObservation,
};

use super::constants;
use super::error::ProvisioningError;

pub(super) fn validate_chain(
    observations: &[RegistryAncestorObservation],
) -> Result<(), ProvisioningError> {
    if observations.is_empty() {
        return Err(ProvisioningError::ExistingStateRejected);
    }
    let private_prefix = format!("{}\\", constants::REGISTRY_ROOT);
    let mut private_count = 0_usize;
    for observation in observations {
        let path = observation.path().as_str();
        if path == constants::REGISTRY_ROOT || path.starts_with(&private_prefix) {
            validate_security(observation.security())?;
            private_count = private_count.saturating_add(1);
        }
    }
    if private_count < 3 {
        return Err(ProvisioningError::ExistingStateRejected);
    }
    Ok(())
}

fn validate_security(security: &SecurityDescriptorObservation) -> Result<(), ProvisioningError> {
    let entries = security.dacl();
    if security.owner_sid() != constants::TRUSTED_INSTALLER_SID
        || security.owner_was_defaulted()
        || !security.dacl_is_present()
        || security.dacl_was_defaulted()
        || !security.dacl_is_protected()
        || entries.len() != 2
        || !exact_ace(&entries[0], constants::SYSTEM_SID, constants::KEY_READ)
        || !exact_ace(
            &entries[1],
            constants::TRUSTED_INSTALLER_SID,
            constants::KEY_ALL_ACCESS,
        )
    {
        return Err(ProvisioningError::ExistingStateRejected);
    }
    Ok(())
}

fn exact_ace(ace: &AceObservation, sid: &[u8], access_mask: u32) -> bool {
    ace.ace_type() == constants::ACCESS_ALLOWED_ACE_TYPE
        && ace.flags() == 0
        && ace.access_mask() == access_mask
        && ace.sid() == sid
        && !ace.raw().is_empty()
}
