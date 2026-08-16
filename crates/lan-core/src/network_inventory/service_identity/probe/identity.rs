use std::net::{Ipv4Addr, SocketAddr, TcpStream};
use std::time::Duration;

use super::super::snmp::probe_snmp_identity_query;
use super::super::wsd::probe_wsd_identity_query;
use super::super::{
    AllowedSnmpResponseObserver, LanServiceIdentityProbeObservation, ProbeTarget, ProbeTransport,
    ServiceIdentityProbeSettings, SERVICE_IDENTITY_PROBE_CONNECT_TIMEOUT_MS,
    SERVICE_IDENTITY_PROBE_READ_TIMEOUT_MS,
};
use super::{probe_service_identity_over_http, probe_service_identity_over_https};

pub(super) fn probe_service_identity(
    ip_address: &str,
    device_id: Option<&str>,
    targets: &[ProbeTarget],
    settings: ServiceIdentityProbeSettings,
    deadline: std::time::Instant,
    allowed_snmp_response_observer: AllowedSnmpResponseObserver<'_>,
) -> Option<LanServiceIdentityProbeObservation> {
    for target in targets {
        if std::time::Instant::now() >= deadline {
            return None;
        }
        if let Some(probe_match) = probe_service_identity_on_target(ip_address, *target) {
            return Some(probe_match);
        }
    }
    if settings.allow_wsd_identity_query && std::time::Instant::now() < deadline {
        if let Some(probe_match) = probe_wsd_identity_query(ip_address, device_id) {
            return Some(probe_match);
        }
    }
    if settings.allow_snmp_identity_query && std::time::Instant::now() < deadline {
        return probe_snmp_identity_query(ip_address, allowed_snmp_response_observer);
    }
    None
}

pub(super) fn probe_service_identity_on_target(
    ip_address: &str,
    target: ProbeTarget,
) -> Option<LanServiceIdentityProbeObservation> {
    let ip_address = ip_address.parse::<Ipv4Addr>().ok()?;
    let endpoint = SocketAddr::new(ip_address.into(), target.port);
    for path in target.request_paths {
        let timeout = Duration::from_millis(SERVICE_IDENTITY_PROBE_CONNECT_TIMEOUT_MS);
        let stream = TcpStream::connect_timeout(&endpoint, timeout).ok()?;
        let read_timeout = Some(Duration::from_millis(
            SERVICE_IDENTITY_PROBE_READ_TIMEOUT_MS,
        ));
        let _ = stream.set_read_timeout(read_timeout);
        let _ = stream.set_write_timeout(read_timeout);
        let probe_match = match target.transport {
            ProbeTransport::Http => probe_service_identity_over_http(stream, &endpoint, path),
            ProbeTransport::Https => {
                probe_service_identity_over_https(stream, &endpoint, ip_address, path)
            }
        };
        if probe_match.is_some() {
            return probe_match;
        }
    }
    None
}
