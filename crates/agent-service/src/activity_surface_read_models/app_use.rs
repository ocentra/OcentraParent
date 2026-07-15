use ocentra_parent_agent_protocol::activity_query::ActivityRecentSummary;
use ocentra_parent_agent_protocol::activity_surface::{
    ActivityAppUseReadModel, ActivitySurfaceRequest,
};
use ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel;

use crate::activity_surface_read_model_states::{
    offline_app_use_read_model, request_targets_remote_device, unavailable_app_use_read_model,
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
