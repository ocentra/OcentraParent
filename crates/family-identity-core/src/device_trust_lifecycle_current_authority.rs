use rusqlite::OptionalExtension;
use sha2::{Digest, Sha256};

use crate::{
    device_trust_lifecycle::DeviceTrustLifecycleRepository,
    device_trust_lifecycle_authority::hex_encode,
    trust_bootstrap::current_authority::{
        CurrentParentDeviceTrustAuthority, CurrentParentDeviceTrustAuthorityError,
        CurrentParentDeviceTrustAuthoritySource,
    },
};

pub(crate) fn redacted_binding(family_id: &str, trust_subject: &str, device_ref: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"ocentra-device-trust-lifecycle-binding-v2\0");
    hasher.update(family_id.as_bytes());
    hasher.update([0]);
    hasher.update(trust_subject.as_bytes());
    hasher.update([0]);
    hasher.update(device_ref.as_bytes());
    hex_encode(&hasher.finalize())
}

pub(crate) fn redacted_signer_binding(
    family_id: &str,
    trust_subject: &str,
    parent_device_id: &str,
    child_device_id: &str,
    installation_id: &str,
    signer_key_id: &str,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"ocentra-device-trust-signer-event-binding-v1\0");
    for identity in [
        family_id,
        trust_subject,
        parent_device_id,
        child_device_id,
        installation_id,
        signer_key_id,
    ] {
        hasher.update(identity.as_bytes());
        hasher.update([0]);
    }
    hex_encode(&hasher.finalize())
}

impl CurrentParentDeviceTrustAuthoritySource for DeviceTrustLifecycleRepository {
    fn current_authorized_parent_device(
        &self,
        family_id: &str,
        trust_subject: &str,
        device_ref: &str,
    ) -> Result<CurrentParentDeviceTrustAuthority, CurrentParentDeviceTrustAuthorityError> {
        let row = self
            .connection
            .query_row(
                "SELECT lifecycle_state, lifecycle_generation, installation_binding_generation, authority_generation FROM device_trust_lifecycle WHERE family_id = ?1 AND trust_subject = ?2 AND device_ref = ?3",
                rusqlite::params![family_id, trust_subject, device_ref],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, i64>(1)?,
                        row.get::<_, i64>(2)?,
                        row.get::<_, i64>(3)?,
                    ))
                },
            )
            .optional()
            .ok()
            .flatten();
        let Some((state, lifecycle_generation, installation_generation, authority_generation)) =
            row
        else {
            return Err(CurrentParentDeviceTrustAuthorityError::NotTrusted);
        };
        if state != "trusted" {
            return Err(CurrentParentDeviceTrustAuthorityError::NotTrusted);
        }
        let authority_generation = u64::try_from(authority_generation)
            .map_err(|_error| CurrentParentDeviceTrustAuthorityError::NotTrusted)?;
        if !self.external_authority.matches(
            family_id,
            trust_subject,
            device_ref,
            authority_generation,
        ) {
            return Err(CurrentParentDeviceTrustAuthorityError::NotTrusted);
        }
        Ok(CurrentParentDeviceTrustAuthority {
            lifecycle_generation: u64::try_from(lifecycle_generation)
                .map_err(|_error| CurrentParentDeviceTrustAuthorityError::NotTrusted)?,
            installation_binding_generation: u64::try_from(installation_generation)
                .map_err(|_error| CurrentParentDeviceTrustAuthorityError::NotTrusted)?,
        })
    }
}
