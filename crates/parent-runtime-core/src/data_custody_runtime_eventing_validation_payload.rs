use super::data_custody_runtime_eventing::{DataCustodyRuntimeEvent, DataCustodyRuntimeRecord};

pub(super) fn payload_recorded_at(event: &DataCustodyRuntimeEvent) -> Option<&str> {
    match &event.record {
        DataCustodyRuntimeRecord::Schedule(_) | DataCustodyRuntimeRecord::ScheduleAndJob { .. } => {
            None
        }
        DataCustodyRuntimeRecord::BackupJob(job) => Some(job.updated_at.as_str()),
        DataCustodyRuntimeRecord::MigrationReceipt(receipt) => Some(receipt.recorded_at.as_str()),
        DataCustodyRuntimeRecord::RestoreReceipt(receipt) => Some(receipt.recorded_at.as_str()),
    }
}
