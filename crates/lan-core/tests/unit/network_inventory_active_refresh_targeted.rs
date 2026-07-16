use super::*;

fn target_ip(value: impl std::fmt::Display) -> std::net::Ipv4Addr {
    value.to_string().parse().value_or_unreachable()
}
use ocentra_lan_core::network_inventory::active_refresh::evidence::{
    targeted_arp_refresh_evidence_from_observation,
    targeted_arp_refresh_targets_with_packet_io_until,
};
use ocentra_lan_core::network_inventory::active_refresh::target_builders::targeted_arp_refresh_targets;
use ocentra_lan_core::network_inventory::active_refresh::throttle::{
    clear_targeted_arp_refresh_attempts, targeted_arp_refresh_throttled_at,
    TARGETED_ARP_REFRESH_ATTEMPTS, TARGETED_ARP_REFRESH_THROTTLE_MS,
};

#[test]
fn targeted_arp_refresh_targets_only_selected_hosts_on_the_selected_interface() {
    let targets = targeted_arp_refresh_targets(
        Some("192.168.2.42"),
        Some("192.168.2.42/24"),
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE),
        &[
            trusted_device(
                constants::lan_pairing::TEST_LAN_MAC,
                Some("192.168.2.20"),
                Some(constants::lan_pairing::TEST_HOSTNAME),
                constants::lan_pairing::TEST_HOSTNAME,
                constants::lan_pairing::PLATFORM_WINDOWS,
            ),
            trusted_device(
                constants::lan_pairing::TEST_ROUTER_MAC,
                Some("192.168.3.20"),
                Some(constants::lan_pairing::TEST_HOSTNAME),
                constants::lan_pairing::TEST_HOSTNAME,
                constants::lan_pairing::PLATFORM_WINDOWS,
            ),
        ],
        &[
            LanNetworkInventoryDevice {
                device_id: "lan-device-previous".to_string(),
                label: "Previous LAN device".to_string(),
                platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
                ip_address: "192.168.2.21".to_string(),
                mac_address: constants::lan_pairing::TEST_ROUTER_MAC.to_string(),
                hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
                network_interface: Some("Wi-Fi".to_string()),
                reachability: LanPairingDeviceReachability::Online,
                agent_status: None,
                scan_sources: vec![
                    constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()
                ],
                observed_at: String::new(),
                used_previous_scan_hint: false,
                service_identity_probe_evidence: Vec::new(),
            },
            LanNetworkInventoryDevice {
                device_id: "lan-device-selected".to_string(),
                label: "Selected LAN device".to_string(),
                platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
                ip_address: "192.168.2.22".to_string(),
                mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
                hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
                network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
                reachability: LanPairingDeviceReachability::Online,
                agent_status: None,
                scan_sources: vec![
                    constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()
                ],
                observed_at: String::new(),
                used_previous_scan_hint: false,
                service_identity_probe_evidence: Vec::new(),
            },
        ],
    );

    assert_eq!(targets.len(), 2);
    assert_eq!(
        targets
            .iter()
            .map(|target| target.ip_address.to_string())
            .collect::<Vec<_>>(),
        vec!["192.168.2.20".to_string(), "192.168.2.22".to_string()]
    );
    assert!(targets
        .iter()
        .all(|target| target.network_interface.as_deref()
            == Some(constants::lan_pairing::TEST_NETWORK_INTERFACE)));
    assert_eq!(
        targets
            .iter()
            .map(|target| target.expected_mac_address.as_deref())
            .collect::<Vec<_>>(),
        vec![
            Some(constants::lan_pairing::TEST_LAN_MAC),
            Some(constants::lan_pairing::TEST_LAN_MAC)
        ]
    );
}

#[test]
fn targeted_arp_refresh_targets_require_a_selected_interface_and_local_subnet() {
    assert!(targeted_arp_refresh_targets(
        Some("192.168.2.42"),
        Some("192.168.2.42/24"),
        Some(""),
        &[trusted_device(
            constants::lan_pairing::TEST_LAN_MAC,
            Some("192.168.2.20"),
            Some(constants::lan_pairing::TEST_HOSTNAME),
            constants::lan_pairing::TEST_HOSTNAME,
            constants::lan_pairing::PLATFORM_WINDOWS,
        )],
        &[]
    )
    .is_empty());

    assert!(targeted_arp_refresh_targets(
        Some("192.168.2.42"),
        Some("192.168.3.42/24"),
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE),
        &[trusted_device(
            constants::lan_pairing::TEST_LAN_MAC,
            Some("192.168.2.20"),
            Some(constants::lan_pairing::TEST_HOSTNAME),
            constants::lan_pairing::TEST_HOSTNAME,
            constants::lan_pairing::PLATFORM_WINDOWS,
        )],
        &[]
    )
    .is_empty());
}

