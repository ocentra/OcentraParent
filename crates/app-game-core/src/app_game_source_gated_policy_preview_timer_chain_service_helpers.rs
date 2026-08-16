const TIMER_SERVICE_READ_API_PROOF_REQUIRED: &str = "service-read-api-proof-required";
const TIMER_PROTOCOL_PROOF_REQUIRED: &str = "protocol-proof-required";
const TIMER_RUNTIME_BLOCKED_BY_SOURCE_FRESHNESS: &str = "blocked-by-source-freshness";
const TIMER_RUNTIME_BLOCKED_BY_COMPILER_DECISION: &str = "blocked-by-compiler-decision";
const PARENT_SURFACE_INTENT_PROOF_REQUIRED: &str = "audit-rollback-parent-surface-proof-required";
const SERVICE_READ_MODEL_PROOF_REQUIRED: &str = "service-read-model-proof-required";

pub(super) fn service_readiness_handoff_state_for_parent_surface(
    parent_surface_state: &str,
) -> &'static str {
    match parent_surface_state {
        PARENT_SURFACE_INTENT_PROOF_REQUIRED => TIMER_SERVICE_READ_API_PROOF_REQUIRED,
        TIMER_RUNTIME_BLOCKED_BY_SOURCE_FRESHNESS => TIMER_RUNTIME_BLOCKED_BY_SOURCE_FRESHNESS,
        _ => TIMER_RUNTIME_BLOCKED_BY_COMPILER_DECISION,
    }
}

pub(super) fn protocol_handoff_state_for_read_model(read_model_state: &str) -> &'static str {
    match read_model_state {
        SERVICE_READ_MODEL_PROOF_REQUIRED => TIMER_PROTOCOL_PROOF_REQUIRED,
        TIMER_RUNTIME_BLOCKED_BY_SOURCE_FRESHNESS => TIMER_RUNTIME_BLOCKED_BY_SOURCE_FRESHNESS,
        _ => TIMER_RUNTIME_BLOCKED_BY_COMPILER_DECISION,
    }
}
