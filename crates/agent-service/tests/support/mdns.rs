use std::fmt::Display;

use ocentra_lan_core::network_inventory::LanNetworkInventoryDevice;
use ocentra_lan_core::read_model_builder;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanChildMdnsAdvertisement, LanChildMdnsAdvertisementInput, LanMdnsAdvertisementLifecycleState,
    LanMdnsAdvertisementSupportState, LanParentMdnsAdvertisement,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanHouseholdDeviceDecision,
};

use crate::lan_pairing::LanPairingRuntime;
use crate::lan_runtime_test_support::LanChildMdnsAdvertisementFixture;

pub(crate) fn default_child_mdns_advertisement_fixture(
    lifecycle_state: LanMdnsAdvertisementLifecycleState,
    support_state: LanMdnsAdvertisementSupportState,
) -> LanChildMdnsAdvertisementFixture {
    LanChildMdnsAdvertisementFixture {
        advertisement_id: "sha256:child-family-1".to_string(),
        opaque_device_id: "sha256:child-device-1".to_string(),
        protocol_version: constants::lan_pairing::SCHEMA_VERSION_TEXT.to_string(),
        family_hash: "sha256:family-1".to_string(),
        platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
        agent_version: "1.2.3".to_string(),
        lifecycle_state,
        support_state,
    }
}

pub(crate) fn parent_mdns_advertisement(
    runtime: &LanPairingRuntime,
    advertisement_id: impl Display,
    protocol_version: impl Display,
    family_hash: impl Display,
    lifecycle_state: LanMdnsAdvertisementLifecycleState,
    support_state: LanMdnsAdvertisementSupportState,
) -> Result<LanParentMdnsAdvertisement, ocentra_eventing::error::EventingError> {
    LanParentMdnsAdvertisement::new(
        advertisement_id.to_string(),
        protocol_version.to_string(),
        family_hash.to_string(),
        runtime.mdns_pairing_state(),
        lifecycle_state,
        support_state,
    )
}

pub(crate) fn child_mdns_advertisement(
    runtime: &LanPairingRuntime,
    fixture: LanChildMdnsAdvertisementFixture,
) -> Result<LanChildMdnsAdvertisement, ocentra_eventing::error::EventingError> {
    LanChildMdnsAdvertisement::new(LanChildMdnsAdvertisementInput {
        advertisement_id: fixture.advertisement_id,
        opaque_device_id: fixture.opaque_device_id,
        protocol_version: fixture.protocol_version,
        family_hash: fixture.family_hash,
        platform: fixture.platform,
        agent_version: fixture.agent_version,
        pairing_state: runtime.mdns_pairing_state(),
        lifecycle_state: fixture.lifecycle_state,
        support_state: fixture.support_state,
    })
}

pub(crate) fn load_scan_history_for_test(
    runtime: &LanPairingRuntime,
) -> Vec<LanNetworkInventoryDevice> {
    crate::lan_pairing_browser_add_device_state::scan_history::load_scan_history_snapshot(runtime)
        .map(|snapshot| snapshot.devices)
        .unwrap_or_default()
}

pub(crate) fn canonical_household_devices_for_test(
    discovered_devices: &[ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceDiscoveryDevice],
    trusted_registry: &[ocentra_parent_agent_protocol::lan_pairing::LanTrustedDeviceRegistryEntry],
    household_device_decisions: &[LanHouseholdDeviceDecision],
) -> Vec<LanCanonicalHouseholdDevice> {
    read_model_builder::canonical_household_devices(
        discovered_devices,
        trusted_registry,
        household_device_decisions,
        &crate::time::timestamp_now::<String>(),
    )
}
