use ocentra_family_identity_core::household_authority_runtime_composer::HouseholdAuthorityRuntimeEffectAuthorization;
use ocentra_schema::export_import_backup_recovery as contracts;

use super::data_custody_backup_runtime::BackupDispatchReservation;

/// Account-owned currentness boundary for backup authorization. Implementors
/// must return the opaque owner-issued authorization; no serialized authority
/// or caller-selected household state is accepted by this port.
pub trait AccountCustodyAuthorityPort: Send + Sync {
    fn current_household_authority(
        &self,
        household_id: &contracts::ExportImportHouseholdId,
    ) -> Result<HouseholdAuthorityRuntimeEffectAuthorization, AuthorityUnavailable>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorityUnavailable {
    Unavailable,
}

/// Opaque export custody issued by the bundle/key owner. The parent runtime
/// never constructs this from a job record: without a mounted owner port the
/// backup remains manual-required and no provider side effect is attempted.
pub struct BackupArtifactBinding {
    bundle_id: contracts::ExportImportBundleId,
    manifest_integrity_ref: contracts::ExportImportIntegrityRef,
    payload_integrity_refs: Vec<(
        contracts::ExportImportDataClass,
        contracts::ExportImportIntegrityRef,
    )>,
    tombstone_cursor_ref: contracts::ExportImportIntegrityRef,
}

impl BackupArtifactBinding {
    /// Issues an opaque binding from the owner that verified the encrypted
    /// bundle. The binding carries typed integrity references only; it never
    /// carries a path, provider credential, or raw key.
    pub fn from_owner(
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

    pub fn bundle_id(&self) -> &contracts::ExportImportBundleId {
        &self.bundle_id
    }

    pub fn is_complete(&self) -> bool {
        !self.payload_integrity_refs.is_empty()
            && !self.manifest_integrity_ref.as_str().is_empty()
            && !self.tombstone_cursor_ref.as_str().is_empty()
    }
}

pub trait BackupCustodyArtifactPort: Send + Sync {
    fn prepare_backup_artifact(
        &self,
        job: &contracts::ExportImportBackupJobRecord,
    ) -> Result<BackupArtifactBinding, BackupArtifactError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupArtifactError {
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
pub struct ProviderOperationReceipt {
    execution_ref: contracts::ExportImportExecutionRef,
    provider_operation_ref: contracts::ExportImportProviderOperationRef,
    bundle_id: contracts::ExportImportBundleId,
    manifest_integrity_ref: contracts::ExportImportIntegrityRef,
    tombstone_cursor_ref: contracts::ExportImportIntegrityRef,
}

impl ProviderOperationReceipt {
    pub fn new(
        reservation: &BackupDispatchReservation,
        artifact: &BackupArtifactBinding,
        provider_operation_ref: contracts::ExportImportProviderOperationRef,
    ) -> Option<Self> {
        if reservation.bundle_id() != artifact.bundle_id() {
            return None;
        }
        Some(Self {
            execution_ref: reservation.execution_ref().clone(),
            provider_operation_ref,
            bundle_id: artifact.bundle_id.clone(),
            manifest_integrity_ref: artifact.manifest_integrity_ref.clone(),
            tombstone_cursor_ref: artifact.tombstone_cursor_ref.clone(),
        })
    }

    pub fn execution_ref(&self) -> &contracts::ExportImportExecutionRef {
        &self.execution_ref
    }

    pub fn provider_operation_ref(&self) -> &contracts::ExportImportProviderOperationRef {
        &self.provider_operation_ref
    }

    pub fn bundle_id(&self) -> &contracts::ExportImportBundleId {
        &self.bundle_id
    }
}

pub trait ProviderNeutralBackupPort: Send + Sync {
    fn execute_backup(
        &self,
        reservation: BackupDispatchReservation,
        artifact: BackupArtifactBinding,
        job: &contracts::ExportImportBackupJobRecord,
    ) -> Result<ProviderOperationReceipt, ProviderBackupError>;
}
