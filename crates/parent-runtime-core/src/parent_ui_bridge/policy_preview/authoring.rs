use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use ocentra_schema::parent_ui_bridge::{
    ParentPolicyPreviewId, ParentPolicyPreviewReadModelSnapshot, ParentPortalParentAccessState,
};
use serde::Deserialize;
use serde_json::Value;

const DRAFT_PAYLOAD_FIELD: &str = "policyPreviewAuthoringDraft";
const HANDLE_PAYLOAD_FIELD: &str = "policyPreviewAuthoringHandle";
const MAX_STAGED_DRAFTS: usize = 32;
const STAGED_DRAFT_TTL: Duration = Duration::from_secs(300);

#[derive(Clone, Debug)]
pub(super) struct StagedPolicyPreviewDraft {
    pub(super) handle: String,
    pub(super) target_value: String,
    pub(super) requested_action: String,
}

#[derive(Clone, Debug)]
struct StoredPolicyPreviewDraft {
    preview_id: String,
    target_value: String,
    requested_action: String,
    parent_access_state: ParentPortalParentAccessState,
    issued_at: Instant,
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

pub(super) fn stage(
    payload: &Value,
    preview_id: &ParentPolicyPreviewId,
    parent_access_state: &ParentPortalParentAccessState,
) -> Result<String, String> {
    let draft_text = exact_payload_field(payload, DRAFT_PAYLOAD_FIELD, "draft")
        .and_then(Value::as_str)
        .ok_or_else(|| "policy preview draft payload is missing".to_string())?;
    let draft: PolicyPreviewAuthoringDraftInput = serde_json::from_str(draft_text)
        .map_err(|_| "policy preview draft payload is invalid".to_string())?;
    let target_value = draft.target_value.trim();
    if target_value.is_empty() || target_value.len() > 256 {
        return Err("policy preview draft target is invalid".to_string());
    }
    let requested_action = draft.requested_action.trim().to_ascii_lowercase();
    if !matches!(
        requested_action.as_str(),
        "allow" | "warn" | "ask-parent" | "time-limit" | "block"
    ) {
        return Err("policy preview draft action is invalid".to_string());
    }

    let mut authoring_store = store()
        .lock()
        .map_err(|_| "policy preview authoring store is unavailable".to_string())?;
    let now = Instant::now();
    authoring_store
        .entries
        .retain(|_, entry| now.duration_since(entry.issued_at) <= STAGED_DRAFT_TTL);
    authoring_store.entries.retain(|_, entry| {
        !(entry.preview_id == preview_id.as_str()
            && entry.parent_access_state == *parent_access_state)
    });
    while authoring_store.entries.len() >= MAX_STAGED_DRAFTS {
        let Some(oldest_handle) = authoring_store
            .entries
            .iter()
            .min_by_key(|(_, entry)| entry.issued_at)
            .map(|(handle, _)| handle.clone())
        else {
            break;
        };
        authoring_store.entries.remove(&oldest_handle);
    }

    let handle = opaque_handle(&authoring_store)?;
    authoring_store.entries.insert(
        handle.clone(),
        StoredPolicyPreviewDraft {
            preview_id: preview_id.as_str().to_string(),
            target_value: target_value.to_string(),
            requested_action,
            parent_access_state: parent_access_state.clone(),
            issued_at: now,
        },
    );
    Ok(handle)
}

pub(super) fn current(
    read_model: &ParentPolicyPreviewReadModelSnapshot,
    parent_access_state: &ParentPortalParentAccessState,
) -> Option<StagedPolicyPreviewDraft> {
    let preview_id = read_model.preview_id.as_ref()?.as_str();
    let authoring_store = store().lock().ok()?;
    authoring_store
        .entries
        .iter()
        .find(|(_, entry)| {
            entry.preview_id == preview_id
                && entry.parent_access_state == *parent_access_state
                && Instant::now().duration_since(entry.issued_at) <= STAGED_DRAFT_TTL
        })
        .map(|(handle, entry)| StagedPolicyPreviewDraft {
            handle: handle.clone(),
            target_value: entry.target_value.clone(),
            requested_action: entry.requested_action.clone(),
        })
}

pub(super) fn handle_payload(handle: &str) -> Value {
    serde_json::json!({ HANDLE_PAYLOAD_FIELD: handle })
}

pub(super) fn consume(
    payload: &Value,
    preview_id: &ParentPolicyPreviewId,
    parent_access_state: &ParentPortalParentAccessState,
) -> Result<StagedPolicyPreviewDraft, String> {
    let handle = exact_payload_field(payload, HANDLE_PAYLOAD_FIELD, "handle")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "policy preview authoring handle is missing".to_string())?;
    let mut authoring_store = store()
        .lock()
        .map_err(|_| "policy preview authoring store is unavailable".to_string())?;
    let Some(entry) = authoring_store.entries.get(handle) else {
        return Err("policy preview authoring handle is unknown or already consumed".to_string());
    };
    if entry.preview_id != preview_id.as_str() {
        return Err("policy preview authoring handle is bound to another preview".to_string());
    }
    if entry.parent_access_state != *parent_access_state {
        return Err(
            "policy preview authoring handle role does not match current authority".to_string(),
        );
    }
    if !matches!(
        parent_access_state,
        ParentPortalParentAccessState::ActiveController
    ) {
        return Err(
            "policy preview authoring confirmation requires active controller authority"
                .to_string(),
        );
    }
    if Instant::now().duration_since(entry.issued_at) > STAGED_DRAFT_TTL {
        authoring_store.entries.remove(handle);
        return Err("policy preview authoring handle is stale".to_string());
    }
    let entry = authoring_store.entries.remove(handle).ok_or_else(|| {
        "policy preview authoring handle is unknown or already consumed".to_string()
    })?;
    Ok(StagedPolicyPreviewDraft {
        handle: handle.to_string(),
        target_value: entry.target_value,
        requested_action: entry.requested_action,
    })
}

pub(super) fn cancel(
    payload: &Value,
    preview_id: &ParentPolicyPreviewId,
    parent_access_state: &ParentPortalParentAccessState,
) -> Result<(), String> {
    if payload.as_object().is_some_and(|object| object.is_empty()) {
        return Ok(());
    }
    let handle = exact_payload_field(payload, HANDLE_PAYLOAD_FIELD, "handle")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "policy preview authoring handle is missing".to_string())?;
    let mut authoring_store = store()
        .lock()
        .map_err(|_| "policy preview authoring store is unavailable".to_string())?;
    let Some(entry) = authoring_store.entries.get(handle) else {
        return Err("policy preview authoring handle is unknown or already consumed".to_string());
    };
    if entry.preview_id != preview_id.as_str() {
        return Err("policy preview authoring handle is bound to another preview".to_string());
    }
    if entry.parent_access_state != *parent_access_state {
        return Err(
            "policy preview authoring handle role does not match current authority".to_string(),
        );
    }
    authoring_store.entries.remove(handle);
    Ok(())
}

fn opaque_handle(authoring_store: &PolicyPreviewAuthoringStore) -> Result<String, String> {
    for _ in 0..4 {
        let mut random = [0_u8; 24];
        getrandom::fill(&mut random)
            .map_err(|_| "policy preview authoring handle entropy is unavailable".to_string())?;
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
