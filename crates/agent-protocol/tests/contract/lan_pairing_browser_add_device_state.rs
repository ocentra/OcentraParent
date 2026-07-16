use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEventHistory, LanDiscoveryEventHistoryState, LanDiscoveryEventKind,
    LanDiscoveryEventRow,
};
use ocentra_parent_agent_protocol::LanBrowserAddDeviceDiscoveryDevice;
use ocentra_parent_agent_protocol::LanBrowserAddDeviceReadModel;
use ocentra_parent_agent_protocol::LanBrowserAddDeviceScanSummary;
use ocentra_parent_agent_protocol::LanCanonicalHouseholdDevice;
use ocentra_parent_agent_protocol::LanCanonicalHouseholdDeviceClassification;
use ocentra_parent_agent_protocol::LanCanonicalHouseholdDeviceConfidence;
use ocentra_parent_agent_protocol::LanCanonicalHouseholdDeviceRole;
use ocentra_parent_agent_protocol::LanCanonicalHouseholdDeviceSource;
use ocentra_parent_agent_protocol::LanCanonicalHouseholdNetworkIdentity;
use ocentra_parent_agent_protocol::LanCanonicalHouseholdRoleState;
use ocentra_parent_agent_protocol::LanCanonicalHouseholdRouteState;
use ocentra_parent_agent_protocol::LanCanonicalHouseholdSurface;
use ocentra_parent_agent_protocol::LanChildAgentInventoryPacket;
use ocentra_parent_agent_protocol::LanDiscoveryEvidenceConfidence;
use ocentra_parent_agent_protocol::LanDiscoveryEvidenceKind;
use ocentra_parent_agent_protocol::LanDiscoveryEvidenceRecord;
use ocentra_parent_agent_protocol::LanDiscoveryEvidenceSource;
use ocentra_parent_agent_protocol::LanPairingDeviceHardwareProfile;
use ocentra_parent_agent_protocol::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::LanPairingDiscoveryRuntimeStatus;
use ocentra_parent_agent_protocol::LanPairingNetworkMode;
use ocentra_parent_agent_protocol::LanPairingProductionDiscoveryState;
use ocentra_parent_agent_protocol::LanPairingTrustState;
use ocentra_parent_agent_protocol::LanServiceIdentityProbeEvidence;
use ocentra_parent_agent_protocol::LanServiceIdentityProbeEvidenceKind;
use ocentra_parent_agent_protocol::LAN_PAIRING_SCHEMA_VERSION;

struct LanDiscoveryEvidenceTextParts {
    value: String,
    merge_key: String,
}

#[path = "lan_pairing_browser_add_device_state/production_household_proof_test_support.rs"]
mod production_household_proof_test_support;
#[path = "lan_pairing_browser_add_device_state/signed_discovery_relay_spine_test_support.rs"]
mod signed_discovery_relay_spine_test_support;
#[path = "lan_pairing_browser_add_device_state/source_matrix_test_support.rs"]
mod source_matrix_test_support;

#[test]
fn browser_add_device_read_model_serializes_honest_states() -> Result<(), Box<dyn std::error::Error>>
{
    let model = production_household_proof_test_support::browser_add_device_read_model_fixture();

    let json = serde_json::to_string(&model)?;
    let value: serde_json::Value = serde_json::from_str(&json)?;
    production_household_proof_test_support::assert_browser_add_device_read_model_json(&value);
    Ok(())
}

#[test]
fn signed_discovery_relay_spine_serializes_adapter_rejection_and_relay_boundaries(
) -> Result<(), Box<dyn std::error::Error>> {
    let spine = signed_discovery_relay_spine_test_support::signed_discovery_relay_spine_fixture();

    let json = serde_json::to_value(&spine)?;
    signed_discovery_relay_spine_test_support::assert_signed_discovery_relay_spine_json(&json);
    Ok(())
}

