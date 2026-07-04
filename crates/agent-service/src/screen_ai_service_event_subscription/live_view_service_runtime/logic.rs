use std::env;

use ocentra_parent_agent_protocol::constants;
use ocentra_screen_live_view_core::live_view_runtime::{
    evaluate_screen_live_view_runtime, ScreenLiveViewRuntimeInput, ScreenLiveViewRuntimeMode,
    ScreenLiveViewRuntimePermission, ScreenLiveViewRuntimeTransport,
};
use ocentra_screen_live_view_core::live_view_worker::{
    evaluate_screen_live_view_worker_startup, start_screen_live_view_worker,
    ScreenLiveViewWorkerExecutionInput, ScreenLiveViewWorkerExecutionRecord,
    ScreenLiveViewWorkerStartupDecision, ScreenLiveViewWorkerStartupInput,
};

use super::ScreenLiveViewServiceRuntimeRecord;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct EnvVarName(&'static str);

pub(crate) fn spawn_screen_live_view_worker_runtime() {
    if !env_flag(EnvVarName(
        constants::screen_flow::SCREEN_LIVE_VIEW_RUNTIME_ENABLED_ENV,
    )) {
        return;
    }
    let _ = run_screen_live_view_worker_runtime();
}

pub(crate) fn run_screen_live_view_worker_runtime() -> ScreenLiveViewServiceRuntimeRecord {
    let runtime_input = runtime_input_from_environment();
    let runtime_decision = evaluate_screen_live_view_runtime(runtime_input);
    let startup_decision =
        evaluate_screen_live_view_worker_startup(ScreenLiveViewWorkerStartupInput {
            mode: runtime_input.mode,
            runtime_decision,
            platform_prompt_artifact_present: env_flag(EnvVarName(
                constants::screen_flow::SCREEN_LIVE_VIEW_PLATFORM_PROMPT_ARTIFACT_ENV,
            )),
            relay_cache_execution_proved: env_flag(EnvVarName(
                constants::screen_flow::SCREEN_LIVE_VIEW_RELAY_CACHE_EXECUTION_ENV,
            )),
            physical_device_parity_proved: env_flag(EnvVarName(
                constants::screen_flow::SCREEN_LIVE_VIEW_PHYSICAL_DEVICE_PARITY_ENV,
            )),
            privacy_legal_approved: env_flag(EnvVarName(
                constants::screen_flow::SCREEN_LIVE_VIEW_PRIVACY_LEGAL_APPROVAL_ENV,
            )),
        });
    let execution_record = start_screen_live_view_worker(ScreenLiveViewWorkerExecutionInput {
        startup_decision,
        cache_raw_frames: env_flag(EnvVarName(
            constants::screen_flow::SCREEN_LIVE_VIEW_CACHE_RAW_FRAMES_ENV,
        )),
        session_recording_allowed: env_flag(EnvVarName(
            constants::screen_flow::SCREEN_LIVE_VIEW_SESSION_RECORDING_ENV,
        )),
        remote_input_control_allowed: env_flag(EnvVarName(
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
        mode: live_view_mode_from_environment(),
        transport: live_view_transport_from_environment(),
        permission: live_view_permission_from_environment(),
        live_transport_proof_present: env_flag(EnvVarName(
            constants::screen_flow::SCREEN_LIVE_VIEW_TRANSPORT_PROOF_ENV,
        )),
        raw_frame_deleted_after_transport: env_flag(EnvVarName(
            constants::screen_flow::SCREEN_LIVE_VIEW_DELETION_PROOF_ENV,
        )),
        parent_ui_persistence_proved: env_flag(EnvVarName(
            constants::screen_flow::SCREEN_LIVE_VIEW_PARENT_UI_PERSISTENCE_ENV,
        )),
        relay_cache_proved: env_flag(EnvVarName(
            constants::screen_flow::SCREEN_LIVE_VIEW_RELAY_CACHE_PROOF_ENV,
        )),
        cache_raw_frames: env_flag(EnvVarName(
            constants::screen_flow::SCREEN_LIVE_VIEW_CACHE_RAW_FRAMES_ENV,
        )),
        session_recording_allowed: env_flag(EnvVarName(
            constants::screen_flow::SCREEN_LIVE_VIEW_SESSION_RECORDING_ENV,
        )),
        remote_input_control_allowed: env_flag(EnvVarName(
            constants::screen_flow::SCREEN_LIVE_VIEW_REMOTE_INPUT_ENV,
        )),
    }
}

fn live_view_mode_from_environment() -> ScreenLiveViewRuntimeMode {
    match env::var(constants::screen_flow::SCREEN_LIVE_VIEW_MODE_ENV).ok() {
        Some(value) if value == constants::screen_flow::SCREEN_LIVE_VIEW_MODE_LAN_ONLY => {
            ScreenLiveViewRuntimeMode::LanOnlyView
        }
        Some(value) if value == constants::screen_flow::SCREEN_LIVE_VIEW_MODE_RELAY_BACKED => {
            ScreenLiveViewRuntimeMode::RelayBackedView
        }
        _ => ScreenLiveViewRuntimeMode::Disabled,
    }
}

fn live_view_transport_from_environment() -> ScreenLiveViewRuntimeTransport {
    match env::var(constants::screen_flow::SCREEN_LIVE_VIEW_TRANSPORT_ENV).ok() {
        Some(value)
            if value == constants::screen_flow::SCREEN_LIVE_VIEW_TRANSPORT_LAN_MUTUAL_AUTH =>
        {
            ScreenLiveViewRuntimeTransport::LanMutualAuth
        }
        Some(value) if value == constants::screen_flow::SCREEN_LIVE_VIEW_TRANSPORT_RELAY_E2EE => {
            ScreenLiveViewRuntimeTransport::RelayEndToEndEncrypted
        }
        _ => ScreenLiveViewRuntimeTransport::None,
    }
}

fn live_view_permission_from_environment() -> ScreenLiveViewRuntimePermission {
    match env::var(constants::screen_flow::SCREEN_LIVE_VIEW_PERMISSION_ENV).ok() {
        Some(value)
            if value == constants::screen_flow::SCREEN_LIVE_VIEW_PERMISSION_CAPTURE_ONLY =>
        {
            ScreenLiveViewRuntimePermission::ScreenCaptureOnly
        }
        Some(value) if value == constants::screen_flow::SCREEN_LIVE_VIEW_PERMISSION_LIVE_VIEW => {
            ScreenLiveViewRuntimePermission::LiveViewPermission
        }
        _ => ScreenLiveViewRuntimePermission::Missing,
    }
}

fn env_flag(env_var_name: EnvVarName) -> bool {
    env::var(env_var_name.0).is_ok_and(|value| value == constants::screen_flow::ENV_TRUE)
}
