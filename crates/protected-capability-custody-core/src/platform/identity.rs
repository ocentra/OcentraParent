use super::PlatformError;

const PHYSICAL_DATABASE_IDENTITY_BYTES: usize = 96;
const DATABASE_IDENTITY_BYTES: usize = 128;

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct PhysicalDatabaseIdentity {
    canonical: [u8; PHYSICAL_DATABASE_IDENTITY_BYTES],
}

impl PhysicalDatabaseIdentity {
    pub(crate) fn from_parts(
        canonical_path_digest: [u8; 32],
        physical_file_digest: [u8; 32],
        rollback_journal_digest: [u8; 32],
    ) -> Result<Self, PlatformError> {
        let components = [
            canonical_path_digest,
            physical_file_digest,
            rollback_journal_digest,
        ];
        if components.contains(&[0_u8; 32]) {
            return Err(PlatformError::InvalidAttestation);
        }
        let mut canonical = [0_u8; PHYSICAL_DATABASE_IDENTITY_BYTES];
        canonical[..32].copy_from_slice(&canonical_path_digest);
        canonical[32..64].copy_from_slice(&physical_file_digest);
        canonical[64..].copy_from_slice(&rollback_journal_digest);
        Ok(Self { canonical })
    }

    pub(crate) fn as_bytes(&self) -> &[u8; PHYSICAL_DATABASE_IDENTITY_BYTES] {
        &self.canonical
    }

    pub(crate) fn canonical_path_digest(&self) -> &[u8] {
        &self.canonical[..32]
    }

    pub(crate) fn physical_file_digest(&self) -> &[u8] {
        &self.canonical[32..64]
    }

    pub(crate) fn rollback_journal_digest(&self) -> &[u8] {
        &self.canonical[64..]
    }

    pub(crate) fn from_bytes(value: &[u8]) -> Result<Self, PlatformError> {
        let canonical: [u8; PHYSICAL_DATABASE_IDENTITY_BYTES] =
            value.try_into().map_err(map_identity_length)?;
        if canonical[..32] == [0_u8; 32]
            || canonical[32..64] == [0_u8; 32]
            || canonical[64..] == [0_u8; 32]
        {
            return Err(PlatformError::InvalidAttestation);
        }
        Ok(Self { canonical })
    }
}

fn map_identity_length(_error: std::array::TryFromSliceError) -> PlatformError {
    PlatformError::InvalidAttestation
}

impl std::fmt::Debug for PhysicalDatabaseIdentity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PhysicalDatabaseIdentity")
            .field("opaque", &"<redacted>")
            .finish()
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct DatabaseIdentity {
    canonical: [u8; DATABASE_IDENTITY_BYTES],
}

impl DatabaseIdentity {
    pub(crate) fn as_bytes(&self) -> &[u8; DATABASE_IDENTITY_BYTES] {
        &self.canonical
    }

    pub(crate) fn from_parts(
        physical: PhysicalDatabaseIdentity,
        database_instance_id: [u8; 32],
    ) -> Result<Self, PlatformError> {
        if database_instance_id == [0_u8; 32] {
            return Err(PlatformError::InvalidAttestation);
        }
        let mut canonical = [0_u8; DATABASE_IDENTITY_BYTES];
        canonical[..PHYSICAL_DATABASE_IDENTITY_BYTES].copy_from_slice(physical.as_bytes());
        canonical[96..].copy_from_slice(&database_instance_id);
        Ok(Self { canonical })
    }

    pub(crate) fn from_bytes(value: &[u8]) -> Result<Self, PlatformError> {
        let canonical: [u8; DATABASE_IDENTITY_BYTES] =
            value.try_into().map_err(map_identity_length)?;
        if canonical[..32] == [0_u8; 32]
            || canonical[32..64] == [0_u8; 32]
            || canonical[64..96] == [0_u8; 32]
            || canonical[96..] == [0_u8; 32]
        {
            return Err(PlatformError::InvalidAttestation);
        }
        Ok(Self { canonical })
    }
}

impl std::fmt::Debug for DatabaseIdentity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DatabaseIdentity")
            .field("opaque", &"<redacted>")
            .finish()
    }
}