#[test]
fn lan_discovery_source_matrix_serializes_workpack_and_source_boundaries(
) -> Result<(), Box<dyn std::error::Error>> {
    let matrix = source_matrix_test_support::source_matrix_fixture();

    let json = serde_json::to_value(&matrix)?;
    source_matrix_test_support::assert_source_matrix_json(&json);
    assert_eq!(
        json[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_WORKPACK_ROWS][15]["title"],
        serde_json::json!(constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_16)
    );
    assert_eq!(
        json[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_SOURCE_ROWS][4]
            [constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_SOURCE],
        serde_json::json!("signed-child-agent-hello")
    );
    assert_eq!(
        json[constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_SOURCE_ROWS][5]
            [constants::lan_pairing::LAN_SOURCE_MATRIX_FIELD_SOURCE],
        serde_json::json!("signed-child-agent-heartbeat")
    );
    assert_eq!(
        json["generatedAt"],
        serde_json::json!("2026-05-23T14:41:00.000Z")
    );
    assert_eq!(
        json["claimsProved"],
        serde_json::json!([
            constants::lan_pairing::LAN_SOURCE_MATRIX_CLAIM_READ_MODEL,
            constants::lan_pairing::LAN_SOURCE_MATRIX_CLAIM_WEAK_SOURCES
        ])
    );
    Ok(())
}

#[test]
fn discovered_device_serializes_network_and_hardware_details(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut child_device = LanPairingDeviceRef::new(
        constants::lan_pairing::LOCAL_AGENT_DEVICE_ID.to_string(),
        None,
        constants::lan_pairing::LOCAL_AGENT_LABEL.to_string(),
        constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
    );
    child_device.ip_address = Some("192.168.2.42".to_string());
    child_device.mac_address = Some("54-27-1e-97-c3-31".to_string());
    child_device.hostname = Some("GAMEDEV".to_string());
    child_device.network_interface = Some("Ethernet 2".to_string());
    child_device.agent_status = Some(constants::lan_pairing::LOCAL_AGENT_STATUS.to_string());
    child_device.install_id = Some("child-install-contract".to_string());
    child_device.hardware_profile = Some(LanPairingDeviceHardwareProfile {
        manufacturer: Some("Gigabyte Technology Co., Ltd.".to_string()),
        model: Some("X570 AORUS MASTER".to_string()),
        cpu_model: Some("AMD Ryzen 9 3900X 12-Core Processor".to_string()),
        cpu_cores: Some("12 cores / 24 logical".to_string()),
        memory_total: Some("63 GiB".to_string()),
        gpu_model: Some("GeForce RTX 2070 SUPER".to_string()),
        gpu_driver: Some("456.71".to_string()),
        gpu_memory: Some("8192 MiB".to_string()),
        nvidia_smi: Some("GeForce RTX 2070 SUPER driver 456.71 8192 MiB VRAM".to_string()),
    });

    let device = LanBrowserAddDeviceDiscoveryDevice {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        discovered_at: "2026-06-01T15:20:00.000Z".to_string(),
        child_device,
        agent_peer_id: constants::lan_pairing::PARENT_PEER_ID.to_string(),
        pairing_id: Some(constants::lan_pairing::PAIRING_ID.to_string()),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        network_mode: LanPairingNetworkMode::LocalNetwork,
        reachability: LanPairingDeviceReachability::Online,
        address_ref: constants::lan_pairing::ADDRESS_REF_DIRECT_WEBSOCKET.to_string(),
        discovery_status: LanPairingDiscoveryRuntimeStatus::WebsocketDirect,
        discovery_state: LanPairingProductionDiscoveryState::Discovered,
        evidence_sources: vec![LanDiscoveryEvidenceSource::LocalService],
        service_identity_probe_evidence: vec![LanServiceIdentityProbeEvidence {
            evidence_kind: LanServiceIdentityProbeEvidenceKind::HttpStatus,
            value: "200".to_string(),
            selected_interface: Some("Wi-Fi".to_string()),
        }],
        hint_sources: Vec::new(),
    };

    let json = serde_json::to_value(&device)?;
    assert_eq!(
        json["childDevice"]["ipAddress"],
        serde_json::json!("192.168.2.42")
    );
    assert_eq!(
        json["childDevice"]["hardwareProfile"]["gpuModel"],
        serde_json::json!("GeForce RTX 2070 SUPER")
    );
    assert_eq!(
        json["childDevice"]["installId"],
        serde_json::json!("child-install-contract")
    );
    assert_eq!(
        json["pairingId"],
        serde_json::json!(constants::lan_pairing::PAIRING_ID)
    );
    assert_eq!(
        json[constants::field::LAN_SERVICE_IDENTITY_PROBE_EVIDENCE][0]["evidenceKind"],
        serde_json::json!(constants::value::LAN_SERVICE_IDENTITY_PROBE_HTTP_STATUS)
    );
    assert_eq!(
        json[constants::field::LAN_SERVICE_IDENTITY_PROBE_EVIDENCE][0]["value"],
        serde_json::json!("200")
    );
    assert_eq!(
        json[constants::field::LAN_SERVICE_IDENTITY_PROBE_EVIDENCE][0]["selectedInterface"],
        serde_json::json!("Wi-Fi")
    );
    Ok(())
}

