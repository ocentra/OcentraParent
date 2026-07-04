use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::activity_query::ActivityRecentSummary;
use ocentra_parent_agent_protocol::activity_surface::{
    ActivityAppUseReadModel, ActivityAppUseReadModelRow, ActivityReadModelState,
    ActivitySurfaceRequest,
};
use ocentra_parent_agent_protocol::app_game::{
    AppGameForegroundEvidenceRow, AppGameInventoryEvidenceRow, AppGameRuntimeEvidenceRow,
    AppGameServiceReadModel, APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED,
    APP_GAME_CLASSIFICATION_ADAPTER_ERROR, APP_GAME_CLASSIFICATION_KNOWN_APP,
    APP_GAME_CLASSIFICATION_PERMISSION_LIMITED, APP_GAME_CLASSIFICATION_STALE,
    APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS, APP_GAME_CLASSIFICATION_UNSUPPORTED_PLATFORM,
    APP_GAME_FOREGROUND_NOT_CLAIMED, APP_GAME_INVENTORY_STATE_UNAVAILABLE,
    APP_GAME_PRODUCT_NATIVE_APP, APP_GAME_PRODUCT_UNKNOWN_EXECUTABLE, APP_GAME_RUNTIME_NOT_CLAIMED,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::ACTIVITY_SURFACE_SCHEMA_VERSION;

use crate::activity_surface_read_model_states::{
    empty_app_use_read_model, offline_app_use_read_model, request_targets_remote_device,
    unavailable_app_use_read_model,
};
use crate::time::timestamp_now;

use super::shared::{
    app_game_boundary_row_counts, app_game_source_status_rows, push_app_game_boundary_evidence,
    push_evidence, row_device_id, row_state, CapabilityStatus,
};
use source::AppUseReadModelSource;

#[path = "app_use/helpers.rs"]
mod helpers;
#[path = "app_use/source.rs"]
mod source;

#[derive(Clone, Debug, PartialEq, Eq)]
struct AppText(String);

#[derive(Clone, Debug, PartialEq, Eq)]
struct AppMaybeText(Option<String>);

#[derive(Clone, Debug, PartialEq, Eq)]
struct ClassificationText(String);

pub(crate) fn app_use_read_model<T>(
    request: ActivitySurfaceRequest,
    source: T,
) -> ActivityAppUseReadModel
where
    T: Into<AppUseReadModelSource>,
{
    if request_targets_remote_device(&request) {
        return offline_app_use_read_model(request);
    }

    match source.into() {
        AppUseReadModelSource::AppGame(model) => app_use_model_from_app_game(request, *model),
        AppUseReadModelSource::Recent(summary) => {
            app_use_model_from_recent_summary(request, summary)
        }
    }
}

fn app_use_model_from_app_game(
    request: ActivitySurfaceRequest,
    model: Option<AppGameServiceReadModel>,
) -> ActivityAppUseReadModel {
    match model {
        Some(model) => app_use_model_from_rows(request, &model),
        None => unavailable_app_use_read_model(request),
    }
}

fn app_use_model_from_rows(
    request: ActivitySurfaceRequest,
    model: &AppGameServiceReadModel,
) -> ActivityAppUseReadModel {
    helpers::app_use_model_from_rows(request, model)
}

fn app_use_model_from_recent_summary(
    request: ActivitySurfaceRequest,
    summary: Option<ActivityRecentSummary>,
) -> ActivityAppUseReadModel {
    helpers::app_use_model_from_recent_summary(request, summary)
}

fn app_use_recent_row(
    request: &ActivitySurfaceRequest,
    summary: ActivityRecentSummary,
) -> ActivityAppUseReadModelRow {
    helpers::app_use_recent_row(request, summary)
}
