use ocentra_eventing::error::EventingError;

use super::data_custody_runtime_eventing::{DataCustodyRuntimeEvent, DataCustodyRuntimeRecord};
use super::data_custody_runtime_eventing_identity::execution_idempotency_ref;
use super::data_custody_runtime_eventing_identity_backup::{
    backup_job_event_idempotency_ref, schedule_job_identity_is_initial,
};
use super::data_custody_runtime_eventing_identity_kind::kind_matches_record;

pub(crate) fn validate_event_identity(
    event: &DataCustodyRuntimeEvent,
) -> Result<(), EventingError> {
    let (expected_operation_ref, expected_idempotency_ref) = match &event.record {
        DataCustodyRuntimeRecord::Schedule(schedule) => (
            schedule.schedule_ref.as_str(),
            format!("schedule:{}:initial-job", schedule.schedule_ref),
        ),
        DataCustodyRuntimeRecord::ScheduleAndJob { schedule, .. } => (
            schedule.schedule_ref.as_str(),
            format!("schedule:{}:initial-job", schedule.schedule_ref),
        ),
        DataCustodyRuntimeRecord::BackupJob(job) => {
            (job.job_ref.as_str(), backup_job_event_idempotency_ref(job))
        }
        DataCustodyRuntimeRecord::MigrationReceipt(receipt) => (
            receipt.operation_ref.as_str(),
            execution_idempotency_ref("migration", &receipt.execution_ref, &event.kind),
        ),
        DataCustodyRuntimeRecord::RestoreReceipt(receipt) => (
            receipt.operation_ref.as_str(),
            execution_idempotency_ref("restore", &receipt.execution_ref, &event.kind),
        ),
    };
    if event.operation_ref != expected_operation_ref {
        return Err(EventingError::InvalidValue {
            field: "data_custody_operation_ref",
            value: event.operation_ref.clone(),
        });
    }
    if event.idempotency_ref != expected_idempotency_ref {
        return Err(EventingError::InvalidValue {
            field: "data_custody_idempotency_ref",
            value: event.idempotency_ref.clone(),
        });
    }
    if !kind_matches_record(&event.kind, &event.record) {
        return Err(EventingError::InvalidValue {
            field: "data_custody_event_kind",
            value: format!("{:?}", event.kind),
        });
    }
    if !schedule_job_identity_is_initial(&event.record) {
        return Err(EventingError::InvalidValue {
            field: "data_custody_schedule_job_identity",
            value: event.operation_ref.clone(),
        });
    }
    Ok(())
}
