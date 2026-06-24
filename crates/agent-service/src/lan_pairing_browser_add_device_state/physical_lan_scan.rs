use chrono::{DateTime, Duration, Utc};
use ocentra_lan_core::network_inventory::{
    discover_lan_network_devices_with_hints_refresh_mode_and_scan_and_probe_suppression,
    plan_lan_discovery_scan_with_active_refresh_suppression, LanDiscoveryRefreshMode,
    LanNetworkInventoryDevice,
};
use ocentra_lan_core::read_model::discovered_devices_from_network_inventory;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceRef, LanPairingTrustState, LanTrustedDeviceRegistryEntry,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanCanonicalHouseholdDeviceClassification,
    LanHouseholdDeviceDecision,
};
use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentCommandName};

use crate::lan_pairing::LanPairingRuntime;

use super::scan_history::{
    load_scan_history_snapshot, save_scan_history, LanScanHistoryMetadata, LanScanHistorySnapshot,
};
use super::{household_device_decisions, known_household_devices, trusted_device_registry};

pub(super) fn network_devices_for_command(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
) -> Vec<LanNetworkInventoryDevice> {
    if command_uses_physical_lan_scan(&command.command) {
        let now = Utc::now();
        let previous_scan_snapshot = load_scan_history_snapshot(runtime);
        let previous_devices = previous_scan_snapshot
            .as_ref()
            .map(|snapshot| snapshot.devices.as_slice())
            .unwrap_or_default();
        let scan_truth = scan_truth_context(runtime, previous_scan_snapshot.as_ref(), now);
        let refresh_mode = refresh_mode_for_command(&command.command);
        let scan_plan = plan_lan_discovery_scan_with_active_refresh_suppression(
            &scan_truth.identity_hint_devices,
            &previous_devices,
            refresh_mode,
            &scan_truth.scan_suppression_devices,
        );
        let devices =
            discover_lan_network_devices_with_hints_refresh_mode_and_scan_and_probe_suppression(
                &scan_truth.identity_hint_devices,
                &previous_devices,
                refresh_mode,
                &scan_truth.scan_suppression_devices,
                &scan_truth.scan_suppression_devices,
            );
        save_scan_history(
            runtime,
            &devices,
            Some(LanScanHistoryMetadata {
                scan_id: scan_session_id(now),
                paired_registry_truth_count: scan_truth.paired_registry_truth_count,
                recent_previous_agent_truth_count: scan_truth.recent_previous_agent_truth_count,
                durable_household_truth_count: scan_truth.durable_household_truth_count,
                scan_plan,
            }),
        );
        return devices;
    }
    Vec::new()
}

struct LanScanTruthContext {
    identity_hint_devices: Vec<LanPairingDeviceRef>,
    scan_suppression_devices: Vec<LanPairingDeviceRef>,
    paired_registry_truth_count: u32,
    recent_previous_agent_truth_count: u32,
    durable_household_truth_count: u32,
}

fn scan_truth_context(
    runtime: &LanPairingRuntime,
    previous_scan_snapshot: Option<&LanScanHistorySnapshot>,
    now: DateTime<Utc>,
) -> LanScanTruthContext {
    let trusted_registry = trusted_device_registry(runtime);
    let stored_known_household_devices = known_household_devices(runtime);
    let household_decisions = household_device_decisions(runtime);
    let mut identity_hint_devices = paired_child_devices(&trusted_registry);
    let paired_registry_truth_count =
        u32::try_from(identity_hint_devices.len()).unwrap_or(u32::MAX);
    let historical_devices = recent_previous_scan_agent_truth_devices(previous_scan_snapshot, now);
    let recent_previous_agent_truth_count =
        u32::try_from(historical_devices.len()).unwrap_or(u32::MAX);
    for historical_device in historical_devices {
        push_unique_scan_truth_device(&mut identity_hint_devices, historical_device);
    }
    let durable_household_truth_devices = durable_household_scan_suppression_devices(
        &stored_known_household_devices,
        previous_scan_snapshot,
        &trusted_registry,
        &household_decisions,
    );
    let durable_household_truth_count =
        u32::try_from(durable_household_truth_devices.len()).unwrap_or(u32::MAX);
    for truth_device in durable_household_truth_devices {
        push_unique_scan_truth_device(&mut identity_hint_devices, truth_device);
    }
    let scan_suppression_devices = identity_hint_devices.clone();
    LanScanTruthContext {
        identity_hint_devices,
        scan_suppression_devices,
        paired_registry_truth_count,
        recent_previous_agent_truth_count,
        durable_household_truth_count,
    }
}

