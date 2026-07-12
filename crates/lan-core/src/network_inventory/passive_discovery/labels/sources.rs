use super::super::LanPassiveDiscoverySource;

pub(super) fn passive_source_label(source: &LanPassiveDiscoverySource) -> &'static str {
    match source {
        LanPassiveDiscoverySource::Arp => "arp",
        LanPassiveDiscoverySource::Dhcp => "dhcp",
        LanPassiveDiscoverySource::Mdns => "mdns",
        LanPassiveDiscoverySource::Ssdp => "ssdp",
        LanPassiveDiscoverySource::WsDiscovery => "ws-discovery",
        LanPassiveDiscoverySource::Llmnr => "llmnr",
        LanPassiveDiscoverySource::Netbios => "netbios",
        LanPassiveDiscoverySource::OcentraBeacon => "ocentra-beacon",
        LanPassiveDiscoverySource::AllowedSnmpResponse => "allowed-snmp-response",
    }
}
