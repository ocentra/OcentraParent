#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenLiveViewRuntimeMode {
    Disabled,
    LanOnlyView,
    RelayBackedView,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenLiveViewRuntimeTransport {
    None,
    LanMutualAuth,
    RelayEndToEndEncrypted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenLiveViewRuntimePermission {
    Missing,
    ScreenCaptureOnly,
    LiveViewPermission,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenLiveViewRuntimeSessionState {
    Disabled,
    Blocked,
    ServiceRuntimeReady,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenLiveViewRuntimeBlockReason {
    Disabled,
    MissingLiveViewPermission,
    MissingTransportProof,
    MissingRawFrameDeletionProof,
    UnsafeRetentionOrControl,
    MissingParentUiPersistence,
    MissingRelayCacheProof,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScreenLiveViewRuntimeInput {
    pub mode: ScreenLiveViewRuntimeMode,
    pub transport: ScreenLiveViewRuntimeTransport,
    pub permission: ScreenLiveViewRuntimePermission,
    pub live_transport_proof_present: bool,
    pub raw_frame_deleted_after_transport: bool,
    pub parent_ui_persistence_proved: bool,
    pub relay_cache_proved: bool,
    pub cache_raw_frames: bool,
    pub session_recording_allowed: bool,
    pub remote_input_control_allowed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScreenLiveViewRuntimeDecision {
    pub session_state: ScreenLiveViewRuntimeSessionState,
    pub block_reason: Option<ScreenLiveViewRuntimeBlockReason>,
    pub product_live_view_ready: bool,
}

pub fn evaluate_screen_live_view_runtime(
    input: ScreenLiveViewRuntimeInput,
) -> ScreenLiveViewRuntimeDecision {
    if input.mode == ScreenLiveViewRuntimeMode::Disabled {
        return ScreenLiveViewRuntimeDecision {
            session_state: ScreenLiveViewRuntimeSessionState::Disabled,
            block_reason: Some(ScreenLiveViewRuntimeBlockReason::Disabled),
            product_live_view_ready: false,
        };
    }

    if input.permission != ScreenLiveViewRuntimePermission::LiveViewPermission {
        return blocked(ScreenLiveViewRuntimeBlockReason::MissingLiveViewPermission);
    }

    if !transport_matches_mode(input.mode, input.transport) || !input.live_transport_proof_present {
        return blocked(ScreenLiveViewRuntimeBlockReason::MissingTransportProof);
    }

    if !input.raw_frame_deleted_after_transport {
        return blocked(ScreenLiveViewRuntimeBlockReason::MissingRawFrameDeletionProof);
    }

    if input.cache_raw_frames
        || input.session_recording_allowed
        || input.remote_input_control_allowed
    {
        return blocked(ScreenLiveViewRuntimeBlockReason::UnsafeRetentionOrControl);
    }

    if !input.parent_ui_persistence_proved {
        return ScreenLiveViewRuntimeDecision {
            session_state: ScreenLiveViewRuntimeSessionState::ServiceRuntimeReady,
            block_reason: Some(ScreenLiveViewRuntimeBlockReason::MissingParentUiPersistence),
            product_live_view_ready: false,
        };
    }

    if input.mode == ScreenLiveViewRuntimeMode::RelayBackedView && !input.relay_cache_proved {
        return ScreenLiveViewRuntimeDecision {
            session_state: ScreenLiveViewRuntimeSessionState::ServiceRuntimeReady,
            block_reason: Some(ScreenLiveViewRuntimeBlockReason::MissingRelayCacheProof),
            product_live_view_ready: false,
        };
    }

    ScreenLiveViewRuntimeDecision {
        session_state: ScreenLiveViewRuntimeSessionState::ServiceRuntimeReady,
        block_reason: None,
        product_live_view_ready: true,
    }
}

fn transport_matches_mode(
    mode: ScreenLiveViewRuntimeMode,
    transport: ScreenLiveViewRuntimeTransport,
) -> bool {
    match mode {
        ScreenLiveViewRuntimeMode::Disabled => transport == ScreenLiveViewRuntimeTransport::None,
        ScreenLiveViewRuntimeMode::LanOnlyView => {
            transport == ScreenLiveViewRuntimeTransport::LanMutualAuth
        }
        ScreenLiveViewRuntimeMode::RelayBackedView => {
            transport == ScreenLiveViewRuntimeTransport::RelayEndToEndEncrypted
        }
    }
}

fn blocked(reason: ScreenLiveViewRuntimeBlockReason) -> ScreenLiveViewRuntimeDecision {
    ScreenLiveViewRuntimeDecision {
        session_state: ScreenLiveViewRuntimeSessionState::Blocked,
        block_reason: Some(reason),
        product_live_view_ready: false,
    }
}
