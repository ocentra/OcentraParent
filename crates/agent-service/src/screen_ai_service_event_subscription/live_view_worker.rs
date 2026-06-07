use super::live_view_runtime::{ScreenLiveViewRuntimeDecision, ScreenLiveViewRuntimeMode};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ScreenLiveViewWorkerStartupState {
    Disabled,
    Blocked,
    ReadyToStart,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ScreenLiveViewWorkerStartupBlockReason {
    RuntimeNotReady,
    MissingPlatformPromptArtifact,
    MissingRelayCacheExecution,
    MissingPhysicalDeviceParity,
    MissingPrivacyLegalApproval,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ScreenLiveViewWorkerStartupInput {
    pub(crate) mode: ScreenLiveViewRuntimeMode,
    pub(crate) runtime_decision: ScreenLiveViewRuntimeDecision,
    pub(crate) platform_prompt_artifact_present: bool,
    pub(crate) relay_cache_execution_proved: bool,
    pub(crate) physical_device_parity_proved: bool,
    pub(crate) privacy_legal_approved: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ScreenLiveViewWorkerStartupDecision {
    pub(crate) startup_state: ScreenLiveViewWorkerStartupState,
    pub(crate) block_reason: Option<ScreenLiveViewWorkerStartupBlockReason>,
    pub(crate) worker_started: bool,
    pub(crate) product_live_view_ready: bool,
}

pub(crate) fn evaluate_screen_live_view_worker_startup(
    input: ScreenLiveViewWorkerStartupInput,
) -> ScreenLiveViewWorkerStartupDecision {
    if input.mode == ScreenLiveViewRuntimeMode::Disabled {
        return ScreenLiveViewWorkerStartupDecision {
            startup_state: ScreenLiveViewWorkerStartupState::Disabled,
            block_reason: None,
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
        worker_started: true,
        product_live_view_ready: true,
    }
}

fn blocked(reason: ScreenLiveViewWorkerStartupBlockReason) -> ScreenLiveViewWorkerStartupDecision {
    ScreenLiveViewWorkerStartupDecision {
        startup_state: ScreenLiveViewWorkerStartupState::Blocked,
        block_reason: Some(reason),
        worker_started: false,
        product_live_view_ready: false,
    }
}
