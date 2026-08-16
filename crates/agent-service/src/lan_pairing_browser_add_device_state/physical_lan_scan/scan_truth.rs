use chrono::{DateTime, Utc};
use ocentra_lan_core::read_model::discovered_devices_from_network_inventory;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceRef, LanTrustedDeviceRegistryEntry,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanHouseholdDeviceDecision,
};

use crate::lan_pairing::LanPairingRuntime;

use super::super::registry_projection::{
    household_device_decisions, known_household_devices, trusted_device_registry,
};
use super::super::scan_history::{
    recent_previous_scan_agent_truth_devices, LanScanHistorySnapshot,
};
use super::suppression_device::{
    household_device_should_suppress_redundant_scan_work, household_scan_suppression_device,
    push_unique_scan_truth_device,
};
use super::LanScanTruthContext;

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
