//! Authorization journal backed by a synchronized filesystem append log.
//!
//! This is deliberately distinct from the proof/replay NDJSON surface: callers
//! must opt into this capability when an authorization boundary needs a durable
//! V3 receipt before dispatching an action-observable event.

use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use super::ndjson::{
    JournalFlushPolicy, JournalHashChain, NdjsonEventJournal, NdjsonJournalOptions,
};
use super::{EventJournal, JournalAppendFuture, SharedEventJournal};
use crate::replay::{ReplayActionReport, ReplayFilter, ReplayReadReport};
use crate::StoredEventEnvelope;

#[derive(Clone, Debug)]
pub struct ProductionFileEventJournal {
    inner: NdjsonEventJournal,
    sync_failure_after_successful_appends_for_debug: Arc<AtomicU64>,
}

impl ProductionFileEventJournal {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            inner: NdjsonEventJournal::with_options(
                path,
                NdjsonJournalOptions {
                    hash_chain: JournalHashChain::Enabled,
                    flush: JournalFlushPolicy::Always,
                },
            ),
            sync_failure_after_successful_appends_for_debug: Arc::new(AtomicU64::new(u64::MAX)),
        }
    }

    pub fn path(&self) -> &Path {
        self.inner.path()
    }

    pub fn shared(self) -> SharedEventJournal {
        std::sync::Arc::new(self)
    }

    /// Validate the existing production journal before its owning service
    /// reports readiness. Constructing a journal does not establish recovery.
    pub async fn recover(&self) -> Result<(), crate::EventingError> {
        self.inner.recover().await
    }

    pub async fn replay_projection(
        &self,
        filter: ReplayFilter,
    ) -> Result<ReplayReadReport, crate::EventingError> {
        self.inner.replay_projection(filter).await
    }

    pub async fn replay_action_records(
        &self,
        filter: ReplayFilter,
    ) -> Result<ReplayActionReport, crate::EventingError> {
        self.inner.replay_action_records(filter).await
    }

    #[cfg(debug_assertions)]
    pub fn inject_next_sync_failure_for_debug(&self) {
        self.inner.inject_next_sync_failure_for_debug();
    }

    #[cfg(debug_assertions)]
    pub fn inject_next_partial_write_failure_for_debug(&self) {
        self.inner.inject_next_partial_write_failure_for_debug();
    }

    #[cfg(debug_assertions)]
    pub fn recovery_count_for_debug(&self) -> u64 {
        self.inner.recovery_count_for_debug()
    }

    /// Causes a real filesystem synchronization fault after exactly `count`
    /// successful appends. Test-only callers use this to exercise a terminal
    /// append failure without substituting a synthetic journal implementation.
    #[cfg(debug_assertions)]
    pub fn inject_sync_failure_after_successful_appends_for_debug(&self, count: u64) {
        self.sync_failure_after_successful_appends_for_debug
            .store(count, Ordering::Release);
    }

    #[cfg(debug_assertions)]
    fn inject_configured_sync_failure_for_debug(&self) {
        let remaining = self
            .sync_failure_after_successful_appends_for_debug
            .load(Ordering::Acquire);
        if remaining == u64::MAX {
            return;
        }
        if remaining > 0 {
            self.sync_failure_after_successful_appends_for_debug
                .fetch_sub(1, Ordering::AcqRel);
            return;
        }
        if self
            .sync_failure_after_successful_appends_for_debug
            .compare_exchange(0, u64::MAX, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            self.inner.inject_next_sync_failure_for_debug();
        }
    }
}

impl EventJournal for ProductionFileEventJournal {
    fn is_production_durable(&self) -> bool {
        true
    }

    fn append<'a>(&'a self, envelope: &'a StoredEventEnvelope) -> JournalAppendFuture<'a> {
        Box::pin(async move {
            #[cfg(debug_assertions)]
            self.inject_configured_sync_failure_for_debug();
            self.inner.append(envelope).await
        })
    }

    fn append_phase<'a>(
        &'a self,
        envelope: &'a StoredEventEnvelope,
        phase: super::policy::JournalDispatchPhase,
    ) -> JournalAppendFuture<'a> {
        Box::pin(async move {
            #[cfg(debug_assertions)]
            self.inject_configured_sync_failure_for_debug();
            self.inner.append_phase(envelope, phase).await
        })
    }

    fn append_phase_idempotent<'a>(
        &'a self,
        envelope: &'a StoredEventEnvelope,
        phase: super::policy::JournalDispatchPhase,
    ) -> JournalAppendFuture<'a> {
        Box::pin(async move {
            #[cfg(debug_assertions)]
            self.inject_configured_sync_failure_for_debug();
            self.inner
                .append_phase_idempotent_by_event_id(envelope, phase)
                .await
        })
    }
}
