use std::io::ErrorKind;

use super::super::{
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStorePaths, BrowserManagedProfileStoreRecord,
};
use super::{
    io::read_profile_store_entry, lock::with_profile_store_lock,
    paths::managed_profile_store_paths, validation::timestamp_now,
};

pub(crate) fn load_managed_browser_profile_store(
    config: &BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let paths = managed_profile_store_paths(config)?;
    super::validation::validate_profile_store_paths(config, &paths)?;
    if matches!(
        std::fs::symlink_metadata(&config.profile_root_dir),
        Err(error) if error.kind() == ErrorKind::NotFound
    ) {
        let now = timestamp_now();
        return Ok(super::load_state::missing_record(
            config,
            paths,
            now.clone(),
            now,
        ));
    }
    with_profile_store_lock(&paths, |guards| {
        super::validation::validate_profile_store_paths(config, &paths)?;
        load_locked(config, paths.clone(), guards)
    })
}

fn load_locked(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
    guards: &super::path_guards::ProfileStorePathGuards,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let stored_entry = read_profile_store_entry(config, &paths, guards)?;
    let now = timestamp_now();

    if guards.directory_exists(&paths.deletion_path)? {
        return super::load_deletion_state::load_deletion_state(config, paths, stored_entry, now);
    }

    if !guards.directory_exists(&paths.profile_dir)? {
        return super::load_state::load_missing_profile_state(config, paths, stored_entry, now);
    }

    super::load_state::load_existing_profile_state(config, paths, stored_entry, now)
}
