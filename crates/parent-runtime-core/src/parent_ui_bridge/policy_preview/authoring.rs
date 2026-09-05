use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use ocentra_schema::parent_ui_bridge::{
    ParentPolicyPreviewId, ParentPolicyPreviewReadModelSnapshot, ParentPortalParentAccessState,
};
use serde::Deserialize;
use serde_json::Value;

#[path = "authoring_confirmation.rs"]
mod confirmation;
#[path = "authoring_consume.rs"]
mod consume_impl;
#[path = "authoring_handles.rs"]
mod handles;
#[path = "authoring_stage.rs"]
mod stage_impl;

const DRAFT_PAYLOAD_FIELD: &str = "policyPreviewAuthoringDraft";
const HANDLE_PAYLOAD_FIELD: &str = "policyPreviewAuthoringHandle";
const MAX_STAGED_DRAFTS: usize = 32;
const STAGED_DRAFT_TTL: Duration = Duration::from_secs(300);

#[derive(Clone, Debug)]
pub(crate) struct StagedPolicyPreviewDraft {
    pub(super) handle: String,
    pub(super) read_model: ParentPolicyPreviewReadModelSnapshot,
    pub(super) target_value: String,
    pub(super) requested_action: String,
}

#[derive(Clone, Debug)]
struct StoredPolicyPreviewDraft {
    preview_id: String,
    read_model: ParentPolicyPreviewReadModelSnapshot,
    target_value: String,
    requested_action: String,
    parent_access_state: ParentPortalParentAccessState,
    issued_at: Instant,
    in_flight: bool,
}

#[derive(Default)]
struct PolicyPreviewAuthoringStore {
    entries: HashMap<String, StoredPolicyPreviewDraft>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PolicyPreviewAuthoringDraftInput {
    target_value: String,
    requested_action: String,
}

fn store() -> &'static Mutex<PolicyPreviewAuthoringStore> {
    static STORE: OnceLock<Mutex<PolicyPreviewAuthoringStore>> = OnceLock::new();
    STORE.get_or_init(|| Mutex::new(PolicyPreviewAuthoringStore::default()))
}

pub(crate) fn stage(
    payload: &Value,
    read_model: &ParentPolicyPreviewReadModelSnapshot,
    parent_access_state: &ParentPortalParentAccessState,
) -> Result<String, String> {
    stage_impl::stage(payload, read_model, parent_access_state)
}

pub(crate) fn current(
    read_model: &ParentPolicyPreviewReadModelSnapshot,
    parent_access_state: &ParentPortalParentAccessState,
) -> Option<StagedPolicyPreviewDraft> {
    stage_impl::current(read_model, parent_access_state)
}

pub(crate) fn handle_payload(handle: &str) -> Value {
    serde_json::json!({ HANDLE_PAYLOAD_FIELD: handle })
}

pub(crate) fn consume(
    payload: &Value,
    preview_id: &ParentPolicyPreviewId,
    parent_access_state: &ParentPortalParentAccessState,
) -> Result<StagedPolicyPreviewDraft, String> {
    consume_impl::consume(payload, preview_id, parent_access_state)
}

pub(crate) fn commit(
    draft: &StagedPolicyPreviewDraft,
    preview_id: &ParentPolicyPreviewId,
    parent_access_state: &ParentPortalParentAccessState,
) -> Result<(), String> {
    handles::commit(draft, preview_id, parent_access_state)
}

pub(crate) fn release(
    draft: &StagedPolicyPreviewDraft,
    preview_id: &ParentPolicyPreviewId,
    parent_access_state: &ParentPortalParentAccessState,
) -> Result<(), String> {
    handles::release(draft, preview_id, parent_access_state)
}

pub(crate) fn typed_confirm_payload(draft: &StagedPolicyPreviewDraft) -> Result<Value, String> {
    confirmation::typed_confirm_payload(draft)
}

pub(crate) fn cancel(
    payload: &Value,
    preview_id: &ParentPolicyPreviewId,
    parent_access_state: &ParentPortalParentAccessState,
) -> Result<(), String> {
    handles::cancel(payload, preview_id, parent_access_state)
}

fn opaque_handle(authoring_store: &PolicyPreviewAuthoringStore) -> Result<String, String> {
    for _ in 0..4 {
        let mut random = [0_u8; 24];
        getrandom::fill(&mut random).map_err(|error| {
            format!("policy preview authoring handle entropy is unavailable: {error}")
        })?;
        let handle = format!(
            "ppah-{}",
            random
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        );
        if !authoring_store.entries.contains_key(&handle) {
            return Ok(handle);
        }
    }
    Err("policy preview authoring handle generation collided".to_string())
}

fn exact_payload_field<'a>(
    payload: &'a Value,
    field: &str,
    label: &str,
) -> Result<&'a Value, String> {
    let object = payload
        .as_object()
        .ok_or_else(|| format!("policy preview {label} payload must be an object"))?;
    if object.len() != 1 || !object.contains_key(field) {
        return Err(format!(
            "policy preview {label} payload must contain only the expected field"
        ));
    }
    object
        .get(field)
        .ok_or_else(|| format!("policy preview {label} payload is missing"))
}
