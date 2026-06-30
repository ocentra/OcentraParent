#![forbid(unsafe_code)]

#[path = "../src/event_builder.rs"]
mod event_builder;
#[path = "../src/fields.rs"]
mod fields;
#[path = "../src/json_contract.rs"]
mod json_contract;
#[path = "../src/lan_pairing.rs"]
mod lan_pairing;
#[path = "../src/lan_pairing_audit.rs"]
mod lan_pairing_audit;
#[path = "../src/lan_pairing_browser_add_device_scan.rs"]
mod lan_pairing_browser_add_device_scan;
#[path = "../src/lan_pairing_browser_add_device_state.rs"]
mod lan_pairing_browser_add_device_state;
#[path = "../src/lan_pairing_browser_runtime.rs"]
mod lan_pairing_browser_runtime;
#[path = "../src/lan_pairing_payload.rs"]
mod lan_pairing_payload;
#[path = "../src/lan_pairing_runtime_state.rs"]
mod lan_pairing_runtime_state;
#[path = "../src/lan_pairing_status.rs"]
mod lan_pairing_status;
#[path = "../src/lan_runtime_stream_api.rs"]
mod lan_runtime_stream_api;
#[path = "../src/lan_runtime_stream_payload.rs"]
mod lan_runtime_stream_payload;
#[path = "../src/time.rs"]
mod time;
#[path = "support/lan_test_websocket.rs"]
mod websocket;

mod app {
    pub(crate) mod fields {
        use ocentra_parent_agent_protocol::logging::LogFields;

        pub(crate) fn fields_from_pairs(
            pairs: Vec<(
                &'static str,
                ocentra_parent_agent_protocol::logging::LogFieldValue,
            )>,
        ) -> LogFields {
            crate::fields::fields_from_pairs(pairs)
        }
    }

    pub(crate) mod lan_pairing {
        pub(crate) type LanPairingRuntime = crate::lan_pairing::LanPairingRuntime;
    }

    pub(crate) mod lan_pairing_browser_add_device_state {
        pub(crate) mod scan_history {
            pub(crate) type LanScanHistorySnapshot =
                crate::lan_pairing_browser_add_device_state::scan_history::LanScanHistorySnapshot;
        }
    }

    pub(crate) mod lan_pairing_household_device_spine {
        pub(crate) fn canonical_household_devices(
            discovered_devices: &[ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceDiscoveryDevice],
            trusted_registry: &[ocentra_parent_agent_protocol::lan_pairing::LanTrustedDeviceRegistryEntry],
            household_device_decisions: &[ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanHouseholdDeviceDecision],
        ) -> Vec<
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice,
        >{
            crate::lan_runtime_test_support::canonical_household_devices_for_test(
                discovered_devices,
                trusted_registry,
                household_device_decisions,
            )
        }
    }

    pub(crate) mod lan_pairing_runtime_state {
        pub(crate) mod mdns_advertisement {
            pub(crate) type LanMdnsAdvertisementSyncState =
                crate::lan_pairing_runtime_state::mdns_advertisement::LanMdnsAdvertisementSyncState;
        }

        pub(crate) mod passive_discovery {
            pub(crate) type LanPassiveDiscoveryLocalNetworkChangeTrigger =
                crate::lan_pairing_runtime_state::passive_discovery::LanPassiveDiscoveryLocalNetworkChangeTrigger;
            pub(crate) type LanPassiveDiscoveryRuntimeObservedState =
                crate::lan_pairing_runtime_state::passive_discovery::LanPassiveDiscoveryRuntimeObservedState;

            pub(crate) fn local_network_change_triggers(
                previous_identity: Option<
                    &ocentra_lan_core::network_inventory::LanPassiveRuntimeLocalNetworkIdentity,
                >,
                current_identity: &ocentra_lan_core::network_inventory::LanPassiveRuntimeLocalNetworkIdentity,
            ) -> Vec<LanPassiveDiscoveryLocalNetworkChangeTrigger> {
                crate::lan_pairing_runtime_state::passive_discovery::local_network_change_triggers(
                    previous_identity,
                    current_identity,
                )
            }

