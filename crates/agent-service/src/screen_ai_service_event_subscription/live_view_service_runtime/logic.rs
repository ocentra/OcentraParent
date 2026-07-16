use ocentra_parent_agent_protocol::constants;
use ocentra_screen_live_view_core::live_view_runtime::{
    evaluate_screen_live_view_runtime, ScreenLiveViewRuntimeInput,
};
use ocentra_screen_live_view_core::live_view_worker::{
    evaluate_screen_live_view_worker_startup, start_screen_live_view_worker,
    ScreenLiveViewWorkerExecutionInput, ScreenLiveViewWorkerStartupInput,
};

#[path = "environment.rs"]
mod environment;

use super::ScreenLiveViewServiceRuntimeRecord;

pub(crate) fn spawn_screen_live_view_worker_runtime() {
    if !environment::env_flag(environment::EnvVarName(
        constants::screen_flow::SCREEN_LIVE_VIEW_RUNTIME_ENABLED_ENV,
    )) {
        return;
    }
    let _ = super::run_screen_live_view_worker_runtime();
}

pub(crate) fn run_screen_live_view_worker_runtime() -> ScreenLiveViewServiceRuntimeRecord {
    let runtime_input = runtime_input_from_environment();
    let runtime_decision = evaluate_screen_live_view_runtime(runtime_input);
    let startup_decision =
        evaluate_screen_live_view_worker_startup(ScreenLiveViewWorkerStartupInput {
            mode: runtime_input.mode,
            runtime_decision,
            platform_prompt_artifact_present: environment::env_flag(environment::EnvVarName(
                constants::screen_flow::SCREEN_LIVE_VIEW_PLATFORM_PROMPT_ARTIFACT_ENV,
            )),
            relay_cache_execution_proved: environment::env_flag(environment::EnvVarName(
                constants::screen_flow::SCREEN_LIVE_VIEW_RELAY_CACHE_EXECUTION_ENV,
            )),
            physical_device_parity_proved: environment::env_flag(environment::EnvVarName(
                constants::screen_flow::SCREEN_LIVE_VIEW_PHYSICAL_DEVICE_PARITY_ENV,
            )),
            privacy_legal_approved: environment::env_flag(environment::EnvVarName(
                constants::screen_flow::SCREEN_LIVE_VIEW_PRIVACY_LEGAL_APPROVAL_ENV,
            )),
        });
    let execution_record = start_screen_live_view_worker(ScreenLiveViewWorkerExecutionInput {
        startup_decision,
        cache_raw_frames: environment::env_flag(environment::EnvVarName(
            constants::screen_flow::SCREEN_LIVE_VIEW_CACHE_RAW_FRAMES_ENV,
        )),
        session_recording_allowed: environment::env_flag(environment::EnvVarName(
            constants::screen_flow::SCREEN_LIVE_VIEW_SESSION_RECORDING_ENV,
        )),
        remote_input_control_allowed: environment::env_flag(environment::EnvVarName(
            constants::screen_flow::SCREEN_LIVE_VIEW_REMOTE_INPUT_ENV,
        )),
    });

    ScreenLiveViewServiceRuntimeRecord {
        runtime_input,
        startup_decision,
        execution_record,
    }
}

fn runtime_input_from_environment() -> ScreenLiveViewRuntimeInput {
    ScreenLiveViewRuntimeInput {
        mode: environment::live_view_mode_from_environment(),
        transport: environment::live_view_transport_from_environment(),
        permission: environment::live_view_permission_from_environment(),
        live_transport_proof_present: environment::env_flag(environment::EnvVarName(
            constants::screen_flow::SCREEN_LIVE_VIEW_TRANSPORT_PROOF_ENV,
        )),
        raw_frame_deleted_after_transport: environment::env_flag(environment::EnvVarName(
            constants::screen_flow::SCREEN_LIVE_VIEW_DELETION_PROOF_ENV,
        )),
        parent_ui_persistence_proved: environment::env_flag(environment::EnvVarName(
            constants::screen_flow::SCREEN_LIVE_VIEW_PARENT_UI_PERSISTENCE_ENV,
        )),
        relay_cache_proved: environment::env_flag(environment::EnvVarName(
            constants::screen_flow::SCREEN_LIVE_VIEW_RELAY_CACHE_PROOF_ENV,
        )),
        cache_raw_frames: environment::env_flag(environment::EnvVarName(
            constants::screen_flow::SCREEN_LIVE_VIEW_CACHE_RAW_FRAMES_ENV,
        )),
        session_recording_allowed: environment::env_flag(environment::EnvVarName(
            constants::screen_flow::SCREEN_LIVE_VIEW_SESSION_RECORDING_ENV,
        )),
        remote_input_control_allowed: environment::env_flag(environment::EnvVarName(
            constants::screen_flow::SCREEN_LIVE_VIEW_REMOTE_INPUT_ENV,
        )),
    }
}
