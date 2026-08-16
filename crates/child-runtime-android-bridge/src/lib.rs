//! JNI entrypoint for the Android child-agent composition boundary.
//!
//! This library owns only native child-runtime startup and health projection.
//! It does not expose transport, device-owner authority, install proof, or
//! platform enforcement. The Android package must keep the manual-required
//! state when the library is absent or startup/query fails.

use jni::sys::jint;

pub mod ffi;

pub const READINESS_UNAVAILABLE: jint = 0;
pub const READINESS_READY: jint = 1;
pub const READINESS_RECOVERY_PENDING: jint = 2;
pub const READINESS_REVOKED: jint = 3;
pub const READINESS_TAMPER_MANUAL_REQUIRED: jint = 4;
