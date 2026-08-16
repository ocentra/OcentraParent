use super::super::super::mdns_dns_sd::passive_mdns_dns_sd_summary;
use super::super::dhcp::passive_dhcp_summary;
use super::super::dns_like::{passive_llmnr_summary, passive_netbios_summary};
use super::super::snmp::passive_allowed_snmp_response_summary;
use super::super::ssdp::passive_ssdp_summary;
use super::super::text::compact_summary;
use super::super::ws_discovery::passive_ws_discovery_summary;
use super::super::LanPassiveDiscoverySource;

pub(super) fn passive_native_datagram_summary(
    source: LanPassiveDiscoverySource,
    payload: &[u8],
) -> Option<String> {
    match source {
        LanPassiveDiscoverySource::Mdns => passive_mdns_dns_sd_summary(payload),
        LanPassiveDiscoverySource::Ssdp => passive_ssdp_summary(payload),
        LanPassiveDiscoverySource::WsDiscovery => passive_ws_discovery_summary(payload),
        LanPassiveDiscoverySource::Llmnr => passive_llmnr_summary(payload),
        LanPassiveDiscoverySource::Netbios => passive_netbios_summary(payload),
        LanPassiveDiscoverySource::Arp => Some(packet_size_summary("ARP", payload)),
        LanPassiveDiscoverySource::Dhcp => {
            passive_dhcp_summary(payload).or_else(|| Some(packet_size_summary("DHCP", payload)))
        }
        LanPassiveDiscoverySource::AllowedSnmpResponse => {
            passive_allowed_snmp_response_summary(payload)
        }
        LanPassiveDiscoverySource::OcentraBeacon => {
            Some(packet_size_summary("Ocentra beacon", payload))
        }
    }
}

fn packet_size_summary(label: &str, payload: &[u8]) -> String {
    compact_summary(format!("{label} packet: {} byte(s)", payload.len()))
}
