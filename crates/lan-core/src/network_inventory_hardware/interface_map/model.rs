use std::net::IpAddr;

use super::super::{
    LocalNetworkInterface, LocalNetworkInterfaceClassification, LocalNetworkInterfaceMap,
    LocalNetworkIdentity,
};
use super::super::network_identity_support;

impl LocalNetworkInterface {
    pub fn is_eligible_by_default(&self) -> bool {
        self.is_up
            && self.is_connected
            && self.state_observed
            && !self.is_loopback
            && self.ignored_reason.is_none()
            && !self.is_link_local_only
            && !matches!(
                self.classification,
                LocalNetworkInterfaceClassification::Virtual
                    | LocalNetworkInterfaceClassification::VpnOrTunnel
                    | LocalNetworkInterfaceClassification::Container
                    | LocalNetworkInterfaceClassification::Wsl
                    | LocalNetworkInterfaceClassification::Loopback
                    | LocalNetworkInterfaceClassification::LinkLocalOnly
            )
            && self
                .ip_addresses
                .iter()
                .any(|address| {
                    network_identity_support::supported_local_ipv4_text(address)
                        || network_identity_support::supported_local_ipv6_text(address)
                })
    }

    pub fn matches_selection(&self, selected_interface_id: &str) -> bool {
        let selected_interface_id = selected_interface_id.trim();
        !selected_interface_id.is_empty()
            && (self.id.eq_ignore_ascii_case(selected_interface_id)
                || self.name.eq_ignore_ascii_case(selected_interface_id))
    }

    pub fn to_local_network_identity(&self) -> Option<LocalNetworkIdentity> {
        let ip_address = self
            .ip_addresses
            .iter()
            .find(|address| network_identity_support::supported_local_ipv4_text(address))
            .cloned()?;
        Some(LocalNetworkIdentity {
            ip_address: Some(ip_address),
            mac_address: self.mac_address.clone(),
            network_interface: (!self.name.trim().is_empty()).then(|| self.name.clone()),
            wifi_ssid: self.wifi_ssid.clone(),
            default_gateway: self.default_gateway.clone(),
            ipv4_cidr: self.ipv4_cidr.clone(),
            dns_servers: self.dns_servers.clone(),
            dhcp_server: self.dhcp_server.clone(),
            broadcast_address: self.broadcast_address.clone(),
            ipv6_prefixes: self.ipv6_prefixes.clone(),
        })
    }

    pub(super) fn merge_from(&mut self, candidate: Self) {
        if interface_id_rank(&candidate.id) > interface_id_rank(&self.id) {
            self.id = candidate.id.clone();
        }
        self.description = self.description.clone().or(candidate.description);
        self.index = self.index.or(candidate.index);
        self.mac_address = self.mac_address.clone().or(candidate.mac_address);
        append_unique_strings(&mut self.ip_addresses, candidate.ip_addresses);
        self.default_gateway = self.default_gateway.clone().or(candidate.default_gateway);
        append_unique_strings(&mut self.dns_servers, candidate.dns_servers);
        self.dhcp_server = self.dhcp_server.clone().or(candidate.dhcp_server);
        self.broadcast_address = self
            .broadcast_address
            .clone()
            .or(candidate.broadcast_address);
        self.ipv4_cidr = self.ipv4_cidr.clone().or(candidate.ipv4_cidr);
        append_unique_strings(&mut self.ipv6_prefixes, candidate.ipv6_prefixes);
        if candidate.state_observed {
            if self.state_observed {
                self.is_up &= candidate.is_up;
                self.is_connected &= candidate.is_connected;
            } else {
                self.is_up = candidate.is_up;
                self.is_connected = candidate.is_connected;
            }
            self.state_observed = true;
        }
        self.is_loopback |= candidate.is_loopback;
        if classification_conservatism(candidate.classification)
            > classification_conservatism(self.classification)
        {
            self.classification = candidate.classification;
        }
        self.is_link_local_only &= candidate.is_link_local_only;
        self.wifi_ssid = self.wifi_ssid.clone().or(candidate.wifi_ssid);
        self.wifi_signal_percent = self.wifi_signal_percent.max(candidate.wifi_signal_percent);
        self.has_default_route |= candidate.has_default_route;
        self.refresh_derived_state();
    }

