use std::path::Path;

use super::platform;
use super::validation;
use super::{digest_path, journal_identity, reject_unsafe_shape, PathSecurityError};
use crate::platform::identity::PhysicalDatabaseIdentity;

pub(crate) fn revalidate(
    canonical: &Path,
    expected: &PhysicalDatabaseIdentity,
) -> Result<(), PathSecurityError> {
    reject_unsafe_shape(canonical)?;
    validation::components(canonical)?;
    validation::metadata(canonical)?;
    let parent = canonical.parent().ok_or(PathSecurityError::UnsafePath)?;
    let (_file_handle, file_digest) = platform::open_guarded(canonical, false)?;
    let (_parent_handle, _) = platform::open_guarded(parent, true)?;
    if file_digest.as_slice() != expected.physical_file_digest()
        || digest_path(canonical)?.as_slice() != expected.canonical_path_digest()
    {
        return Err(PathSecurityError::Replaced);
    }
    journal_identity::revalidate(canonical, expected.rollback_journal_digest())?;
    Ok(())
}
