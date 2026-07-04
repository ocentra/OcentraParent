use super::super::mdns_dns_sd::{passive_mdns_dns_sd_device_id, passive_mdns_dns_sd_summary};
use super::dhcp::{passive_dhcp_device_id, passive_dhcp_summary};
use super::dns_like::{
    passive_llmnr_device_id, passive_llmnr_summary, passive_netbios_device_id,
    passive_netbios_summary,
};
use super::snmp::{passive_allowed_snmp_response_device_id, passive_allowed_snmp_response_summary};
use super::ssdp::{passive_ssdp_device_id, passive_ssdp_summary};
use super::text::compact_summary;
use super::ws_discovery::{passive_ws_discovery_device_id, passive_ws_discovery_summary};
use super::LanPassiveDiscoverySource;

pub fn passive_native_datagram_summary(
    source: LanPassiveDiscoverySource,
    payload: &[u8],
) -> Option<String> {
    match source {
        LanPassiveDiscoverySource::Mdns => passive_mdns_dns_sd_summary(payload),
        LanPassiveDiscoverySource::Ssdp => passive_ssdp_summary(payload),
        LanPassiveDiscoverySource::WsDiscovery => passive_ws_discovery_summary(payload),
        LanPassiveDiscoverySource::Llmnr => passive_llmnr_summary(payload),
        LanPassiveDiscoverySource::Netbios => passive_netbios_summary(payload),
        LanPassiveDiscoverySource::Arp => Some(compact_summary(format!(
            "ARP packet: {} byte(s)",
            payload.len()
        ))),
        LanPassiveDiscoverySource::Dhcp => passive_dhcp_summary(payload).or_else(|| {
            Some(compact_summary(format!(
                "DHCP packet: {} byte(s)",
                payload.len()
            )))
        }),
        LanPassiveDiscoverySource::AllowedSnmpResponse => {
            passive_allowed_snmp_response_summary(payload)
        }
        LanPassiveDiscoverySource::OcentraBeacon => Some(compact_summary(format!(
            "Ocentra beacon packet: {} byte(s)",
            payload.len()
        ))),
    }
}

pub fn passive_native_datagram_device_id(
    source: LanPassiveDiscoverySource,
    payload: &[u8],
) -> Option<String> {
    match source {
        LanPassiveDiscoverySource::Mdns => passive_mdns_dns_sd_device_id(payload),
        LanPassiveDiscoverySource::Ssdp => passive_ssdp_device_id(payload),
        LanPassiveDiscoverySource::WsDiscovery => passive_ws_discovery_device_id(payload),
        LanPassiveDiscoverySource::Llmnr => passive_llmnr_device_id(payload),
        LanPassiveDiscoverySource::Netbios => passive_netbios_device_id(payload),
        LanPassiveDiscoverySource::Dhcp => passive_dhcp_device_id(payload),
        LanPassiveDiscoverySource::AllowedSnmpResponse => {
            passive_allowed_snmp_response_device_id(payload)
        }
        _ => None,
    }
}
