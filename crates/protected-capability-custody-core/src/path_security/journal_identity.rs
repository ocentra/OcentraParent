use std::path::Path;

use super::journal;
use super::PathSecurityError;

pub(crate) fn revalidate(
    database: &Path,
    expected_digest: &[u8],
) -> Result<[u8; 32], PathSecurityError> {
    journal::reject_untracked_sidecars(database)?;
    let path = journal::sidecar(database, "-journal");
    super::validation::metadata(&path)?;
    let (_handle, digest) = super::platform::open_guarded(&path, false)?;
    if digest.as_slice() != expected_digest {
        return Err(PathSecurityError::Replaced);
    }
    let metadata = std::fs::metadata(&path).map_err(|_| PathSecurityError::Unavailable)?;
    if metadata.len() != 0 {
        return Err(PathSecurityError::UnsafePath);
    }
    Ok(digest)
}
