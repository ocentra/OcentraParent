use super::super::super::network_identity_support;

pub(super) fn interface_ignored_reason(
    interface_name: &str,
    is_up: bool,
    is_connected: bool,
    is_loopback: bool,
    is_link_local_only: bool,
) -> Option<network_identity_support::LocalNetworkInterfaceIgnoreReason> {
    if !is_connected {
        return Some(network_identity_support::LocalNetworkInterfaceIgnoreReason::Disconnected);
    }
    if !is_up {
        return Some(network_identity_support::LocalNetworkInterfaceIgnoreReason::Down);
    }
    if is_loopback {
        return Some(network_identity_support::LocalNetworkInterfaceIgnoreReason::Loopback);
    }
    let normalized = interface_name.trim().to_ascii_lowercase();
    if normalized.contains("wsl") {
        return Some(network_identity_support::LocalNetworkInterfaceIgnoreReason::Wsl);
    }
    if let Some(reason) = network_identity_support::ignored_interface_reason(interface_name) {
        return Some(reason);
    }
    if is_link_local_only {
        return Some(network_identity_support::LocalNetworkInterfaceIgnoreReason::LinkLocalOnly);
    }
    None
}
