use super::*;

macro_rules! restore_optional_probe_env {
    ($name:expr, $value:expr) => {
        match $value {
            Some(value) => env::set_var($name, value),
            None => env::remove_var($name),
        }
    };
}

#[test]
fn service_identity_probe_family_policy_keeps_optional_queries_disabled_by_default() {
    let decisions =
        service_identity_probe_family_decisions(ServiceIdentityProbeSettings::default());

    assert_eq!(
        decisions
            .iter()
            .find(|decision| decision.family == ServiceIdentityProbeFamily::HttpTcp)
            .map(|decision| decision.decision),
        Some(ServiceIdentityProbeDecision::Execute)
    );
    assert!(decisions
        .iter()
        .all(|decision| decision.requires_discovered_host && decision.weak_evidence_only));
    assert_eq!(
        decisions
            .iter()
            .find(|decision| decision.family == ServiceIdentityProbeFamily::WsdIdentityQuery)
            .map(|decision| {
                (
                    decision.decision,
                    decision.requires_discovered_host,
                    decision.weak_evidence_only,
                )
            }),
        Some((
            ServiceIdentityProbeDecision::OperatorSettingRequired,
            true,
            true
        ))
    );
    assert_eq!(
        decisions
            .iter()
            .find(|decision| decision.family == ServiceIdentityProbeFamily::SnmpIdentityQuery)
            .map(|decision| {
                (
                    decision.decision,
                    decision.requires_discovered_host,
                    decision.weak_evidence_only,
                )
            }),
        Some((
            ServiceIdentityProbeDecision::OperatorSettingRequired,
            true,
            true
        ))
    );
    assert_eq!(
        decisions
            .iter()
            .find(|decision| decision.family == ServiceIdentityProbeFamily::OsFingerprint)
            .map(|decision| {
                (
                    decision.decision,
                    decision.requires_discovered_host,
                    decision.weak_evidence_only,
                )
            }),
        Some((ServiceIdentityProbeDecision::ManualGateRequired, true, true))
    );
}

#[test]
fn optional_identity_queries_become_bounded_execute_only_when_enabled() {
    let decisions = service_identity_probe_family_decisions(ServiceIdentityProbeSettings {
        allow_wsd_identity_query: true,
        allow_snmp_identity_query: true,
        allow_os_fingerprint: true,
    });

    assert_eq!(
        decisions
            .iter()
            .find(|decision| decision.family == ServiceIdentityProbeFamily::WsdIdentityQuery)
            .map(|decision| {
                (
                    &decision.decision,
                    decision.allowed_ports.as_slice(),
                    decision.requires_discovered_host,
                    decision.weak_evidence_only,
                )
            }),
        Some((
            &ServiceIdentityProbeDecision::Execute,
            &[5357][..],
            true,
            true,
        ))
    );
    assert_eq!(
        decisions
            .iter()
            .find(|decision| decision.family == ServiceIdentityProbeFamily::SnmpIdentityQuery)
            .map(|decision| {
                (
                    &decision.decision,
                    decision.allowed_ports.as_slice(),
                    decision.requires_discovered_host,
                    decision.weak_evidence_only,
                )
            }),
        Some((
            &ServiceIdentityProbeDecision::Execute,
            &[161][..],
            true,
            true,
        ))
    );
    assert_eq!(
        decisions
            .iter()
            .find(|decision| decision.family == ServiceIdentityProbeFamily::OsFingerprint)
            .map(|decision| {
                (
                    decision.decision,
                    decision.requires_discovered_host,
                    decision.weak_evidence_only,
                )
            }),
        Some((
            ServiceIdentityProbeDecision::RuntimeNotImplemented,
            true,
            true
        ))
    );

    let executable = decisions
        .iter()
        .filter(|decision| decision.decision == ServiceIdentityProbeDecision::Execute)
        .collect::<Vec<_>>();
    assert_eq!(executable.len(), 3);
    assert!(executable
        .iter()
        .any(|decision| decision.family == ServiceIdentityProbeFamily::HttpTcp));
    assert!(executable
        .iter()
        .any(|decision| decision.family == ServiceIdentityProbeFamily::WsdIdentityQuery));
    assert!(executable
        .iter()
        .any(|decision| decision.family == ServiceIdentityProbeFamily::SnmpIdentityQuery));
}

