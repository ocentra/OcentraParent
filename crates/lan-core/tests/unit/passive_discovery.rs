use crate::support::OptionTestExt as _;
use std::time::Duration;

#[path = "../../src/network_inventory/name_evidence.rs"]
mod name_evidence;

use ocentra_lan_core::network_inventory::passive_discovery::raw_socket::{
    collect_raw_socket_protocol_passive_updates, raw_socket_protocol_support,
    raw_socket_protocol_support_for_platform,
};
use ocentra_lan_core::network_inventory::passive_discovery::{
    LanPassiveDiscoveryListenerState, LanPassiveDiscoveryRawSocketCaptureOutcome,
    LanPassiveDiscoveryRawSocketProtocol, LanPassiveDiscoveryRawSocketSupport,
    LanPassiveDiscoverySource,
};
use ocentra_parent_agent_protocol::constants;

#[test]
fn arp_support_reports_real_collector_backing_for_supported_platforms() {
    assert_eq!(
        raw_socket_protocol_support_for_platform(LanPassiveDiscoveryRawSocketProtocol::Arp, "windows"),
        LanPassiveDiscoveryRawSocketSupport::AvailableCollector {
            protocol: LanPassiveDiscoveryRawSocketProtocol::Arp,
            platform: "windows".to_string(),
            collector_labels: vec!["windows-neighbor-table".to_string()],
            reason: "lan-core records passive ARP weak hints from OS neighbor tables instead of raw frames"
                .to_string(),
        }
    );
    assert_eq!(
        raw_socket_protocol_support_for_platform(LanPassiveDiscoveryRawSocketProtocol::Arp, "linux"),
        LanPassiveDiscoveryRawSocketSupport::AvailableCollector {
            protocol: LanPassiveDiscoveryRawSocketProtocol::Arp,
            platform: "linux".to_string(),
            collector_labels: vec![
                "linux-proc-net-arp".to_string(),
                "linux-ip-neigh".to_string(),
            ],
            reason: "lan-core records passive ARP weak hints from OS neighbor tables instead of raw frames"
                .to_string(),
        }
    );
    assert_eq!(
        raw_socket_protocol_support_for_platform(LanPassiveDiscoveryRawSocketProtocol::Arp, "android"),
        LanPassiveDiscoveryRawSocketSupport::AvailableCollector {
            protocol: LanPassiveDiscoveryRawSocketProtocol::Arp,
            platform: "android".to_string(),
            collector_labels: vec![
                "linux-proc-net-arp".to_string(),
                "linux-ip-neigh".to_string(),
            ],
            reason: "lan-core records passive ARP weak hints from OS neighbor tables instead of raw frames"
                .to_string(),
        }
    );
    assert_eq!(
        raw_socket_protocol_support_for_platform(LanPassiveDiscoveryRawSocketProtocol::Arp, "macos"),
        LanPassiveDiscoveryRawSocketSupport::AvailableCollector {
            protocol: LanPassiveDiscoveryRawSocketProtocol::Arp,
            platform: "macos".to_string(),
            collector_labels: vec!["macos-arp".to_string()],
            reason: "lan-core records passive ARP weak hints from OS neighbor tables instead of raw frames"
                .to_string(),
        }
    );
    assert_eq!(
        raw_socket_protocol_support_for_platform(
            LanPassiveDiscoveryRawSocketProtocol::Arp,
            "freebsd"
        ),
        LanPassiveDiscoveryRawSocketSupport::UnsupportedPlatform {
            protocol: LanPassiveDiscoveryRawSocketProtocol::Arp,
            platform: "freebsd".to_string(),
            reason: "no passive ARP collector is implemented for this platform".to_string(),
        }
    );
}

#[test]
fn dhcp_support_remains_honestly_unsupported() {
    assert_eq!(
        raw_socket_protocol_support_for_platform(
            LanPassiveDiscoveryRawSocketProtocol::Dhcp,
            "linux"
        ),
        LanPassiveDiscoveryRawSocketSupport::UnsupportedPlatform {
            protocol: LanPassiveDiscoveryRawSocketProtocol::Dhcp,
            platform: "linux".to_string(),
            reason: "raw-socket passive capture is not implemented in lan-core".to_string(),
        }
    );
}

