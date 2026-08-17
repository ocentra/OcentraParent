use ocentra_eventing::journal::ndjson::{NdjsonEventJournal, NdjsonJournalOptions};
use tokio::sync::mpsc;

use crate::retention_delete_tombstone_store::RetentionDeleteTombstoneStore;

use super::storage_custody_runtime::ChildStorageCustodyAuthorityHandle;
use super::{
    ChildAgentIngress, ChildAgentService, ChildAgentServiceError, ChildRuntimeTombstoneEventFlow,
    CHILD_AGENT_COMMAND_CAPACITY, CHILD_RUNTIME_DOMAINS,
};

impl ChildAgentService {
    pub async fn initialize() -> Result<Self, ChildAgentServiceError> {
        Self::initialize_with_paths(super::ChildAgentServicePaths::from_environment()?).await
    }

    pub async fn initialize_with_paths(
        paths: super::ChildAgentServicePaths,
    ) -> Result<Self, ChildAgentServiceError> {
        Self::initialize_with_paths_and_custody_authority(
            paths,
            ChildStorageCustodyAuthorityHandle::manual_required(),
        )
        .await
    }

    /// Trusted composition entry point for the account-owned current member /
    /// device authority.  The authority is retained as an opaque handle and
    /// is never accepted on an individual command or decoded from JSON/TS.
    pub async fn initialize_with_paths_and_custody_authority(
        paths: super::ChildAgentServicePaths,
        authority: ChildStorageCustodyAuthorityHandle,
    ) -> Result<Self, ChildAgentServiceError> {
        paths.prepare()?;
        let trust_binding = paths.current_trust_binding().ok();
        let identity = trust_binding
            .as_ref()
            .map(super::ChildAgentServiceIdentity::from_trust_binding)
            .transpose()
            .map_err(ChildAgentServiceError::Storage)?;
        let journal =
            NdjsonEventJournal::with_options(paths.journal(), NdjsonJournalOptions::hash_chain());
        let store = RetentionDeleteTombstoneStore::open(paths.tombstones())
            .map_err(ChildAgentServiceError::Storage)?;
        let removal =
            super::ChildAgentRemovalBoundary::open_with_identity(paths.removal(), identity)
                .map_err(ChildAgentServiceError::Storage)?;
        let tombstone_flow = ChildRuntimeTombstoneEventFlow::new(journal.clone(), store);
        let storage_custody = super::storage_custody_runtime::ChildStorageCustodyRuntime::open(
            paths.root(),
            tombstone_flow.clone(),
            authority,
        )?;
        journal.recover().await?;
        let recovery = tombstone_flow
            .recover_pending()
            .await
            .map_err(ChildAgentServiceError::Storage)?;
        let recovery_pending =
            (!recovery.pending_journal_retry.is_empty()).then_some(recovery.pending_journal_retry);
        let mut domain_flows = Vec::with_capacity(CHILD_RUNTIME_DOMAINS.len());
        for domain in CHILD_RUNTIME_DOMAINS {
            domain_flows.push(super::ChildDomainRuntimeEventFlow::for_domain(domain).await?);
        }
        let (sender, commands) = mpsc::channel(CHILD_AGENT_COMMAND_CAPACITY);
        storage_custody.recover_pending().await?;

        Ok(Self {
            paths,
            domain_flows,
            tombstone_flow,
            storage_custody,
            removal,
            trust_binding,
            recovery_pending,
            ingress: ChildAgentIngress { sender },
            commands,
        })
    }
}
