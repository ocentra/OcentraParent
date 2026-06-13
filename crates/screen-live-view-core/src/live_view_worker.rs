use crate::live_view_runtime::{ScreenLiveViewRuntimeDecision, ScreenLiveViewRuntimeMode};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenLiveViewWorkerStartupState {
    Disabled,
    Blocked,
    ReadyToStart,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenLiveViewWorkerStartupBlockReason {
    RuntimeNotReady,
    MissingPlatformPromptArtifact,
    MissingRelayCacheExecution,
    MissingPhysicalDeviceParity,
    MissingPrivacyLegalApproval,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenLiveViewWorkerExecutionState {
    NotStarted,
    Started,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenLiveViewWorkerExecutionBlockReason {
    StartupNotPermitted,
    UnsafeRetentionOrControl,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScreenLiveViewWorkerStartupInput {
    pub mode: ScreenLiveViewRuntimeMode,
    pub runtime_decision: ScreenLiveViewRuntimeDecision,
    pub platform_prompt_artifact_present: bool,
    pub relay_cache_execution_proved: bool,
    pub physical_device_parity_proved: bool,
    pub privacy_legal_approved: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScreenLiveViewWorkerStartupDecision {
    pub startup_state: ScreenLiveViewWorkerStartupState,
    pub block_reason: Option<ScreenLiveViewWorkerStartupBlockReason>,
    pub startup_permitted: bool,
    pub worker_started: bool,
    pub product_live_view_ready: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScreenLiveViewWorkerExecutionInput {
    pub startup_decision: ScreenLiveViewWorkerStartupDecision,
    pub cache_raw_frames: bool,
    pub session_recording_allowed: bool,
    pub remote_input_control_allowed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScreenLiveViewWorkerExecutionRecord {
    pub execution_state: ScreenLiveViewWorkerExecutionState,
    pub block_reason: Option<ScreenLiveViewWorkerExecutionBlockReason>,
    pub startup_permitted: bool,
    pub worker_started: bool,
    pub product_live_view_ready: bool,
}

pub fn evaluate_screen_live_view_worker_startup(
    input: ScreenLiveViewWorkerStartupInput,
) -> ScreenLiveViewWorkerStartupDecision {
    if input.mode == ScreenLiveViewRuntimeMode::Disabled {
        return ScreenLiveViewWorkerStartupDecision {
            startup_state: ScreenLiveViewWorkerStartupState::Disabled,
            block_reason: None,
            startup_permitted: false,
            worker_started: false,
            product_live_view_ready: false,
        };
    }

    if !input.runtime_decision.product_live_view_ready {
        return blocked(ScreenLiveViewWorkerStartupBlockReason::RuntimeNotReady);
    }

    if !input.platform_prompt_artifact_present {
        return blocked(ScreenLiveViewWorkerStartupBlockReason::MissingPlatformPromptArtifact);
    }

    if input.mode == ScreenLiveViewRuntimeMode::RelayBackedView
        && !input.relay_cache_execution_proved
    {
        return blocked(ScreenLiveViewWorkerStartupBlockReason::MissingRelayCacheExecution);
    }

    if !input.physical_device_parity_proved {
        return blocked(ScreenLiveViewWorkerStartupBlockReason::MissingPhysicalDeviceParity);
    }

    if !input.privacy_legal_approved {
        return blocked(ScreenLiveViewWorkerStartupBlockReason::MissingPrivacyLegalApproval);
    }

    ScreenLiveViewWorkerStartupDecision {
        startup_state: ScreenLiveViewWorkerStartupState::ReadyToStart,
        block_reason: None,
        startup_permitted: true,
        worker_started: false,
        product_live_view_ready: true,
    }
}

pub fn start_screen_live_view_worker(
    input: ScreenLiveViewWorkerExecutionInput,
) -> ScreenLiveViewWorkerExecutionRecord {
    if !input.startup_decision.startup_permitted {
        return worker_not_started(ScreenLiveViewWorkerExecutionBlockReason::StartupNotPermitted);
    }

    if input.cache_raw_frames
        || input.session_recording_allowed
        || input.remote_input_control_allowed
    {
        return worker_not_started(
            ScreenLiveViewWorkerExecutionBlockReason::UnsafeRetentionOrControl,
        );
    }

    ScreenLiveViewWorkerExecutionRecord {
        execution_state: ScreenLiveViewWorkerExecutionState::Started,
        block_reason: None,
        startup_permitted: true,
        worker_started: true,
        product_live_view_ready: input.startup_decision.product_live_view_ready,
    }
}

fn blocked(reason: ScreenLiveViewWorkerStartupBlockReason) -> ScreenLiveViewWorkerStartupDecision {
    ScreenLiveViewWorkerStartupDecision {
        startup_state: ScreenLiveViewWorkerStartupState::Blocked,
        block_reason: Some(reason),
        startup_permitted: false,
        worker_started: false,
        product_live_view_ready: false,
    }
}

fn worker_not_started(
    reason: ScreenLiveViewWorkerExecutionBlockReason,
) -> ScreenLiveViewWorkerExecutionRecord {
    ScreenLiveViewWorkerExecutionRecord {
        execution_state: ScreenLiveViewWorkerExecutionState::NotStarted,
        block_reason: Some(reason),
        startup_permitted: false,
        worker_started: false,
        product_live_view_ready: false,
    }
}
