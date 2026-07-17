use chrono::DateTime;
use ocentra_lan_core::network_inventory::LanPassiveRuntimeLocalNetworkIdentity;
use ocentra_lan_core::network_inventory::passive_discovery::{
    LanPassiveDiscoverySource, LanPassiveDiscoveryTriggerReason,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEventHistory, LanDiscoveryEventHistoryState, LanDiscoveryEventKind,
    LanDiscoveryEventRow,
};
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};

#[test]
fn lan_runtime_fields_and_time_preserve_typed_values() -> Result<(), chrono::ParseError> {
    let fields = crate::fields::fields_from_pairs(vec![(
        constants::field::LAN_PAIRING_STATE,
        LogFieldValue::String(constants::value::LAN_PAIRING_UNPAIRED.to_string()),
    )]);

    assert_eq!(
        fields.get(constants::field::LAN_PAIRING_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_PAIRING_UNPAIRED.to_string()
        ))
    );
    assert_eq!(
        crate::time::timestamp_after_epoch_seconds::<String>(10, 5),
        "1970-01-01T00:00:15.000Z"
    );
    let timestamp = crate::time::timestamp_now::<String>();
    let parsed = DateTime::parse_from_rfc3339(&timestamp)?;
    assert_eq!(parsed.offset().local_minus_utc(), 0);
    Ok(())
}

#[test]
fn lan_runtime_passive_discovery_reports_source_inventory_and_network_changes() {
    let sources =
        crate::lan_pairing_runtime_state::passive_discovery::passive_discovery_udp_sources();
    assert_eq!(sources.len(), 6);
    assert_eq!(
        sources,
        &[
            LanPassiveDiscoverySource::Dhcp,
            LanPassiveDiscoverySource::Mdns,
            LanPassiveDiscoverySource::Ssdp,
            LanPassiveDiscoverySource::WsDiscovery,
            LanPassiveDiscoverySource::Llmnr,
            LanPassiveDiscoverySource::Netbios,
        ]
    );

    let previous_identity = LanPassiveRuntimeLocalNetworkIdentity {
        ip_address: Some("192.0.2.10".to_string()),
        network_interface: Some("wifi0".to_string()),
        wifi_ssid: Some("home".to_string()),
        default_gateway: Some("192.0.2.1".to_string()),
    };
    let current_identity = LanPassiveRuntimeLocalNetworkIdentity {
        ip_address: Some("192.0.2.11".to_string()),
        network_interface: Some("wifi0".to_string()),
        wifi_ssid: Some("guest".to_string()),
        default_gateway: Some("192.0.2.254".to_string()),
    };

    let triggers =
        crate::lan_pairing_runtime_state::passive_discovery::local_network_change_triggers(
            Some(&previous_identity),
            &current_identity,
        );

    assert_eq!(triggers.len(), 3);
    assert!(
        triggers
            .iter()
            .any(|trigger| trigger.reason == LanPassiveDiscoveryTriggerReason::WifiSsidChanged)
    );
    assert!(
        triggers
            .iter()
            .any(|trigger| trigger.reason == LanPassiveDiscoveryTriggerReason::IpAddressChanged)
    );
    assert!(triggers.iter().any(|trigger| {
        trigger.reason == LanPassiveDiscoveryTriggerReason::DefaultGatewayChanged
    }));
}

#[test]
fn lan_runtime_stream_payload_preserves_manual_required_event_history() {
    let history = LanDiscoveryEventHistory {
        schema_version: 1,
        generated_at: "2026-07-14T12:00:00.000Z".to_string(),
        state: LanDiscoveryEventHistoryState::ManualRequired,
        latest_event_id: Some("lan-event-1".to_string()),
        latest_observed_at: Some("2026-07-14T12:00:00.000Z".to_string()),
        rows: vec![LanDiscoveryEventRow {
            schema_version: 1,
            event_id: "lan-event-1".to_string(),
            event_kind: LanDiscoveryEventKind::ScanStarted,
            occurred_at: "2026-07-14T12:00:00.000Z".to_string(),
            previous_event_id: None,
            scan_session_id: Some("scan-1".to_string()),
            affected_device_id: None,
            evidence_id: None,
            summary: "scan started".to_string(),
        }],
    };

    let report =
        crate::lan_runtime_stream_payload::stream_lan_runtime_event_chain_for_history(&history);
    let payload =
        crate::lan_runtime_stream_payload::lan_runtime_event_chain_stream_payload(&report);

    assert_eq!(report.observed_events, 1);
    assert_eq!(report.streamed_events, 1);
    assert!(report.manual_required_state);
    assert_eq!(
        payload.get(constants::field::LAN_RUNTIME_MANUAL_REQUIRED_STATE),
        Some(&LogFieldValue::Boolean(true))
    );
    assert_eq!(
        payload.get(constants::field::LAN_RUNTIME_OBSERVED_EVENTS),
        Some(&LogFieldValue::Number(1.0))
    );
}

#[test]
fn lan_runtime_status_event_projects_empty_runtime_state() {
    let event = crate::lan_pairing_status::pairing_status_event(
        &crate::lan_pairing::LanPairingRuntime::empty(),
        status_command(),
    );

    assert_eq!(event.event, AgentEventName::AgentLanPairingStatusReported);
    assert_eq!(
        event.payload.get(constants::field::LAN_PAIRING_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_PAIRING_UNPAIRED.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_DISCOVERY_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_DISCOVERY_STATE_DISCOVERED.to_string()
        ))
    );
}

fn status_command() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: "lan-runtime-status".to_string(),
        sent_at: "2026-07-14T12:00:00.000Z".to_string(),
        source: AgentPeer {
            peer_id: "parent-portal".to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
            platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::LocalNetwork,
        },
        command: AgentCommandName::AgentLanPairingStatusGet,
        payload: LogFields::new(),
    }
}