            pub(crate) fn passive_discovery_udp_sources(
            ) -> &'static [ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoverySource]
            {
                crate::lan_pairing_runtime_state::passive_discovery::passive_discovery_udp_sources()
            }
        }

        pub(crate) mod provider_heartbeat {
            pub(crate) type LanAiProviderHeartbeatState =
                crate::lan_pairing_runtime_state::provider_heartbeat::LanAiProviderHeartbeatState;
        }
    }

    pub(crate) mod lan_pairing_status {
        pub(crate) fn pairing_status_event(
            runtime: &crate::lan_pairing::LanPairingRuntime,
            command: ocentra_parent_agent_protocol::transport::AgentCommandEnvelope,
        ) -> ocentra_parent_agent_protocol::transport::AgentEventEnvelope {
            crate::lan_pairing_status::pairing_status_event(runtime, command)
        }
    }

    pub(crate) mod lan_runtime_stream_payload {
        pub(crate) type LanRuntimeServiceStreamReport =
            crate::lan_runtime_stream_payload::LanRuntimeServiceStreamReport;

        pub(crate) fn stream_lan_runtime_event_chain_for_history(
            history: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEventHistory,
        ) -> LanRuntimeServiceStreamReport {
            crate::lan_runtime_stream_payload::stream_lan_runtime_event_chain_for_history(history)
        }

        pub(crate) fn lan_runtime_event_chain_stream_payload(
            report: &LanRuntimeServiceStreamReport,
        ) -> ocentra_parent_agent_protocol::logging::LogFields {
            crate::lan_runtime_stream_payload::lan_runtime_event_chain_stream_payload(report)
        }
    }

    pub(crate) mod time {
        pub(crate) fn timestamp_now() -> String {
            crate::time::timestamp_now()
        }
    }

    pub(crate) mod websocket {
        pub(crate) async fn handle_command_text_for_test(
            text: &str,
            lan_pairing: crate::lan_pairing::LanPairingRuntime,
            origin: Option<String>,
        ) -> ocentra_parent_agent_protocol::transport::AgentEventEnvelope {
            crate::websocket::handle_command_text_for_test(text, lan_pairing, origin).await
        }
    }
}

#[path = "unit/lan_pairing_household_device_spine_test_fixtures.rs"]
mod lan_pairing_household_device_spine_test_fixtures;
#[path = "unit/lan_pairing_provider_selection_read_model_support.rs"]
mod lan_pairing_provider_selection_read_model;
#[path = "unit/lan_pairing_test_assertions.rs"]
mod lan_pairing_test_assertions;
#[path = "unit/lan_pairing_test_commands.rs"]
mod lan_pairing_test_commands;
#[path = "support/lan_runtime_test_support.rs"]
mod lan_runtime_test_support;
#[path = "support/test_invariants.rs"]
mod test_invariants;

#[test]
fn lan_runtime_path_included_scheduler_hooks_and_time_helpers_remain_linked() {
    let _spawn_mdns: fn(crate::lan_pairing::LanPairingRuntime) =
        crate::lan_pairing_runtime_state::mdns_advertisement::spawn_lan_mdns_advertisement_runtime;
    let _spawn_passive: fn(crate::lan_pairing::LanPairingRuntime) =
        crate::lan_pairing_runtime_state::passive_discovery::spawn_lan_passive_discovery_runtime;

    assert_eq!(
        crate::time::timestamp_from_epoch_seconds(0),
        "1970-01-01T00:00:00.000Z"
    );
    assert_eq!(
        crate::time::timestamp_after_epoch_seconds(10, 5),
        "1970-01-01T00:00:15.000Z"
    );

    let decoded: serde_json::Value =
        crate::test_invariants::require_json_decode("{\"linked\":true}", "json helper is linked");
    assert_eq!(decoded["linked"], true);
    let field = ocentra_parent_agent_protocol::logging::LogFieldValue::String("linked".to_string());
    assert_eq!(
        crate::test_invariants::require_log_string_field(Some(&field), "log helper is linked"),
        "linked"
    );
}

