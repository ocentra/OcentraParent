use std::{
    fs, io,
    path::{Path, PathBuf},
};

use ocentra_app_game_core::app_game_child_ux_scheduler_store::AppGameChildUxSchedulerProofStore;
use ocentra_app_game_core::app_game_notification_preference_preflight_bridge::build_app_game_notification_preference_preflight_bridge;
use ocentra_app_game_core::app_game_notification_preference_preflight_bridge_types::AppGameNotificationPreferencePreflightBridgeReadModel;
use ocentra_app_game_core::app_game_notification_provider_preflight_bridge::build_app_game_notification_provider_preflight_bridge;
use ocentra_app_game_core::app_game_notification_provider_preflight_bridge_types::AppGameNotificationProviderPreflightBridgeReadModel;
use ocentra_app_game_core::app_game_notification_scheduler_bridge_types::AppGameNotificationSchedulerBridgeReadModel;

use crate::activity_store_path::activity_db_path;

use super::scheduler_constants::{
    scheduler_bridge_not_private, scheduler_bridge_too_large, scheduler_directory_not_private,
    scheduler_proof_not_private, DEFAULT_ACTIVITY_DB, DEFAULT_PATH, PROOF_UNAVAILABLE_PREFIX,
};

const SCHEDULER_DIRECTORY_SUFFIX: &str = ".app-game-notification";
const SCHEDULER_BRIDGE_FILE: &str = "scheduler-bridge.json";
const SCHEDULER_PROOF_DIRECTORY: &str = "scheduler-proof";
const MAX_SCHEDULER_BRIDGE_BYTES: u64 = 4 * 1024 * 1024;

pub(crate) struct VerifiedNotificationPreflight {
    pub(crate) provider: AppGameNotificationProviderPreflightBridgeReadModel,
    pub(crate) preference: AppGameNotificationPreferencePreflightBridgeReadModel,
}

pub(super) fn load_verified_notification_preflight(
) -> io::Result<Option<VerifiedNotificationPreflight>> {
    let directory = scheduler_directory();
    let directory_metadata = match fs::symlink_metadata(&directory) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };
    if directory_metadata.file_type().is_symlink() || !directory_metadata.is_dir() {
        return Err(invalid_scheduler_data(scheduler_directory_not_private()));
    }
    let scheduler_bridge_path = directory.join(SCHEDULER_BRIDGE_FILE);
    let bridge_metadata = match fs::symlink_metadata(&scheduler_bridge_path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };
    if bridge_metadata.file_type().is_symlink() || !bridge_metadata.is_file() {
        return Err(invalid_scheduler_data(scheduler_bridge_not_private()));
    }
    if bridge_metadata.len() > MAX_SCHEDULER_BRIDGE_BYTES {
        return Err(invalid_scheduler_data(scheduler_bridge_too_large()));
    }
    let scheduler_bytes = fs::read(&scheduler_bridge_path)?;
    let scheduler_bridge: AppGameNotificationSchedulerBridgeReadModel =
        serde_json::from_slice(&scheduler_bytes)
            .map_err(|error| invalid_scheduler_data(error.to_string()))?;

    let proof_directory = directory.join(SCHEDULER_PROOF_DIRECTORY);
    let metadata = fs::symlink_metadata(&proof_directory).map_err(|error| {
        io::Error::new(error.kind(), format!("{PROOF_UNAVAILABLE_PREFIX}{error}"))
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(invalid_scheduler_data(scheduler_proof_not_private()));
    }
    let proof_store = AppGameChildUxSchedulerProofStore::open(&proof_directory)?;

    let provider = build_app_game_notification_provider_preflight_bridge(
        &proof_store,
        ocentra_app_game_core::app_game_notification_provider_preflight_bridge_types::
            AppGameNotificationProviderPreflightBridgeOptions {
            bridge_id: format!("service-provider:{}", scheduler_bridge.bridge_id),
            generated_at: scheduler_bridge.generated_at.clone(),
        },
        scheduler_bridge.clone(),
    )?;
    let preference = build_app_game_notification_preference_preflight_bridge(
        &proof_store,
        ocentra_app_game_core::app_game_notification_preference_preflight_bridge_types::
            AppGameNotificationPreferencePreflightBridgeOptions {
            bridge_id: format!("service-preference:{}", scheduler_bridge.bridge_id),
            generated_at: scheduler_bridge.generated_at.clone(),
        },
        scheduler_bridge,
    )?;

    Ok(Some(VerifiedNotificationPreflight {
        provider,
        preference,
    }))
}

fn scheduler_directory() -> PathBuf {
    let activity_path: PathBuf = activity_db_path().into();
    let file_name = activity_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(DEFAULT_ACTIVITY_DB);
    let directory_name = format!("{file_name}{SCHEDULER_DIRECTORY_SUFFIX}");
    activity_path
        .parent()
        .unwrap_or_else(|| Path::new(DEFAULT_PATH))
        .join(directory_name)
}

fn invalid_scheduler_data(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}
