use super::super::super::LocalNetworkInterfaceClassification;

pub(super) fn classify_interface(
    interface_name: &str,
    is_loopback: bool,
    is_link_local_only: bool,
    current: LocalNetworkInterfaceClassification,
) -> LocalNetworkInterfaceClassification {
    if is_loopback {
        return LocalNetworkInterfaceClassification::Loopback;
    }
    if is_link_local_only {
        return LocalNetworkInterfaceClassification::LinkLocalOnly;
    }
    if !matches!(current, LocalNetworkInterfaceClassification::Unknown) {
        return current;
    }
    let normalized = interface_name.trim().to_ascii_lowercase();
    if normalized.contains("wsl") {
        return LocalNetworkInterfaceClassification::Wsl;
    }
    if normalized.starts_with("docker")
        || (normalized.starts_with("veth") && !normalized.starts_with("vethernet"))
        || normalized.starts_with("br-")
        || normalized.contains("container")
        || normalized.starts_with("cni")
        || normalized.starts_with("podman")
    {
        return LocalNetworkInterfaceClassification::Container;
    }
    if normalized.contains("vpn")
        || normalized.starts_with("tailscale")
        || normalized.starts_with("wg")
        || normalized.starts_with("tun")
        || normalized.starts_with("tap")
        || normalized.starts_with("zt")
        || normalized.starts_with("utun")
    {
        return LocalNetworkInterfaceClassification::VpnOrTunnel;
    }
    if normalized.starts_with("vethernet")
        || normalized.starts_with("virbr")
        || normalized.starts_with("vboxnet")
        || normalized.starts_with("vmnet")
        || normalized.contains("virtual")
        || normalized.contains("hyper-v")
        || normalized.contains("hyperv")
    {
        return LocalNetworkInterfaceClassification::Virtual;
    }
    if normalized.starts_with("ethernet")
        || normalized.starts_with("wi-fi")
        || normalized.starts_with("wifi")
        || normalized.starts_with("wlan")
        || normalized.starts_with("wireless")
        || normalized.starts_with("eth")
        || normalized.starts_with("en")
        || normalized.starts_with("wl")
        || normalized.starts_with("wwan")
    {
        return LocalNetworkInterfaceClassification::Physical;
    }
    current
}

pub(super) fn interface_id_rank(id: &str) -> u8 {
    if id.starts_with("mac:") {
        3
    } else if id.starts_with("index:") {
        2
    } else if id.starts_with("name:") {
        1
    } else {
        0
    }
}
