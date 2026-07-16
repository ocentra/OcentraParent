fn protocol_lookup<T, const N: usize>(value: &str, variants: [(&'static str, T); N]) -> Option<T> {
    variants
        .into_iter()
        .find_map(|(protocol, variant)| (value == protocol).then_some(variant))
}

mod action;
mod boundary_state;
mod capability_state;
mod decision_source;
mod delivery_state;
mod exact_url_claim_state;
mod mechanism;
mod outcome;
mod target_type;
mod unmanaged_detection_state;
mod unmanaged_enforcement_state;
mod unmanaged_fallback_action_state;
