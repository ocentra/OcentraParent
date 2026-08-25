use ocentra_schema::export_import_backup_recovery as contracts;

use ocentra_family_identity_core::household_authority_runtime_composer::HouseholdAuthorityRuntimeEffectAuthorization;

use super::{authorize_backup_request, BackupRequestError, BackupRequestInput};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupScheduleRequest {
    pub input: BackupRequestInput,
    pub schedule_ref: String,
    pub next_run_at: String,
    pub interval_seconds: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackupScheduleError {
    Authorization(BackupRequestError),
    ManualRequired(contracts::ExportImportBackupRequestState),
    InvalidScheduleRef,
    InvalidTimestamp,
    ScheduledIntervalRequired,
    ManualScheduleCannotHaveInterval,
}

/// Derives the durable schedule contract from the current family authority.
///
/// The household in the returned schedule is taken from the verified authority
/// binding after the request has been authorized. A caller can supply a bundle
/// or schedule reference, but cannot supply an authority selector or turn a
/// failed authority/integrity check into an accepted schedule.
pub fn derive_backup_schedule(
    request: BackupScheduleRequest,
    authority: HouseholdAuthorityRuntimeEffectAuthorization,
) -> Result<contracts::ExportImportBackupSchedule, BackupScheduleError> {
    let schedule_ref = contracts::ExportImportScheduleRef::parse(request.schedule_ref)
        .ok_or(BackupScheduleError::InvalidScheduleRef)?;
    let next_run_at = contracts::ExportImportTimestamp::parse(request.next_run_at)
        .ok_or(BackupScheduleError::InvalidTimestamp)?;
    let household_id = request.input.household_id.clone();

    let cadence = request.input.cadence;
    let authorization = authorize_backup_request(request.input, authority)
        .map_err(BackupScheduleError::Authorization)?;
    if authorization.state == contracts::ExportImportBackupState::ManualRequired {
        // Keep the owner decision attached to the failure. A manual-required
        // projection cannot be converted into scheduler authority or an
        // enabled durable job by this pure contract layer.
        return Err(BackupScheduleError::ManualRequired(authorization));
    }

    match (cadence, request.interval_seconds) {
        (contracts::ExportImportBackupCadence::Scheduled, Some(interval)) if interval > 0 => {}
        (contracts::ExportImportBackupCadence::Scheduled, _) => {
            return Err(BackupScheduleError::ScheduledIntervalRequired);
        }
        (contracts::ExportImportBackupCadence::Manual, Some(_)) => {
            return Err(BackupScheduleError::ManualScheduleCannotHaveInterval);
        }
        (contracts::ExportImportBackupCadence::Manual, None) => {}
    }

    Ok(contracts::ExportImportBackupSchedule {
        schedule_ref,
        bundle_id: authorization.bundle_id,
        household_id,
        cadence,
        interval_seconds: request.interval_seconds,
        next_run_at,
        enabled: cadence == contracts::ExportImportBackupCadence::Scheduled,
    })
}
