#![forbid(unsafe_code)]

//! JNI entrypoint for the Android child-agent composition boundary.
//!
//! This library owns only native child-runtime startup and health projection.
//! It does not expose transport, device-owner authority, install proof, or
//! platform enforcement. The Android package must keep the manual-required
//! state when the library is absent or startup/query fails.

use std::sync::{Mutex, OnceLock};

use jni::{
    objects::{JClass, JString},
    sys::{jboolean, jint, jlong, jstring},
    JNIEnv,
};
use ocentra_child_runtime::service::{
    ChildAgentReadiness, ChildAgentService, ChildAgentServicePaths,
};
use tokio::runtime::Runtime;

const BRIDGE_HANDLE: jlong = 1;
pub const READINESS_UNAVAILABLE: jint = 0;
pub const READINESS_READY: jint = 1;
pub const READINESS_RECOVERY_PENDING: jint = 2;
pub const READINESS_REVOKED: jint = 3;

struct BridgeState {
    runtime: Option<Runtime>,
    service: Option<ChildAgentService>,
    handle: jlong,
    durable_root: Option<String>,
    last_error: Option<String>,
}

impl Default for BridgeState {
    fn default() -> Self {
        Self {
            runtime: None,
            service: None,
            handle: 0,
            durable_root: None,
            last_error: None,
        }
    }
}

static BRIDGE_STATE: OnceLock<Mutex<BridgeState>> = OnceLock::new();

fn bridge_state() -> &'static Mutex<BridgeState> {
    BRIDGE_STATE.get_or_init(|| Mutex::new(BridgeState::default()))
}

fn with_state<T>(operation: impl FnOnce(&mut BridgeState) -> T) -> Option<T> {
    bridge_state()
        .lock()
        .ok()
        .map(|mut state| operation(&mut state))
}

fn java_string(mut env: JNIEnv<'_>, value: &str) -> jstring {
    env.new_string(value)
        .map(JString::into_raw)
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "system" fn Java_ca_ocentra_child_agent_ChildAgentComposition_nativeStart(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    durable_root: JString<'_>,
) -> jlong {
    let root = match env.get_string(&durable_root) {
        Ok(value) => value.to_string_lossy().into_owned(),
        Err(_) => return 0,
    };

    with_state(|state| {
        if state.service.is_some() {
            if state.durable_root.as_deref() == Some(root.as_str()) {
                return state.handle;
            }
            state.last_error = Some(
                "native child-runtime bridge is already initialized for another durable root"
                    .to_owned(),
            );
            return 0;
        }

        let runtime = match state.runtime.take().or_else(|| Runtime::new().ok()) {
            Some(runtime) => runtime,
            None => {
                state.last_error = Some("native tokio runtime unavailable".to_owned());
                return 0;
            }
        };
        let durable_root = root.clone();
        let result = runtime.block_on(ChildAgentService::initialize_with_paths(
            ChildAgentServicePaths::from_root(root),
        ));
        state.runtime = Some(runtime);

        match result {
            Ok(service) => {
                state.service = Some(service);
                state.handle = BRIDGE_HANDLE;
                state.durable_root = Some(durable_root);
                state.last_error = None;
                BRIDGE_HANDLE
            }
            Err(error) => {
                state.handle = 0;
                state.last_error = Some(error.to_string());
                0
            }
        }
    })
    .unwrap_or(0)
}

#[no_mangle]
pub extern "system" fn Java_ca_ocentra_child_agent_ChildAgentComposition_nativeReadiness(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
) -> jint {
    with_state(|state| {
        if state.handle != handle || handle == 0 {
            return READINESS_UNAVAILABLE;
        }
        let Some(service) = state.service.as_ref() else {
            return READINESS_UNAVAILABLE;
        };
        match service.health() {
            Ok(health) => readiness_code(&health.readiness),
            Err(error) => {
                state.last_error = Some(error.to_string());
                READINESS_UNAVAILABLE
            }
        }
    })
    .unwrap_or(READINESS_UNAVAILABLE)
}

#[no_mangle]
pub extern "system" fn Java_ca_ocentra_child_agent_ChildAgentComposition_nativeDomainFlowCount(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
) -> jint {
    with_state(|state| {
        if state.handle != handle || handle == 0 {
            return 0;
        }
        let Some(service) = state.service.as_ref() else {
            return 0;
        };
        match service.health() {
            Ok(health) => match jint::try_from(health.domain_flow_count) {
                Ok(count) => count,
                Err(_) => jint::MAX,
            },
            Err(error) => {
                state.last_error = Some(error.to_string());
                0
            }
        }
    })
    .unwrap_or(0)
}

#[no_mangle]
pub extern "system" fn Java_ca_ocentra_child_agent_ChildAgentComposition_nativeLastError(
    env: JNIEnv<'_>,
    _class: JClass<'_>,
) -> jstring {
    let error = with_state(|state| state.last_error.clone())
        .flatten()
        .unwrap_or_default();
    java_string(env, &error)
}

#[no_mangle]
pub extern "system" fn Java_ca_ocentra_child_agent_ChildAgentComposition_nativeStop(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
) -> jboolean {
    with_state(|state| {
        if state.handle != handle || handle == 0 {
            return 0;
        }
        state.service = None;
        state.handle = 0;
        state.durable_root = None;
        1
    })
    .unwrap_or(0)
}

fn readiness_code(readiness: &ChildAgentReadiness) -> jint {
    match readiness {
        ChildAgentReadiness::Ready => READINESS_READY,
        ChildAgentReadiness::RecoveryPending { .. } => READINESS_RECOVERY_PENDING,
        ChildAgentReadiness::Revoked { .. } => READINESS_REVOKED,
    }
}