    pub(super) fn refresh_derived_state(&mut self) {
        self.is_link_local_only = self.ip_addresses.iter().any(|address| {
            address
                .parse::<IpAddr>()
                .is_ok_and(|parsed| match parsed {
                    IpAddr::V4(value) => value.is_link_local(),
                    IpAddr::V6(value) => value.is_unicast_link_local(),
                })
        }) && self.ip_addresses.iter().all(|address| {
            address
                .parse::<IpAddr>()
                .is_ok_and(|parsed| match parsed {
                    IpAddr::V4(value) => value.is_link_local(),
                    IpAddr::V6(value) => value.is_unicast_link_local(),
                })
        });
        self.is_loopback = self.is_loopback
            || (self.ip_addresses.iter().any(|address| {
                address
                    .parse::<IpAddr>()
                    .is_ok_and(|parsed| parsed.is_loopback())
            }) && self.ip_addresses.iter().all(|address| {
                address
                    .parse::<IpAddr>()
                    .is_ok_and(|parsed| parsed.is_loopback())
            }));
        self.classification = classify_interface(
            &self.name,
            self.is_loopback,
            self.is_link_local_only,
            self.classification,
        );
        self.ignored_reason = interface_ignored_reason(
            &self.name,
            self.is_up,
            self.is_connected,
            self.is_loopback,
            self.is_link_local_only,
        );
    }
}

fn interface_id_rank(id: &str) -> u8 {
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

impl LocalNetworkInterfaceMap {
    pub fn new(
        interfaces: Vec<LocalNetworkInterface>,
        recommended_interface_id: Option<String>,
    ) -> Self {
        let recommended_interface_id = recommended_interface_id.and_then(|id| {
            interfaces
                .iter()
                .find(|interface| interface.id.eq_ignore_ascii_case(&id))
                .map(|interface| interface.id.clone())
        });
        Self {
            interfaces,
            recommended_interface_id,
        }
    }

    pub fn recommended_interface(&self) -> Option<&LocalNetworkInterface> {
        self.recommended_interface_id.as_deref().and_then(|id| {
            self.interfaces
                .iter()
                .find(|interface| interface.id.eq_ignore_ascii_case(id))
        })
    }

    pub fn select_manual_interface(
        &self,
        selected_interface_id: &str,
    ) -> Option<&LocalNetworkInterface> {
        self.interfaces
            .iter()
            .find(|interface| interface.matches_selection(selected_interface_id))
    }

    pub fn select_interface(
        &self,
        selected_interface_id: Option<&str>,
    ) -> Option<&LocalNetworkInterface> {
        match selected_interface_id {
            Some(id) => self.select_manual_interface(id),
            None => self.recommended_interface(),
        }
    }

    pub fn selected_identity(
        &self,
        selected_interface_id: Option<&str>,
    ) -> Option<LocalNetworkIdentity> {
        if let Some(selected_interface_id) = selected_interface_id {
            return self
                .select_manual_interface(selected_interface_id)
                .and_then(LocalNetworkInterface::to_local_network_identity);
        }
        if self.recommended_interface_id.is_none() {
            return None;
        }
        self.recommended_interface()
            .and_then(LocalNetworkInterface::to_local_network_identity)
            .or_else(|| {
                self.interfaces
                    .iter()
                    .filter(|interface| interface.is_eligible_by_default())
                    .find_map(LocalNetworkInterface::to_local_network_identity)
            })
    }
}

fn append_unique_strings(target: &mut Vec<String>, values: Vec<String>) {
    for value in values {
        network_identity_support::push_unique_string(target, value);
    }
}

fn classify_interface(
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
        || normalized.starts_with("veth")
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

fn classification_conservatism(
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

fn interface_ignored_reason(
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