#[test]
fn weak_name_evidence_wrappers_stay_weak_and_normalized() {
    let observed_at = "2026-06-26T00:00:00Z";
    let cases = [
        (
            name_evidence::dns_cache_name_evidence(
                " Kid-Laptop.local. ",
                observed_at,
                Some(" Wi-Fi "),
            ),
            constants::lan_pairing::LAN_SCAN_SOURCE_DNS_CACHE,
        ),
        (
            name_evidence::reverse_dns_name_evidence(
                " Kid-Laptop.local. ",
                observed_at,
                Some(" Wi-Fi "),
            ),
            constants::lan_pairing::LAN_SCAN_SOURCE_DNS_CACHE,
        ),
        (
            name_evidence::netbios_name_evidence(
                " Kid-Laptop.local. ",
                observed_at,
                Some(" Wi-Fi "),
            ),
            constants::lan_pairing::LAN_SCAN_SOURCE_NETBIOS,
        ),
        (
            name_evidence::llmnr_name_evidence(" Kid-Laptop.local. ", observed_at, Some(" Wi-Fi ")),
            constants::lan_pairing::LAN_SCAN_SOURCE_LLMNR,
        ),
    ];

    for (evidence, expected_source_label) in cases {
        let evidence = evidence.value_or_unreachable();
        assert_eq!(evidence.source_label(), expected_source_label);
        assert_eq!(evidence.confidence_label(), "weak");
        assert_eq!(evidence.value, "Kid-Laptop.local");
        assert_eq!(evidence.normalized_value, "kid-laptop.local");
        assert_eq!(evidence.first_seen_at, observed_at);
        assert_eq!(evidence.last_seen_at, observed_at);
        assert_eq!(evidence.network_interface.as_deref(), Some("Wi-Fi"));
    }

    assert!(name_evidence::reverse_dns_name_evidence(
        "Kid-Laptop.local..",
        observed_at,
        Some(" Wi-Fi ")
    )
    .is_none());
}

#[test]
fn arp_collection_uses_current_platform_neighbor_collectors() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-26T00:00:00Z".to_string());
    let outcome = collect_raw_socket_protocol_passive_updates(
        &mut state,
        LanPassiveDiscoveryRawSocketProtocol::Arp,
        Duration::from_millis(250),
    );
    let expected_collector_labels = if cfg!(target_os = "windows") {
        vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()]
    } else if cfg!(any(target_os = "linux", target_os = "android")) {
        vec![
            constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string(),
        ]
    } else if cfg!(target_os = "macos") {
        vec![constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP.to_string()]
    } else {
        Vec::new()
    };

    match outcome {
        LanPassiveDiscoveryRawSocketCaptureOutcome::Captured {
            protocol,
            collector_labels,
            observed_count,
            recorded_count,
        } => {
            assert_eq!(protocol, LanPassiveDiscoveryRawSocketProtocol::Arp);
            assert_eq!(collector_labels, expected_collector_labels);
            assert_eq!(observed_count, recorded_count);
            assert_eq!(state.snapshot().rows.len(), recorded_count);
        }
        LanPassiveDiscoveryRawSocketCaptureOutcome::Unsupported(support) => {
            assert_eq!(
                support,
                LanPassiveDiscoveryRawSocketSupport::AvailableCollector {
                    protocol: LanPassiveDiscoveryRawSocketProtocol::Arp,
                    platform: std::env::consts::OS.to_string(),
                    collector_labels: expected_collector_labels,
                    reason: "lan-core records passive ARP weak hints from OS neighbor tables instead of raw frames"
                        .to_string(),
                }
            );
        }
    }
}

#[test]
fn dhcp_collection_remains_unsupported_until_a_real_listener_exists() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-26T00:00:00Z".to_string());
    assert_eq!(
        collect_raw_socket_protocol_passive_updates(
            &mut state,
            LanPassiveDiscoveryRawSocketProtocol::Dhcp,
            Duration::from_millis(50),
        ),
        LanPassiveDiscoveryRawSocketCaptureOutcome::Unsupported(raw_socket_protocol_support(
            LanPassiveDiscoveryRawSocketProtocol::Dhcp
        ),)
    );
}

