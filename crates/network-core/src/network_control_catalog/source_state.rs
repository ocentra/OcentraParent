use super::NetworkControlCapabilityState;

pub fn capability_state_from_source_state(source_state: &str) -> NetworkControlCapabilityState {
    match source_state {
        "ready" => NetworkControlCapabilityState::Available,
        "ready-if-browser-capability-ready" => NetworkControlCapabilityState::Protected,
        "manual-required" => NetworkControlCapabilityState::ManualRequired,
        "not-implemented" => NetworkControlCapabilityState::Unavailable,
        "authoring-only" => NetworkControlCapabilityState::Disabled,
        _ => NetworkControlCapabilityState::Degraded,
    }
}
