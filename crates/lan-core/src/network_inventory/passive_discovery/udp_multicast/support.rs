use super::super::{LanPassiveDiscoverySource, LanPassiveDiscoveryUdpMulticastSupport};

pub(super) fn udp_multicast_support(
    source: LanPassiveDiscoverySource,
) -> LanPassiveDiscoveryUdpMulticastSupport {
    match source {
        LanPassiveDiscoverySource::Mdns => multicast_available(source, "224.0.0.251", 5353),
        LanPassiveDiscoverySource::Ssdp => multicast_available(source, "239.255.255.250", 1900),
        LanPassiveDiscoverySource::WsDiscovery => {
            multicast_available(source, "239.255.255.250", 3702)
        }
        LanPassiveDiscoverySource::Llmnr => multicast_available(source, "224.0.0.252", 5355),
        LanPassiveDiscoverySource::Dhcp => {
            LanPassiveDiscoveryUdpMulticastSupport::AvailableBroadcast { source, port: 67 }
        }
        LanPassiveDiscoverySource::Netbios => {
            LanPassiveDiscoveryUdpMulticastSupport::AvailableBroadcast { source, port: 137 }
        }
        LanPassiveDiscoverySource::Arp => unsupported_udp_source(
            source,
            "raw-socket passive capture is not implemented in lan-core",
        ),
        LanPassiveDiscoverySource::AllowedSnmpResponse => unsupported_udp_source(
            source,
            "passive SNMP response capture requires an explicit allowed probe socket",
        ),
        LanPassiveDiscoverySource::OcentraBeacon => unsupported_udp_source(
            source,
            "Ocentra beacon packets are accepted through the signed child hello path",
        ),
    }
}

fn multicast_available(
    source: LanPassiveDiscoverySource,
    multicast_group: &str,
    port: u16,
) -> LanPassiveDiscoveryUdpMulticastSupport {
    LanPassiveDiscoveryUdpMulticastSupport::Available {
        source,
        multicast_group: multicast_group.to_string(),
        port,
    }
}

fn unsupported_udp_source(
    source: LanPassiveDiscoverySource,
    reason: &str,
) -> LanPassiveDiscoveryUdpMulticastSupport {
    LanPassiveDiscoveryUdpMulticastSupport::Unsupported {
        source,
        reason: reason.to_string(),
    }
}
