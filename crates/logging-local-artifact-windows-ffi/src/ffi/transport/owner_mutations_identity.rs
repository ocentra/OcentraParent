use super::*;

pub(super) fn expected_file(
    file: &OwnedFile,
    expected: Option<&IdentityRecord>,
) -> Result<(), ArtifactError> {
    let current = verify_metadata(file, false)?.identity;
    match expected {
        Some(identity)
            if identity.volume_serial_number == current.volume_serial_number
                && identity.file_id == current.file_id =>
        {
            Ok(())
        }
        Some(_) => Err(ArtifactError::OwnershipChanged),
        None => Err(ArtifactError::OwnershipChanged),
    }
}

pub(super) fn expected_directory(
    file: &OwnedFile,
    expected: &IdentityRecord,
) -> Result<(), ArtifactError> {
    let current = verify_metadata(file, true)?.identity;
    if expected.volume_serial_number == current.volume_serial_number
        && expected.file_id == current.file_id
    {
        Ok(())
    } else {
        Err(ArtifactError::OwnershipChanged)
    }
}

pub(super) fn expected_new_file(
    file: &mut OwnedFile,
    expected_identity: &IdentityRecord,
    expected_digest: &str,
) -> Result<(), ArtifactError> {
    let metadata = verify_metadata(file, false)?;
    if identity_record(metadata.identity) != *expected_identity {
        return Err(ArtifactError::OwnershipChanged);
    }
    let bytes = file.read_bounded(MAX_ARTIFACT_BYTES)?;
    if payload_digest::<_, String>(bytes.as_slice()) != expected_digest {
        return Err(ArtifactError::RecoveryRequired);
    }
    Ok(())
}
