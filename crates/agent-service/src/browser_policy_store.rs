use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateKind;
use ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyEffectivePolicy;
use ocentra_parent_agent_protocol::browser_policy_model::BrowserPolicyValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::policy_constants;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct BrowserPolicyStoredState {
    pub(crate) schema_version: String,
    pub(crate) active_revision_id: Option<String>,
    pub(crate) revisions: Vec<BrowserPolicyRevisionRecord>,
    pub(crate) audit_events: Vec<BrowserPolicyAuditRecord>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct BrowserPolicyRevisionRecord {
    pub(crate) revision_id: String,
    pub(crate) policy: BrowserPolicyValue,
    pub(crate) effective_policy: BrowserPolicyEffectivePolicy,
    pub(crate) created_at: String,
    pub(crate) audit_event_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct BrowserPolicyAuditRecord {
    pub(crate) audit_event_id: String,
    pub(crate) request_id: String,
    pub(crate) kind: BrowserPolicyUpdateKind,
    pub(crate) revision_id: String,
    pub(crate) created_at: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BrowserPolicyStoreError {
    Unavailable,
}

impl BrowserPolicyStoredState {
    pub(crate) fn empty() -> Self {
        Self {
            schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            active_revision_id: None,
            revisions: Vec::new(),
            audit_events: Vec::new(),
        }
    }

    pub(crate) fn active_revision(&self) -> Option<&BrowserPolicyRevisionRecord> {
        self.active_revision_id
            .as_ref()
            .and_then(|revision_id| self.revision_by_id(revision_id))
    }

    pub(crate) fn revision_by_id(&self, revision_id: &str) -> Option<&BrowserPolicyRevisionRecord> {
        self.revisions
            .iter()
            .find(|revision| revision.revision_id == revision_id)
    }
}

fn unavailable_from_error<E>(error: &E) -> BrowserPolicyStoreError {
    let _ = error;
    BrowserPolicyStoreError::Unavailable
}

pub(crate) async fn read_browser_policy_state(
    path: &Path,
) -> Result<BrowserPolicyStoredState, BrowserPolicyStoreError> {
    let path = path.to_path_buf();
    tokio::task::spawn_blocking(move || read_browser_policy_state_sync(&path))
        .await
        .map_err(|error| unavailable_from_error(&error))?
}

pub(crate) async fn write_browser_policy_state(
    path: &Path,
    state: &BrowserPolicyStoredState,
) -> Result<(), BrowserPolicyStoreError> {
    let path = path.to_path_buf();
    let state = state.clone();
    tokio::task::spawn_blocking(move || write_browser_policy_state_sync(&path, &state))
        .await
        .map_err(|error| unavailable_from_error(&error))?
}

pub(crate) fn browser_policy_store_path_from_env() -> PathBuf {
    std::env::var(constants::env_var::AGENT_BROWSER_POLICY_STORE_PATH)
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let mut path = std::env::temp_dir();
            path.push(constants::browser_policy::STORE_FILE_NAME);
            path
        })
}

fn read_browser_policy_state_sync(
    path: &Path,
) -> Result<BrowserPolicyStoredState, BrowserPolicyStoreError> {
    match fs::read_to_string(path) {
        Ok(text) => serde_json::from_str(&text).map_err(|error| unavailable_from_error(&error)),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(BrowserPolicyStoredState::empty()),
        Err(error) => Err(unavailable_from_error(&error)),
    }
}

fn write_browser_policy_state_sync(
    path: &Path,
    state: &BrowserPolicyStoredState,
) -> Result<(), BrowserPolicyStoreError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| unavailable_from_error(&error))?;
    }
    let text =
        serde_json::to_string_pretty(state).map_err(|error| unavailable_from_error(&error))?;
    fs::write(path, text).map_err(|error| unavailable_from_error(&error))
}
