use ocentra_parent_agent_protocol::browser::{
    BrowserCustodyLabel, BROWSER_EVIDENCE_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::browser_managed::BrowserManagedProfileStoreEntry;
use ocentra_parent_agent_protocol::constants;

use super::super::{
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStorePaths, BrowserManagedProfileStoreRecord, ProfileStoreRecordInput,
};

pub(crate) fn profile_store_error_reason(error: &BrowserManagedProfileStoreError) -> &'static str {
    match error {
        BrowserManagedProfileStoreError::DefaultProfileRejected
        | BrowserManagedProfileStoreError::UnownedProfileRejected
        | BrowserManagedProfileStoreError::BindingMismatch
        | BrowserManagedProfileStoreError::UnsafePath => {
            constants::value::MANAGED_BROWSER_INVALID_PROFILE
        }
        BrowserManagedProfileStoreError::MetadataCorrupt => {
            constants::value::MANAGED_BROWSER_PROFILE_METADATA_CORRUPT
        }
        BrowserManagedProfileStoreError::StoreBusy | BrowserManagedProfileStoreError::Io => {
            constants::value::MANAGED_BROWSER_PROFILE_STORE_IO_ERROR
        }
    }
}

pub(crate) fn profile_store_record(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
    input: ProfileStoreRecordInput,
) -> BrowserManagedProfileStoreRecord {
    BrowserManagedProfileStoreRecord {
        profile_dir: paths.profile_dir,
        metadata_path: paths.metadata_path,
        entry: BrowserManagedProfileStoreEntry {
            schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
            profile_id: config.profile_id.clone(),
            profile_path_ref: constants::browser::PROFILE_PATH_REF_MANAGED.to_string(),
            profile_root_ref: constants::browser::PROFILE_ROOT_REF_MANAGED.to_string(),
            profile_scope_id: config.profile_scope_id.clone(),
            device_id: config.device_id.clone(),
            browser_family: config.browser_family,
            browser_channel: config.browser_channel,
            lifecycle_state: input.lifecycle_state,
            custody_label: BrowserCustodyLabel::ChildDeviceLocal,
            policy_revision: config.policy_revision.clone(),
            created_at: input.created_at,
            updated_at: input.updated_at,
            missing_since: input.missing_since,
            repaired_at: input.repaired_at,
            deleted_at: input.deleted_at,
            repair_reason: input.repair_reason,
        },
    }
}