fn paired_child_devices(
    trusted_registry: &[LanTrustedDeviceRegistryEntry],
) -> Vec<LanPairingDeviceRef> {
    trusted_registry
        .iter()
        .filter(|entry| {
            entry.trust_state == LanPairingTrustState::Paired && entry.revoked_at.is_none()
        })
        .map(|entry| entry.child_device.clone())
        .collect()
}

fn recent_previous_scan_agent_truth_devices(
    previous_scan_snapshot: Option<&LanScanHistorySnapshot>,
    now: DateTime<Utc>,
) -> Vec<LanPairingDeviceRef> {
    let Some(previous_scan_snapshot) = previous_scan_snapshot else {
        return Vec::new();
    };
    if !scan_history_is_recent(&previous_scan_snapshot.updated_at, now) {
        return Vec::new();
    }
    previous_scan_snapshot
        .devices
        .iter()
        .filter(|device| historical_agent_truth_should_suppress_probe(device))
        .map(previous_scan_truth_device)
        .collect()
}

fn scan_history_is_recent(updated_at: &str, now: DateTime<Utc>) -> bool {
    DateTime::parse_from_rfc3339(updated_at)
        .map(|parsed| {
            let parsed = parsed.with_timezone(&Utc);
            parsed <= now
                && now.signed_duration_since(parsed)
                    <= Duration::seconds(
                        constants::lan_pairing::LAN_PREVIOUS_SCAN_AGENT_TRUTH_REUSE_WINDOW_SECONDS,
                    )
        })
        .unwrap_or(false)
}

fn historical_agent_truth_should_suppress_probe(device: &LanNetworkInventoryDevice) -> bool {
    matches!(
        device.agent_status.as_deref(),
        Some(constants::lan_pairing::LOCAL_AGENT_STATUS)
            | Some(constants::lan_pairing::SERVICE_IDENTITY_PROBE_AGENT_STATUS)
    )
}

fn previous_scan_truth_device(device: &LanNetworkInventoryDevice) -> LanPairingDeviceRef {
    let mut truth_device = LanPairingDeviceRef::new(
        device.device_id.clone(),
        None,
        device.label.clone(),
        device.platform.clone(),
    );
    truth_device.ip_address = Some(device.ip_address.clone());
    truth_device.mac_address = Some(device.mac_address.clone());
    truth_device.hostname = device.hostname.clone();
    truth_device.network_interface = device.network_interface.clone();
    truth_device.agent_status = device.agent_status.clone();
    truth_device
}

fn durable_household_scan_suppression_devices(
    stored_known_household_devices: &[LanCanonicalHouseholdDevice],
    previous_scan_snapshot: Option<&LanScanHistorySnapshot>,
    trusted_registry: &[LanTrustedDeviceRegistryEntry],
    household_device_decisions: &[LanHouseholdDeviceDecision],
) -> Vec<LanPairingDeviceRef> {
    let mut devices = stored_known_household_devices
        .iter()
        .filter(|device| household_device_should_suppress_redundant_scan_work(device))
        .cloned()
        .filter_map(household_scan_suppression_device)
        .collect::<Vec<_>>();

    let Some(previous_scan_snapshot) = previous_scan_snapshot else {
        return devices;
    };
    let discovered_devices = discovered_devices_from_network_inventory(
        &previous_scan_snapshot.devices,
        &previous_scan_snapshot.updated_at,
    );
    let historical_devices = ocentra_lan_core::read_model_builder::canonical_household_devices(
        &discovered_devices,
        trusted_registry,
        household_device_decisions,
        &previous_scan_snapshot.updated_at,
    );
    for device in historical_devices
        .into_iter()
        .filter(household_device_should_suppress_redundant_scan_work)
        .filter_map(household_scan_suppression_device)
    {
        push_unique_scan_truth_device(&mut devices, device);
    }
    devices
}

