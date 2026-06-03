use ocentra_parent_agent_protocol::{
    constants, ActivityEvidenceRef, ActivityReadModelState, ActivitySurfaceRequest,
    APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR, APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED,
    APP_GAME_CAPABILITY_STATUS_STALE, APP_GAME_CAPABILITY_STATUS_UNAVAILABLE,
    APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM,
};

pub(super) fn row_device_id(request: &ActivitySurfaceRequest) -> String {
    request
        .scope
        .device_id
        .clone()
        .unwrap_or_else(|| constants::activity_surface::DEFAULT_DEVICE_ID.to_string())
}

pub(super) fn row_state(capability_status: &str) -> ActivityReadModelState {
    match capability_status {
        APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED => ActivityReadModelState::PermissionRequired,
        APP_GAME_CAPABILITY_STATUS_STALE => ActivityReadModelState::Stale,
        APP_GAME_CAPABILITY_STATUS_UNAVAILABLE
        | APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM
        | APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR => ActivityReadModelState::Unavailable,
        _ => ActivityReadModelState::Ready,
    }
}

pub(super) fn push_evidence(target: &mut Vec<ActivityEvidenceRef>, rows: &[ActivityEvidenceRef]) {
    for evidence in rows {
        if target
            .iter()
            .any(|candidate| candidate.evidence_id == evidence.evidence_id)
        {
            continue;
        }
        target.push(evidence.clone());
    }
}
