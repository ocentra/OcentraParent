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
    if !config.profile_root_dir.exists() {
        let now = timestamp_now();
        return Ok(super::load_state::missing_record(
            config,
            paths,
            now.clone(),
            now,
        ));
    }
    with_profile_store_lock(&paths, || {
        super::validation::validate_profile_store_paths(config, &paths)?;
        load_locked(config, paths.clone())
    })
}

fn load_locked(
    config: &BrowserManagedProfileStoreConfig,
    paths: BrowserManagedProfileStorePaths,
) -> Result<BrowserManagedProfileStoreRecord, BrowserManagedProfileStoreError> {
    let stored_entry = read_profile_store_entry(config, &paths)?;
    let now = timestamp_now();

    if paths.deletion_path.is_dir() {
        return super::load_deletion_state::load_deletion_state(config, paths, stored_entry, now);
    }

    if !paths.profile_dir.is_dir() {
        return super::load_state::load_missing_profile_state(config, paths, stored_entry, now);
    }

    super::load_state::load_existing_profile_state(config, paths, stored_entry, now)
}
