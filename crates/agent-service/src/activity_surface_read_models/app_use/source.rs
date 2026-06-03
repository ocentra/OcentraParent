use ocentra_parent_agent_protocol::{ActivityRecentSummary, AppGameServiceReadModel};

pub(crate) enum AppUseReadModelSource {
    AppGame(Option<AppGameServiceReadModel>),
    Recent(Option<ActivityRecentSummary>),
}

impl From<Option<AppGameServiceReadModel>> for AppUseReadModelSource {
    fn from(model: Option<AppGameServiceReadModel>) -> Self {
        Self::AppGame(model)
    }
}

impl From<Option<ActivityRecentSummary>> for AppUseReadModelSource {
    fn from(summary: Option<ActivityRecentSummary>) -> Self {
        Self::Recent(summary)
    }
}