#[test]
fn runtime_visible_service_identity_policy_keeps_optional_families_weak_and_bounded() {
    let decisions =
        service_identity_probe_family_decisions(ServiceIdentityProbeSettings::default());

    let wsd = decisions
        .iter()
        .find(|decision| decision.family == ServiceIdentityProbeFamily::WsdIdentityQuery)
        .value_or_unreachable();
    assert_eq!(
        wsd.decision,
        ServiceIdentityProbeDecision::OperatorSettingRequired
    );
    assert_eq!(wsd.allowed_ports, vec![5357]);
    assert!(wsd.requires_discovered_host);
    assert!(wsd.weak_evidence_only);

    let snmp = decisions
        .iter()
        .find(|decision| decision.family == ServiceIdentityProbeFamily::SnmpIdentityQuery)
        .value_or_unreachable();
    assert_eq!(
        snmp.decision,
        ServiceIdentityProbeDecision::OperatorSettingRequired
    );
    assert_eq!(snmp.allowed_ports, vec![161]);
    assert!(snmp.requires_discovered_host);
    assert!(snmp.weak_evidence_only);

    let os = decisions
        .iter()
        .find(|decision| decision.family == ServiceIdentityProbeFamily::OsFingerprint)
        .value_or_unreachable();
    assert_eq!(
        os.decision,
        ServiceIdentityProbeDecision::ManualGateRequired
    );
    assert!(os.allowed_ports.is_empty());
    assert!(os.requires_discovered_host);
    assert!(os.weak_evidence_only);
}

#[test]
fn runtime_service_identity_settings_keep_optional_queries_disabled_by_default() {
    let _guard = service_identity_env_lock();
    let previous_wsd = env::var(constants::lan_pairing::LAN_ALLOW_WSD_IDENTITY_QUERY_ENV).ok();
    let previous_snmp = env::var(constants::lan_pairing::LAN_ALLOW_SNMP_IDENTITY_QUERY_ENV).ok();
    env::remove_var(constants::lan_pairing::LAN_ALLOW_WSD_IDENTITY_QUERY_ENV);
    env::remove_var(constants::lan_pairing::LAN_ALLOW_SNMP_IDENTITY_QUERY_ENV);

    let settings = runtime_service_identity_probe_settings();

    assert_eq!(
        settings,
        ServiceIdentityProbeSettings {
            allow_wsd_identity_query: false,
            allow_snmp_identity_query: false,
            allow_os_fingerprint: false,
        }
    );

    let decisions = service_identity_probe_family_decisions(settings);

    assert_eq!(
        decisions
            .iter()
            .find(|decision| decision.family == ServiceIdentityProbeFamily::WsdIdentityQuery)
            .map(|decision| decision.decision),
        Some(ServiceIdentityProbeDecision::OperatorSettingRequired)
    );
    assert_eq!(
        decisions
            .iter()
            .find(|decision| decision.family == ServiceIdentityProbeFamily::SnmpIdentityQuery)
            .map(|decision| decision.decision),
        Some(ServiceIdentityProbeDecision::OperatorSettingRequired)
    );

    restore_optional_probe_env!(
        constants::lan_pairing::LAN_ALLOW_WSD_IDENTITY_QUERY_ENV,
        previous_wsd.as_deref()
    );
    restore_optional_probe_env!(
        constants::lan_pairing::LAN_ALLOW_SNMP_IDENTITY_QUERY_ENV,
        previous_snmp.as_deref()
    );
}

#[test]
fn runtime_service_identity_settings_enable_optional_queries_from_env() {
    let _guard = service_identity_env_lock();
    let previous_wsd = env::var(constants::lan_pairing::LAN_ALLOW_WSD_IDENTITY_QUERY_ENV).ok();
    let previous_snmp = env::var(constants::lan_pairing::LAN_ALLOW_SNMP_IDENTITY_QUERY_ENV).ok();
    env::set_var(
        constants::lan_pairing::LAN_ALLOW_WSD_IDENTITY_QUERY_ENV,
        "true",
    );
    env::set_var(
        constants::lan_pairing::LAN_ALLOW_SNMP_IDENTITY_QUERY_ENV,
        "1",
    );

    let settings = runtime_service_identity_probe_settings();

    assert_eq!(
        settings,
        ServiceIdentityProbeSettings {
            allow_wsd_identity_query: true,
            allow_snmp_identity_query: true,
            allow_os_fingerprint: false,
        }
    );

    restore_optional_probe_env!(
        constants::lan_pairing::LAN_ALLOW_WSD_IDENTITY_QUERY_ENV,
        previous_wsd.as_deref()
    );
    restore_optional_probe_env!(
        constants::lan_pairing::LAN_ALLOW_SNMP_IDENTITY_QUERY_ENV,
        previous_snmp.as_deref()
    );
}

#[test]
fn executable_service_identity_target_catalog_remains_curated_tcp_only() {
    let targets = service_identity_probe_targets();
    let decisions =
        service_identity_probe_family_decisions(ServiceIdentityProbeSettings::default());
    let http_tcp_ports = decisions
        .iter()
        .find(|decision| decision.family == ServiceIdentityProbeFamily::HttpTcp)
        .map(|decision| decision.allowed_ports.clone())
        .value_or_unreachable();

    assert_eq!(targets.len(), http_tcp_ports.len());
    assert!(targets
        .iter()
        .all(|target| http_tcp_ports.contains(&target.port)));
    assert!(targets.iter().all(|target| {
        matches!(
            target.transport,
            ProbeTransport::Http | ProbeTransport::Https
        )
    }));
    assert!(!targets.iter().any(|target| target.port == 161));
    assert!(!targets.iter().any(|target| target.port == 3702));
}
