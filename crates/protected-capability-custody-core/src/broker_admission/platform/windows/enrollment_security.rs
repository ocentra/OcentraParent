use ocentra_protected_capability_custody_windows_ffi::{
    AceObservation, OwnedRegistryChain, SecurityDescriptorObservation,
};

use crate::platform::PlatformError;

use super::{map_ffi_error, ObservationDigest};

const REGISTRY_SECURITY_DOMAIN: &[u8] = b"ocentra.pcc.registry-chain.v1";
const SYSTEM_SID: &str = "S-1-5-18";
const TRUSTED_INSTALLER_SID: &str =
    "S-1-5-80-956008885-3418522649-1831038044-1853292631-2271478464";
const ACCESS_ALLOWED_ACE_TYPE: u8 = 0;
const KEY_READ: u32 = 0x0002_0019;
const KEY_ALL_ACCESS: u32 = 0x000f_003f;

pub(super) fn validate_leaf_security(chain: &OwnedRegistryChain) -> Result<(), PlatformError> {
    let observations = chain.observations().map_err(map_ffi_error)?;
    let leaf = observations.last().ok_or(PlatformError::Tampered)?;
    let security = leaf.security();
    let system_sid = sid_from_sddl(SYSTEM_SID)?;
    let installer_sid = sid_from_sddl(TRUSTED_INSTALLER_SID)?;
    let entries = security.dacl();
    if security.owner_sid() != installer_sid
        || security.owner_was_defaulted()
        || !security.dacl_is_present()
        || security.dacl_was_defaulted()
        || !security.dacl_is_protected()
        || entries.len() != 2
        || !exact_ace(&entries[0], &system_sid, KEY_READ)
        || !exact_ace(&entries[1], &installer_sid, KEY_ALL_ACCESS)
    {
        return Err(PlatformError::Tampered);
    }
    Ok(())
}

fn exact_ace(ace: &AceObservation, sid: &[u8], access_mask: u32) -> bool {
    ace.ace_type() == ACCESS_ALLOWED_ACE_TYPE
        && ace.flags() == 0
        && ace.access_mask() == access_mask
        && ace.sid() == sid
}

pub(super) fn registry_security_digest(
    chain: &OwnedRegistryChain,
) -> Result<[u8; 32], PlatformError> {
    let observations = chain.observations().map_err(map_ffi_error)?;
    let mut digest = ObservationDigest::new(REGISTRY_SECURITY_DOMAIN);
    digest.u32(u32::try_from(observations.len()).map_err(|_error| PlatformError::Tampered)?);
    for observation in observations {
        digest.text(observation.path().as_str());
        append_security(&mut digest, observation.security());
    }
    Ok(digest.finish())
}

pub(super) fn append_security(
    digest: &mut ObservationDigest,
    security: &SecurityDescriptorObservation,
) {
    digest.field(security.descriptor());
}

fn sid_from_sddl(value: &str) -> Result<Vec<u8>, PlatformError> {
    let mut parts = value.split('-');
    if parts.next() != Some("S") || parts.next() != Some("1") {
        return Err(PlatformError::Tampered);
    }
    let authority: u64 = parts
        .next()
        .ok_or(PlatformError::Tampered)?
        .parse()
        .map_err(|_error| PlatformError::Tampered)?;
    if authority > 0x0000_ffff_ffff_ffff {
        return Err(PlatformError::Tampered);
    }
    let sub_authorities: Vec<u32> = parts
        .map(|part| part.parse().map_err(|_error| PlatformError::Tampered))
        .collect::<Result<_, _>>()?;
    if sub_authorities.is_empty() || sub_authorities.len() > 15 {
        return Err(PlatformError::Tampered);
    }
    let mut sid = vec![1, sub_authorities.len() as u8];
    sid.extend_from_slice(&authority.to_be_bytes()[2..]);
    for sub_authority in sub_authorities {
        sid.extend_from_slice(&sub_authority.to_le_bytes());
    }
    Ok(sid)
}
