use std::path::PathBuf;

use ocentra_eventing::{
    envelope::{
        DomainEvent, EventContract, EventEnvelope, EventMetadata, EventSource, StoredEventEnvelope,
    },
    error::EventingError,
    ids::{
        AggregateKey, CorrelationId, EventCustody, EventId, EventType, IdempotencyKey, RecordedAt,
        RuntimeInstanceId, RuntimeRole, SchemaVersion, SourceComponent, SourceService,
    },
    journal::policy::JournalDispatchPhase,
    journal::{production_file::ProductionFileEventJournal, EventJournal, JournalAppend},
    replay::{ReplayFilter, ReplayReadReport},
};
use ocentra_schema::export_import_backup_recovery as contracts;
use serde::{Deserialize, Serialize};

use super::data_custody_parent_runtime_clock::{
    clock_error, DataCustodyRuntimeClock, RuntimeClockError, SharedDataCustodyRuntimeClock,
};
use super::data_custody_runtime_eventing_identity::execution_idempotency_ref;
use super::data_custody_runtime_eventing_identity_backup::backup_job_event_idempotency_ref;

#[cfg(test)]
#[path = "data_custody_runtime_eventing_tests.rs"]
mod data_custody_runtime_eventing_tests;

const DATA_CUSTODY_EVENT_TYPE: &str = "parent-runtime.data-custody.transition";
const DATA_CUSTODY_SCHEMA_VERSION: u16 = 3;
const DATA_CUSTODY_EVENT_CUSTODY: &str = "parent-runtime";
const DATA_CUSTODY_RUNTIME_ROLE: &str = "parent";
const DATA_CUSTODY_SOURCE_SERVICE: &str = "parent-runtime-core";
const DATA_CUSTODY_SOURCE_COMPONENT: &str = "data-custody-runtime";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DataCustodyRuntimeEventKind {
    BackupScheduled,
    BackupJobTransition,
    RestorePlanned,
    RestoreBeforeDispatch,
    RestoreApplied,
    MigrationPlanned,
    MigrationBeforeDispatch,
    MigrationReceipt,
    RollbackBeforeDispatch,
    Rollback,
    Reconciliation,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "recordType", content = "record")]
pub enum DataCustodyRuntimeRecord {
    Schedule(contracts::ExportImportBackupSchedule),
    ScheduleAndJob {
        schedule: contracts::ExportImportBackupSchedule,
        job: contracts::ExportImportBackupJobRecord,
    },
    BackupJob(contracts::ExportImportBackupJobRecord),
    MigrationReceipt(contracts::ExportImportMigrationReceipt),
    RestoreReceipt(contracts::ExportImportRestoreReceipt),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataCustodyRuntimeEvent {
    pub kind: DataCustodyRuntimeEventKind,
    pub operation_ref: String,
    pub idempotency_ref: String,
    pub recorded_at: String,
    pub record: DataCustodyRuntimeRecord,
    pub note: Option<String>,
}

impl DataCustodyRuntimeEvent {
    pub(crate) fn schedule_and_job(
        schedule: contracts::ExportImportBackupSchedule,
        job: contracts::ExportImportBackupJobRecord,
        idempotency_ref: impl Into<String>,
        recorded_at: String,
    ) -> Self {
        Self {
            kind: DataCustodyRuntimeEventKind::BackupScheduled,
            operation_ref: schedule.schedule_ref.as_str().to_owned(),
            idempotency_ref: idempotency_ref.into(),
            recorded_at,
            record: DataCustodyRuntimeRecord::ScheduleAndJob { schedule, job },
            note: None,
        }
    }

    pub(crate) fn backup_job(
        job: contracts::ExportImportBackupJobRecord,
        kind: DataCustodyRuntimeEventKind,
        note: Option<String>,
    ) -> Self {
        let idempotency_ref = backup_job_event_idempotency_ref(&job);
        Self {
            kind,
            operation_ref: job.job_ref.as_str().to_owned(),
            idempotency_ref,
            recorded_at: job.updated_at.as_str().to_owned(),
            record: DataCustodyRuntimeRecord::BackupJob(job),
            note,
        }
    }

    pub(crate) fn migration_receipt(
        receipt: contracts::ExportImportMigrationReceipt,
        kind: DataCustodyRuntimeEventKind,
        note: Option<String>,
    ) -> Self {
        let idempotency_ref = execution_idempotency_ref("migration", &receipt.execution_ref, &kind);
        Self {
            kind,
            operation_ref: receipt.operation_ref.as_str().to_owned(),
            idempotency_ref,
            recorded_at: receipt.recorded_at.as_str().to_owned(),
            record: DataCustodyRuntimeRecord::MigrationReceipt(receipt),
            note,
        }
    }

