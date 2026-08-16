use jni::sys::{jint, jlong};
use ocentra_child_runtime::service::{ChildAgentHealth, ChildAgentServiceError};

use super::{
    bridge_health_failure::readiness_failure,
    bridge_state::{with_state, BridgeState},
    readiness_code,
    runtime::ChildRuntimeAndroidBridge,
};
use crate::READINESS_UNAVAILABLE;

const INVALID_HANDLE_ERROR: &str = "native child-runtime bridge handle is not active";

pub fn readiness(handle: jlong) -> jint {
    with_state(|state| match health(state, handle) {
        Some(Ok(health)) => {
            state.last_error = readiness_failure(&health.readiness);
            readiness_code(&health.readiness)
        }
        Some(Err(error)) => {
            state.last_error = Some(error.to_string());
            READINESS_UNAVAILABLE
        }
        None => {
            state.last_error = Some(INVALID_HANDLE_ERROR.to_owned());
            READINESS_UNAVAILABLE
        }
    })
    .unwrap_or(READINESS_UNAVAILABLE)
}

pub fn domain_flow_count(handle: jlong) -> jint {
    with_state(|state| match health(state, handle) {
        Some(Ok(health)) => jint::try_from(health.domain_flow_count).unwrap_or(jint::MAX),
        Some(Err(error)) => {
            state.last_error = Some(error.to_string());
            0
        }
        None => {
            state.last_error = Some(INVALID_HANDLE_ERROR.to_owned());
            0
        }
    })
    .unwrap_or(0)
}

fn health(
    state: &BridgeState,
    handle: jlong,
) -> Option<Result<ChildAgentHealth, ChildAgentServiceError>> {
    state
        .owns_handle(handle)
        .then(|| state.bridge.as_ref().map(ChildRuntimeAndroidBridge::health))
        .flatten()
}
