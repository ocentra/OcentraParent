use ocentra_protected_capability_custody_windows_ffi::RegistryAncestorObservation;
use sha2::{Digest, Sha256};

use super::error::ProvisioningError;

const REGISTRY_SECURITY_DOMAIN: &str = "ocentra.pcc.registry-chain.v1";

pub(super) fn security_digest(
    observations: &[RegistryAncestorObservation],
) -> Result<[u8; 32], ProvisioningError> {
    let mut digest = Sha256::new();
    digest.update((REGISTRY_SECURITY_DOMAIN.len() as u32).to_be_bytes());
    digest.update(REGISTRY_SECURITY_DOMAIN.as_bytes());
    let count =
        u32::try_from(observations.len()).map_err(|_| ProvisioningError::ExistingStateRejected)?;
    digest_field(&mut digest, &count.to_be_bytes());
    for observation in observations {
        digest_field(&mut digest, observation.path().as_str().as_bytes());
        digest_field(&mut digest, observation.security().descriptor());
    }
    Ok(digest.finalize().into())
}

fn digest_field(digest: &mut Sha256, value: &[u8]) {
    digest.update((value.len() as u32).to_be_bytes());
    digest.update(value);
}