#[test]
fn service_identity_probe_evidence_serializes_wsd_metadata_variants(
) -> Result<(), Box<dyn std::error::Error>> {
    let evidence = vec![
        LanServiceIdentityProbeEvidence {
            evidence_kind: LanServiceIdentityProbeEvidenceKind::WsdEndpointAddress,
            value: "urn:uuid:camera-1".to_string(),
            selected_interface: Some("Wi-Fi".to_string()),
        },
        LanServiceIdentityProbeEvidence {
            evidence_kind: LanServiceIdentityProbeEvidenceKind::WsdTypes,
            value: "dn:NetworkVideoTransmitter".to_string(),
            selected_interface: None,
        },
    ];

    let json = serde_json::to_value(&evidence)?;

    assert_eq!(
        json[0]["evidenceKind"],
        serde_json::json!("wsd-endpoint-address")
    );
    assert_eq!(json[0]["value"], serde_json::json!("urn:uuid:camera-1"));
    assert_eq!(json[1]["evidenceKind"], serde_json::json!("wsd-types"));
    assert_eq!(
        json[1]["value"],
        serde_json::json!("dn:NetworkVideoTransmitter")
    );
    assert_eq!(json[0]["selectedInterface"], serde_json::json!("Wi-Fi"));
    assert!(json[1].get("selectedInterface").is_none());
    Ok(())
}

#[test]
fn service_identity_probe_evidence_serializes_snmp_metadata_variants(
) -> Result<(), Box<dyn std::error::Error>> {
    let evidence = vec![
        LanServiceIdentityProbeEvidence {
            evidence_kind: LanServiceIdentityProbeEvidenceKind::SnmpSysDescr,
            value: "Linux camera controller".to_string(),
            selected_interface: Some("Ethernet".to_string()),
        },
        LanServiceIdentityProbeEvidence {
            evidence_kind: LanServiceIdentityProbeEvidenceKind::SnmpSysName,
            value: "cam-1".to_string(),
            selected_interface: None,
        },
    ];

    let json = serde_json::to_value(&evidence)?;

    assert_eq!(json[0]["evidenceKind"], serde_json::json!("snmp-sys-descr"));
    assert_eq!(
        json[0]["value"],
        serde_json::json!("Linux camera controller")
    );
    assert_eq!(json[1]["evidenceKind"], serde_json::json!("snmp-sys-name"));
    assert_eq!(json[1]["value"], serde_json::json!("cam-1"));
    assert_eq!(json[0]["selectedInterface"], serde_json::json!("Ethernet"));
    assert!(json[1].get("selectedInterface").is_none());
    Ok(())
}