    pub(crate) fn restore_receipt(
        receipt: contracts::ExportImportRestoreReceipt,
        kind: DataCustodyRuntimeEventKind,
        note: Option<String>,
    ) -> Self {
        let idempotency_ref = execution_idempotency_ref("restore", &receipt.execution_ref, &kind);
        Self {
            kind,
            operation_ref: receipt.operation_ref.as_str().to_owned(),
            idempotency_ref,
            recorded_at: receipt.recorded_at.as_str().to_owned(),
            record: DataCustodyRuntimeRecord::RestoreReceipt(receipt),
            note,
        }
    }
}

impl DomainEvent for DataCustodyRuntimeEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(DATA_CUSTODY_EVENT_TYPE)?,
            SchemaVersion::new(DATA_CUSTODY_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(format!("data-custody:{}", self.operation_ref))
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(self.idempotency_ref.clone())
    }
}

#[derive(Clone, Debug)]
pub struct DataCustodyRuntimeEventJournal {
    journal: ProductionFileEventJournal,
    source: EventSource,
    clock: SharedDataCustodyRuntimeClock,
}

impl DataCustodyRuntimeEventJournal {
    pub fn new(
        path: impl Into<PathBuf>,
        instance_id: impl Into<String>,
    ) -> Result<Self, EventingError> {
        let source = EventSource::new(
            EventCustody::parse(DATA_CUSTODY_EVENT_CUSTODY)?,
            RuntimeRole::parse(DATA_CUSTODY_RUNTIME_ROLE)?,
            SourceService::parse(DATA_CUSTODY_SOURCE_SERVICE)?,
            SourceComponent::parse(DATA_CUSTODY_SOURCE_COMPONENT)?,
            RuntimeInstanceId::parse(instance_id)?,
        );
        Ok(Self {
            journal: ProductionFileEventJournal::new(path),
            source,
            clock: DataCustodyRuntimeClock::shared(),
        })
    }

    pub async fn recover(&self) -> Result<(), EventingError> {
        DataCustodyRuntimeClock::begin_recovery(&self.clock).map_err(clock_error)?;
        self.journal.recover().await?;
        let report = self.replay().await?;
        if report.skipped_count != 0 {
            return Err(EventingError::InvalidValue {
                field: "data_custody_runtime_replay",
                value: "replay contained skipped records".to_owned(),
            });
        }
        for record in report.records {
            let event = Self::decode(&record.envelope)?;
            DataCustodyRuntimeClock::commit_timestamp(&self.clock, &event.recorded_at)
                .map_err(clock_error)?;
        }
        DataCustodyRuntimeClock::mark_recovered(&self.clock).map_err(clock_error)?;
        Ok(())
    }

    pub(crate) fn next_recorded_at(&self) -> Result<String, RuntimeClockError> {
        DataCustodyRuntimeClock::next_timestamp(&self.clock)
    }

    pub(crate) async fn append_record(
        &self,
        event: DataCustodyRuntimeEvent,
        phase: JournalDispatchPhase,
    ) -> Result<JournalAppend, EventingError> {
        DataCustodyRuntimeClock::ensure_recovered(&self.clock).map_err(clock_error)?;
        super::data_custody_runtime_eventing_validation::validate_event_identity(&event)?;
        let correlation_id = CorrelationId::parse(format!("data-custody:{}", event.operation_ref))?;
        let event_id = EventId::parse(format!("data-custody-event-{}", event.idempotency_ref))?;
        let observed_at = RecordedAt::parse(event.recorded_at.clone())?;
        let recorded_at = event.recorded_at.clone();
        let envelope = EventEnvelope::from_event(
            event,
            EventMetadata::from_parts(
                event_id,
                correlation_id,
                self.source.clone(),
                observed_at,
                None,
            ),
        )?;
        let stored = envelope.store()?;
        let append = self.journal.append_phase_idempotent(&stored, phase).await?;
        DataCustodyRuntimeClock::commit_timestamp(&self.clock, &recorded_at)
            .map_err(clock_error)?;
        Ok(append)
    }

    pub async fn replay(&self) -> Result<ReplayReadReport, EventingError> {
        let event_type = EventType::parse(DATA_CUSTODY_EVENT_TYPE)?;
        self.journal
            .replay_projection(ReplayFilter::for_event_type(event_type))
            .await
    }

    pub fn is_production_durable(&self) -> bool {
        self.journal.is_production_durable()
    }

    pub(crate) fn decode(
        record: &StoredEventEnvelope,
    ) -> Result<DataCustodyRuntimeEvent, EventingError> {
        record
            .decode::<DataCustodyRuntimeEvent>()
            .map(|envelope| envelope.into_payload())
            .and_then(|event| {
                super::data_custody_runtime_eventing_validation::validate_event_identity(&event)?;
                Ok(event)
            })
    }
}
