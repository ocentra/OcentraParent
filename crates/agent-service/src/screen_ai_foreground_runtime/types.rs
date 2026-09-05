use crate::screen_ai_cadence_runtime_event::ScreenAiServiceCaptureClock;

pub(crate) type ScreenAiForegroundTickClock = ScreenAiServiceCaptureClock;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenAiForegroundKey(pub(crate) String);
