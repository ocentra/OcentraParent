use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use chrono::Utc;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel;
use ocentra_parent_agent_protocol::transport::{
    PolicyRequestAssistantPreviewConfirmAction, PolicyRequestAssistantPreviewConfirmActorRole,
    PolicyRequestAssistantPreviewConfirmActorState, PolicyRequestParentResolutionDecision,
    PolicyRequestParentResolutionDeliveryBinding, PolicyRequestParentResolutionRequest,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_schema::parent_ui_bridge::{
    ParentPolicyPreviewReadModelSnapshot, ParentPortalParentAccessState,
};
use serde::Deserialize;
use serde_json::Value;

#[path = "resolution_begin.rs"]
mod begin_impl;
#[path = "resolution_inputs.rs"]
mod inputs;
#[path = "resolution_lifecycle.rs"]
mod lifecycle;
#[path = "resolution_request.rs"]
mod request;
#[path = "resolution_validation.rs"]
mod validation;

const RESOLUTION_PAYLOAD_FIELD: &str = constants::field::POLICY_REQUEST_PARENT_RESOLUTION_REQUEST;
const MAX_STAGED_RESOLUTIONS: usize = 32;
const STAGED_RESOLUTION_TTL: Duration = Duration::from_secs(300);

#[derive(Clone, Debug)]
pub(crate) struct StagedParentResolution {
    pub(super) handle: String,
    pub(super) preview_id: String,
    pub(super) parent_actor_id: String,
    pub(super) request: PolicyRequestParentResolutionRequest,
}

#[derive(Clone, Debug)]
struct StoredParentResolution {
    preview_id: String,
    parent_actor_id: String,
    issued_at: Instant,
    in_flight: bool,
}

#[derive(Default)]
struct ParentResolutionStore {
    entries: HashMap<String, StoredParentResolution>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ParentResolutionDecisionInput {
    decision: PolicyRequestParentResolutionDecision,
}

fn store() -> &'static Mutex<ParentResolutionStore> {
    static STORE: OnceLock<Mutex<ParentResolutionStore>> = OnceLock::new();
    STORE.get_or_init(|| Mutex::new(ParentResolutionStore::default()))
}

pub(crate) fn begin(
    payload: &Value,
    read_model: &ParentPolicyPreviewReadModelSnapshot,
    parent_access_state: &ParentPortalParentAccessState,
    lan_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> Result<StagedParentResolution, String> {
    begin_impl::begin(payload, read_model, parent_access_state, lan_read_model)
}

pub(crate) fn request_payload(staged: &StagedParentResolution) -> Result<Value, String> {
    let request = serde_json::to_string(&staged.request)
        .map_err(|error| format!("parent resolution request could not be serialized: {error}"))?;
    Ok(serde_json::json!({ RESOLUTION_PAYLOAD_FIELD: request }))
}

pub(crate) fn commit(staged: &StagedParentResolution) -> Result<(), String> {
    let mut resolution_store = store()
        .lock()
        .map_err(|error| format!("parent resolution store is unavailable: {error}"))?;
    let Some(entry) = resolution_store.entries.get(&staged.handle) else {
        return Err("parent resolution handle is unknown or already consumed".to_string());
    };
    if entry.preview_id != staged.preview_id || entry.parent_actor_id != staged.parent_actor_id {
        return Err("parent resolution handle context does not match".to_string());
    }
    if !entry.in_flight {
        return Err("parent resolution is not pending".to_string());
    }
    resolution_store.entries.remove(&staged.handle);
    Ok(())
}

pub(crate) fn restore(staged: &StagedParentResolution) -> Result<(), String> {
    let mut resolution_store = store()
        .lock()
        .map_err(|error| format!("parent resolution store is unavailable: {error}"))?;
    let Some(entry) = resolution_store.entries.get_mut(&staged.handle) else {
        return Err("parent resolution handle is unknown or already consumed".to_string());
    };
    if entry.preview_id != staged.preview_id || entry.parent_actor_id != staged.parent_actor_id {
        return Err("parent resolution handle context does not match".to_string());
    }
    entry.in_flight = false;
    Ok(())
}
