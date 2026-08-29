use std::net::{Ipv4Addr, SocketAddr};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use super::super::snmp::probe_snmp_identity_query_until;
use super::super::wsd::probe_wsd_identity_query_until;
use super::super::{
    AllowedSnmpResponseObserver, LanServiceIdentityProbeObservation, ProbeTarget, ProbeTransport,
    ServiceIdentityProbeSettings, SERVICE_IDENTITY_PROBE_CONNECT_TIMEOUT_MS,
};
use super::transport::connect_until;
use super::{probe_service_identity_over_http_until, probe_service_identity_over_https_until};

pub(super) fn probe_service_identity(
    ip_address: &str,
    device_id: Option<&str>,
    targets: &[ProbeTarget],
    settings: ServiceIdentityProbeSettings,
    deadline: std::time::Instant,
    allowed_snmp_response_observer: AllowedSnmpResponseObserver<'_>,
    cancellation: Option<&AtomicBool>,
) -> Option<LanServiceIdentityProbeObservation> {
    for target in targets {
        if unavailable(deadline, cancellation) {
            return None;
        }
        if let Some(probe_match) =
            probe_service_identity_on_target_until(ip_address, *target, deadline, cancellation)
        {
            return Some(probe_match);
        }
    }
    if settings.allow_wsd_identity_query && !unavailable(deadline, cancellation) {
        if let Some(probe_match) =
            probe_wsd_identity_query_until(ip_address, device_id, deadline, cancellation)
        {
            return Some(probe_match);
        }
    }
    if settings.allow_snmp_identity_query && !unavailable(deadline, cancellation) {
        return probe_snmp_identity_query_until(
            ip_address,
            allowed_snmp_response_observer,
            deadline,
            cancellation,
        );
    }
    None
}

pub(super) fn probe_service_identity_on_target(
    ip_address: &str,
    target: ProbeTarget,
) -> Option<LanServiceIdentityProbeObservation> {
    let deadline = std::time::Instant::now()
        + Duration::from_millis(SERVICE_IDENTITY_PROBE_CONNECT_TIMEOUT_MS);
    probe_service_identity_on_target_until(ip_address, target, deadline, None)
}

fn probe_service_identity_on_target_until(
    ip_address: &str,
    target: ProbeTarget,
    deadline: std::time::Instant,
    cancellation: Option<&AtomicBool>,
) -> Option<LanServiceIdentityProbeObservation> {
    let ip_address = ip_address.parse::<Ipv4Addr>().ok()?;
    let endpoint = SocketAddr::new(ip_address.into(), target.port);
    for path in target.request_paths {
        let connect_deadline = deadline.min(
            Instant::now() + Duration::from_millis(SERVICE_IDENTITY_PROBE_CONNECT_TIMEOUT_MS),
        );
        let stream = connect_until(endpoint, connect_deadline, cancellation)?;
        let probe_match = match target.transport {
            ProbeTransport::Http => probe_service_identity_over_http_until(
                stream,
                &endpoint,
                path,
                deadline,
                cancellation,
            ),
            ProbeTransport::Https => probe_service_identity_over_https_until(
                stream,
                &endpoint,
                ip_address,
                path,
                deadline,
                cancellation,
            ),
        };
        if probe_match.is_some() {
            return probe_match;
        }
    }
    None
}

fn unavailable(deadline: std::time::Instant, cancellation: Option<&AtomicBool>) -> bool {
    cancellation.is_some_and(|value| value.load(Ordering::Acquire))
        || std::time::Instant::now() >= deadline
}
