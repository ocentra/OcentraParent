use ocentra_parent_agent_protocol::{browser_managed::BrowserManagedProfileStoreEntry, constants};

use super::super::BrowserManagedProfileStoreError;

pub(super) fn validate_repair_required_state(
    entry: &BrowserManagedProfileStoreEntry,
) -> Result<(), BrowserManagedProfileStoreError> {
    if entry.missing_since.is_none()
        && entry.repaired_at.is_none()
        && entry.deleted_at.is_none()
        && matches!(
            entry.repair_reason.as_deref(),
            Some(constants::browser::PROFILE_STORE_REASON_DELETE_PENDING)
                | Some(constants::browser::PROFILE_STORE_REASON_METADATA_MISSING)
                | Some(constants::browser::PROFILE_STORE_REASON_PROFILE_DIR_MISSING)
                | Some(constants::browser::PROFILE_STORE_REASON_METADATA_CORRUPT)
        )
    {
        Ok(())
    } else {
        Err(BrowserManagedProfileStoreError::MetadataCorrupt)
    }
}
