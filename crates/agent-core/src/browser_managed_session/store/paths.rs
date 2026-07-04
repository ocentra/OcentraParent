use std::path::Path;

use ocentra_parent_agent_protocol::constants;

use super::super::{
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStorePaths,
};

pub(crate) fn managed_profile_store_paths(
    config: &BrowserManagedProfileStoreConfig,
) -> Result<BrowserManagedProfileStorePaths, BrowserManagedProfileStoreError> {
    let profile_dir = config.profile_root_dir.join(&config.profile_id);
    if default_profile_path_rejected(&config.profile_root_dir)
        || default_profile_path_rejected(&profile_dir)
    {
        return Err(BrowserManagedProfileStoreError::DefaultProfileRejected);
    }
    if profile_id_contains_path_separator(&config.profile_id)
        || !config
            .profile_id
            .starts_with(constants::browser::PROFILE_ID_PREFIX_MANAGED)
        || !managed_profile_path_owned(&profile_dir)
    {
        return Err(BrowserManagedProfileStoreError::UnownedProfileRejected);
    }

    let mut metadata_file_name = config.profile_id.clone();
    metadata_file_name.push_str(constants::browser::PROFILE_STORE_METADATA_SUFFIX);
    Ok(BrowserManagedProfileStorePaths {
        profile_dir,
        metadata_path: config.profile_root_dir.join(metadata_file_name),
    })
}

pub(crate) fn profile_id_contains_path_separator(profile_id: &str) -> bool {
    profile_id.contains(constants::browser::PATH_SEPARATOR_FORWARD)
        || profile_id.contains(constants::browser::PATH_SEPARATOR_BACKSLASH)
        || profile_id.contains(constants::browser::PATH_SEPARATOR_COLON)
}

pub(crate) fn default_profile_path_rejected(path: &Path) -> bool {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy().to_ascii_lowercase())
        .any(|name| {
            name == constants::browser::PATH_SEGMENT_DEFAULT_NORMALIZED
                || name == constants::browser::PATH_SEGMENT_USER_DATA_NORMALIZED
        })
}

pub(crate) fn managed_profile_path_owned(path: &Path) -> bool {
    path.file_name()
        .map(|name| {
            name.to_string_lossy()
                .to_ascii_lowercase()
                .starts_with(constants::browser::PROFILE_ID_PREFIX_MANAGED)
        })
        .unwrap_or(false)
}
