use super::*;

pub(super) fn consume(
    payload: &Value,
    preview_id: &ParentPolicyPreviewId,
    parent_access_state: &ParentPortalParentAccessState,
) -> Result<StagedPolicyPreviewDraft, String> {
    let handle = exact_payload_field(payload, HANDLE_PAYLOAD_FIELD, "handle")?
        .as_str()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "policy preview authoring handle is missing".to_string())?;
    let mut authoring_store = store()
        .lock()
        .map_err(|error| format!("policy preview authoring store is unavailable: {error}"))?;
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
    if entry.in_flight {
        return Err("policy preview confirmation is already pending".to_string());
    }
    let entry = authoring_store.entries.get_mut(handle).ok_or_else(|| {
        "policy preview authoring handle is unknown or already consumed".to_string()
    })?;
    entry.in_flight = true;
    Ok(StagedPolicyPreviewDraft {
        handle: handle.to_string(),
        read_model: entry.read_model.clone(),
        target_value: entry.target_value.clone(),
        requested_action: entry.requested_action.clone(),
    })
}
