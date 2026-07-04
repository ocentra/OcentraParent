use std::fmt::Display;
use std::net::UdpSocket;

use ocentra_lan_core::network_inventory::passive_discovery::{
    LanPassiveDiscoveryEventHistory, LanPassiveDiscoveryListenerState,
};
use ocentra_lan_core::network_inventory::LanNetworkInventoryDevice;
use ocentra_lan_core::read_model_builder;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    DeviceRoleRuntimeReadModel, LanChildMdnsAdvertisement, LanChildMdnsAdvertisementInput,
    LanMdnsAdvertisementLifecycleState, LanMdnsAdvertisementSupportState,
    LanPairingDeviceReachability, LanParentMdnsAdvertisement,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanHouseholdDeviceDecision,
};

use crate::lan_pairing::LanPairingRuntime;
use crate::lan_pairing_runtime_state::job_leases::LanAiJobLeaseState;
use crate::test_text::TestText;

#[path = "lan_runtime_test_support/discovery.rs"]
mod discovery;
#[path = "lan_runtime_test_support/lan_ai.rs"]
mod lan_ai;
#[path = "lan_runtime_test_support/mdns.rs"]
mod mdns;
#[path = "lan_runtime_test_support/registry.rs"]
mod registry;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TestLeaseState {
    Claimed,
    Completed,
    DuplicateRejected,
    ExpiredRequeued,
    DeadLettered,
}

pub(crate) struct LanChildMdnsAdvertisementFixture {
    pub(crate) advertisement_id: String,
    pub(crate) opaque_device_id: String,
    pub(crate) protocol_version: String,
    pub(crate) family_hash: String,
    pub(crate) platform: String,
    pub(crate) agent_version: String,
    pub(crate) lifecycle_state: LanMdnsAdvertisementLifecycleState,
    pub(crate) support_state: LanMdnsAdvertisementSupportState,
}

pub(crate) fn default_child_mdns_advertisement_fixture(
    lifecycle_state: LanMdnsAdvertisementLifecycleState,
    support_state: LanMdnsAdvertisementSupportState,
) -> LanChildMdnsAdvertisementFixture {
    mdns::default_child_mdns_advertisement_fixture(lifecycle_state, support_state)
}

impl LanPairingRuntime {
    pub(crate) fn persistent_json(path: impl AsRef<std::path::Path>) -> Self {
        registry::persistent_json(path)
    }

    pub(crate) fn empty_with_local_child_device_id(
        local_child_device_id: Option<TestText>,
    ) -> Self {
        registry::empty_with_local_child_device_id(local_child_device_id)
    }

    pub(crate) fn empty_with_signed_child_agent_context(
        local_child_device_id: Option<TestText>,
        parent_device_id: impl Display,
        family_hash: impl Display,
        route_id: impl Display,
    ) -> Self {
        registry::empty_with_signed_child_agent_context(
            local_child_device_id,
            parent_device_id,
            family_hash,
            route_id,
        )
    }

    pub(crate) fn empty_with_device_role_read_model(
        device_roles: DeviceRoleRuntimeReadModel,
    ) -> Self {
        registry::empty_with_device_role_read_model(device_roles)
    }

    pub(crate) fn passive_discovery_history_snapshot(&self) -> LanPassiveDiscoveryEventHistory {
        discovery::passive_discovery_history_snapshot(self)
    }

    pub(crate) fn mark_selected_offline_for_test(&self) -> bool {
        lan_ai::mark_selected_offline_for_test(self)
    }

    pub(crate) fn mark_selected_stale_for_test(&self) -> bool {
        lan_ai::mark_selected_stale_for_test(self)
    }

    pub(crate) fn mark_lan_ai_provider_heartbeat_stale_for_test(&self) {
        lan_ai::mark_lan_ai_provider_heartbeat_stale_for_test(self)
    }

    pub(crate) fn mark_lan_ai_provider_heartbeat_offline_for_test(&self) {
        lan_ai::mark_lan_ai_provider_heartbeat_offline_for_test(self)
    }

    pub(crate) fn seed_lan_ai_job_lease_for_test(
        &self,
        job_id: impl Display,
        lease_state: impl Display,
        attempt_count: u64,
        expires_at: impl Display,
    ) {
        lan_ai::seed_lan_ai_job_lease_for_test(self, job_id, lease_state, attempt_count, expires_at)
    }

    pub(crate) fn parent_mdns_advertisement(
        &self,
        advertisement_id: impl Display,
        protocol_version: impl Display,
        family_hash: impl Display,
        lifecycle_state: LanMdnsAdvertisementLifecycleState,
        support_state: LanMdnsAdvertisementSupportState,
    ) -> Result<LanParentMdnsAdvertisement, ocentra_eventing::error::EventingError> {
        mdns::parent_mdns_advertisement(
            self,
            advertisement_id,
            protocol_version,
            family_hash,
            lifecycle_state,
            support_state,
        )
    }

    pub(crate) fn child_mdns_advertisement(
        &self,
        fixture: LanChildMdnsAdvertisementFixture,
    ) -> Result<LanChildMdnsAdvertisement, ocentra_eventing::error::EventingError> {
        mdns::child_mdns_advertisement(self, fixture)
    }

    pub(crate) fn record_allowed_snmp_probe_responses(
        &self,
        socket: &UdpSocket,
        max_datagram_count: usize,
    ) -> usize {
        discovery::record_allowed_snmp_probe_responses(self, socket, max_datagram_count)
    }
}

pub(crate) fn load_scan_history_for_test(
    runtime: &LanPairingRuntime,
) -> Vec<LanNetworkInventoryDevice> {
    mdns::load_scan_history_for_test(runtime)
}

pub(crate) fn canonical_household_devices_for_test(
    discovered_devices: &[ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceDiscoveryDevice],
    trusted_registry: &[ocentra_parent_agent_protocol::lan_pairing::LanTrustedDeviceRegistryEntry],
    household_device_decisions: &[LanHouseholdDeviceDecision],
) -> Vec<LanCanonicalHouseholdDevice> {
    mdns::canonical_household_devices_for_test(
        discovered_devices,
        trusted_registry,
        household_device_decisions,
    )
}