#[test]
fn targeted_arp_refresh_response_records_mac_and_strong_identity_match_only_when_macs_align() {
    let target_ip_address = "192.168.2.23";
    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip(target_ip_address),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };

    let evidence = targeted_arp_refresh_evidence_from_observation(
        &target,
        Some(constants::lan_pairing::TEST_LAN_MAC.to_ascii_lowercase()),
        1_717_000_000_000,
        false,
    );

    assert_eq!(evidence.target_ip_address, target_ip_address);
    assert_eq!(
        evidence.selected_interface.as_deref(),
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE)
    );
    assert_eq!(
        evidence.expected_mac_address.as_deref(),
        Some(constants::lan_pairing::TEST_LAN_MAC)
    );
    assert_eq!(
        evidence.observed_mac_address.as_deref(),
        Some(constants::lan_pairing::TEST_LAN_MAC)
    );
    assert_eq!(evidence.observed_at_unix_ms, 1_717_000_000_000);
    assert_eq!(
        evidence.source,
        if cfg!(target_os = "windows") {
            constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()
        } else if cfg!(target_os = "macos") {
            constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP.to_string()
        } else {
            constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string()
        }
    );
    assert_eq!(
        evidence.outcome,
        Some(LanTargetedArpRefreshOutcome::Response)
    );
    assert!(evidence.strong_identity_match);
    assert!(!evidence.throttled);
}

#[test]
fn targeted_arp_refresh_no_response_is_stale_presence_evidence_not_deletion() {
    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip("192.168.2.250"),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };

    let evidence =
        targeted_arp_refresh_evidence_from_observation(&target, None, 1_717_000_000_000, false);

    assert_eq!(
        evidence.outcome,
        Some(LanTargetedArpRefreshOutcome::NoResponse)
    );
    assert!(evidence.observed_mac_address.is_none());
    assert!(!evidence.strong_identity_match);
    assert!(!evidence.throttled);
    assert_eq!(
        evidence.source,
        if cfg!(target_os = "windows") {
            constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()
        } else if cfg!(target_os = "macos") {
            constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP.to_string()
        } else {
            constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string()
        }
    );
}

#[test]
fn targeted_arp_refresh_malformed_response_is_rejected_without_identity_match() {
    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip("192.168.2.20"),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };

    let evidence = targeted_arp_refresh_evidence_from_observation(
        &target,
        Some("not-a-mac-address".to_string()),
        1_717_000_000_000,
        false,
    );

    assert_eq!(
        evidence.outcome,
        Some(LanTargetedArpRefreshOutcome::NoResponse)
    );
    assert!(evidence.observed_mac_address.is_none());
    assert!(!evidence.strong_identity_match);
}

#[test]
fn targeted_arp_refresh_throttles_repeated_checks_per_host_and_interface() {
    let _guard = targeted_arp_refresh_attempt_state_guard();
    clear_targeted_arp_refresh_attempts();

    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip("192.168.2.250"),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };
    let first = Instant::now();
    assert!(targeted_arp_refresh_throttled_at(&target, first).is_none());
    assert_eq!(
        targeted_arp_refresh_throttled_at(
            &target,
            first + Duration::from_millis(TARGETED_ARP_REFRESH_THROTTLE_MS - 1)
        ),
        Some(first)
    );
    assert!(targeted_arp_refresh_throttled_at(
        &target,
        first + Duration::from_millis(TARGETED_ARP_REFRESH_THROTTLE_MS + 1)
    )
    .is_none());

    let same_ip_other_interface =
        ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
            ip_address: target_ip("192.168.2.250"),
            expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
            network_interface: Some("Ethernet 3".to_string()),
        };
    assert!(targeted_arp_refresh_throttled_at(
        &same_ip_other_interface,
        first + Duration::from_millis(1)
    )
    .is_none());
}

#[test]
fn targeted_arp_refresh_recovers_from_poisoned_attempt_state() {
    let _guard = targeted_arp_refresh_attempt_state_guard();
    clear_targeted_arp_refresh_attempts();

    let attempts = TARGETED_ARP_REFRESH_ATTEMPTS.get_or_init(|| Mutex::new(HashMap::new()));
    let _ = std::panic::catch_unwind(|| {
        let _lock = attempts.lock().value_or_unreachable();
        assert_eq!(0, 1, "poison targeted arp refresh attempt state");
    });

    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip("192.168.2.250"),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };
    let first = Instant::now();

    assert!(targeted_arp_refresh_throttled_at(&target, first).is_none());
    assert_eq!(
        targeted_arp_refresh_throttled_at(
            &target,
            first + Duration::from_millis(TARGETED_ARP_REFRESH_THROTTLE_MS - 1)
        ),
        Some(first)
    );

    clear_targeted_arp_refresh_attempts();
}

#[derive(Default)]
struct FakePacketIo {
    probed_targets: Vec<std::net::Ipv4Addr>,
    observations:
        Vec<ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshObservation>,
}

impl ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshPacketIo
    for FakePacketIo
{
    fn probe_target(
        &mut self,
        target: &ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget,
        _deadline: Instant,
    ) -> bool {
        self.probed_targets.push(target.ip_address);
        true
    }

    fn observations(
        &mut self,
        _deadline: Instant,
    ) -> Vec<ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshObservation>
    {
        self.observations.clone()
    }
}

