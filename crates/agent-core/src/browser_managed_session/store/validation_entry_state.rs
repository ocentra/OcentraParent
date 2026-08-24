use ocentra_parent_agent_protocol::{
    browser_managed::{BrowserManagedProfileLifecycleState, BrowserManagedProfileStoreEntry},
    constants,
};

use super::super::BrowserManagedProfileStoreError;

pub(super) fn validate_entry_state(
    entry: &BrowserManagedProfileStoreEntry,
) -> Result<(), BrowserManagedProfileStoreError> {
    match entry.lifecycle_state {
        BrowserManagedProfileLifecycleState::Ready => validate_ready_state(entry),
        BrowserManagedProfileLifecycleState::RepairRequired => {
            super::validation_entry_state_repair::validate_repair_required_state(entry)
        }
        BrowserManagedProfileLifecycleState::Deleted => validate_deleted_state(entry),
        _ => Err(BrowserManagedProfileStoreError::MetadataCorrupt),
    }
}

fn validate_ready_state(
    entry: &BrowserManagedProfileStoreEntry,
) -> Result<(), BrowserManagedProfileStoreError> {
    if entry.missing_since.is_some() || entry.deleted_at.is_some() {
        return Err(BrowserManagedProfileStoreError::MetadataCorrupt);
    }
    match (entry.repaired_at.is_some(), entry.repair_reason.as_deref()) {
        (true, Some(reason)) if reason == constants::browser::PROFILE_STORE_REASON_REPAIRED => {
            Ok(())
        }
        (false, None) => Ok(()),
        (false, Some(reason)) if reason == constants::browser::PROFILE_STORE_REASON_CREATED => {
            Ok(())
        }
        _ => Err(BrowserManagedProfileStoreError::MetadataCorrupt),
    }
}

fn validate_deleted_state(
    entry: &BrowserManagedProfileStoreEntry,
) -> Result<(), BrowserManagedProfileStoreError> {
    if entry.missing_since.is_none()
        && entry.repaired_at.is_none()
        && entry.deleted_at.is_some()
        && entry.repair_reason.as_deref() == Some(constants::browser::PROFILE_STORE_REASON_DELETED)
    {
        Ok(())
    } else {
        Err(BrowserManagedProfileStoreError::MetadataCorrupt)
    }
}