#[test]
fn expanded_household_classification_variants_serialize_with_stable_wire_names(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut device = canonical_child_agent_device();
    device.classification = LanCanonicalHouseholdDeviceClassification::Television;
    device.enrollable = false;
    device.child_agent_inventory = None;

    let json = serde_json::to_value(&device)?;

    assert_eq!(json["classification"], serde_json::json!("television"));
    Ok(())
}

#[test]
fn discovery_evidence_records_keep_device_and_timestamp_fields_explicit(
) -> Result<(), Box<dyn std::error::Error>> {
    let record = evidence_record(
        LanDiscoveryEvidenceSource::LocalService,
        LanDiscoveryEvidenceKind::ChildAgentPresence,
        LanDiscoveryEvidenceTextParts {
            value: constants::lan_pairing::LOCAL_AGENT_STATUS.to_string(),
            merge_key: "agent:lan-physical-mac-54271e97c331".to_string(),
        },
        LanDiscoveryEvidenceConfidence::Confirmed,
    );

    let json = serde_json::to_value(record)?;

    assert_eq!(
        json[constants::field::EVIDENCE_ID],
        serde_json::json!("agent-lan-physical-mac-54271e97c331")
    );
    assert_eq!(
        json[constants::field::DEVICE_ID],
        serde_json::json!("lan-physical-mac-54271e97c331")
    );
    assert_eq!(
        json[constants::field::FIRST_SEEN_AT],
        serde_json::json!("2026-06-01T15:20:00.000Z")
    );
    assert_eq!(
        json[constants::field::LAST_SEEN_AT],
        serde_json::json!("2026-06-01T15:20:00.000Z")
    );
    assert_eq!(
        json["mergeKey"],
        serde_json::json!("agent:lan-physical-mac-54271e97c331")
    );
    Ok(())
}

#[test]
fn discovery_event_rows_keep_event_session_and_device_fields_explicit(
) -> Result<(), Box<dyn std::error::Error>> {
    let history = LanDiscoveryEventHistory {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        generated_at: "2026-06-01T15:20:00.000Z".to_string(),
        state: LanDiscoveryEventHistoryState::Ready,
        latest_event_id: Some("lan-discovery-scan-finished-lan-scan-1717255200000".to_string()),
        latest_observed_at: Some("2026-06-01T15:20:00.000Z".to_string()),
        rows: vec![LanDiscoveryEventRow {
            schema_version: LAN_PAIRING_SCHEMA_VERSION,
            event_id: "lan-discovery-scan-started-lan-scan-1717255200000".to_string(),
            event_kind: LanDiscoveryEventKind::ScanStarted,
            occurred_at: "2026-06-01T15:19:58.000Z".to_string(),
            previous_event_id: None,
            scan_session_id: Some("lan-scan-1717255200000".to_string()),
            affected_device_id: Some(constants::lan_pairing::LOCAL_AGENT_DEVICE_ID.to_string()),
            evidence_id: Some("agent-lan-physical-mac-54271e97c331".to_string()),
            summary: "LAN scan started".to_string(),
        }],
    };

    let json = serde_json::to_value(history)?;

    assert_eq!(json["state"], serde_json::json!("ready"));
    assert_eq!(
        json["latestEventId"],
        serde_json::json!("lan-discovery-scan-finished-lan-scan-1717255200000")
    );
    assert_eq!(
        json["rows"][0][constants::field::EVENT_ID],
        serde_json::json!("lan-discovery-scan-started-lan-scan-1717255200000")
    );
    assert_eq!(
        json["rows"][0]["occurredAt"],
        serde_json::json!("2026-06-01T15:19:58.000Z")
    );
    assert_eq!(
        json["rows"][0]["scanSessionId"],
        serde_json::json!("lan-scan-1717255200000")
    );
    assert_eq!(
        json["rows"][0]["affectedDeviceId"],
        serde_json::json!(constants::lan_pairing::LOCAL_AGENT_DEVICE_ID)
    );
    Ok(())
}

