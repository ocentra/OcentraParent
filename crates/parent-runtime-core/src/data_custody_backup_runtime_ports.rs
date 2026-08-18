use ocentra_family_identity_core::household_authority_proof::CurrentVerifiedHouseholdAuthority;
use ocentra_schema::export_import_backup_recovery as contracts;

use super::data_custody_backup_runtime::BackupDispatchReservation;

pub(crate) trait AccountCustodyAuthorityPort: Send + Sync {
    fn current_household_authority(
        &self,
        household_id: &contracts::ExportImportHouseholdId,
    ) -> Result<CurrentVerifiedHouseholdAuthority, AuthorityUnavailable>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorityUnavailable {
    Unavailable,
}

mod backup_artifact_sealed {
    pub trait Port {}
}

/// Opaque export custody issued by the bundle/key owner. The parent runtime
/// never constructs this from a job record: without a mounted owner port the
/// backup remains manual-required and no provider side effect is attempted.
pub(crate) struct BackupArtifactBinding {
    bundle_id: contracts::ExportImportBundleId,
    manifest_integrity_ref: contracts::ExportImportIntegrityRef,
    payload_integrity_refs: Vec<(
        contracts::ExportImportDataClass,
        contracts::ExportImportIntegrityRef,
    )>,
    tombstone_cursor_ref: contracts::ExportImportIntegrityRef,
}

impl BackupArtifactBinding {
    pub(crate) fn from_owner(
        bundle_id: contracts::ExportImportBundleId,
        manifest_integrity_ref: contracts::ExportImportIntegrityRef,
        payload_integrity_refs: Vec<(
            contracts::ExportImportDataClass,
            contracts::ExportImportIntegrityRef,
        )>,
        tombstone_cursor_ref: contracts::ExportImportIntegrityRef,
    ) -> Self {
        Self {
            bundle_id,
            manifest_integrity_ref,
            payload_integrity_refs,
            tombstone_cursor_ref,
        }
    }

    pub(crate) fn bundle_id(&self) -> &contracts::ExportImportBundleId {
        &self.bundle_id
    }

    pub(crate) fn is_complete(&self) -> bool {
        !self.payload_integrity_refs.is_empty()
            && !self.manifest_integrity_ref.as_str().is_empty()
            && !self.tombstone_cursor_ref.as_str().is_empty()
    }
}

pub(crate) trait BackupCustodyArtifactPort:
    backup_artifact_sealed::Port + Send + Sync
{
    fn prepare_backup_artifact(
        &self,
        job: &contracts::ExportImportBackupJobRecord,
    ) -> Result<BackupArtifactBinding, BackupArtifactError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BackupArtifactError {
    Unavailable,
    BundleMismatch,
    IntegrityBindingMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderBackupError {
    Unavailable,
    Failed,
}

#[derive(Debug)]
pub(crate) struct ProviderOperationReceipt {
    execution_ref: contracts::ExportImportExecutionRef,
    provider_operation_ref: contracts::ExportImportProviderOperationRef,
    bundle_id: contracts::ExportImportBundleId,
    manifest_integrity_ref: contracts::ExportImportIntegrityRef,
    tombstone_cursor_ref: contracts::ExportImportIntegrityRef,
}

impl ProviderOperationReceipt {
    pub(crate) fn new(
        reservation: &BackupDispatchReservation,
        artifact: &BackupArtifactBinding,
        provider_operation_ref: impl Into<String>,
    ) -> Option<Self> {
        if reservation.bundle_id() != artifact.bundle_id() {
            return None;
        }
        Some(Self {
            execution_ref: reservation.execution_ref().clone(),
            provider_operation_ref: contracts::ExportImportProviderOperationRef::parse(
                provider_operation_ref,
            )?,
            bundle_id: artifact.bundle_id.clone(),
            manifest_integrity_ref: artifact.manifest_integrity_ref.clone(),
            tombstone_cursor_ref: artifact.tombstone_cursor_ref.clone(),
        })
    }

    pub(crate) fn execution_ref(&self) -> &contracts::ExportImportExecutionRef {
        &self.execution_ref
    }

    pub(crate) fn provider_operation_ref(&self) -> &contracts::ExportImportProviderOperationRef {
        &self.provider_operation_ref
    }

    pub(crate) fn bundle_id(&self) -> &contracts::ExportImportBundleId {
        &self.bundle_id
    }
}

mod backup_provider_sealed {
    pub trait Port {}
}

pub(crate) trait ProviderNeutralBackupPort:
    backup_provider_sealed::Port + Send + Sync
{
    fn execute_backup(
        &self,
        reservation: BackupDispatchReservation,
        artifact: BackupArtifactBinding,
        job: &contracts::ExportImportBackupJobRecord,
    ) -> Result<ProviderOperationReceipt, ProviderBackupError>;
}
