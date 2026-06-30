use std::collections::BTreeSet;
use std::env;

use ocentra_parent_agent_protocol::constants;

use super::{
    ProbeTarget, ProbeTransport, ServiceIdentityProbeDecision, ServiceIdentityProbeFamily,
    ServiceIdentityProbeFamilyDecision, ServiceIdentityProbeSettings,
};

pub const HTTP_SAFE_IDENTITY_PORTS: &[u16] = &[80, 8000, 8080];
pub const HTTPS_SAFE_IDENTITY_PORTS: &[u16] = &[443, 8443];

pub fn service_identity_probe_targets() -> Vec<ProbeTarget> {
    if !service_identity_probe_family_decisions(ServiceIdentityProbeSettings::default())
        .iter()
        .any(|decision| {
            decision.family == ServiceIdentityProbeFamily::HttpTcp
                && decision.decision == ServiceIdentityProbeDecision::Execute
        })
    {
        return Vec::new();
    }

    let mut targets = Vec::new();
    let mut seen = BTreeSet::new();

    push_probe_target(
        &mut targets,
        &mut seen,
        ProbeTarget {
            port: default_agent_probe_port(),
            transport: ProbeTransport::Http,
            request_paths: &["/health", "/"],
        },
    );

    if let Some(configured_port) = configured_agent_probe_port() {
        push_probe_target(
            &mut targets,
            &mut seen,
            ProbeTarget {
                port: configured_port,
                transport: ProbeTransport::Http,
                request_paths: &["/health", "/"],
            },
        );
    }

    for port in HTTP_SAFE_IDENTITY_PORTS {
        push_probe_target(
            &mut targets,
            &mut seen,
            ProbeTarget {
                port: *port,
                transport: ProbeTransport::Http,
                request_paths: &["/"],
            },
        );
    }

    for port in HTTPS_SAFE_IDENTITY_PORTS {
        push_probe_target(
            &mut targets,
            &mut seen,
            ProbeTarget {
                port: *port,
                transport: ProbeTransport::Https,
                request_paths: &["/"],
            },
        );
    }

    targets
}

pub fn service_identity_probe_family_decisions_for_discovered_host(
    settings: ServiceIdentityProbeSettings,
) -> Vec<ServiceIdentityProbeFamilyDecision> {
    vec![
        discovered_host_service_identity_probe_family_decision(
            ServiceIdentityProbeFamily::HttpTcp,
            ServiceIdentityProbeDecision::Execute,
            safe_identity_tcp_ports(),
        ),
        discovered_host_service_identity_probe_family_decision(
            ServiceIdentityProbeFamily::WsdIdentityQuery,
            if settings.allow_wsd_identity_query {
                ServiceIdentityProbeDecision::Execute
            } else {
                ServiceIdentityProbeDecision::OperatorSettingRequired
            },
            vec![5357],
        ),
        discovered_host_service_identity_probe_family_decision(
            ServiceIdentityProbeFamily::SnmpIdentityQuery,
            if settings.allow_snmp_identity_query {
                ServiceIdentityProbeDecision::Execute
            } else {
                ServiceIdentityProbeDecision::OperatorSettingRequired
            },
            vec![161],
        ),
        discovered_host_service_identity_probe_family_decision(
            ServiceIdentityProbeFamily::OsFingerprint,
            if settings.allow_os_fingerprint {
                ServiceIdentityProbeDecision::RuntimeNotImplemented
            } else {
                ServiceIdentityProbeDecision::ManualGateRequired
            },
            Vec::new(),
        ),
    ]
}

pub fn discovered_host_service_identity_probe_family_decision(
    family: ServiceIdentityProbeFamily,
    decision: ServiceIdentityProbeDecision,
    allowed_ports: Vec<u16>,
) -> ServiceIdentityProbeFamilyDecision {
    ServiceIdentityProbeFamilyDecision {
        family,
        decision,
        allowed_ports,
        requires_discovered_host: true,
        weak_evidence_only: true,
    }
}

pub fn service_identity_probe_family_decisions(
    settings: ServiceIdentityProbeSettings,
) -> Vec<ServiceIdentityProbeFamilyDecision> {
    service_identity_probe_family_decisions_for_discovered_host(settings)
}

pub fn safe_identity_tcp_ports() -> Vec<u16> {
    let mut ports = HTTP_SAFE_IDENTITY_PORTS.to_vec();
    ports.extend_from_slice(HTTPS_SAFE_IDENTITY_PORTS);
    ports.push(default_agent_probe_port());
    if let Some(configured_port) = configured_agent_probe_port() {
        ports.push(configured_port);
    }
    ports.sort_unstable();
    ports.dedup();
    ports
}

pub fn default_agent_probe_port() -> u16 {
    constants::bind::DEFAULT_AGENT_ADDR
        .parse::<std::net::SocketAddr>()
        .ok()
        .map(|socket| socket.port())
        .unwrap_or(4477)
}

pub fn configured_agent_probe_port() -> Option<u16> {
    env::var(constants::env_var::AGENT_ADDR)
        .ok()
        .and_then(|value| value.parse::<std::net::SocketAddr>().ok())
        .map(|socket| socket.port())
}

pub fn push_probe_target(
    targets: &mut Vec<ProbeTarget>,
    seen: &mut BTreeSet<(u16, ProbeTransport)>,
    target: ProbeTarget,
) {
    if seen.insert((target.port, target.transport)) {
        targets.push(target);
    }
}
