use super::super::super::LocalNetworkInterfaceClassification;

pub(super) fn classification_conservatism(
    classification: LocalNetworkInterfaceClassification,
) -> u8 {
    match classification {
        LocalNetworkInterfaceClassification::Unknown => 0,
        LocalNetworkInterfaceClassification::Physical => 1,
        LocalNetworkInterfaceClassification::Virtual => 2,
        LocalNetworkInterfaceClassification::VpnOrTunnel
        | LocalNetworkInterfaceClassification::Container => 3,
        LocalNetworkInterfaceClassification::Wsl => 4,
        LocalNetworkInterfaceClassification::Loopback => 5,
        LocalNetworkInterfaceClassification::LinkLocalOnly => 6,
    }
}