#[test]
fn discovery_evidence_and_event_payloads_reject_missing_required_fields(
) -> Result<(), Box<dyn std::error::Error>> {
    let evidence_error = serde_json::from_value::<LanDiscoveryEvidenceRecord>(serde_json::json!({
        "schemaVersion": LAN_PAIRING_SCHEMA_VERSION,
        "evidenceId": "lan-evidence-1"
    }))
    .err()
    .ok_or_else(|| {
        std::io::Error::other("discovery evidence must reject missing required fields")
    })?;
    let event_error = serde_json::from_value::<LanDiscoveryEventRow>(serde_json::json!({
        "schemaVersion": LAN_PAIRING_SCHEMA_VERSION,
        "eventId": "lan-discovery-scan-1"
    }))
    .err()
    .ok_or_else(|| std::io::Error::other("discovery events must reject missing required fields"))?;

    assert_eq!(evidence_error.classify(), serde_json::error::Category::Data);
    assert_eq!(event_error.classify(), serde_json::error::Category::Data);
    Ok(())
}

#[test]
fn browser_add_device_contracts_reject_wrong_schema_versions(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut read_model_json = serde_json::to_value(
        production_household_proof_test_support::browser_add_device_read_model_fixture(),
    )?;
    read_model_json["schemaVersion"] = serde_json::json!(LAN_PAIRING_SCHEMA_VERSION + 1);

    let read_model_error = serde_json::from_value::<LanBrowserAddDeviceReadModel>(read_model_json)
        .err()
        .ok_or_else(|| {
            std::io::Error::other("future LAN browser schema version must fail closed")
        })?;
    assert!(read_model_error
        .to_string()
        .contains("unsupported LAN schema version"));

    let mut evidence_json = serde_json::to_value(evidence_record(
        LanDiscoveryEvidenceSource::LocalService,
        LanDiscoveryEvidenceKind::Vendor,
        LanDiscoveryEvidenceTextParts {
            value: "AzureWave Technology Inc.".to_string(),
            merge_key: "vendor:azurewavetechnologyinc".to_string(),
        },
        LanDiscoveryEvidenceConfidence::Strong,
    ))?;
    evidence_json["schemaVersion"] = serde_json::json!(LAN_PAIRING_SCHEMA_VERSION + 1);

    let evidence_error = serde_json::from_value::<LanDiscoveryEvidenceRecord>(evidence_json)
        .err()
        .ok_or_else(|| {
            std::io::Error::other("nested LAN browser schema version must fail closed")
        })?;
    assert!(evidence_error
        .to_string()
        .contains("unsupported LAN schema version"));
    Ok(())
}

#[test]
fn browser_add_device_contracts_reject_unknown_enum_variants(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut event_json = serde_json::to_value(LanDiscoveryEventRow {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        event_id: "lan-discovery-event-unknown-kind".to_string(),
        event_kind: LanDiscoveryEventKind::ScanStarted,
        occurred_at: "2026-06-01T15:19:58.000Z".to_string(),
        previous_event_id: None,
        scan_session_id: Some("lan-scan-1717255200000".to_string()),
        affected_device_id: Some(constants::lan_pairing::LOCAL_AGENT_DEVICE_ID.to_string()),
        evidence_id: Some("agent-lan-physical-mac-54271e97c331".to_string()),
        summary: "LAN scan started".to_string(),
    })?;
    event_json["eventKind"] = serde_json::json!("future-lan-discovery-kind");

    let error = serde_json::from_value::<LanDiscoveryEventRow>(event_json)
        .err()
        .ok_or_else(|| {
            std::io::Error::other("unknown LAN browser enum variant must be rejected")
        })?;

    assert!(error.is_data());
    Ok(())
}

