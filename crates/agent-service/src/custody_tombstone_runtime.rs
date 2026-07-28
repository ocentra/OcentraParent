//! Service-owned restart recovery for durable child-custody tombstone actions.

use std::path::PathBuf;

use ocentra_child_runtime::runtime_gate_tombstone::{
    acknowledge_child_runtime_tombstone_publication, persist_child_runtime_tombstone_action,
};
use ocentra_eventing::journal::ndjson::{NdjsonEventJournal, NdjsonJournalOptions};
use ocentra_storage_custody_core::retention_delete_tombstone_store::{
    RetentionDeleteOutboxRecord, RetentionDeleteTombstoneStore,
};

use crate::activity_store_path::{activity_db_path, activity_journal_path};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TombstoneRecoveryReport {
    pub recovered_count: usize,
    pub failed_count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TombstoneStoreDirectory(PathBuf);

impl From<PathBuf> for TombstoneStoreDirectory {
    fn from(value: PathBuf) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TombstoneJournalPath(PathBuf);

impl From<PathBuf> for TombstoneJournalPath {
    fn from(value: PathBuf) -> Self {
        Self(value)
    }
}

/// Retries only durable, non-terminal typed actions after the service owns its
/// runtime path. A failed append stays pending for the next restart.
pub(crate) fn spawn_startup_recovery() {
    tokio::spawn(async {
        let journal_path: PathBuf = activity_journal_path().into();
        let _ = recover_pending_tombstone_actions(
            activity_store_directory(),
            TombstoneJournalPath::from(journal_path),
        )
        .await;
    });
}

pub async fn recover_pending_tombstone_actions(
    store_directory: TombstoneStoreDirectory,
    journal_path: TombstoneJournalPath,
) -> TombstoneRecoveryReport {
    let Ok(store) = RetentionDeleteTombstoneStore::open(store_directory.0) else {
        return failed_report();
    };
    let Ok(records) = store.records() else {
        return failed_report();
    };
    let journal =
        NdjsonEventJournal::with_options(journal_path.0, NdjsonJournalOptions::hash_chain());
    let mut report = TombstoneRecoveryReport {
        recovered_count: 0,
        failed_count: 0,
    };
    for record in records.into_iter().filter(|record| record.terminal_pending) {
        if recover_record(&journal, &store, &record).await {
            report.recovered_count += 1;
        } else {
            report.failed_count += 1;
        }
    }
    report
}

async fn recover_record(
    journal: &NdjsonEventJournal,
    store: &RetentionDeleteTombstoneStore,
    record: &RetentionDeleteOutboxRecord,
) -> bool {
    let Some((action, envelope)) = record.typed_action_and_envelope() else {
        return false;
    };
    if persist_child_runtime_tombstone_action(journal, store, envelope, action)
        .await
        .is_err()
    {
        return false;
    }
    acknowledge_child_runtime_tombstone_publication(store, &record.deletion_ref)
        .await
        .is_ok()
}

fn activity_store_directory() -> TombstoneStoreDirectory {
    let database_path: PathBuf = activity_db_path().into();
    TombstoneStoreDirectory(
        database_path
            .parent()
            .map_or_else(std::env::temp_dir, std::path::Path::to_path_buf),
    )
}

fn failed_report() -> TombstoneRecoveryReport {
    TombstoneRecoveryReport {
        recovered_count: 0,
        failed_count: 1,
    }
}
