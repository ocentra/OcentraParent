use ocentra_parent_agent_protocol::app_game::AppGameSessionSummary;

use crate::{
    activity_store_app_game::app_game_session_summaries, ActivityStore, ActivityStoreError,
};

impl ActivityStore {
    pub fn app_game_session_summaries(
        &self,
        limit: u64,
    ) -> Result<Vec<AppGameSessionSummary>, ActivityStoreError> {
        app_game_session_summaries(&self.connection, limit)
    }
}
