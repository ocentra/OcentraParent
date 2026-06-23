use ocentra_parent_agent_protocol::activity_query::ActivityRecentSummary;
use ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel;

pub(crate) enum AppUseReadModelSource {
    AppGame(Box<Option<AppGameServiceReadModel>>),
    Recent(Option<ActivityRecentSummary>),
}

impl From<Option<AppGameServiceReadModel>> for AppUseReadModelSource {
    fn from(model: Option<AppGameServiceReadModel>) -> Self {
        Self::AppGame(Box::new(model))
    }
}

impl From<Option<ActivityRecentSummary>> for AppUseReadModelSource {
    fn from(summary: Option<ActivityRecentSummary>) -> Self {
        Self::Recent(summary)
    }
}
