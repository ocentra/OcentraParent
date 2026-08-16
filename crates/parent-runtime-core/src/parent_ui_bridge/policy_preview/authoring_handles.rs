use super::*;

pub(super) fn commit(
    draft: &StagedPolicyPreviewDraft,
    preview_id: &ParentPolicyPreviewId,
    parent_access_state: &ParentPortalParentAccessState,
) -> Result<(), String> {
    let mut authoring_store = store()
        .lock()
        .map_err(|_| "policy preview authoring store is unavailable".to_string())?;
    let Some(entry) = authoring_store.entries.get(&draft.handle) else {
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
    if !entry.in_flight {
        return Err("policy preview authoring handle is not pending confirmation".to_string());
    }
    authoring_store.entries.remove(&draft.handle);
    Ok(())
}

pub(super) fn release(
    draft: &StagedPolicyPreviewDraft,
    preview_id: &ParentPolicyPreviewId,
    parent_access_state: &ParentPortalParentAccessState,
) -> Result<(), String> {
    let mut authoring_store = store()
        .lock()
        .map_err(|_| "policy preview authoring store is unavailable".to_string())?;
    let Some(entry) = authoring_store.entries.get_mut(&draft.handle) else {
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
    entry.in_flight = false;
    Ok(())
}

pub(super) fn cancel(
    payload: &Value,
    preview_id: &ParentPolicyPreviewId,
    parent_access_state: &ParentPortalParentAccessState,
) -> Result<(), String> {
    if payload.as_object().is_some_and(|object| object.is_empty()) {
        return Ok(());
    }
    let handle = exact_payload_field(payload, HANDLE_PAYLOAD_FIELD, "handle")?
        .as_str()
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
    if entry.in_flight {
        return Err("policy preview confirmation is already pending".to_string());
    }
    authoring_store.entries.remove(handle);
    Ok(())
}
