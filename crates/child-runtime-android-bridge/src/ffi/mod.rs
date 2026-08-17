pub mod bridge_health;
mod bridge_health_failure;
pub mod bridge_lifecycle;
mod bridge_state;
mod jni_exports;
pub mod runtime;

use jni::sys::jint;
use ocentra_child_runtime::service::ChildAgentReadiness;

use crate::{
    READINESS_READY, READINESS_RECOVERY_PENDING, READINESS_REVOKED,
    READINESS_TAMPER_MANUAL_REQUIRED, READINESS_TRUST_BINDING_MANUAL_REQUIRED,
};

fn readiness_code(readiness: &ChildAgentReadiness) -> jint {
    match readiness {
        ChildAgentReadiness::Ready => READINESS_READY,
        ChildAgentReadiness::RecoveryPending { .. } => READINESS_RECOVERY_PENDING,
        ChildAgentReadiness::TrustBindingManualRequired => READINESS_TRUST_BINDING_MANUAL_REQUIRED,
        ChildAgentReadiness::TamperManualRequired { .. } => READINESS_TAMPER_MANUAL_REQUIRED,
        ChildAgentReadiness::Revoked { .. } => READINESS_REVOKED,
    }
}
