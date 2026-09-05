use super::*;

pub(super) fn opaque_handle() -> Result<String, String> {
    for _ in 0..4 {
        let mut random = [0_u8; 24];
        getrandom::fill(&mut random)
            .map_err(|error| format!("parent resolution handle entropy is unavailable: {error}"))?;
        let handle = format!(
            "pprh-{}",
            random
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        );
        if !store()
            .lock()
            .map_err(|error| format!("parent resolution store is unavailable: {error}"))?
            .entries
            .contains_key(&handle)
        {
            return Ok(handle);
        }
    }
    Err("parent resolution handle generation collided".to_string())
}

pub(super) fn local_controller_actor_id(
    read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> Result<String, String> {
    let read_model = read_model.ok_or_else(|| {
        "local controller authority is unavailable; manual review required".to_string()
    })?;
    let mut actor_ids = read_model
        .household_device_decisions
        .iter()
        .filter(|decision| decision.revoked_at.is_none())
        .map(|decision| decision.parent_actor_id.as_str())
        .collect::<Vec<_>>();
    actor_ids.sort_unstable();
    actor_ids.dedup();
    match actor_ids.as_slice() {
        [actor_id] => Ok((*actor_id).to_string()),
        _ => Err(
            "local controller actor is unavailable or ambiguous; manual review required"
                .to_string(),
        ),
    }
}

pub(super) fn store_new(handle: &str, stored: StoredParentResolution) -> Result<(), String> {
    let mut resolution_store = store()
        .lock()
        .map_err(|error| format!("parent resolution store is unavailable: {error}"))?;
    let now = Instant::now();
    resolution_store
        .entries
        .retain(|_, entry| now.duration_since(entry.issued_at) <= STAGED_RESOLUTION_TTL);
    if resolution_store.entries.values().any(|entry| {
        entry.preview_id == stored.preview_id && entry.parent_actor_id == stored.parent_actor_id
    }) {
        return Err(
            "parent resolution already has a relay attempt; refresh for manual review".to_string(),
        );
    }
    while resolution_store.entries.len() >= MAX_STAGED_RESOLUTIONS {
        let Some(oldest_handle) = resolution_store
            .entries
            .iter()
            .min_by_key(|(_, entry)| entry.issued_at)
            .map(|(existing_handle, _)| existing_handle.clone())
        else {
            break;
        };
        resolution_store.entries.remove(&oldest_handle);
    }
    resolution_store.entries.insert(handle.to_string(), stored);
    Ok(())
}
