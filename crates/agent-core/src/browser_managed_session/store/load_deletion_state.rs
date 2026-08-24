use ocentra_parent_agent_protocol::browser_managed::{
    BrowserManagedProfileLifecycleState, BrowserManagedProfileStoreEntry,
};
use ocentra_parent_agent_protocol::constants;

use super::super::{
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStorePaths, BrowserManagedProfileStoreRecord,
};

pub(super) fn load_deletion_state(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
    stored_entry: Option<BrowserManagedProfileStoreEntry>,
    now: String,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let entry = stored_entry.ok_or(BrowserManagedProfileStoreError::MetadataCorrupt)?;
    match entry.lifecycle_state {
        BrowserManagedProfileLifecycleState::Ready
        | BrowserManagedProfileLifecycleState::Deleted
        | BrowserManagedProfileLifecycleState::RepairRequired => {
            // A residual staging directory is authoritative evidence that
            // deletion cleanup did not reach its durable postcondition. Never
            // expose Deleted while `.deleting` remains, even if metadata was
            // already replaced before a crash.
            Ok(super::load_state::repair_required_record(
                config,
                paths,
                entry.created_at,
                now,
                constants::browser::PROFILE_STORE_REASON_DELETE_PENDING,
            ))
        }
        _ => Err(BrowserManagedProfileStoreError::MetadataCorrupt),
    }
}
