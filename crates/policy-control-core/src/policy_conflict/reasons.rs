#![forbid(unsafe_code)]

use ocentra_parent_agent_protocol::constants::policy_control;

use super::PolicyConflictKind;

pub(super) fn policy_conflict_reason(kind: PolicyConflictKind) -> &'static str {
    match kind {
        PolicyConflictKind::AmbiguousLocalTime => {
            policy_control::conflict::REASON_AMBIGUOUS_LOCAL_TIME
        }
        PolicyConflictKind::NonexistentLocalTime => {
            policy_control::conflict::REASON_NONEXISTENT_LOCAL_TIME
        }
        PolicyConflictKind::ClockSkew => policy_control::conflict::REASON_CLOCK_SKEW,
        PolicyConflictKind::TimezoneBoundary => policy_control::conflict::REASON_TIMEZONE_BOUNDARY,
        PolicyConflictKind::EqualPriority => policy_control::conflict::REASON_EQUAL_PRIORITY,
        PolicyConflictKind::UnknownDeviceTarget => {
            policy_control::conflict::REASON_UNKNOWN_DEVICE_TARGET
        }
        PolicyConflictKind::OverlappingActions => {
            policy_control::conflict::REASON_OVERLAPPING_ACTIONS
        }
    }
}