#[test]
fn targeted_arp_refresh_uses_packet_io_abstraction_and_dedupes_replies() {
    let _guard = targeted_arp_refresh_attempt_state_guard();
    clear_targeted_arp_refresh_attempts();

    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip("192.168.2.24"),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };
    let mut packet_io = FakePacketIo {
        probed_targets: Vec::new(),
        observations: vec![
            ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshObservation {
                ip_address: target_ip("192.168.2.24"),
                mac_address: constants::lan_pairing::TEST_LAN_MAC.to_ascii_lowercase(),
            },
            ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshObservation {
                ip_address: target_ip("192.168.2.24"),
                mac_address: "not-a-mac".to_string(),
            },
            ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshObservation {
                ip_address: target_ip("192.168.2.25"),
                mac_address: constants::lan_pairing::TEST_ROUTER_MAC.to_string(),
            },
        ],
    };

    let evidence = targeted_arp_refresh_targets_with_packet_io_until(
        &[target],
        Instant::now() + Duration::from_secs(1),
        &mut packet_io,
    );

    assert_eq!(packet_io.probed_targets, vec![target_ip("192.168.2.24")]);
    assert_eq!(evidence.len(), 1);
    assert_eq!(evidence[0].target_ip_address, "192.168.2.24");
    assert_eq!(
        evidence[0].selected_interface.as_deref(),
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE)
    );
    assert_eq!(
        evidence[0].observed_mac_address.as_deref(),
        Some(constants::lan_pairing::TEST_LAN_MAC)
    );
    assert!(evidence[0].observed_at_unix_ms > 0);
    assert_eq!(
        evidence[0].source,
        if cfg!(target_os = "windows") {
            constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()
        } else if cfg!(target_os = "macos") {
            constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP.to_string()
        } else {
            constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string()
        }
    );
    assert!(evidence[0].strong_identity_match);
}

#[test]
fn targeted_arp_refresh_budget_skips_without_recording_false_no_response() {
    let _guard = targeted_arp_refresh_attempt_state_guard();
    clear_targeted_arp_refresh_attempts();

    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip("192.168.2.250"),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };
    let mut packet_io = FakePacketIo {
        probed_targets: Vec::new(),
        observations: Vec::new(),
    };
    let evidence = targeted_arp_refresh_targets_with_packet_io_until(
        &[target],
        Instant::now() - Duration::from_millis(1),
        &mut packet_io,
    );

    assert!(evidence.is_empty());
    assert!(packet_io.probed_targets.is_empty());
}

struct NoAttemptPacketIo {
    observations_called: bool,
}

impl ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshPacketIo
    for NoAttemptPacketIo
{
    fn probe_target(
        &mut self,
        _target: &ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget,
        _deadline: Instant,
    ) -> bool {
        false
    }

    fn observations(
        &mut self,
        _deadline: Instant,
    ) -> Vec<ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshObservation>
    {
        self.observations_called = true;
        Vec::new()
    }
}

struct ExhaustedObservationBudgetPacketIo {
    probed_targets: Vec<std::net::Ipv4Addr>,
    observations_called: bool,
}

impl ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshPacketIo
    for ExhaustedObservationBudgetPacketIo
{
    fn probe_target(
        &mut self,
        target: &ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget,
        _deadline: Instant,
    ) -> bool {
        self.probed_targets.push(target.ip_address);
        true
    }

    fn observations(
        &mut self,
        _deadline: Instant,
    ) -> Vec<ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshObservation>
    {
        self.observations_called = true;
        Vec::new()
    }

    fn has_observation_budget(&mut self, _deadline: Instant) -> bool {
        false
    }
}

#[test]
fn targeted_arp_refresh_without_probe_attempt_records_no_false_no_response() {
    let _guard = targeted_arp_refresh_attempt_state_guard();
    clear_targeted_arp_refresh_attempts();

    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip("192.168.2.251"),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };
    let mut packet_io = NoAttemptPacketIo {
        observations_called: false,
    };
    let evidence = targeted_arp_refresh_targets_with_packet_io_until(
        &[target],
        Instant::now() + Duration::from_secs(1),
        &mut packet_io,
    );

    assert!(evidence.is_empty());
    assert!(!packet_io.observations_called);
}

#[test]
fn targeted_arp_refresh_skips_no_response_when_observation_budget_is_exhausted_after_probe() {
    let _guard = targeted_arp_refresh_attempt_state_guard();
    clear_targeted_arp_refresh_attempts();

    let target = ocentra_lan_core::network_inventory::active_refresh::TargetedArpRefreshTarget {
        ip_address: target_ip("192.168.2.252"),
        expected_mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    };
    let mut packet_io = ExhaustedObservationBudgetPacketIo {
        probed_targets: Vec::new(),
        observations_called: false,
    };
    let evidence = targeted_arp_refresh_targets_with_packet_io_until(
        &[target],
        Instant::now() + Duration::from_secs(1),
        &mut packet_io,
    );

    assert_eq!(packet_io.probed_targets, vec![target_ip("192.168.2.252")]);
    assert!(evidence.is_empty());
    assert!(!packet_io.observations_called);
}
