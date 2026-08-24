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
    validate_field_lengths(entry)?;
    validate_immutable_binding(config, entry)?;
    super::validation_timestamps::validate_entry_timestamps(entry)?;
    super::validation_entry_state::validate_entry_state(entry)
}

fn validate_field_lengths(
    entry: &BrowserManagedProfileStoreEntry,
) -> Result<(), BrowserManagedProfileStoreError> {
    let limits = constants::browser::PROFILE_STORE_MAX_PROFILE_ID_BYTES;
    if entry.profile_id.len() > limits
        || entry.profile_scope_id.len()
            > constants::browser::PROFILE_STORE_MAX_PROFILE_SCOPE_ID_BYTES
        || entry.device_id.len() > constants::browser::PROFILE_STORE_MAX_DEVICE_ID_BYTES
        || entry.policy_revision.len() > constants::browser::PROFILE_STORE_MAX_POLICY_REVISION_BYTES
        || entry.created_at.len() > constants::browser::PROFILE_STORE_MAX_TIMESTAMP_BYTES
        || entry.updated_at.len() > constants::browser::PROFILE_STORE_MAX_TIMESTAMP_BYTES
        || entry.missing_since.as_deref().is_some_and(|value| {
            value.len() > constants::browser::PROFILE_STORE_MAX_TIMESTAMP_BYTES
        })
        || entry.repaired_at.as_deref().is_some_and(|value| {
            value.len() > constants::browser::PROFILE_STORE_MAX_TIMESTAMP_BYTES
        })
        || entry.deleted_at.as_deref().is_some_and(|value| {
            value.len() > constants::browser::PROFILE_STORE_MAX_TIMESTAMP_BYTES
        })
        || entry.repair_reason.as_deref().is_some_and(|value| {
            value.len() > constants::browser::PROFILE_STORE_MAX_REPAIR_REASON_BYTES
        })
    {
        Err(BrowserManagedProfileStoreError::MetadataCorrupt)
    } else {
        Ok(())
    }
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
