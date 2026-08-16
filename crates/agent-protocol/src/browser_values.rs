fn protocol_lookup<T, const N: usize>(
    value: impl AsRef<str>,
    variants: [(&'static str, T); N],
) -> Option<T> {
    let value = value.as_ref();
    variants
        .into_iter()
        .find_map(|(protocol, variant)| (value == protocol).then_some(variant))
}

mod active_proof_source;
mod active_tab_capability;
mod active_tab_state;
mod browser_capability_status;
mod browser_channel;
mod browser_custody_label;
mod browser_exact_url_capability;
mod browser_family;
mod browser_inventory_install_state;
mod browser_management_tier;
mod browser_query_visibility_label;
mod browser_unmanaged_detection_confidence;
mod browser_unmanaged_detection_reason;
mod browser_unmanaged_process_kind;
