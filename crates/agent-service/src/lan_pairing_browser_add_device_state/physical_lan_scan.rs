use chrono::{DateTime, Utc};
use ocentra_lan_core::network_inventory::{
    discover_lan_network_devices_with_hints_refresh_mode_and_scan_and_probe_suppression_and_allowed_snmp_observer,
    plan_lan_discovery_scan_with_active_refresh_suppression,
    targeted_arp_refresh_evidence_for_scan, LanDiscoveryRefreshMode, LanNetworkInventoryDevice,
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
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentRoute,
};

use crate::lan_pairing::LanPairingRuntime;

use super::scan_history::{
    load_scan_history_snapshot, recent_previous_scan_agent_truth_devices, save_scan_history,
    scan_history_is_recent, LanScanHistoryMetadata, LanScanHistorySnapshot,
};
use super::{household_device_decisions, known_household_devices, trusted_device_registry};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct LanNetworkDeviceScanResult {
    pub(crate) devices: Vec<LanNetworkInventoryDevice>,
    pub(crate) previous_scan_snapshot: Option<LanScanHistorySnapshot>,
    pub(crate) current_scan_snapshot: Option<LanScanHistorySnapshot>,
    pub(crate) reused_recent_snapshot: bool,
}

pub(crate) fn network_device_scan_result_for_command(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
) -> LanNetworkDeviceScanResult {
    let now = Utc::now();
    let previous_scan_snapshot = load_scan_history_snapshot(runtime);
    if let Some(scan_result) =
        cached_localhost_status_scan_result(command, previous_scan_snapshot.clone(), now)
    {
        return scan_result;
    }

    if command_uses_physical_lan_scan(&command.command) {
        if let Some(previous_devices) =
            cached_status_snapshot_devices(command, previous_scan_snapshot.as_ref(), now)
        {
            return LanNetworkDeviceScanResult {
                devices: previous_devices,
                current_scan_snapshot: previous_scan_snapshot.clone(),
                previous_scan_snapshot,
                reused_recent_snapshot: true,
            };
        }
        let previous_devices = previous_scan_snapshot
            .as_ref()
            .map(|snapshot| snapshot.devices.as_slice())
            .unwrap_or_default();
        let scan_truth = scan_truth_context(runtime, previous_scan_snapshot.as_ref(), now);
        let refresh_mode = refresh_mode_for_command(&command.command);
        let mut scan_plan = plan_lan_discovery_scan_with_active_refresh_suppression(
            &scan_truth.identity_hint_devices,
            previous_devices,
            refresh_mode,
            &scan_truth.scan_suppression_devices,
        );
        scan_plan.targeted_arp_refresh_evidence = targeted_arp_refresh_evidence_for_scan(
            previous_devices,
            refresh_mode,
            &scan_truth.scan_suppression_devices,
        );
        let inventory_refresh_mode = inventory_refresh_mode_after_targeted_refresh(refresh_mode);
        let selected_interface_scope = scan_plan.selected_interface.as_deref();
        let allowed_snmp_response_observer = |payload: &[u8]| {
            let _ = runtime.record_allowed_snmp_probe_response_packet(payload);
        };
        let devices = discover_lan_network_devices_with_hints_refresh_mode_and_scan_and_probe_suppression_and_allowed_snmp_observer(
            &scan_truth.identity_hint_devices,
            previous_devices,
            inventory_refresh_mode,
            &scan_truth.scan_suppression_devices,
            &scan_truth.scan_suppression_devices,
            selected_interface_scope,
            Some(&allowed_snmp_response_observer),
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
        let current_scan_snapshot = load_scan_history_snapshot(runtime);
        return LanNetworkDeviceScanResult {
            devices: current_scan_snapshot
                .as_ref()
                .map(|snapshot| snapshot.devices.clone())
                .unwrap_or(devices),
            previous_scan_snapshot,
            current_scan_snapshot,
            reused_recent_snapshot: false,
        };
    }
    LanNetworkDeviceScanResult::default()
}

pub(super) fn refresh_network_device_scan_history(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
) -> LanNetworkDeviceScanResult {
    network_device_scan_result_for_command(runtime, command)
}

pub(crate) fn cached_localhost_status_scan_result(
    command: &AgentCommandEnvelope,
    previous_scan_snapshot: Option<LanScanHistorySnapshot>,
    now: DateTime<Utc>,
) -> Option<LanNetworkDeviceScanResult> {
    if command.command != AgentCommandName::AgentLanPairingStatusGet
        || command.target.route != AgentRoute::Localhost
    {
        return None;
    }

    let Some(snapshot) = previous_scan_snapshot else {
        return Some(LanNetworkDeviceScanResult::default());
    };
    if !scan_history_is_recent(&snapshot.updated_at, now) {
        return Some(LanNetworkDeviceScanResult {
            previous_scan_snapshot: Some(snapshot),
            ..LanNetworkDeviceScanResult::default()
        });
    }

    Some(LanNetworkDeviceScanResult {
        devices: snapshot.devices.clone(),
        previous_scan_snapshot: Some(snapshot.clone()),
        current_scan_snapshot: Some(snapshot),
        reused_recent_snapshot: true,
    })
}

pub(crate) fn cached_status_snapshot_devices(
    command: &AgentCommandEnvelope,
    previous_scan_snapshot: Option<&LanScanHistorySnapshot>,
    now: DateTime<Utc>,
) -> Option<Vec<LanNetworkInventoryDevice>> {
    if command.command != AgentCommandName::AgentLanPairingStatusGet
        || command.target.route != AgentRoute::Localhost
    {
        return None;
    }
    let previous_scan_snapshot = previous_scan_snapshot?;
    scan_history_is_recent(&previous_scan_snapshot.updated_at, now)
        .then(|| previous_scan_snapshot.devices.clone())
}

pub(crate) struct LanScanTruthContext {
    pub(crate) identity_hint_devices: Vec<LanPairingDeviceRef>,
    pub(crate) scan_suppression_devices: Vec<LanPairingDeviceRef>,
    pub(crate) paired_registry_truth_count: u32,
    pub(crate) recent_previous_agent_truth_count: u32,
    pub(crate) durable_household_truth_count: u32,
}

pub(crate) fn scan_truth_context(
    runtime: &LanPairingRuntime,
    previous_scan_snapshot: Option<&LanScanHistorySnapshot>,
    now: DateTime<Utc>,
) -> LanScanTruthContext {
    let trusted_registry = trusted_device_registry(runtime);
    let stored_known_household_devices = known_household_devices(runtime);
    let household_decisions = household_device_decisions(runtime);
    let mut identity_hint_devices = trusted_scan_truth_devices(runtime);
    let paired_registry_truth_count =
        u32::try_from(trusted_registry_count(runtime)).unwrap_or(u32::MAX);
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

fn trusted_registry_count(runtime: &LanPairingRuntime) -> usize {
    runtime
        .registry
        .lock()
        .map(|registry| registry.trusted_device_count())
        .unwrap_or_default()
}

fn trusted_scan_truth_devices(runtime: &LanPairingRuntime) -> Vec<LanPairingDeviceRef> {
    runtime
        .registry
        .lock()
        .map(|registry| registry.scan_truth_devices())
        .unwrap_or_default()
}

pub(crate) fn durable_household_scan_suppression_devices(
    stored_known_household_devices: &[LanCanonicalHouseholdDevice],
    previous_scan_snapshot: Option<&LanScanHistorySnapshot>,
    trusted_registry: &[LanTrustedDeviceRegistryEntry],
    household_device_decisions: &[LanHouseholdDeviceDecision],
) -> Vec<LanPairingDeviceRef> {
    let mut devices = stored_known_household_devices
        .iter()
        .filter(|device| household_device_should_suppress_redundant_scan_work(device))
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
        .iter()
        .filter(|device| household_device_should_suppress_redundant_scan_work(device))
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
    device: &LanCanonicalHouseholdDevice,
) -> Option<LanPairingDeviceRef> {
    let platform = scan_suppression_platform(device);
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

pub(crate) fn command_uses_physical_lan_scan(command: &AgentCommandName) -> bool {
    matches!(
        command,
        AgentCommandName::AgentLanPairingStatusGet
            | AgentCommandName::AgentLanPairingBrowserDiscoveryScan
            | AgentCommandName::AgentLanPairingAddDeviceRequest
    )
}

pub(crate) fn refresh_mode_for_command(command: &AgentCommandName) -> LanDiscoveryRefreshMode {
    match command {
        AgentCommandName::AgentLanPairingBrowserDiscoveryScan
        | AgentCommandName::AgentLanPairingAddDeviceRequest => {
            LanDiscoveryRefreshMode::ActiveSubnetRefresh
        }
        AgentCommandName::AgentLanPairingStatusGet => LanDiscoveryRefreshMode::Passive,
        _ => LanDiscoveryRefreshMode::Passive,
    }
}

pub(crate) fn inventory_refresh_mode_after_targeted_refresh(
    refresh_mode: LanDiscoveryRefreshMode,
) -> LanDiscoveryRefreshMode {
    if refresh_mode == LanDiscoveryRefreshMode::ActiveSubnetRefresh {
        LanDiscoveryRefreshMode::Passive
    } else {
        refresh_mode
    }
}
