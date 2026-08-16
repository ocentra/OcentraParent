use ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryOutcome;
use ocentra_parent_agent_core::enforcement_timer_state::active_timer_state_from_outcome_with_app_game_session;
use ocentra_parent_agent_protocol::enforcement::{
    AppGameTimerSessionBinding, EnforcementActiveTimerState,
};

use crate::enforcement_timer_state_path::EnforcementTimerStatePath;

#[path = "enforcement_timer_state_file/io.rs"]
mod io;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EnforcementTimerStateFileError {
    ActiveTimerStateRequired,
    Store,
    Serialize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct EnforcementTimerStoredAtTextRef<'a>(pub(crate) &'a str);

pub(crate) trait EnforcementTimerStoredAtSource {
    fn as_timer_stored_at_text_ref(&self) -> EnforcementTimerStoredAtTextRef<'_>;
}

impl EnforcementTimerStoredAtSource for EnforcementTimerStoredAtTextRef<'_> {
    fn as_timer_stored_at_text_ref(&self) -> EnforcementTimerStoredAtTextRef<'_> {
        *self
    }
}

impl EnforcementTimerStoredAtSource for String {
    fn as_timer_stored_at_text_ref(&self) -> EnforcementTimerStoredAtTextRef<'_> {
        EnforcementTimerStoredAtTextRef(self.as_str())
    }
}

impl EnforcementTimerStoredAtSource for &String {
    fn as_timer_stored_at_text_ref(&self) -> EnforcementTimerStoredAtTextRef<'_> {
        EnforcementTimerStoredAtTextRef(self.as_str())
    }
}

impl EnforcementTimerStoredAtSource for &str {
    fn as_timer_stored_at_text_ref(&self) -> EnforcementTimerStoredAtTextRef<'_> {
        EnforcementTimerStoredAtTextRef(self)
    }
}

pub(crate) async fn store_active_timer_state_for_outcome_with_app_game_session(
    outcome: &EnforcementBoundaryOutcome,
    path: &EnforcementTimerStatePath,
    stored_at: impl EnforcementTimerStoredAtSource,
    app_game_session: Option<AppGameTimerSessionBinding>,
) -> Result<Option<EnforcementActiveTimerState>, EnforcementTimerStateFileError> {
    match active_timer_state_from_outcome_with_app_game_session(
        outcome,
        stored_at.as_timer_stored_at_text_ref().0,
        app_game_session,
    ) {
        Some(state) => {
            io::write_active_timer_state(path, &state).await?;
            Ok(Some(state))
        }
        None => Ok(None),
    }
}

pub(crate) async fn read_active_timer_state(
    path: &EnforcementTimerStatePath,
) -> Result<Option<EnforcementActiveTimerState>, EnforcementTimerStateFileError> {
    io::read_active_timer_state(path).await
}

pub(crate) async fn remove_active_timer_state(
    path: &EnforcementTimerStatePath,
) -> Result<(), EnforcementTimerStateFileError> {
    io::remove_active_timer_state(path).await
}
