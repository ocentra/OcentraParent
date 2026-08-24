use std::fs::OpenOptions;
use std::path::PathBuf;

use directories::ProjectDirs;
use ocentra_protected_capability_custody_protocol::constants;

use super::{error_status, BrokerRuntimeError};

pub(super) fn open_fixed_database() -> Result<PathBuf, BrokerRuntimeError> {
    let project = ProjectDirs::from(
        constants::BROKER_PROJECT_QUALIFIER,
        constants::BROKER_PROJECT_ORGANIZATION,
        constants::BROKER_PROJECT_APPLICATION,
    )
    .ok_or(BrokerRuntimeError::Unavailable)?;
    let directory = project
        .data_local_dir()
        .join(constants::BROKER_STORAGE_DIRECTORY);
    std::fs::create_dir_all(&directory).map_err(error_status::storage_io)?;
    let database = directory.join(constants::BROKER_DATABASE_FILE);
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&database)
        .map_err(error_status::storage_io)?;
    dunce::canonicalize(database).map_err(error_status::storage_io)
}