#[path = "security/lan_api_boundary.rs"]
mod lan_api_boundary_tests;
#[path = "unit/lan_pairing_browser_add_device_state.rs"]
mod lan_pairing_browser_add_device_state_tests;
#[path = "unit/lan_pairing_browser_runtime.rs"]
mod lan_pairing_browser_runtime_tests;
#[path = "unit/lan_pairing_household_device_spine.rs"]
mod lan_pairing_household_device_spine_tests;
#[path = "unit/lan_pairing_multidevice.rs"]
mod lan_pairing_multidevice_tests;
#[path = "unit/lan_pairing_provider_selection_read_model.rs"]
mod lan_pairing_provider_selection_read_model_tests;
#[path = "unit/lan_pairing_status_get.rs"]
mod lan_pairing_status_get_tests;
#[path = "unit/lan_pairing.rs"]
mod lan_pairing_tests;
#[path = "concurrency/lan_replay_concurrency.rs"]
mod lan_replay_concurrency_tests;
#[path = "unit/lan_runtime_stream.rs"]
mod lan_runtime_stream_tests;

mod lan_pairing_runtime_state_tests {
    use super::lan_pairing::{LanPairingRegistryPersistence, LanPairingRuntime};
    use ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoverySource;
    use ocentra_parent_agent_protocol::constants;

    #[path = "../unit/lan_pairing_runtime_state.rs"]
    mod tests;
}

mod lan_pairing_status_selection_tests {
    use super::lan_pairing_status::selection::*;
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::lan_pairing::{
        LanPairingDeviceReachability, LanPairingTrustState,
    };

    #[path = "../unit/lan_pairing_status_selection.rs"]
    mod tests;
}

mod lan_pairing_browser_add_device_scan_tests {
    use super::lan_pairing_browser_add_device_scan::*;
    use ocentra_lan_core::network_inventory::LanNetworkInventoryDevice;

    #[path = "../unit/lan_pairing_browser_add_device_scan.rs"]
    mod tests;
}

mod lan_pairing_browser_add_device_state_private_tests {
    use super::lan_pairing_browser_add_device_state::discovery_projection::selected_device_readiness;
    use super::lan_pairing_browser_add_device_state::physical_lan_scan::LanNetworkDeviceScanResult;
    use super::lan_pairing_browser_add_device_state::{
        discovery_event_history_state, network_neighbor_child_device, ordered_discovery_event_rows,
        platform_data_available_for_scan_result_with_manual_required_override,
    };

    mod scan_history {
        pub(crate) type LanScanHistoryMetadata =
            crate::lan_pairing_browser_add_device_state::scan_history::LanScanHistoryMetadata;
        pub(crate) type LanScanHistorySnapshot =
            crate::lan_pairing_browser_add_device_state::scan_history::LanScanHistorySnapshot;
    }

    #[path = "../unit/lan_pairing_browser_add_device_state_private.rs"]
    mod tests;
}

mod lan_pairing_browser_add_device_state_physical_lan_scan_tests {
    use super::lan_pairing_browser_add_device_state::physical_lan_scan::*;
    use super::lan_pairing_browser_add_device_state::scan_history::{
        recent_previous_scan_agent_truth_devices, scan_history_is_recent,
    };

    #[path = "../unit/lan_pairing_browser_add_device_state_physical_lan_scan.rs"]
    mod tests;
}

mod lan_pairing_browser_add_device_state_scan_history_tests {
    use super::lan_pairing::LanPairingRuntime;
    use super::lan_pairing_browser_add_device_state::scan_history::*;
    use crate::lan_runtime_test_support::load_scan_history_for_test as load_scan_history;
    use chrono::Utc;
    use ocentra_lan_core::network_inventory::{LanDiscoveryScanPlan, LanNetworkInventoryDevice};
    use std::fs;
    use std::path::{Path, PathBuf};

    #[path = "../unit/lan_pairing_browser_add_device_state_scan_history.rs"]
    mod tests;
}
