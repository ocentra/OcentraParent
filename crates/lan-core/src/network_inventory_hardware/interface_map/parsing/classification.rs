use serde_json::Value;

use super::super::LocalNetworkInterfaceClassification;
use super::{record_text_any, record_text_values_any, INTERFACE_NAME_KEYS};

pub(super) fn interface_classification_hint(
    record: &Value,
    interface_name: &str,
) -> LocalNetworkInterfaceClassification {
    let hints = record_text_values_any(
        record,
        &[
            "InterfaceType",
            "interfaceType",
            "Type",
            "type",
            "MediaType",
            "HardwareType",
        ],
    )
    .join(" ")
    .to_ascii_lowercase();
    let normalized_name = interface_name.to_ascii_lowercase();
    if hints.contains("loopback") || normalized_name.contains("loopback") {
        return LocalNetworkInterfaceClassification::Loopback;
    }
    if hints.contains("wsl") || normalized_name.contains("wsl") {
        return LocalNetworkInterfaceClassification::Wsl;
    }
    if hints.contains("container") || hints.contains("docker") || hints.contains("bridge") {
        return LocalNetworkInterfaceClassification::Container;
    }
    if hints.contains("vpn") || hints.contains("tunnel") {
        return LocalNetworkInterfaceClassification::VpnOrTunnel;
    }
    if hints.contains("virtual") || hints.contains("hyper-v") || hints.contains("hyperv") {
        return LocalNetworkInterfaceClassification::Virtual;
    }
    if hints.contains("ethernet")
        || hints.contains("wireless")
        || hints.contains("wi-fi")
        || hints.contains("wifi")
    {
        return LocalNetworkInterfaceClassification::Physical;
    }
    LocalNetworkInterfaceClassification::Unknown
}

pub(super) fn is_wireless_interface_name(record: &Value) -> bool {
    let interface_name = record_text_any(record, INTERFACE_NAME_KEYS).unwrap_or_default();
    let hints = record_text_values_any(
        record,
        &[
            "InterfaceType",
            "interfaceType",
            "Type",
            "type",
            "MediaType",
        ],
    )
    .join(" ")
    .to_ascii_lowercase();
    let normalized_name = interface_name.to_ascii_lowercase();
    hints.contains("wireless")
        || hints.contains("wi-fi")
        || hints.contains("wifi")
        || hints.contains("wlan")
        || normalized_name.contains("wi-fi")
        || normalized_name.contains("wifi")
        || normalized_name.contains("wlan")
        || normalized_name.contains("wireless")
}
