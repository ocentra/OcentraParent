use ocentra_screen_live_view_core::live_view_runtime::{
    evaluate_screen_live_view_runtime, ScreenLiveViewRuntimeBlockReason,
    ScreenLiveViewRuntimeInput, ScreenLiveViewRuntimeMode, ScreenLiveViewRuntimePermission,
    ScreenLiveViewRuntimeSessionState, ScreenLiveViewRuntimeTransport,
};

#[test]
fn screen_live_view_runtime_blocks_capture_only_permission() {
    let missing_permission = evaluate_screen_live_view_runtime(ScreenLiveViewRuntimeInput {
        permission: ScreenLiveViewRuntimePermission::Missing,
        live_transport_proof_present: true,
        raw_frame_deleted_after_transport: true,
        ..lan_input()
    });
    let capture_only_permission = evaluate_screen_live_view_runtime(ScreenLiveViewRuntimeInput {
        permission: ScreenLiveViewRuntimePermission::ScreenCaptureOnly,
        live_transport_proof_present: true,
        raw_frame_deleted_after_transport: true,
        ..lan_input()
    });

    assert_eq!(
        missing_permission.session_state,
        ScreenLiveViewRuntimeSessionState::Blocked
    );
    assert_eq!(
        missing_permission.block_reason,
        Some(ScreenLiveViewRuntimeBlockReason::MissingLiveViewPermission)
    );
    assert_eq!(
        capture_only_permission.block_reason,
        Some(ScreenLiveViewRuntimeBlockReason::MissingLiveViewPermission)
    );
    assert!(!missing_permission.product_live_view_ready);
    assert!(!capture_only_permission.product_live_view_ready);
}

#[test]
fn screen_live_view_runtime_requires_transport_and_deletion_proof() {
    let missing_transport = evaluate_screen_live_view_runtime(ScreenLiveViewRuntimeInput {
        live_transport_proof_present: false,
        raw_frame_deleted_after_transport: true,
        ..lan_input()
    });
    let missing_deletion = evaluate_screen_live_view_runtime(ScreenLiveViewRuntimeInput {
        live_transport_proof_present: true,
        raw_frame_deleted_after_transport: false,
        ..lan_input()
    });

    assert_eq!(
        missing_transport.block_reason,
        Some(ScreenLiveViewRuntimeBlockReason::MissingTransportProof)
    );
    assert_eq!(
        missing_deletion.block_reason,
        Some(ScreenLiveViewRuntimeBlockReason::MissingRawFrameDeletionProof)
    );
    assert!(!missing_transport.product_live_view_ready);
    assert!(!missing_deletion.product_live_view_ready);
}

#[test]
fn screen_live_view_runtime_rejects_frame_cache_recording_and_remote_input() {
    for unsafe_input in [
        ScreenLiveViewRuntimeInput {
            cache_raw_frames: true,
            ..lan_input()
        },
        ScreenLiveViewRuntimeInput {
            session_recording_allowed: true,
            ..lan_input()
        },
        ScreenLiveViewRuntimeInput {
            remote_input_control_allowed: true,
            ..lan_input()
        },
    ] {
        let decision = evaluate_screen_live_view_runtime(unsafe_input);

        assert_eq!(
            decision.block_reason,
            Some(ScreenLiveViewRuntimeBlockReason::UnsafeRetentionOrControl)
        );
        assert!(!decision.product_live_view_ready);
    }
}

#[test]
fn screen_live_view_runtime_can_be_service_ready_without_product_ready() {
    let decision = evaluate_screen_live_view_runtime(ScreenLiveViewRuntimeInput {
        parent_ui_persistence_proved: false,
        ..lan_input()
    });

    assert_eq!(
        decision.session_state,
        ScreenLiveViewRuntimeSessionState::ServiceRuntimeReady
    );
    assert_eq!(
        decision.block_reason,
        Some(ScreenLiveViewRuntimeBlockReason::MissingParentUiPersistence)
    );
    assert!(!decision.product_live_view_ready);
}

#[test]
fn screen_live_view_runtime_requires_relay_cache_for_relay_mode() {
    let missing_relay_cache = evaluate_screen_live_view_runtime(ScreenLiveViewRuntimeInput {
        mode: ScreenLiveViewRuntimeMode::RelayBackedView,
        transport: ScreenLiveViewRuntimeTransport::RelayEndToEndEncrypted,
        relay_cache_proved: false,
        ..lan_input()
    });
    let relay_ready = evaluate_screen_live_view_runtime(ScreenLiveViewRuntimeInput {
        mode: ScreenLiveViewRuntimeMode::RelayBackedView,
        transport: ScreenLiveViewRuntimeTransport::RelayEndToEndEncrypted,
        relay_cache_proved: true,
        ..lan_input()
    });

    assert_eq!(
        missing_relay_cache.block_reason,
        Some(ScreenLiveViewRuntimeBlockReason::MissingRelayCacheProof)
    );
    assert!(!missing_relay_cache.product_live_view_ready);
    assert_eq!(relay_ready.block_reason, None);
    assert!(relay_ready.product_live_view_ready);
}

fn lan_input() -> ScreenLiveViewRuntimeInput {
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
