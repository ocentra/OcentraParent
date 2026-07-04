use ocentra_screen_live_view_core::live_view_runtime::{
    evaluate_screen_live_view_runtime, ScreenLiveViewRuntimeInput, ScreenLiveViewRuntimeMode,
    ScreenLiveViewRuntimePermission, ScreenLiveViewRuntimeTransport,
};
use ocentra_screen_live_view_core::live_view_worker::{
    evaluate_screen_live_view_worker_startup, start_screen_live_view_worker,
    ScreenLiveViewWorkerExecutionBlockReason, ScreenLiveViewWorkerExecutionInput,
    ScreenLiveViewWorkerExecutionState, ScreenLiveViewWorkerStartupBlockReason,
    ScreenLiveViewWorkerStartupDecision, ScreenLiveViewWorkerStartupInput,
    ScreenLiveViewWorkerStartupState,
};

#[test]
fn screen_live_view_worker_startup_stays_disabled_when_live_view_is_disabled() {
    let decision = evaluate_screen_live_view_worker_startup(ScreenLiveViewWorkerStartupInput {
        mode: ScreenLiveViewRuntimeMode::Disabled,
        runtime_decision: evaluate_screen_live_view_runtime(ScreenLiveViewRuntimeInput {
            mode: ScreenLiveViewRuntimeMode::Disabled,
            transport: ScreenLiveViewRuntimeTransport::None,
            ..lan_runtime_input()
        }),
        ..ready_startup_input()
    });

    assert_eq!(
        decision.startup_state,
        ScreenLiveViewWorkerStartupState::Disabled
    );
    assert!(!decision.startup_permitted);
    assert!(!decision.worker_started);
    assert!(!decision.product_live_view_ready);
}

#[test]
fn screen_live_view_worker_startup_requires_runtime_readiness() {
    let decision = evaluate_screen_live_view_worker_startup(ScreenLiveViewWorkerStartupInput {
        runtime_decision: evaluate_screen_live_view_runtime(ScreenLiveViewRuntimeInput {
            permission: ScreenLiveViewRuntimePermission::ScreenCaptureOnly,
            ..lan_runtime_input()
        }),
        ..ready_startup_input()
    });

    assert_eq!(
        decision.startup_state,
        ScreenLiveViewWorkerStartupState::Blocked
    );
    assert_eq!(
        decision.block_reason,
        Some(ScreenLiveViewWorkerStartupBlockReason::RuntimeNotReady)
    );
    assert!(!decision.startup_permitted);
    assert!(!decision.worker_started);
}

#[test]
fn screen_live_view_worker_startup_requires_platform_prompt_artifact() {
    let decision = evaluate_screen_live_view_worker_startup(ScreenLiveViewWorkerStartupInput {
        platform_prompt_artifact_present: false,
        ..ready_startup_input()
    });

    assert_eq!(
        decision.block_reason,
        Some(ScreenLiveViewWorkerStartupBlockReason::MissingPlatformPromptArtifact)
    );
    assert!(!decision.startup_permitted);
    assert!(!decision.worker_started);
}

#[test]
fn screen_live_view_worker_startup_requires_relay_cache_for_relay_mode() {
    let decision = evaluate_screen_live_view_worker_startup(ScreenLiveViewWorkerStartupInput {
        mode: ScreenLiveViewRuntimeMode::RelayBackedView,
        runtime_decision: evaluate_screen_live_view_runtime(ScreenLiveViewRuntimeInput {
            mode: ScreenLiveViewRuntimeMode::RelayBackedView,
            transport: ScreenLiveViewRuntimeTransport::RelayEndToEndEncrypted,
            relay_cache_proved: true,
            ..lan_runtime_input()
        }),
        relay_cache_execution_proved: false,
        ..ready_startup_input()
    });

    assert_eq!(
        decision.block_reason,
        Some(ScreenLiveViewWorkerStartupBlockReason::MissingRelayCacheExecution)
    );
    assert!(!decision.startup_permitted);
    assert!(!decision.worker_started);
    assert!(!decision.product_live_view_ready);
}

#[test]
fn screen_live_view_worker_startup_requires_physical_parity_and_privacy_approval() {
    let missing_physical_parity =
        evaluate_screen_live_view_worker_startup(ScreenLiveViewWorkerStartupInput {
            physical_device_parity_proved: false,
            ..ready_startup_input()
        });
    let missing_privacy =
        evaluate_screen_live_view_worker_startup(ScreenLiveViewWorkerStartupInput {
            privacy_legal_approved: false,
            ..ready_startup_input()
        });

    assert_eq!(
        missing_physical_parity.block_reason,
        Some(ScreenLiveViewWorkerStartupBlockReason::MissingPhysicalDeviceParity)
    );
    assert_eq!(
        missing_privacy.block_reason,
        Some(ScreenLiveViewWorkerStartupBlockReason::MissingPrivacyLegalApproval)
    );
    assert!(!missing_physical_parity.startup_permitted);
    assert!(!missing_privacy.startup_permitted);
    assert!(!missing_physical_parity.worker_started);
    assert!(!missing_privacy.worker_started);
}

