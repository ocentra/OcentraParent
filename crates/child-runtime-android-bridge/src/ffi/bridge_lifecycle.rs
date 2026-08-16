use jni::sys::jlong;

use super::{
    bridge_state::{with_state, BridgeState},
    runtime::ChildRuntimeAndroidBridge,
};

const ALREADY_INITIALIZED_ERROR_PREFIX: &str = "native child-runtime bridge is already initialized";
const ALREADY_INITIALIZED_ERROR_SUFFIX: &str = " for another durable root";
const HANDLE_EXHAUSTED_ERROR: &str = "native child-runtime bridge handle space is exhausted";
const INVALID_HANDLE_ERROR: &str = "native child-runtime bridge handle is not active";

pub(super) fn record_error(error: impl Into<String>) {
    let error = error.into();
    let _ = with_state(|state| state.last_error = Some(error));
}

pub fn start(root: String) -> jlong {
    with_state(|state| match state.bridge.is_some() {
        true => reuse_handle(state, &root),
        false => start_service(state, root),
    })
    .unwrap_or(0)
}

fn reuse_handle(state: &mut BridgeState, root: &str) -> jlong {
    if state.durable_root.as_deref() == Some(root) {
        return allocate_handle(state);
    }
    state.last_error = Some(
        [
            ALREADY_INITIALIZED_ERROR_PREFIX,
            ALREADY_INITIALIZED_ERROR_SUFFIX,
        ]
        .concat(),
    );
    0
}

fn allocate_handle(state: &mut BridgeState) -> jlong {
    state.allocate_handle().unwrap_or_else(|| {
        state.last_error = Some(HANDLE_EXHAUSTED_ERROR.to_owned());
        0
    })
}

fn start_service(state: &mut BridgeState, root: String) -> jlong {
    let Some(handle) = state.allocate_handle() else {
        state.last_error = Some(HANDLE_EXHAUSTED_ERROR.to_owned());
        return 0;
    };
    let durable_root = root.clone();
    match ChildRuntimeAndroidBridge::start(root) {
        Ok(bridge) => {
            state.bridge = Some(bridge);
            state.durable_root = Some(durable_root);
            state.last_error = None;
            handle
        }
        Err(error) => {
            state.active_handles.remove(&handle);
            state.last_error = Some(error.to_string());
            0
        }
    }
}

pub fn last_error() -> String {
    with_state(|state| state.last_error.clone())
        .flatten()
        .unwrap_or_default()
}

pub fn stop(handle: jlong) -> bool {
    with_state(|state| {
        if handle == 0 || !state.active_handles.remove(&handle) {
            state.last_error = Some(INVALID_HANDLE_ERROR.to_owned());
            return false;
        }
        if state.active_handles.is_empty() {
            state.bridge = None;
            state.durable_root = None;
        }
        state.last_error = None;
        true
    })
    .unwrap_or(false)
}
