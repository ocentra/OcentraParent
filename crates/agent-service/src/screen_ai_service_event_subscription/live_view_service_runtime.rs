#[path = "live_view_service_runtime/logic.rs"]
mod logic;

use ocentra_screen_live_view_core::live_view_runtime::ScreenLiveViewRuntimeInput;
use ocentra_screen_live_view_core::live_view_worker::{
    ScreenLiveViewWorkerExecutionRecord, ScreenLiveViewWorkerStartupDecision,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ScreenLiveViewServiceRuntimeRecord {
    pub(crate) runtime_input: ScreenLiveViewRuntimeInput,
    pub(crate) startup_decision: ScreenLiveViewWorkerStartupDecision,
    pub(crate) execution_record: ScreenLiveViewWorkerExecutionRecord,
}

pub(crate) fn spawn_screen_live_view_worker_runtime() {
    logic::spawn_screen_live_view_worker_runtime();
}

pub(crate) fn run_screen_live_view_worker_runtime() -> ScreenLiveViewServiceRuntimeRecord {
    logic::run_screen_live_view_worker_runtime()
}
