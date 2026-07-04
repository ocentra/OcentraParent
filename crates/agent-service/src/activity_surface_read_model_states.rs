use ocentra_parent_agent_protocol::activity_surface::{
    ActivityAppUseReadModel, ActivityBrowserReadModel, ActivityGamesReadModel,
    ActivityNetworkReadModel, ActivityReadModelState, ActivityScreenReadModel,
    ActivitySurfaceRequest, ActivitySurfaceScopeKind,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::ACTIVITY_SURFACE_SCHEMA_VERSION;

use crate::time::timestamp_now;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct GeneratedAtText(pub(crate) String);

pub(crate) fn request_targets_remote_device(request: &ActivitySurfaceRequest) -> bool {
    request.scope.scope_kind == ActivitySurfaceScopeKind::Device
        && request.scope.device_id.as_deref()
            != Some(constants::activity_surface::DEFAULT_DEVICE_ID)
}

pub(crate) fn empty_screen_read_model(
    request: ActivitySurfaceRequest,
    generated_at: GeneratedAtText,
) -> ActivityScreenReadModel {
    ActivityScreenReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request,
        state: ActivityReadModelState::Empty,
        generated_at: generated_at.0,
        summary: constants::activity_surface::SUMMARY_EMPTY.to_string(),
        rows: Vec::new(),
    }
}

pub(crate) fn unavailable_screen_read_model(
    request: ActivitySurfaceRequest,
) -> ActivityScreenReadModel {
    ActivityScreenReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request,
        state: ActivityReadModelState::Unavailable,
        generated_at: timestamp_now(),
        summary: constants::activity_surface::SUMMARY_STORE_UNAVAILABLE.to_string(),
        rows: Vec::new(),
    }
}

pub(crate) fn offline_screen_read_model(
    request: ActivitySurfaceRequest,
) -> ActivityScreenReadModel {
    ActivityScreenReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request,
        state: ActivityReadModelState::Offline,
        generated_at: timestamp_now(),
        summary: constants::activity_surface::SUMMARY_DEVICE_OFFLINE.to_string(),
        rows: Vec::new(),
    }
}

pub(crate) fn empty_app_use_read_model(request: ActivitySurfaceRequest) -> ActivityAppUseReadModel {
    ActivityAppUseReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request,
        state: ActivityReadModelState::Empty,
        generated_at: timestamp_now(),
        summary: constants::activity_surface::SUMMARY_EMPTY.to_string(),
        rows: Vec::new(),
    }
}

pub(crate) fn unavailable_app_use_read_model(
    request: ActivitySurfaceRequest,
) -> ActivityAppUseReadModel {
    ActivityAppUseReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request,
        state: ActivityReadModelState::Unavailable,
        generated_at: timestamp_now(),
        summary: constants::activity_surface::SUMMARY_STORE_UNAVAILABLE.to_string(),
        rows: Vec::new(),
    }
}

pub(crate) fn offline_app_use_read_model(
    request: ActivitySurfaceRequest,
) -> ActivityAppUseReadModel {
    ActivityAppUseReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request,
        state: ActivityReadModelState::Offline,
        generated_at: timestamp_now(),
        summary: constants::activity_surface::SUMMARY_DEVICE_OFFLINE.to_string(),
        rows: Vec::new(),
    }
}

pub(crate) fn unavailable_games_read_model(
    request: ActivitySurfaceRequest,
) -> ActivityGamesReadModel {
    ActivityGamesReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request,
        state: ActivityReadModelState::Unavailable,
        generated_at: timestamp_now(),
        summary: constants::activity_surface::SUMMARY_STORE_UNAVAILABLE.to_string(),
        rows: Vec::new(),
    }
}

pub(crate) fn offline_games_read_model(request: ActivitySurfaceRequest) -> ActivityGamesReadModel {
    ActivityGamesReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request,
        state: ActivityReadModelState::Offline,
        generated_at: timestamp_now(),
        summary: constants::activity_surface::SUMMARY_DEVICE_OFFLINE.to_string(),
        rows: Vec::new(),
    }
}

pub(crate) fn unavailable_browser_read_model(
    request: ActivitySurfaceRequest,
) -> ActivityBrowserReadModel {
    ActivityBrowserReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request,
        state: ActivityReadModelState::Unavailable,
        generated_at: timestamp_now(),
        summary: constants::activity_surface::SUMMARY_STORE_UNAVAILABLE.to_string(),
        rows: Vec::new(),
    }
}

pub(crate) fn offline_browser_read_model(
    request: ActivitySurfaceRequest,
) -> ActivityBrowserReadModel {
    ActivityBrowserReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request,
        state: ActivityReadModelState::Offline,
        generated_at: timestamp_now(),
        summary: constants::activity_surface::SUMMARY_DEVICE_OFFLINE.to_string(),
        rows: Vec::new(),
    }
}

pub(crate) fn unavailable_network_read_model(
    request: ActivitySurfaceRequest,
) -> ActivityNetworkReadModel {
    ActivityNetworkReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request,
        state: ActivityReadModelState::Unavailable,
        generated_at: timestamp_now(),
        summary: constants::activity_surface::SUMMARY_STORE_UNAVAILABLE.to_string(),
        rows: Vec::new(),
    }
}

pub(crate) fn offline_network_read_model(
    request: ActivitySurfaceRequest,
) -> ActivityNetworkReadModel {
    ActivityNetworkReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request,
        state: ActivityReadModelState::Offline,
        generated_at: timestamp_now(),
        summary: constants::activity_surface::SUMMARY_DEVICE_OFFLINE.to_string(),
        rows: Vec::new(),
    }
}
