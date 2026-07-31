use crate::{EventingError, NdjsonEventJournal, ReplayFilter, ReplayMode, ReplayReadReport};

#[path = "read/runner.rs"]
mod runner;

#[path = "read/records.rs"]
mod records;

impl NdjsonEventJournal {
    pub async fn replay_projection(
        &self,
        filter: ReplayFilter,
    ) -> Result<ReplayReadReport, EventingError> {
        self.read(filter, ReplayMode::ProjectionOnly).await
    }

    pub async fn replay_action_records(
        &self,
        filter: ReplayFilter,
    ) -> Result<ReplayReadReport, EventingError> {
        self.read(filter, ReplayMode::ActionHandlersAllowed).await
    }

    async fn read(
        &self,
        filter: ReplayFilter,
        mode: ReplayMode,
    ) -> Result<ReplayReadReport, EventingError> {
        runner::read(self, filter, mode).await
    }
}
