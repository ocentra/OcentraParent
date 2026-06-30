use std::{
    net::UdpSocket,
    path::Path,
    sync::{Arc, Mutex},
};

use ocentra_lan_core::network_inventory::passive_discovery::{
    collect_allowed_snmp_response_packets, LanPassiveDiscoveryEventHistory,
    LanPassiveDiscoveryListenerState,
};
use ocentra_lan_core::network_inventory::LanNetworkInventoryDevice;
use ocentra_lan_core::read_model_builder;
use ocentra_parent_agent_core::trusted_device_registry::TrustedDeviceRegistry;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    DeviceRoleRuntimeReadModel, LanChildMdnsAdvertisement, LanChildMdnsAdvertisementInput,
    LanMdnsAdvertisementLifecycleState, LanMdnsAdvertisementSupportState,
    LanPairingDeviceReachability, LanParentMdnsAdvertisement,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanHouseholdDeviceDecision,
};

use crate::{
    lan_pairing::{LanPairingRegistryPersistence, LanPairingRuntime},
    lan_pairing_runtime_state::{
        job_leases::LanAiJobLeaseState, provider_heartbeat::LanAiProviderHeartbeatState,
    },
};

pub(crate) struct LanChildMdnsAdvertisementFixture<'a> {
    pub(crate) advertisement_id: &'a str,
    pub(crate) opaque_device_id: &'a str,
    pub(crate) protocol_version: &'a str,
    pub(crate) family_hash: &'a str,
    pub(crate) platform: &'a str,
    pub(crate) agent_version: &'a str,
    pub(crate) lifecycle_state: LanMdnsAdvertisementLifecycleState,
    pub(crate) support_state: LanMdnsAdvertisementSupportState,
}

pub(crate) fn default_child_mdns_advertisement_fixture(
    lifecycle_state: LanMdnsAdvertisementLifecycleState,
    support_state: LanMdnsAdvertisementSupportState,
) -> LanChildMdnsAdvertisementFixture<'static> {
    LanChildMdnsAdvertisementFixture {
        advertisement_id: "sha256:child-family-1",
        opaque_device_id: "sha256:child-device-1",
        protocol_version: constants::lan_pairing::SCHEMA_VERSION_TEXT,
        family_hash: "sha256:family-1",
        platform: constants::lan_pairing::PLATFORM_WINDOWS,
        agent_version: "1.2.3",
        lifecycle_state,
        support_state,
    }
}

impl LanPairingRuntime {
    pub(crate) fn persistent_json(path: &Path) -> Self {
        let mut runtime = Self::empty();
        runtime.registry = Arc::new(Mutex::new(TrustedDeviceRegistry::load_json(path)));
        runtime.persistence = LanPairingRegistryPersistence::LocalJsonRegistry(path.to_path_buf());
        runtime
    }

    pub(crate) fn empty_with_local_child_device_id(local_child_device_id: Option<String>) -> Self {
        let mut runtime = Self::empty();
        runtime.local_child_device_id = local_child_device_id;
        runtime
    }

    pub(crate) fn empty_with_signed_child_agent_context(
        local_child_device_id: Option<String>,
        parent_device_id: String,
        family_hash: String,
        route_id: String,
    ) -> Self {
        let mut runtime = Self::empty_with_local_child_device_id(local_child_device_id);
        runtime.signed_child_agent_parent_device_id = Some(parent_device_id);
        runtime.signed_child_agent_family_hash = Some(family_hash);
        runtime.signed_child_agent_route_id = route_id;
        runtime
    }

    pub(crate) fn empty_with_device_role_read_model(
        device_roles: DeviceRoleRuntimeReadModel,
    ) -> Self {
        let mut runtime = Self::empty();
        runtime.device_roles = device_roles;
        runtime.lan_ai_provider_capabilities = vec![
            constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION.to_string(),
            constants::local_ai_runtime::CAPABILITY_SUMMARIZATION.to_string(),
        ];
        runtime
    }

    pub(crate) fn passive_discovery_history_snapshot(&self) -> LanPassiveDiscoveryEventHistory {
        self.passive_discovery_listener_state
            .lock()
            .map(|state| state.snapshot())
            .unwrap_or_else(|_| LanPassiveDiscoveryListenerState::running(String::new()).snapshot())
    }

