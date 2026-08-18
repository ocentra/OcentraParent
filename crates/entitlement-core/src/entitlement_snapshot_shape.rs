#![forbid(unsafe_code)]

use crate::entitlement_access::EntitlementCapability;
use crate::entitlement_snapshot::{
    EntitlementSnapshotShapeError, SignedEntitlementSnapshot, ENTITLEMENT_SNAPSHOT_SCHEMA_VERSION,
    ENTITLEMENT_SNAPSHOT_SIGNATURE_BYTES,
};

impl SignedEntitlementSnapshot {
    pub fn validate_shape(&self) -> Result<(), EntitlementSnapshotShapeError> {
        if self.schema_version != ENTITLEMENT_SNAPSHOT_SCHEMA_VERSION {
            return Err(EntitlementSnapshotShapeError::UnsupportedSchemaVersion);
        }
        if self.authority_generation == 0 {
            return Err(EntitlementSnapshotShapeError::InvalidAuthorityGeneration);
        }
        if self.signature.len() != ENTITLEMENT_SNAPSHOT_SIGNATURE_BYTES {
            return Err(EntitlementSnapshotShapeError::InvalidSignatureLength);
        }
        self.validate_shape_without_signature()
    }

    pub(crate) fn validate_shape_without_signature(
        &self,
    ) -> Result<(), EntitlementSnapshotShapeError> {
        if self.schema_version != ENTITLEMENT_SNAPSHOT_SCHEMA_VERSION {
            return Err(EntitlementSnapshotShapeError::UnsupportedSchemaVersion);
        }
        if self.authority_generation == 0 {
            return Err(EntitlementSnapshotShapeError::InvalidAuthorityGeneration);
        }

        let issued_at = parse_snapshot_timestamp(&self.issued_at)?;
        let expires_at = parse_snapshot_timestamp(&self.expires_at)?;
        if issued_at >= expires_at {
            return Err(EntitlementSnapshotShapeError::InvalidTimeWindow);
        }
        if let Some(grace_until) = self.grace_until.as_deref() {
            if parse_snapshot_timestamp(grace_until)? < expires_at {
                return Err(EntitlementSnapshotShapeError::InvalidGraceWindow);
            }
        }

        for (index, flag) in self.feature_flags.iter().enumerate() {
            if self.feature_flags[index + 1..]
                .iter()
                .any(|other| other.capability == flag.capability)
            {
                return Err(EntitlementSnapshotShapeError::DuplicateCapability);
            }
        }
        Ok(())
    }

    pub(crate) fn enables(&self, capability: EntitlementCapability) -> bool {
        self.feature_flags
            .iter()
            .any(|flag| flag.capability == capability && flag.enabled)
    }
}

fn parse_snapshot_timestamp(
    value: &str,
) -> Result<chrono::DateTime<chrono::FixedOffset>, EntitlementSnapshotShapeError> {
    chrono::DateTime::parse_from_rfc3339(value)
        .map_err(|_error| EntitlementSnapshotShapeError::InvalidTimestamp)
}
