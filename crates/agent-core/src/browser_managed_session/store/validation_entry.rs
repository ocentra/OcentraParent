use ocentra_parent_agent_protocol::{
    browser::{BrowserCustodyLabel, BROWSER_EVIDENCE_SCHEMA_VERSION},
    browser_managed::BrowserManagedProfileStoreEntry,
    constants,
};

use super::super::{BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError};

pub(super) fn validate_stored_entry(
    config: &BrowserManagedProfileStoreConfig,
    entry: &BrowserManagedProfileStoreEntry,
) -> Result<(), BrowserManagedProfileStoreError> {
    validate_immutable_binding(config, entry)?;
    super::validation_timestamps::validate_entry_timestamps(entry)?;
    super::validation_entry_state::validate_entry_state(entry)
}

fn validate_immutable_binding(
    config: &BrowserManagedProfileStoreConfig,
    entry: &BrowserManagedProfileStoreEntry,
) -> Result<(), BrowserManagedProfileStoreError> {
    if entry.schema_version != BROWSER_EVIDENCE_SCHEMA_VERSION
        || entry.profile_id != config.profile_id
        || entry.profile_path_ref != constants::browser::PROFILE_PATH_REF_MANAGED
        || entry.profile_root_ref != constants::browser::PROFILE_ROOT_REF_MANAGED
        || entry.profile_scope_id != config.profile_scope_id
        || entry.device_id != config.device_id
        || entry.browser_family != config.browser_family
        || entry.browser_channel != config.browser_channel
        || entry.custody_label != BrowserCustodyLabel::ChildDeviceLocal
        || entry.policy_revision != config.policy_revision
    {
        Err(BrowserManagedProfileStoreError::BindingMismatch)
    } else {
        Ok(())
    }
}
