use super::super::network_identity_support;
use super::super::{
    LocalNetworkIdentity, LocalNetworkInterface, LocalNetworkInterfaceClassification,
    LocalNetworkInterfaceMap,
};

#[path = "model_classification.rs"]
mod model_classification;
#[path = "model_classification_rank.rs"]
mod model_classification_rank;
#[path = "model_ignored_reason.rs"]
mod model_ignored_reason;
#[path = "model_refresh.rs"]
mod model_refresh;

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
            && self.ip_addresses.iter().any(|address| {
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
        if model_classification::interface_id_rank(&candidate.id)
            > model_classification::interface_id_rank(&self.id)
        {
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
        if model_classification_rank::classification_conservatism(candidate.classification)
            > model_classification_rank::classification_conservatism(self.classification)
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
        model_refresh::refresh(self);
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
        self.recommended_interface_id.as_ref()?;
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
