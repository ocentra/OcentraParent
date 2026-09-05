use std::sync::Arc;

use ocentra_eventing::{
    bus::EventBus,
    error::EventingError,
    journal::policy::{JournalPolicy, JournalSelector},
    journal::production_file::ProductionFileEventJournal,
    journal::JournalAppend,
    replay::{ReplayFilter, ReplayReadReport},
};

use super::{NetworkRuntimeJournalPath, NetworkRuntimeSpine};

impl NetworkRuntimeSpine {
    /// Construct the production runtime against its durable journal.
    ///
    /// This spine intentionally has no subscribers: WP09 owns capture-time
    /// observation event persistence and projection replay. Downstream AI,
    /// policy, enforcement, audit, and portal handlers must be added by their
    /// owning workpacks rather than implied by construction.
    pub async fn with_durable_journal(
        path: &NetworkRuntimeJournalPath,
    ) -> Result<Self, EventingError> {
        let journal = Arc::new(ProductionFileEventJournal::new(
            path.as_path().to_path_buf(),
        ));
        journal.recover().await?;
        let journal_for_bus = Arc::clone(&journal);
        let shared_journal: ocentra_eventing::journal::SharedEventJournal = journal_for_bus;
        let bus = EventBus::with_journal(
            JournalPolicy::before_dispatch(JournalSelector::All),
            shared_journal,
        );
        Ok(Self {
            bus,
            chain_lock: Arc::new(tokio::sync::Mutex::new(())),
            durable_journal: journal,
        })
    }

    pub async fn replay_projection(
        &self,
        filter: ReplayFilter,
    ) -> Result<ReplayReadReport, EventingError> {
        self.durable_journal.replay_projection(filter).await
    }
}

pub(super) fn require_verified_v3_synchronization_receipt(
    append: &JournalAppend,
) -> Result<(), EventingError> {
    if append.has_verified_synchronization_proof() {
        return Ok(());
    }
    Err(EventingError::InvalidHandlerPolicy {
        reason:
            "network runtime durable publication requires a verified V3 synchronization receipt"
                .to_owned(),
    })
}
