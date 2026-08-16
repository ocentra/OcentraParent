use std::env;
use std::sync::Mutex;

use ocentra_parent_agent_protocol::constants;
use ocentra_screen_live_view_core::live_view_runtime::{
    evaluate_screen_live_view_runtime, ScreenLiveViewRuntimeBlockReason,
    ScreenLiveViewRuntimeInput, ScreenLiveViewRuntimeMode, ScreenLiveViewRuntimeSessionState,
};
use ocentra_screen_live_view_core::live_view_worker::{
    ScreenLiveViewWorkerExecutionBlockReason, ScreenLiveViewWorkerExecutionState,
    ScreenLiveViewWorkerStartupBlockReason, ScreenLiveViewWorkerStartupState,
};

use super::live_view_service_runtime::run_screen_live_view_worker_runtime;
use crate::test_invariants::require_ok;

static LIVE_VIEW_ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn service_runtime_defaults_to_disabled_without_env_gates() {
    with_clean_live_view_env(|| {
        let record = run_screen_live_view_worker_runtime();

        assert_eq!(
            record.runtime_input.mode,
            ScreenLiveViewRuntimeMode::Disabled
        );
        assert_eq!(
            record.startup_decision.startup_state,
            ScreenLiveViewWorkerStartupState::Disabled
        );
        assert!(!record.startup_decision.startup_permitted);
        assert_eq!(
            record.execution_record.execution_state,
            ScreenLiveViewWorkerExecutionState::NotStarted
        );
        assert_eq!(
            record.execution_record.block_reason,
            Some(ScreenLiveViewWorkerExecutionBlockReason::StartupNotPermitted)
        );
        assert!(!record.execution_record.worker_started);
        assert!(!record.execution_record.product_live_view_ready);
    });
}

#[test]
fn service_runtime_blocks_capture_only_permission_before_worker_start() {
    with_clean_live_view_env(|| {
        set_lan_defaults();
        env::set_var(
            constants::screen_flow::SCREEN_LIVE_VIEW_PERMISSION_ENV,
            constants::screen_flow::SCREEN_LIVE_VIEW_PERMISSION_CAPTURE_ONLY,
        );

        let record = run_screen_live_view_worker_runtime();

        assert_eq!(
            record.startup_decision.startup_state,
            ScreenLiveViewWorkerStartupState::Blocked
        );
        assert_eq!(
            record.startup_decision.block_reason,
            Some(ScreenLiveViewWorkerStartupBlockReason::RuntimeNotReady)
        );
        assert_eq!(
            record.execution_record.execution_state,
            ScreenLiveViewWorkerExecutionState::NotStarted
        );
        assert!(!record.execution_record.worker_started);
    });
}

#[test]
fn service_runtime_preserves_runtime_blocking_reasons() {
    with_clean_live_view_env(|| {
        set_lan_defaults();
        env::remove_var(constants::screen_flow::SCREEN_LIVE_VIEW_DELETION_PROOF_ENV);

        let runtime_decision = evaluate_screen_live_view_runtime(record_input());
        let record = run_screen_live_view_worker_runtime();

        assert_eq!(
            runtime_decision.session_state,
            ScreenLiveViewRuntimeSessionState::Blocked
        );
        assert_eq!(
            runtime_decision.block_reason,
            Some(ScreenLiveViewRuntimeBlockReason::MissingRawFrameDeletionProof)
        );
        assert_eq!(
            record.startup_decision.block_reason,
            Some(ScreenLiveViewWorkerStartupBlockReason::RuntimeNotReady)
        );
        assert!(!record.execution_record.product_live_view_ready);
    });
}

#[test]
fn service_runtime_starts_worker_only_when_all_gates_are_present() {
    with_clean_live_view_env(|| {
        set_lan_defaults();

        let record = run_screen_live_view_worker_runtime();

        assert_eq!(
            record.startup_decision.startup_state,
            ScreenLiveViewWorkerStartupState::ReadyToStart
        );
        assert!(record.startup_decision.startup_permitted);
        assert_eq!(
            record.execution_record.execution_state,
            ScreenLiveViewWorkerExecutionState::Started
        );
        assert!(record.execution_record.worker_started);
        assert!(record.execution_record.product_live_view_ready);
    });
}

#[test]
fn service_runtime_refuses_unsafe_worker_options_even_after_startup_gates() {
    with_clean_live_view_env(|| {
        set_lan_defaults();
        env::set_var(
            constants::screen_flow::SCREEN_LIVE_VIEW_REMOTE_INPUT_ENV,
            constants::screen_flow::ENV_TRUE,
        );

        let record = run_screen_live_view_worker_runtime();

        assert_eq!(
            record.execution_record.execution_state,
            ScreenLiveViewWorkerExecutionState::NotStarted
        );
        assert_eq!(
            record.execution_record.block_reason,
            Some(ScreenLiveViewWorkerExecutionBlockReason::StartupNotPermitted)
        );
        assert!(!record.execution_record.worker_started);
        assert!(!record.execution_record.product_live_view_ready);
    });
}

fn record_input() -> ScreenLiveViewRuntimeInput {
    run_screen_live_view_worker_runtime().runtime_input
}

fn set_lan_defaults() {
    env::set_var(
        constants::screen_flow::SCREEN_LIVE_VIEW_MODE_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_MODE_LAN_ONLY,
    );
    env::set_var(
        constants::screen_flow::SCREEN_LIVE_VIEW_TRANSPORT_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_TRANSPORT_LAN_MUTUAL_AUTH,
    );
    env::set_var(
        constants::screen_flow::SCREEN_LIVE_VIEW_PERMISSION_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_PERMISSION_LIVE_VIEW,
    );
    for key in [
        constants::screen_flow::SCREEN_LIVE_VIEW_TRANSPORT_PROOF_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_DELETION_PROOF_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_PARENT_UI_PERSISTENCE_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_PLATFORM_PROMPT_ARTIFACT_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_PHYSICAL_DEVICE_PARITY_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_PRIVACY_LEGAL_APPROVAL_ENV,
    ] {
        env::set_var(key, constants::screen_flow::ENV_TRUE);
    }
}

fn with_clean_live_view_env(test: impl FnOnce()) {
    let _guard = require_ok(
        LIVE_VIEW_ENV_LOCK.lock(),
        constants::screen_flow::ERROR_SCREEN_LIVE_VIEW_ENV_LOCKS,
    );
    clear_live_view_env();
    test();
    clear_live_view_env();
}

fn clear_live_view_env() {
    for key in [
        constants::screen_flow::SCREEN_LIVE_VIEW_RUNTIME_ENABLED_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_MODE_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_TRANSPORT_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_PERMISSION_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_TRANSPORT_PROOF_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_DELETION_PROOF_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_PARENT_UI_PERSISTENCE_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_RELAY_CACHE_PROOF_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_PLATFORM_PROMPT_ARTIFACT_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_RELAY_CACHE_EXECUTION_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_PHYSICAL_DEVICE_PARITY_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_PRIVACY_LEGAL_APPROVAL_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_CACHE_RAW_FRAMES_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_SESSION_RECORDING_ENV,
        constants::screen_flow::SCREEN_LIVE_VIEW_REMOTE_INPUT_ENV,
    ] {
        env::remove_var(key);
    }
}
