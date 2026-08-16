use super::super::super::mdns_dns_sd::passive_mdns_dns_sd_device_id;
use super::super::dhcp::passive_dhcp_device_id;
use super::super::dns_like::{passive_llmnr_device_id, passive_netbios_device_id};
use super::super::snmp::passive_allowed_snmp_response_device_id;
use super::super::ssdp::passive_ssdp_device_id;
use super::super::ws_discovery::passive_ws_discovery_device_id;
use super::super::LanPassiveDiscoverySource;

pub(super) fn passive_native_datagram_device_id(
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