fn household_device_should_suppress_redundant_scan_work(
    device: &LanCanonicalHouseholdDevice,
) -> bool {
    matches!(
        device.classification,
        LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure
            | LanCanonicalHouseholdDeviceClassification::ChildAgent
    ) || matches!(
        device.trust_state,
        LanPairingTrustState::Paired | LanPairingTrustState::Revoked
    ) || device.child_agent_inventory.is_some()
}

fn household_scan_suppression_device(
    device: LanCanonicalHouseholdDevice,
) -> Option<LanPairingDeviceRef> {
    let platform = scan_suppression_platform(&device);
    let mut truth_device = LanPairingDeviceRef::new(
        device.canonical_device_id.clone(),
        None,
        device.display_name.clone(),
        platform,
    );
    truth_device.ip_address = device.network_identity.ip_addresses.first().cloned();
    truth_device.mac_address = device.network_identity.mac_address.clone();
    truth_device.hostname = device.network_identity.hostname.clone();
    truth_device.network_interface = device.network_identity.network_interfaces.first().cloned();
    (truth_device.ip_address.is_some() || truth_device.mac_address.is_some())
        .then_some(truth_device)
}

fn scan_suppression_platform(device: &LanCanonicalHouseholdDevice) -> String {
    if device.classification == LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure {
        return constants::lan_pairing::PLATFORM_ROUTER.to_string();
    }
    device
        .child_agent_inventory
        .as_ref()
        .map(|inventory| inventory.platform.clone())
        .unwrap_or_else(|| constants::lan_pairing::PLATFORM_UNKNOWN.to_string())
}

fn push_unique_scan_truth_device(
    devices: &mut Vec<LanPairingDeviceRef>,
    candidate: LanPairingDeviceRef,
) {
    if devices.iter().any(|existing| {
        existing
            .mac_address
            .as_deref()
            .zip(candidate.mac_address.as_deref())
            .map(|(left, right)| left.eq_ignore_ascii_case(right))
            .unwrap_or(false)
            || existing
                .ip_address
                .as_deref()
                .zip(candidate.ip_address.as_deref())
                .map(|(left, right)| left.eq_ignore_ascii_case(right))
                .unwrap_or(false)
    }) {
        return;
    }
    devices.push(candidate);
}

fn scan_session_id(now: DateTime<Utc>) -> String {
    format!("lan-scan-{}", now.timestamp_millis())
}

fn command_uses_physical_lan_scan(command: &AgentCommandName) -> bool {
    matches!(
        command,
        AgentCommandName::AgentLanPairingStatusGet
            | AgentCommandName::AgentLanPairingBrowserDiscoveryScan
            | AgentCommandName::AgentLanPairingAddDeviceRequest
    )
}

