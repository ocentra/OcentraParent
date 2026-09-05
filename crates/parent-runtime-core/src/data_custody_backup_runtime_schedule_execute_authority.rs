use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::{
    authorize_backup_request, BackupRequestError, BackupRequestInput,
};

use super::data_custody_backup_runtime_ports::{AccountCustodyAuthorityPort, AuthorityUnavailable};

#[derive(Debug)]
pub(super) enum BackupAuthorityCheckError {
    OwnerPortMissing,
    Authority(AuthorityUnavailable),
    Request(BackupRequestError),
    NotAuthorized,
}

impl std::fmt::Display for BackupAuthorityCheckError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OwnerPortMissing => formatter.write_str("backup authority owner port is missing"),
            Self::Authority(error) => write!(formatter, "backup authority unavailable: {error:?}"),
            Self::Request(error) => write!(formatter, "backup request was rejected: {error:?}"),
            Self::NotAuthorized => formatter.write_str("backup request was not authorized"),
        }
    }
}

pub(super) fn current_backup_authority(
    authority_port: Option<&dyn AccountCustodyAuthorityPort>,
    job: &contracts::ExportImportBackupJobRecord,
) -> Result<(), BackupAuthorityCheckError> {
    let Some(authority_port) = authority_port else {
        return Err(BackupAuthorityCheckError::OwnerPortMissing);
    };
    let authority = authority_port
        .current_household_authority(&job.household_id)
        .map_err(BackupAuthorityCheckError::Authority)?;
    let request = BackupRequestInput {
        bundle_id: job.bundle_id.clone(),
        cadence: job.cadence,
        household_id: job.household_id.clone(),
    };
    let decision =
        authorize_backup_request(request, authority).map_err(BackupAuthorityCheckError::Request)?;
    (decision.state == contracts::ExportImportBackupState::Authorized)
        .then_some(())
        .ok_or(BackupAuthorityCheckError::NotAuthorized)
}
