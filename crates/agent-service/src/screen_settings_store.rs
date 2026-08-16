use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_settings::ScreenAnalysisParentSetting;
use ocentra_parent_agent_protocol::screen_settings::ScreenSettingsUpdateKind;
use ocentra_parent_agent_protocol::SCREEN_EVIDENCE_SCHEMA_VERSION;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct ScreenSettingsStoredState {
    pub(crate) schema_version: u16,
    pub(crate) active_setting_version: Option<u64>,
    pub(crate) settings: Vec<ScreenSettingsRevisionRecord>,
    pub(crate) audit_events: Vec<ScreenSettingsAuditRecord>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct ScreenSettingsRevisionRecord {
    pub(crate) revision_id: String,
    pub(crate) setting: ScreenAnalysisParentSetting,
    pub(crate) created_at: String,
    pub(crate) audit_event_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ScreenSettingsAuditRecord {
    pub(crate) audit_event_id: String,
    pub(crate) request_id: String,
    pub(crate) kind: ScreenSettingsUpdateKind,
    pub(crate) setting_version: u64,
    pub(crate) created_at: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ScreenSettingsStoreError {
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenSettingsStorePath(PathBuf);

impl ScreenSettingsStoredState {
    pub(crate) fn empty() -> Self {
        Self {
            schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
            active_setting_version: None,
            settings: Vec::new(),
            audit_events: Vec::new(),
        }
    }

    pub(crate) fn active_setting(&self) -> Option<&ScreenSettingsRevisionRecord> {
        self.active_setting_version
            .and_then(|setting_version| self.setting_by_version(setting_version))
    }

    pub(crate) fn setting_by_version(
        &self,
        setting_version: u64,
    ) -> Option<&ScreenSettingsRevisionRecord> {
        self.settings
            .iter()
            .find(|setting| setting.setting.setting_version == setting_version)
    }
}

pub(crate) async fn read_screen_settings_state(
    path: &Path,
) -> Result<ScreenSettingsStoredState, ScreenSettingsStoreError> {
    let path = path.to_path_buf();
    tokio::task::spawn_blocking(move || read_screen_settings_state_sync(&path))
        .await
        .map_err(|_join_error| ScreenSettingsStoreError::Unavailable)?
}

pub(crate) async fn write_screen_settings_state(
    path: &Path,
    state: &ScreenSettingsStoredState,
) -> Result<(), ScreenSettingsStoreError> {
    let path = path.to_path_buf();
    let state = state.clone();
    tokio::task::spawn_blocking(move || write_screen_settings_state_sync(&path, &state))
        .await
        .map_err(|_join_error| ScreenSettingsStoreError::Unavailable)?
}

pub(crate) fn screen_settings_store_path_from_env() -> ScreenSettingsStorePath {
    ScreenSettingsStorePath::from_environment()
}

fn read_screen_settings_state_sync(
    path: &Path,
) -> Result<ScreenSettingsStoredState, ScreenSettingsStoreError> {
    match fs::read_to_string(path) {
        Ok(text) => serde_json::from_str(&text)
            .map_err(|_parse_error| ScreenSettingsStoreError::Unavailable),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(ScreenSettingsStoredState::empty()),
        Err(_) => Err(ScreenSettingsStoreError::Unavailable),
    }
}

fn write_screen_settings_state_sync(
    path: &Path,
    state: &ScreenSettingsStoredState,
) -> Result<(), ScreenSettingsStoreError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|_create_dir_error| ScreenSettingsStoreError::Unavailable)?;
    }
    let text = serde_json::to_string_pretty(state)
        .map_err(|_serialize_error| ScreenSettingsStoreError::Unavailable)?;
    fs::write(path, text).map_err(|_write_error| ScreenSettingsStoreError::Unavailable)
}

impl ScreenSettingsStorePath {
    fn from_environment() -> Self {
        std::env::var(constants::env_var::AGENT_SCREEN_SETTINGS_STORE_PATH)
            .map(PathBuf::from)
            .map(Self)
            .unwrap_or_else(|_| {
                let mut path = std::env::temp_dir();
                path.push(constants::screen_settings::STORE_FILE_NAME);
                Self(path)
            })
    }
}

impl AsRef<Path> for ScreenSettingsStorePath {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}
