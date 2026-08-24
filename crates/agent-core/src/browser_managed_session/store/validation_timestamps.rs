use chrono::{DateTime, FixedOffset};
use ocentra_parent_agent_protocol::browser_managed::BrowserManagedProfileStoreEntry;

use super::super::BrowserManagedProfileStoreError;

pub(super) fn validate_entry_timestamps(
    entry: &BrowserManagedProfileStoreEntry,
) -> Result<(), BrowserManagedProfileStoreError> {
    let created_at = parse_timestamp(&entry.created_at)?;
    let updated_at = parse_timestamp(&entry.updated_at)?;
    if updated_at < created_at {
        return Err(BrowserManagedProfileStoreError::MetadataCorrupt);
    }
    validate_optional_timestamp(entry.missing_since.as_deref(), &created_at, &updated_at)?;
    validate_optional_timestamp(entry.repaired_at.as_deref(), &created_at, &updated_at)?;
    validate_optional_timestamp(entry.deleted_at.as_deref(), &created_at, &updated_at)
}

fn parse_timestamp(value: &str) -> Result<DateTime<FixedOffset>, BrowserManagedProfileStoreError> {
    DateTime::parse_from_rfc3339(value)
        .map_err(|_error| BrowserManagedProfileStoreError::MetadataCorrupt)
}

fn validate_optional_timestamp(
    value: Option<&str>,
    created_at: &DateTime<FixedOffset>,
    updated_at: &DateTime<FixedOffset>,
) -> Result<(), BrowserManagedProfileStoreError> {
    let Some(value) = value else {
        return Ok(());
    };
    let value = parse_timestamp(value)?;
    if value < *created_at || value > *updated_at {
        Err(BrowserManagedProfileStoreError::MetadataCorrupt)
    } else {
        Ok(())
    }
}
