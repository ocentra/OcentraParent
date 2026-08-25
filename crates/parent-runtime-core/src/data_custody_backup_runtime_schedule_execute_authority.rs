use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::{
    authorize_backup_request, BackupRequestInput,
};

use super::data_custody_backup_runtime_ports::AccountCustodyAuthorityPort;

pub(super) fn current_backup_authority(
    authority_port: Option<&dyn AccountCustodyAuthorityPort>,
    job: &contracts::ExportImportBackupJobRecord,
) -> Result<(), ()> {
    let Some(authority_port) = authority_port else {
        return Err(());
    };
    let authority = authority_port
        .current_household_authority(&job.household_id)
        .map_err(|_| ())?;
    let request = BackupRequestInput {
        bundle_id: job.bundle_id.clone(),
        cadence: job.cadence,
        household_id: job.household_id.clone(),
    };
    let decision = authorize_backup_request(request, authority).map_err(|_| ())?;
    (decision.state == contracts::ExportImportBackupState::Authorized)
        .then_some(())
        .ok_or(())
}