#[test]
fn screen_live_view_worker_startup_can_be_permitted_only_after_all_product_gates() {
    let decision = evaluate_screen_live_view_worker_startup(ready_startup_input());

    assert_eq!(
        decision.startup_state,
        ScreenLiveViewWorkerStartupState::ReadyToStart
    );
    assert_eq!(decision.block_reason, None);
    assert!(decision.startup_permitted);
    assert!(!decision.worker_started);
    assert!(decision.product_live_view_ready);
}

#[test]
fn screen_live_view_worker_execution_refuses_blocked_startup() {
    let blocked_startup =
        evaluate_screen_live_view_worker_startup(ScreenLiveViewWorkerStartupInput {
            platform_prompt_artifact_present: false,
            ..ready_startup_input()
        });
    let record = start_screen_live_view_worker(ready_execution_input(blocked_startup));

    assert_eq!(
        record.execution_state,
        ScreenLiveViewWorkerExecutionState::NotStarted
    );
    assert_eq!(
        record.block_reason,
        Some(ScreenLiveViewWorkerExecutionBlockReason::StartupNotPermitted)
    );
    assert!(!record.worker_started);
    assert!(!record.product_live_view_ready);
}

#[test]
fn screen_live_view_worker_execution_refuses_unsafe_retention_or_control() {
    for unsafe_input in [
        ScreenLiveViewWorkerExecutionInput {
            cache_raw_frames: true,
            ..ready_execution_input(ready_startup_decision())
        },
        ScreenLiveViewWorkerExecutionInput {
            session_recording_allowed: true,
            ..ready_execution_input(ready_startup_decision())
        },
        ScreenLiveViewWorkerExecutionInput {
            remote_input_control_allowed: true,
            ..ready_execution_input(ready_startup_decision())
        },
    ] {
        let record = start_screen_live_view_worker(unsafe_input);

        assert_eq!(
            record.execution_state,
            ScreenLiveViewWorkerExecutionState::NotStarted
        );
        assert_eq!(
            record.block_reason,
            Some(ScreenLiveViewWorkerExecutionBlockReason::UnsafeRetentionOrControl)
        );
        assert!(!record.worker_started);
        assert!(!record.product_live_view_ready);
    }
}

#[test]
fn screen_live_view_worker_execution_starts_after_all_gates() {
    let record = start_screen_live_view_worker(ready_execution_input(ready_startup_decision()));

    assert_eq!(
        record.execution_state,
        ScreenLiveViewWorkerExecutionState::Started
    );
    assert_eq!(record.block_reason, None);
    assert!(record.startup_permitted);
    assert!(record.worker_started);
    assert!(record.product_live_view_ready);
}

fn ready_startup_decision() -> ScreenLiveViewWorkerStartupDecision {
    evaluate_screen_live_view_worker_startup(ready_startup_input())
}

fn ready_execution_input(
    startup_decision: ScreenLiveViewWorkerStartupDecision,
) -> ScreenLiveViewWorkerExecutionInput {
    ScreenLiveViewWorkerExecutionInput {
        startup_decision,
        cache_raw_frames: false,
        session_recording_allowed: false,
        remote_input_control_allowed: false,
    }
}

fn ready_startup_input() -> ScreenLiveViewWorkerStartupInput {
    ScreenLiveViewWorkerStartupInput {
        mode: ScreenLiveViewRuntimeMode::LanOnlyView,
        runtime_decision: evaluate_screen_live_view_runtime(lan_runtime_input()),
        platform_prompt_artifact_present: true,
        relay_cache_execution_proved: false,
        physical_device_parity_proved: true,
        privacy_legal_approved: true,
    }
}

fn lan_runtime_input() -> ScreenLiveViewRuntimeInput {
    ScreenLiveViewRuntimeInput {
        mode: ScreenLiveViewRuntimeMode::LanOnlyView,
        transport: ScreenLiveViewRuntimeTransport::LanMutualAuth,
        permission: ScreenLiveViewRuntimePermission::LiveViewPermission,
        live_transport_proof_present: true,
        raw_frame_deleted_after_transport: true,
        parent_ui_persistence_proved: true,
        relay_cache_proved: false,
        cache_raw_frames: false,
        session_recording_allowed: false,
        remote_input_control_allowed: false,
    }
}