fn refresh_mode_for_command(command: &AgentCommandName) -> LanDiscoveryRefreshMode {
    match command {
        AgentCommandName::AgentLanPairingBrowserDiscoveryScan
        | AgentCommandName::AgentLanPairingAddDeviceRequest => {
            LanDiscoveryRefreshMode::ActiveSubnetRefresh
        }
        AgentCommandName::AgentLanPairingStatusGet => LanDiscoveryRefreshMode::Passive,
        _ => LanDiscoveryRefreshMode::Passive,
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, SecondsFormat, Utc};
    use ocentra_lan_core::network_inventory::LanDiscoveryRefreshMode;
    use ocentra_parent_agent_protocol::lan_pairing::{
        LanPairingDeviceReachability, LanPairingNetworkMode, LanPairingProductionDiscoveryState,
        LanPairingTrustState,
    };
    use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
        LanCanonicalHouseholdDevice, LanCanonicalHouseholdDeviceClassification,
        LanCanonicalHouseholdDeviceConfidence, LanCanonicalHouseholdDeviceSource,
        LanCanonicalHouseholdNetworkIdentity, LanCanonicalHouseholdRoleState,
        LanCanonicalHouseholdRouteState, LanCanonicalHouseholdSurface,
        LanChildAgentInventoryPacket, LanHouseholdDeviceActionKind, LanHouseholdDeviceDecision,
    };
    use ocentra_parent_agent_protocol::transport::AgentCommandName;

    use super::{
        command_uses_physical_lan_scan, durable_household_scan_suppression_devices,
        recent_previous_scan_agent_truth_devices, refresh_mode_for_command, scan_history_is_recent,
        scan_truth_context,
    };
    use crate::lan_pairing::LanPairingRuntime;
    use crate::lan_pairing_browser_add_device_state::scan_history::LanScanHistorySnapshot;
    use ocentra_lan_core::network_inventory::LanNetworkInventoryDevice;
    use ocentra_parent_agent_protocol::constants;

    #[test]
    fn status_and_scan_commands_keep_physical_lan_inventory_enabled() {
        assert!(command_uses_physical_lan_scan(
            &AgentCommandName::AgentLanPairingStatusGet
        ));
        assert!(command_uses_physical_lan_scan(
            &AgentCommandName::AgentLanPairingBrowserDiscoveryScan
        ));
        assert!(command_uses_physical_lan_scan(
            &AgentCommandName::AgentLanPairingAddDeviceRequest
        ));
    }

    #[test]
    fn unrelated_lan_commands_do_not_trigger_physical_lan_inventory() {
        assert!(!command_uses_physical_lan_scan(
            &AgentCommandName::AgentLanPairingControllerLeaseRenew
        ));
    }

    #[test]
    fn recent_previous_scan_child_truth_is_reused_for_probe_suppression_only() {
        let now = Utc::now();
        let snapshot = LanScanHistorySnapshot {
            schema_version: 1,
            updated_at: now.to_rfc3339_opts(SecondsFormat::Millis, true),
            metadata: None,
            devices: vec![previous_scan_device(
                constants::lan_pairing::LOCAL_AGENT_STATUS,
            )],
        };

        let devices = recent_previous_scan_agent_truth_devices(Some(&snapshot), now);

        assert_eq!(devices.len(), 1);
        assert_eq!(
            devices[0].agent_status.as_deref(),
            Some(constants::lan_pairing::LOCAL_AGENT_STATUS)
        );
        assert_eq!(
            devices[0].mac_address.as_deref(),
            Some(constants::lan_pairing::TEST_LAN_MAC)
        );
    }

    #[test]
    fn stale_previous_scan_child_truth_does_not_suppress_probe_forever() {
        let now = Utc::now();
        let stale_updated_at = (now
            - Duration::seconds(
                constants::lan_pairing::LAN_PREVIOUS_SCAN_AGENT_TRUTH_REUSE_WINDOW_SECONDS + 1,
            ))
        .to_rfc3339_opts(SecondsFormat::Millis, true);
        let snapshot = LanScanHistorySnapshot {
            schema_version: 1,
            updated_at: stale_updated_at,
            metadata: None,
            devices: vec![previous_scan_device(
                constants::lan_pairing::LOCAL_AGENT_STATUS,
            )],
        };

        assert!(recent_previous_scan_agent_truth_devices(Some(&snapshot), now).is_empty());
    }

    #[test]
    fn invalid_history_timestamp_is_not_treated_as_recent() {
        assert!(!scan_history_is_recent("not-a-timestamp", Utc::now()));
    }

    #[test]
    fn status_reads_stay_passive_while_browser_scan_and_add_device_refresh_actively() {
        assert_eq!(
            refresh_mode_for_command(&AgentCommandName::AgentLanPairingStatusGet),
            LanDiscoveryRefreshMode::Passive
        );
        assert_eq!(
            refresh_mode_for_command(&AgentCommandName::AgentLanPairingBrowserDiscoveryScan),
            LanDiscoveryRefreshMode::ActiveSubnetRefresh
        );
        assert_eq!(
            refresh_mode_for_command(&AgentCommandName::AgentLanPairingAddDeviceRequest),
            LanDiscoveryRefreshMode::ActiveSubnetRefresh
        );
    }

    #[test]
    fn previous_router_truth_becomes_scan_suppression_without_upgrading_identity_truth() {
        let now = Utc::now();
        let snapshot = LanScanHistorySnapshot {
            schema_version: 2,
            updated_at: now.to_rfc3339_opts(SecondsFormat::Millis, true),
            metadata: None,
            devices: vec![router_scan_device()],
        };

        let devices = durable_household_scan_suppression_devices(&[], Some(&snapshot), &[], &[]);

        assert_eq!(devices.len(), 1);
        assert_eq!(
            devices[0].mac_address.as_deref(),
            Some(constants::lan_pairing::TEST_ROUTER_MAC)
        );
        assert_eq!(
            devices[0].ip_address.as_deref(),
            Some(constants::lan_pairing::TEST_ROUTER_IP)
        );
    }

    #[test]
    fn ignored_previous_household_device_becomes_scan_suppression_truth() {
        let now = Utc::now();
        let snapshot = LanScanHistorySnapshot {
            schema_version: 2,
            updated_at: now.to_rfc3339_opts(SecondsFormat::Millis, true),
            metadata: None,
            devices: vec![previous_scan_device(
                constants::lan_pairing::LOCAL_AGENT_STATUS,
            )],
        };
        let decisions = vec![LanHouseholdDeviceDecision {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            action_id: constants::lan_pairing::HOUSEHOLD_ACTION_ID.to_string(),
            action_kind: LanHouseholdDeviceActionKind::Ignore,
            canonical_device_id: canonical_device_id_for_mac(constants::lan_pairing::TEST_LAN_MAC),
            child_profile_id: None,
            display_name: None,
            device_kind: None,
            parent_actor_id: constants::lan_pairing::PARENT_ACTOR_ID.to_string(),
            decided_at: now.to_rfc3339_opts(SecondsFormat::Millis, true),
            revoked_at: None,
        }];

        let devices =
            durable_household_scan_suppression_devices(&[], Some(&snapshot), &[], &decisions);

        assert_eq!(devices.len(), 1);
        assert_eq!(
            devices[0].mac_address.as_deref(),
            Some(constants::lan_pairing::TEST_LAN_MAC)
        );
    }

    #[test]
    fn stored_known_router_truth_suppresses_scan_work_without_scan_history() {
        let devices =
            durable_household_scan_suppression_devices(&[stored_known_router()], None, &[], &[]);

        assert_eq!(devices.len(), 1);
        assert_eq!(
            devices[0].mac_address.as_deref(),
            Some(constants::lan_pairing::TEST_ROUTER_MAC)
        );
    }

    #[test]
    fn stored_known_child_agent_truth_feeds_identity_and_scan_context_without_scan_history() {
        let runtime = LanPairingRuntime::empty();
        {
            let mut registry = runtime
                .registry
                .lock()
                .unwrap_or_else(|_| unreachable!("registry lock available for test"));
            let changed = registry.merge_known_household_devices(vec![stored_known_child_agent()]);
            assert!(changed);
        }

        let context = scan_truth_context(&runtime, None, Utc::now());

        assert_eq!(context.identity_hint_devices.len(), 1);
        assert_eq!(context.durable_household_truth_count, 1);
        assert_eq!(context.scan_suppression_devices.len(), 1);
        assert_eq!(
            context.identity_hint_devices[0].mac_address.as_deref(),
            Some(constants::lan_pairing::TEST_LAN_MAC)
        );
        assert_eq!(
            context.scan_suppression_devices[0].mac_address.as_deref(),
            Some(constants::lan_pairing::TEST_LAN_MAC)
        );
        assert_eq!(
            context.scan_suppression_devices[0].ip_address.as_deref(),
            Some(constants::lan_pairing::TEST_LAN_IP)
        );
    }

    fn previous_scan_device(agent_status: &str) -> LanNetworkInventoryDevice {
        LanNetworkInventoryDevice {
            device_id: "lan-device-1".to_string(),
            label: "GameDev".to_string(),
            platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
            ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
            mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
            hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
            network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
            reachability: LanPairingDeviceReachability::Online,
            agent_status: Some(agent_status.to_string()),
            scan_sources: vec![
                constants::lan_pairing::LAN_SCAN_SOURCE_SERVICE_IDENTITY_PROBE.to_string(),
            ],
            used_previous_scan_hint: false,
        }
    }

    fn router_scan_device() -> LanNetworkInventoryDevice {
        LanNetworkInventoryDevice {
            device_id: "lan-router-1".to_string(),
            label: "Home Router".to_string(),
            platform: constants::lan_pairing::PLATFORM_ROUTER.to_string(),
            ip_address: constants::lan_pairing::TEST_ROUTER_IP.to_string(),
            mac_address: constants::lan_pairing::TEST_ROUTER_MAC.to_string(),
            hostname: Some("home-router".to_string()),
            network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
            reachability: LanPairingDeviceReachability::Online,
            agent_status: None,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
            used_previous_scan_hint: false,
        }
    }

    fn canonical_device_id_for_mac(mac_address: &str) -> String {
        let mut canonical_device_id =
            String::from(constants::lan_pairing::CANONICAL_DEVICE_MAC_PREFIX);
        canonical_device_id.push_str(
            &mac_address
                .chars()
                .filter(|character| character.is_ascii_alphanumeric())
                .flat_map(char::to_lowercase)
                .collect::<String>(),
        );
        canonical_device_id
    }

    fn stored_known_router() -> LanCanonicalHouseholdDevice {
        LanCanonicalHouseholdDevice {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            canonical_device_id: canonical_device_id_for_mac(
                constants::lan_pairing::TEST_ROUTER_MAC,
            ),
            display_name: "Home Router".to_string(),
            classification: LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure,
            role_badges: Vec::new(),
            enrollable: false,
            discovery_state: LanPairingProductionDiscoveryState::Discovered,
            trust_state: LanPairingTrustState::Unpaired,
            route_id: None,
            route_state: LanCanonicalHouseholdRouteState::Unavailable,
            network_mode: LanPairingNetworkMode::LocalNetwork,
            source_labels: vec![LanCanonicalHouseholdDeviceSource::NetworkNeighbor],
            network_identity: LanCanonicalHouseholdNetworkIdentity {
                hostname: Some("home-router".to_string()),
                ip_addresses: vec![constants::lan_pairing::TEST_ROUTER_IP.to_string()],
                mac_address: Some(constants::lan_pairing::TEST_ROUTER_MAC.to_string()),
                mac_vendor: Some("Example Vendor".to_string()),
                network_interfaces: vec![constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()],
                reachability: LanPairingDeviceReachability::Online,
                confidence: LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor,
                stale_at: None,
                offline_at: None,
                evidence_records: Vec::new(),
            },
            child_agent_inventory: None,
            policy_target_surfaces: vec![
                LanCanonicalHouseholdSurface::Devices,
                LanCanonicalHouseholdSurface::Network,
            ],
        }
    }

    fn stored_known_child_agent() -> LanCanonicalHouseholdDevice {
        LanCanonicalHouseholdDevice {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            canonical_device_id: canonical_device_id_for_mac(constants::lan_pairing::TEST_LAN_MAC),
            display_name: "GameDev".to_string(),
            classification: LanCanonicalHouseholdDeviceClassification::ChildAgent,
            role_badges: Vec::new(),
            enrollable: true,
            discovery_state: LanPairingProductionDiscoveryState::Discovered,
            trust_state: LanPairingTrustState::Unpaired,
            route_id: Some(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string()),
            route_state: LanCanonicalHouseholdRouteState::LocalNetwork,
            network_mode: LanPairingNetworkMode::LocalNetwork,
            source_labels: vec![LanCanonicalHouseholdDeviceSource::LocalService],
            network_identity: LanCanonicalHouseholdNetworkIdentity {
                hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
                ip_addresses: vec![constants::lan_pairing::TEST_LAN_IP.to_string()],
                mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
                mac_vendor: Some("Example Vendor".to_string()),
                network_interfaces: vec![constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()],
                reachability: LanPairingDeviceReachability::Online,
                confidence: LanCanonicalHouseholdDeviceConfidence::AgentConfirmed,
                stale_at: None,
                offline_at: None,
                evidence_records: Vec::new(),
            },
            child_agent_inventory: Some(LanChildAgentInventoryPacket {
                device_name: "GameDev".to_string(),
                platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
                os: "Windows".to_string(),
                cpu_model: None,
                cpu_cores: None,
                memory_total: None,
                gpu_model: None,
                gpu_driver: None,
                gpu_memory: None,
                nvidia_smi: None,
                network_interfaces: vec![constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()],
                capabilities: vec!["screen".to_string()],
                role_state: LanCanonicalHouseholdRoleState::Implemented,
                route_state: LanCanonicalHouseholdRouteState::LocalNetwork,
                pairing_trust_state: LanPairingTrustState::Unpaired,
            }),
            policy_target_surfaces: vec![
                LanCanonicalHouseholdSurface::Devices,
                LanCanonicalHouseholdSurface::Screen,
            ],
        }
    }
}