fn scan_summary() -> LanBrowserAddDeviceScanSummary {
    LanBrowserAddDeviceScanSummary {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        source_labels: vec![constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string()],
        scanned_device_count: 0,
        agent_device_count: 0,
        passive_device_count: 0,
        infrastructure_device_count: 0,
        unsupported_device_count: 0,
        passive_local_neighbor_collection_summaries: Vec::new(),
    }
}

fn canonical_child_agent_device() -> LanCanonicalHouseholdDevice {
    LanCanonicalHouseholdDevice {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        canonical_device_id: "lan-physical-mac-54271e97c331".to_string(),
        display_name: "GAMEDEV".to_string(),
        classification: LanCanonicalHouseholdDeviceClassification::ChildAgent,
        role_badges: vec![
            LanCanonicalHouseholdDeviceRole::ChildAgent,
            LanCanonicalHouseholdDeviceRole::Portal,
            LanCanonicalHouseholdDeviceRole::ParentController,
        ],
        enrollable: true,
        discovery_state: LanPairingProductionDiscoveryState::Paired,
        trust_state: LanPairingTrustState::Paired,
        route_id: Some(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string()),
        route_state: LanCanonicalHouseholdRouteState::LocalNetwork,
        network_mode: LanPairingNetworkMode::LocalNetwork,
        source_labels: vec![
            LanCanonicalHouseholdDeviceSource::LocalService,
            LanCanonicalHouseholdDeviceSource::NetworkNeighbor,
            LanCanonicalHouseholdDeviceSource::TrustedRegistry,
        ],
        network_identity: canonical_network_identity(),
        child_agent_inventory: Some(canonical_inventory_packet()),
        policy_target_surfaces: all_child_agent_surfaces(),
    }
}

fn canonical_network_identity() -> LanCanonicalHouseholdNetworkIdentity {
    LanCanonicalHouseholdNetworkIdentity {
        hostname: Some("GAMEDEV".to_string()),
        ip_addresses: vec!["192.168.2.42".to_string()],
        mac_address: Some("54-27-1e-97-c3-31".to_string()),
        mac_vendor: Some("AzureWave Technology Inc.".to_string()),
        network_interfaces: vec!["Ethernet 2".to_string()],
        reachability: LanPairingDeviceReachability::Online,
        confidence: LanCanonicalHouseholdDeviceConfidence::AgentConfirmed,
        stale_at: None,
        offline_at: None,
        evidence_records: canonical_evidence_records(),
    }
}

fn canonical_evidence_records() -> Vec<LanDiscoveryEvidenceRecord> {
    vec![
        evidence_record(
            LanDiscoveryEvidenceSource::LocalService,
            LanDiscoveryEvidenceKind::IpAddress,
            LanDiscoveryEvidenceTextParts {
                value: "192.168.2.42".to_string(),
                merge_key: "ip:192.168.2.42".to_string(),
            },
            LanDiscoveryEvidenceConfidence::Confirmed,
        ),
        evidence_record(
            LanDiscoveryEvidenceSource::LocalService,
            LanDiscoveryEvidenceKind::MacAddress,
            LanDiscoveryEvidenceTextParts {
                value: "54-27-1e-97-c3-31".to_string(),
                merge_key: "mac:54271e97c331".to_string(),
            },
            LanDiscoveryEvidenceConfidence::Confirmed,
        ),
        evidence_record(
            LanDiscoveryEvidenceSource::LocalService,
            LanDiscoveryEvidenceKind::Vendor,
            LanDiscoveryEvidenceTextParts {
                value: "AzureWave Technology Inc.".to_string(),
                merge_key: "vendor:azurewavetechnologyinc".to_string(),
            },
            LanDiscoveryEvidenceConfidence::Strong,
        ),
        evidence_record(
            LanDiscoveryEvidenceSource::LocalService,
            LanDiscoveryEvidenceKind::Hostname,
            LanDiscoveryEvidenceTextParts {
                value: "GAMEDEV".to_string(),
                merge_key: "hostname:gamedev".to_string(),
            },
            LanDiscoveryEvidenceConfidence::Confirmed,
        ),
        evidence_record(
            LanDiscoveryEvidenceSource::LocalService,
            LanDiscoveryEvidenceKind::Interface,
            LanDiscoveryEvidenceTextParts {
                value: "Ethernet 2".to_string(),
                merge_key: "interface:ethernet2".to_string(),
            },
            LanDiscoveryEvidenceConfidence::Confirmed,
        ),
        evidence_record(
            LanDiscoveryEvidenceSource::LocalService,
            LanDiscoveryEvidenceKind::ChildAgentPresence,
            LanDiscoveryEvidenceTextParts {
                value: constants::lan_pairing::LOCAL_AGENT_STATUS.to_string(),
                merge_key: "agent:lan-physical-mac-54271e97c331".to_string(),
            },
            LanDiscoveryEvidenceConfidence::Confirmed,
        ),
    ]
}

