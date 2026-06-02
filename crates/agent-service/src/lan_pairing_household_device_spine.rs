mod builders;
mod merge;
mod values;
#[cfg(test)]
#[path = "lan_pairing_household_device_spine_tests.rs"]
mod tests;

use builders::{device_from_discovery, device_from_registry};
use merge::merge_device;
use ocentra_parent_agent_protocol::{
    LanBrowserAddDeviceDiscoveryDevice, LanCanonicalHouseholdDevice, LanPairingDeviceRef,
    LanTrustedDeviceRegistryEntry,
};
use values::option_overlaps;

pub(crate) fn canonical_household_devices(
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
    trusted_registry: &[LanTrustedDeviceRegistryEntry],
) -> Vec<LanCanonicalHouseholdDevice> {
    let mut devices: Vec<LanCanonicalHouseholdDevice> = Vec::new();

    for discovered in discovered_devices {
        upsert_device(
            &mut devices,
            device_from_discovery(discovered),
            &discovered.child_device,
        );
    }

    for entry in trusted_registry {
        upsert_device(&mut devices, device_from_registry(entry), &entry.child_device);
    }

    devices
}

fn upsert_device(
    devices: &mut Vec<LanCanonicalHouseholdDevice>,
    device: LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
) {
    if let Some(existing) = devices
        .iter_mut()
        .find(|candidate| devices_match(candidate, source_ref, &device))
    {
        merge_device(existing, device);
        return;
    }

    devices.push(device);
}

fn devices_match(
    existing: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    device: &LanCanonicalHouseholdDevice,
) -> bool {
    existing.canonical_device_id == device.canonical_device_id
        || option_overlaps(
            existing.network_identity.mac_address.as_ref(),
            source_ref.mac_address.as_ref(),
        )
        || source_ref
            .ip_address
            .as_ref()
            .map(|ip| existing.network_identity.ip_addresses.contains(ip))
            .unwrap_or(false)
}
