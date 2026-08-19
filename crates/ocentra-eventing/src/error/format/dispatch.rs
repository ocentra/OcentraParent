use std::fmt;

use super::super::EventingError;

pub(super) fn fmt_dispatch_error(
    error: &EventingError,
    formatter: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    match error {
        EventingError::OrderedDispatchCycle {
            bus_identity,
            aggregate_key,
        } => write!(
            formatter,
            "ordered dispatch cycle for bus {bus_identity} and aggregate {}",
            aggregate_key.as_str()
        ),
        EventingError::OrderedDispatchDepthExceeded { max_depth } => write!(
            formatter,
            "ordered dispatch causal chain exceeds maximum depth {max_depth}"
        ),
        EventingError::OrderedDispatchLockOrderViolation {
            held_bus_identity,
            held_aggregate_key,
            requested_bus_identity,
            requested_aggregate_key,
        } => write!(
            formatter,
            "ordered dispatch lock order violation: held bus {held_bus_identity} aggregate {}, requested bus {requested_bus_identity} aggregate {}",
            held_aggregate_key.as_str(),
            requested_aggregate_key.as_str()
        ),
        EventingError::OrderedDispatchChainExpired { aggregate_key } => write!(
            formatter,
            "ordered dispatch causal chain expired before aggregate {} could be dispatched",
            aggregate_key.as_str()
        ),
        EventingError::CausalDispatchCancelled => {
            formatter.write_str("causal dispatch cancelled with its owning handler")
        }
        _ => {
            debug_assert!(false, "dispatch formatter received non-dispatch error");
            formatter.write_str("eventing dispatch error")
        }
    }
}
