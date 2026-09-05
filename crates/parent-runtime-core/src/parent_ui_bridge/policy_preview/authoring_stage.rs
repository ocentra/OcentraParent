use super::*;

pub(super) fn stage(
    payload: &Value,
    read_model: &ParentPolicyPreviewReadModelSnapshot,
    parent_access_state: &ParentPortalParentAccessState,
) -> Result<String, String> {
    let preview_id = read_model
        .preview_id
        .as_ref()
        .ok_or_else(|| "policy preview identifier is missing".to_string())?;
    let draft_text = exact_payload_field(payload, DRAFT_PAYLOAD_FIELD, "draft")?
        .as_str()
        .ok_or_else(|| "policy preview draft payload is missing".to_string())?;
    let draft: PolicyPreviewAuthoringDraftInput = serde_json::from_str(draft_text)
        .map_err(|error| format!("policy preview draft payload is invalid: {error}"))?;
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
        .map_err(|error| format!("policy preview authoring store is unavailable: {error}"))?;
    let now = Instant::now();
    authoring_store
        .entries
        .retain(|_, entry| now.duration_since(entry.issued_at) <= STAGED_DRAFT_TTL);
    if authoring_store.entries.values().any(|entry| {
        entry.preview_id == preview_id.as_str()
            && entry.parent_access_state == *parent_access_state
            && entry.in_flight
    }) {
        return Err("policy preview confirmation is already pending".to_string());
    }
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
            read_model: read_model.clone(),
            target_value: target_value.to_string(),
            requested_action,
            parent_access_state: parent_access_state.clone(),
            issued_at: now,
            in_flight: false,
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
                && !entry.in_flight
                && Instant::now().duration_since(entry.issued_at) <= STAGED_DRAFT_TTL
        })
        .map(|(handle, entry)| StagedPolicyPreviewDraft {
            handle: handle.clone(),
            read_model: entry.read_model.clone(),
            target_value: entry.target_value.clone(),
            requested_action: entry.requested_action.clone(),
        })
}