fn canonical_inventory_packet() -> LanChildAgentInventoryPacket {
    LanChildAgentInventoryPacket {
        device_name: "GAMEDEV".to_string(),
        platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
        os: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
        cpu_model: Some("AMD Ryzen 9 3900X 12-Core Processor".to_string()),
        cpu_cores: Some("12 cores / 24 logical".to_string()),
        memory_total: Some("63 GiB".to_string()),
        gpu_model: Some("GeForce RTX 2070 SUPER".to_string()),
        gpu_driver: Some("456.71".to_string()),
        gpu_memory: Some("8192 MiB".to_string()),
        nvidia_smi: Some("GeForce RTX 2070 SUPER driver 456.71 8192 MiB VRAM".to_string()),
        network_interfaces: vec!["Ethernet 2".to_string()],
        capabilities: vec![
            constants::lan_pairing::CHILD_AGENT_CAPABILITY_DIRECT_WEBSOCKET.to_string(),
            constants::lan_pairing::CHILD_AGENT_CAPABILITY_DEVICE_INVENTORY.to_string(),
            constants::lan_pairing::CHILD_AGENT_CAPABILITY_PAIRING_ROUTE.to_string(),
        ],
        role_state: LanCanonicalHouseholdRoleState::Implemented,
        route_state: LanCanonicalHouseholdRouteState::LocalNetwork,
        pairing_trust_state: LanPairingTrustState::Paired,
    }
}

fn evidence_record(
    source: LanDiscoveryEvidenceSource,
    evidence_kind: LanDiscoveryEvidenceKind,
    text: LanDiscoveryEvidenceTextParts,
    confidence: LanDiscoveryEvidenceConfidence,
) -> LanDiscoveryEvidenceRecord {
    LanDiscoveryEvidenceRecord {
        schema_version: LAN_PAIRING_SCHEMA_VERSION,
        evidence_id: text.merge_key.replace(':', "-"),
        source,
        evidence_kind,
        device_id: "lan-physical-mac-54271e97c331".to_string(),
        normalized_value: text.value.to_ascii_lowercase(),
        first_seen_at: "2026-06-01T15:20:00.000Z".to_string(),
        last_seen_at: "2026-06-01T15:20:00.000Z".to_string(),
        expires_at: None,
        confidence,
        value: text.value,
        merge_key: text.merge_key,
        note: None,
    }
}

fn all_child_agent_surfaces() -> Vec<LanCanonicalHouseholdSurface> {
    vec![
        LanCanonicalHouseholdSurface::Devices,
        LanCanonicalHouseholdSurface::Policy,
        LanCanonicalHouseholdSurface::Browser,
        LanCanonicalHouseholdSurface::App,
        LanCanonicalHouseholdSurface::Screen,
        LanCanonicalHouseholdSurface::Network,
        LanCanonicalHouseholdSurface::Activity,
        LanCanonicalHouseholdSurface::Tracking,
        LanCanonicalHouseholdSurface::Ai,
    ]
}
