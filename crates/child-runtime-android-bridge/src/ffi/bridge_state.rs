use std::{
    collections::BTreeSet,
    sync::{Mutex, OnceLock},
};

use jni::sys::jlong;

use super::runtime::ChildRuntimeAndroidBridge;

pub(super) struct BridgeState {
    pub(super) bridge: Option<ChildRuntimeAndroidBridge>,
    pub(super) active_handles: BTreeSet<jlong>,
    pub(super) next_handle: jlong,
    pub(super) durable_root: Option<String>,
    pub(super) last_error: Option<String>,
}

impl Default for BridgeState {
    fn default() -> Self {
        Self {
            bridge: None,
            active_handles: BTreeSet::new(),
            next_handle: 1,
            durable_root: None,
            last_error: None,
        }
    }
}

impl BridgeState {
    pub(super) fn allocate_handle(&mut self) -> Option<jlong> {
        let handle = self.next_handle;
        if handle == 0 {
            return None;
        }
        self.next_handle = handle.checked_add(1).unwrap_or(0);
        self.active_handles.insert(handle);
        Some(handle)
    }

    pub(super) fn owns_handle(&self, handle: jlong) -> bool {
        handle != 0 && self.active_handles.contains(&handle)
    }
}

static BRIDGE_STATE: OnceLock<Mutex<BridgeState>> = OnceLock::new();

fn bridge_state() -> &'static Mutex<BridgeState> {
    BRIDGE_STATE.get_or_init(|| Mutex::new(BridgeState::default()))
}

pub(super) fn with_state<T>(operation: impl FnOnce(&mut BridgeState) -> T) -> Option<T> {
    bridge_state()
        .lock()
        .ok()
        .map(|mut state| operation(&mut state))
}
