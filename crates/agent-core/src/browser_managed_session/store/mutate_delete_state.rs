use ocentra_parent_agent_protocol::browser_managed::BrowserManagedProfileStoreEntry;

use super::super::{
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStorePaths, BrowserManagedProfileStoreRecord,
};
use super::path_guards::ProfileStorePathGuards;

pub(super) fn complete_pending_deletion(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
    stored_entry: Option<BrowserManagedProfileStoreEntry>,
    guards: &ProfileStorePathGuards,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let _ = (config, paths, stored_entry, guards);
    // A `.deleting` directory and a DELETE_PENDING reason are not an
    // authenticated transaction binding.  Do not promote terminal `Deleted`
    // state or remove the directory during restart recovery.  A future
    // owner-issued handle-relative transaction must provide the binding before
    // this path can be enabled.
    Err(BrowserManagedProfileStoreError::UnsafePath)
}

pub(super) fn stage_profile_for_deletion(
    paths: &BrowserManagedProfileStorePaths,
    stored_entry: Option<&BrowserManagedProfileStoreEntry>,
    guards: &ProfileStorePathGuards,
) -> Result<(), BrowserManagedProfileStoreError> {
    let _ = (paths, stored_entry, guards);
    // Persisting a plain marker before a name-based rename would still allow
    // a substituted profile to be moved.  Since the safe owner-issued
    // handle-relative primitive is not available under `forbid(unsafe_code)`,
    // reject the entire transition before changing any path or metadata.
    Err(BrowserManagedProfileStoreError::UnsafePath)
}
