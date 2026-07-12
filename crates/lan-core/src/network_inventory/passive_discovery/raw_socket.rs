use std::time::Duration;

use super::{
    LanPassiveDiscoveryListenerState, LanPassiveDiscoveryRawSocketCaptureOutcome,
    LanPassiveDiscoveryRawSocketProtocol, LanPassiveDiscoveryRawSocketSupport,
};

mod arp;

pub fn raw_socket_protocol_support_for_platform(
    protocol: LanPassiveDiscoveryRawSocketProtocol,
    platform: &str,
) -> LanPassiveDiscoveryRawSocketSupport {
    match protocol {
        LanPassiveDiscoveryRawSocketProtocol::Arp => arp::protocol_support(protocol, platform),
        LanPassiveDiscoveryRawSocketProtocol::Dhcp => {
            LanPassiveDiscoveryRawSocketSupport::UnsupportedPlatform {
                protocol,
                platform: platform.to_string(),
                reason: "raw-socket passive capture is not implemented in lan-core".to_string(),
            }
        }
    }
}

pub fn raw_socket_protocol_support(
    protocol: LanPassiveDiscoveryRawSocketProtocol,
) -> LanPassiveDiscoveryRawSocketSupport {
    raw_socket_protocol_support_for_platform(protocol, std::env::consts::OS)
}

pub fn collect_raw_socket_protocol_passive_updates(
    state: &mut LanPassiveDiscoveryListenerState,
    protocol: LanPassiveDiscoveryRawSocketProtocol,
    read_timeout: Duration,
) -> LanPassiveDiscoveryRawSocketCaptureOutcome {
    match protocol {
        LanPassiveDiscoveryRawSocketProtocol::Arp => {
            arp::collect_passive_updates(state, read_timeout)
        }
        LanPassiveDiscoveryRawSocketProtocol::Dhcp => {
            LanPassiveDiscoveryRawSocketCaptureOutcome::Unsupported(raw_socket_protocol_support(
                LanPassiveDiscoveryRawSocketProtocol::Dhcp,
            ))
        }
    }
}