#[test]
fn passive_listener_records_ocentra_beacon_updates_with_explicit_timestamp() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-26T00:00:00Z".to_string());

    assert_eq!(
        state.record_passive_update(
            LanPassiveDiscoverySource::OcentraBeacon,
            ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryTriggerReason::AppResumed,
            "2026-06-26T00:00:01Z",
            Some("device-beacon-1"),
            Some("scan-beacon-1"),
            "ocentra beacon update",
        ),
        ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryRecordOutcome::Recorded
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(snapshot.rows[0].source, Some(ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoverySource::OcentraBeacon));
    assert_eq!(
        snapshot.rows[0].trigger_reason,
        ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryTriggerReason::AppResumed
    );
    assert_eq!(
        snapshot.rows[0].device_id.as_ref().map(AsRef::as_ref),
        Some("device-beacon-1")
    );
    assert_eq!(
        snapshot.rows[0].scan_session_id.as_ref().map(AsRef::as_ref),
        Some("scan-beacon-1")
    );
    assert_eq!(snapshot.rows[0].summary, "ocentra beacon update");
}

#[test]
fn passive_listener_records_later_return_for_same_device_without_deduping() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-26T00:00:00Z".to_string());

    assert_eq!(
        state.record_passive_update(
            LanPassiveDiscoverySource::Mdns,
            ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryTriggerReason::PassivePacketObserved,
            "2026-06-26T00:00:01Z",
            Some("device-return-1"),
            None,
            "mdns return",
        ),
        ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryRecordOutcome::Recorded
    );
    assert_eq!(
        state.record_passive_update(
            LanPassiveDiscoverySource::Mdns,
            ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryTriggerReason::PassivePacketObserved,
            "2026-06-26T00:05:01Z",
            Some("device-return-1"),
            None,
            "mdns return",
        ),
        ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryRecordOutcome::Recorded
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 2);
    assert_eq!(
        snapshot.rows[0].device_id.as_ref().map(AsRef::as_ref),
        Some("device-return-1")
    );
    assert_eq!(
        snapshot.rows[1].device_id.as_ref().map(AsRef::as_ref),
        Some("device-return-1")
    );
    assert_ne!(snapshot.rows[0].event_id, snapshot.rows[1].event_id);
    assert_eq!(
        snapshot.rows[1].previous_event_id,
        Some(snapshot.rows[0].event_id.clone())
    );
    assert_eq!(
        snapshot.latest_event_id,
        Some(snapshot.rows[1].event_id.clone())
    );
}

#[test]
fn passive_listener_records_rescan_trigger_rows_with_explicit_reason_and_no_source() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-26T00:00:00Z".to_string());

    assert_eq!(
        state.record_rescan_trigger(
            ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryTriggerReason::WifiSsidChanged,
            "2026-06-26T00:10:00Z",
            "wifi changed",
        ),
        ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryRecordOutcome::Recorded
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(
        snapshot.rows[0].event_kind,
        ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryEventKind::RescanTrigger
    );
    assert_eq!(snapshot.rows[0].source, None);
    assert_eq!(
        snapshot.rows[0].trigger_reason,
        ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryTriggerReason::WifiSsidChanged
    );
    assert_eq!(snapshot.rows[0].summary, "wifi changed");
}

#[test]
fn stopped_passive_listener_refuses_updates_and_rescan_triggers() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-26T00:00:00Z".to_string());
    state.stop();

    assert_eq!(
        state.record_passive_update(
            LanPassiveDiscoverySource::Mdns,
            ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryTriggerReason::PassivePacketObserved,
            "2026-06-26T00:00:01Z",
            Some("device-stopped-1"),
            None,
            "mdns update after stop",
        ),
        ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryRecordOutcome::Stopped
    );
    assert_eq!(
        state.record_rescan_trigger(
            ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryTriggerReason::HeartbeatLost,
            "2026-06-26T00:00:02Z",
            "heartbeat lost after stop",
        ),
        ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryRecordOutcome::Stopped
    );

    let snapshot = state.snapshot();
    assert_eq!(
        snapshot.lifecycle_state,
        ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryListenerLifecycleState::Stopped
    );
    assert!(snapshot.rows.is_empty());
}