    pub(crate) fn mark_selected_offline_for_test(&self, offline_at: &str) -> bool {
        self.registry
            .lock()
            .map(|mut registry| registry.mark_selected_offline(offline_at))
            .unwrap_or(false)
    }

    pub(crate) fn mark_selected_stale_for_test(&self, stale_at: &str) -> bool {
        self.registry
            .lock()
            .map(|mut registry| registry.mark_selected_stale(stale_at))
            .unwrap_or(false)
    }

    pub(crate) fn mark_lan_ai_provider_heartbeat_stale_for_test(&self, observed_at: &str) {
        self.record_lan_ai_provider_heartbeat_state_for_test(
            observed_at,
            LanPairingDeviceReachability::Stale,
        );
    }

    pub(crate) fn mark_lan_ai_provider_heartbeat_offline_for_test(&self, observed_at: &str) {
        self.record_lan_ai_provider_heartbeat_state_for_test(
            observed_at,
            LanPairingDeviceReachability::Offline,
        );
    }

    pub(crate) fn seed_lan_ai_job_lease_for_test(
        &self,
        job_id: &str,
        lease_state: &'static str,
        attempt_count: u64,
        expires_at: &str,
    ) {
        if let Ok(mut leases) = self.lan_ai_job_leases.lock() {
            leases.retain(|lease| lease.job_id != job_id);
            leases.push(LanAiJobLeaseState {
                job_id: job_id.to_string(),
                claim_id: lan_ai_claim_id(job_id),
                lease_id: lan_ai_lease_id(job_id),
                lease_state,
                attempt_count,
                expires_at: expires_at.to_string(),
                dead_letter_reason: None,
            });
        }
    }

    fn record_lan_ai_provider_heartbeat_state_for_test(
        &self,
        observed_at: &str,
        reachability: LanPairingDeviceReachability,
    ) {
        if let Ok(mut state) = self.lan_ai_provider_heartbeat.lock() {
            *state = Some(LanAiProviderHeartbeatState {
                observed_at: observed_at.to_string(),
                reachability,
            });
        }
    }

    pub(crate) fn parent_mdns_advertisement(
        &self,
        advertisement_id: &str,
        protocol_version: &str,
        family_hash: &str,
        lifecycle_state: LanMdnsAdvertisementLifecycleState,
        support_state: LanMdnsAdvertisementSupportState,
    ) -> Result<LanParentMdnsAdvertisement, ocentra_eventing::error::EventingError> {
        LanParentMdnsAdvertisement::new(
            advertisement_id.to_string(),
            protocol_version.to_string(),
            family_hash.to_string(),
            self.mdns_pairing_state(),
            lifecycle_state,
            support_state,
        )
    }

    pub(crate) fn child_mdns_advertisement(
        &self,
        fixture: LanChildMdnsAdvertisementFixture<'_>,
    ) -> Result<LanChildMdnsAdvertisement, ocentra_eventing::error::EventingError> {
        LanChildMdnsAdvertisement::new(LanChildMdnsAdvertisementInput {
            advertisement_id: fixture.advertisement_id.to_string(),
            opaque_device_id: fixture.opaque_device_id.to_string(),
            protocol_version: fixture.protocol_version.to_string(),
            family_hash: fixture.family_hash.to_string(),
            platform: fixture.platform.to_string(),
            agent_version: fixture.agent_version.to_string(),
            pairing_state: self.mdns_pairing_state(),
            lifecycle_state: fixture.lifecycle_state,
            support_state: fixture.support_state,
        })
    }

    pub(crate) fn record_allowed_snmp_probe_responses(
        &self,
        socket: &UdpSocket,
        max_datagram_count: usize,
    ) -> usize {
        if let Ok(mut state) = self.passive_discovery_listener_state.lock() {
            if !state.is_running() {
                return 0;
            }
            return collect_allowed_snmp_response_packets(socket, &mut state, max_datagram_count);
        }
        0
    }
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
        &crate::time::timestamp_now(),
    )
}

fn lan_ai_claim_id(job_id: &str) -> String {
    let mut claim_id = String::from(constants::lan_pairing::LAN_AI_CLAIM_ID_PREFIX);
    claim_id.push_str(job_id);
    claim_id
}

fn lan_ai_lease_id(job_id: &str) -> String {
    let mut lease_id = String::from(constants::lan_pairing::LAN_AI_LEASE_ID_PREFIX);
    lease_id.push_str(job_id);
    lease_id
}
