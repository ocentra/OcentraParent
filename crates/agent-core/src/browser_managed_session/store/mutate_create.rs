use std::fs;

use ocentra_parent_agent_protocol::{
    browser_managed::{BrowserManagedProfileLifecycleState, BrowserManagedProfileStoreEntry},
    constants,
};

use super::super::{
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStorePaths, BrowserManagedProfileStoreRecord, ProfileStoreRecordInput,
};
use super::path_guards::ProfileStorePathGuards;

pub(super) fn create_or_repair_locked(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
    guards: &ProfileStorePathGuards,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let stored_entry = super::io::read_profile_store_entry(config, &paths, guards)?;
    if guards.directory_exists(&paths.deletion_path)? {
        return super::mutate_delete::complete_pending_deletion(
            config,
            paths,
            stored_entry,
            guards,
        );
    }

    let profile_dir_exists = guards.directory_exists(&paths.profile_dir)?;
    match (stored_entry, profile_dir_exists) {
        (None, false) => create_new_profile(config, paths, guards),
        (Some(entry), true)
            if entry.lifecycle_state == BrowserManagedProfileLifecycleState::Ready =>
        {
            Ok(super::load_state::stored_record(paths, entry))
        }
        (Some(entry), true)
            if entry.lifecycle_state == BrowserManagedProfileLifecycleState::RepairRequired =>
        {
            repair_pending_profile(config, paths, entry, guards)
        }
        (Some(entry), false)
            if matches!(
                entry.lifecycle_state,
                BrowserManagedProfileLifecycleState::Ready
                    | BrowserManagedProfileLifecycleState::RepairRequired
            ) =>
        {
            repair_missing_profile(config, paths, entry, guards)
        }
        (Some(entry), false)
            if entry.lifecycle_state == BrowserManagedProfileLifecycleState::Deleted =>
        {
            Ok(super::load_state::stored_record(paths, entry))
        }
        _ => Err(BrowserManagedProfileStoreError::MetadataCorrupt),
    }
}

fn create_new_profile(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
    guards: &ProfileStorePathGuards,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let now = super::validation::timestamp_now();
    let pending = super::record::profile_store_record(
        config,
        paths.clone(),
        ProfileStoreRecordInput {
            created_at: now.clone(),
            updated_at: now.clone(),
            lifecycle_state: BrowserManagedProfileLifecycleState::RepairRequired,
            missing_since: None,
            repaired_at: None,
            deleted_at: None,
            repair_reason: Some(
                constants::browser::PROFILE_STORE_REASON_METADATA_MISSING.to_string(),
            ),
        },
    );
    super::io::write_profile_store_entry(config, &paths, &pending.entry, guards)?;

    create_profile_dir_or_remove(config, &paths, &pending.profile_dir, guards)?;

    let now = super::validation::timestamp_now();
    let record = super::record::profile_store_record(
        config,
        paths.clone(),
        ProfileStoreRecordInput {
            created_at: pending.entry.created_at,
            updated_at: now,
            lifecycle_state: BrowserManagedProfileLifecycleState::Ready,
            missing_since: None,
            repaired_at: None,
            deleted_at: None,
            repair_reason: Some(constants::browser::PROFILE_STORE_REASON_CREATED.to_string()),
        },
    );
    persist_ready_or_remove(config, &paths, &record, guards)?;
    Ok(record)
}

fn repair_missing_profile(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
    entry: BrowserManagedProfileStoreEntry,
    guards: &ProfileStorePathGuards,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let now = super::validation::timestamp_now();
    let pending = super::record::profile_store_record(
        config,
        paths.clone(),
        ProfileStoreRecordInput {
            created_at: entry.created_at,
            updated_at: now.clone(),
            lifecycle_state: BrowserManagedProfileLifecycleState::RepairRequired,
            missing_since: None,
            repaired_at: None,
            deleted_at: None,
            repair_reason: Some(
                constants::browser::PROFILE_STORE_REASON_PROFILE_DIR_MISSING.to_string(),
            ),
        },
    );
    super::io::write_profile_store_entry(config, &paths, &pending.entry, guards)?;

    create_profile_dir_or_remove(config, &paths, &paths.profile_dir, guards)?;

    let now = super::validation::timestamp_now();
    let record = super::record::profile_store_record(
        config,
        paths.clone(),
        ProfileStoreRecordInput {
            created_at: pending.entry.created_at,
            updated_at: now.clone(),
            lifecycle_state: BrowserManagedProfileLifecycleState::Ready,
            missing_since: None,
            repaired_at: Some(now),
            deleted_at: None,
            repair_reason: Some(constants::browser::PROFILE_STORE_REASON_REPAIRED.to_string()),
        },
    );
    persist_ready_or_remove(config, &paths, &record, guards)?;
    Ok(record)
}

fn repair_pending_profile(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
    entry: BrowserManagedProfileStoreEntry,
    guards: &ProfileStorePathGuards,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let now = super::validation::timestamp_now();
    let record = super::record::profile_store_record(
        config,
        paths.clone(),
        ProfileStoreRecordInput {
            created_at: entry.created_at,
            updated_at: now.clone(),
            lifecycle_state: BrowserManagedProfileLifecycleState::Ready,
            missing_since: None,
            repaired_at: Some(now),
            deleted_at: None,
            repair_reason: Some(constants::browser::PROFILE_STORE_REASON_REPAIRED.to_string()),
        },
    );
    persist_ready_or_remove(config, &paths, &record, guards)?;
    Ok(record)
}

fn persist_ready_or_remove(
    config: &BrowserManagedProfileStoreConfig,
    paths: &BrowserManagedProfileStorePaths,
    record: &BrowserManagedProfileStoreRecord,
    guards: &ProfileStorePathGuards,
) -> Result<(), BrowserManagedProfileStoreError> {
    if let Err(error) = super::io::write_profile_store_entry(config, paths, &record.entry, guards) {
        let _ = guards.remove_directory(&record.profile_dir);
        return Err(error);
    }
    Ok(())
}

fn create_profile_dir_or_remove(
    config: &BrowserManagedProfileStoreConfig,
    paths: &BrowserManagedProfileStorePaths,
    profile_dir: &std::path::Path,
    guards: &ProfileStorePathGuards,
) -> Result<(), BrowserManagedProfileStoreError> {
    fs::create_dir(profile_dir).map_err(|_error| BrowserManagedProfileStoreError::Io)?;
    if let Err(error) = super::atomic_write::sync_parent_directory(profile_dir)
        .and_then(|()| super::validation::validate_profile_store_paths(config, paths))
    {
        let _ = guards.remove_directory(profile_dir);
        return Err(error);
    }
    Ok(())
}
