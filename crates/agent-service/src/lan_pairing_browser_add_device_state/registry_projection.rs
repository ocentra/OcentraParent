use ocentra_parent_agent_protocol::{
    constants,
    lan_pairing::LanPairingText,
    lan_pairing::LanTrustedDeviceRegistryEntry,
    lan_pairing_browser_add_device_state::{
        LanBrowserAddDevicePairingRequest, LanCanonicalHouseholdDevice, LanHouseholdDeviceDecision,
    },
};

use crate::lan_pairing::LanPairingRuntime;

use super::discovery_projection::pairing_request_state;

pub(crate) fn trusted_device_registry(
    runtime: &LanPairingRuntime,
) -> Vec<LanTrustedDeviceRegistryEntry> {
    runtime
        .registry
        .lock()
        .map(|registry| registry.entries().to_vec())
        .unwrap_or_default()
}

pub(crate) fn household_device_decisions(
    runtime: &LanPairingRuntime,
) -> Vec<LanHouseholdDeviceDecision> {
    runtime
        .registry
        .lock()
        .map(|registry| registry.household_device_decisions().to_vec())
        .unwrap_or_default()
}

pub(crate) fn known_household_devices(
    runtime: &LanPairingRuntime,
) -> Vec<LanCanonicalHouseholdDevice> {
    runtime
        .registry
        .lock()
        .map(|registry| registry.known_household_devices().to_vec())
        .unwrap_or_default()
}

pub(crate) fn persist_known_household_devices(
    runtime: &LanPairingRuntime,
    devices: &[LanCanonicalHouseholdDevice],
) {
    let Ok(mut registry) = runtime.registry.lock() else {
        return;
    };
    if registry.merge_known_household_devices(devices.to_vec()) {
        let _ = runtime.persist_registry(&registry);
    }
}

pub(crate) fn merged_known_household_devices_for_read_model(
    runtime: &LanPairingRuntime,
    current_devices: &[LanCanonicalHouseholdDevice],
    observed_at: &LanPairingText,
) -> Vec<LanCanonicalHouseholdDevice> {
    runtime
        .registry
        .lock()
        .map(|registry| {
            registry.known_household_devices_for_read_model(current_devices, observed_at.0.as_str())
        })
        .unwrap_or_else(|_| current_devices.to_vec())
}

pub(crate) fn pairing_requests(
    runtime: &LanPairingRuntime,
    generated_at: &LanPairingText,
) -> Vec<LanBrowserAddDevicePairingRequest> {
    runtime
        .challenges
        .lock()
        .map(|challenges| {
            challenges
                .iter()
                .map(|challenge| LanBrowserAddDevicePairingRequest {
                    schema_version: constants::lan_pairing::SCHEMA_VERSION,
                    challenge_id: challenge.challenge_id.clone(),
                    child_device_id: challenge.child_device_id.clone(),
                    parent_device_id: challenge.parent_device_id.clone(),
                    route_id: challenge.route_id.clone(),
                    origin: challenge.origin.clone(),
                    pairing_state: pairing_request_state(
                        challenge.accepted,
                        LanPairingText(generated_at.0.clone()),
                        LanPairingText(challenge.expires_at.clone()),
                    ),
                    rejection_reason: None,
                    issued_at: challenge.issued_at.clone(),
                    expires_at: challenge.expires_at.clone(),
                })
                .collect()
        })
        .unwrap_or_default()
}
