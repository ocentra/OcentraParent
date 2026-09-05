#![forbid(unsafe_code)]

static LAN_RUNTIME_ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[path = "../support/test_invariants/require_json_decode.rs"]
mod test_require_json_decode;
#[path = "../support/test_invariants/require_log_string_field.rs"]
mod test_require_log_string_field;
#[path = "../support/test_invariants/require_ok.rs"]
mod test_require_ok;
#[path = "../support/test_invariants/require_some.rs"]
mod test_require_some;
#[path = "../support/test_invariants/serialize_test_json.rs"]
mod test_serialize_json;

#[path = "../../src/event_builder.rs"]
mod event_builder;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../../src/json_contract.rs"]
mod json_contract;
#[path = "../support/lan_pairing_mod.rs"]
mod lan_pairing;
#[path = "../support/lan_pairing_audit_mod.rs"]
mod lan_pairing_audit;
#[path = "../../src/lan_pairing_browser_add_device_scan.rs"]
mod lan_pairing_browser_add_device_scan;
#[path = "../support/lan_pairing_browser_add_device_state_mod.rs"]
mod lan_pairing_browser_add_device_state;
#[path = "../../src/lan_pairing_browser_runtime.rs"]
mod lan_pairing_browser_runtime;
#[path = "../../src/lan_pairing_payload.rs"]
mod lan_pairing_payload;
#[path = "../support/lan_pairing_runtime_state_mod.rs"]
mod lan_pairing_runtime_state;
#[path = "../../src/lan_pairing_status.rs"]
mod lan_pairing_status;
#[path = "../../src/lan_runtime_stream_api.rs"]
mod lan_runtime_stream_api;
#[path = "../../src/lan_runtime_stream_payload.rs"]
mod lan_runtime_stream_payload;
#[path = "../support/lan_test_websocket_dispatch.rs"]
pub(crate) mod lan_test_websocket_dispatch;
#[path = "../support/lan_test_text.rs"]
mod test_text;
#[path = "../../src/time.rs"]
mod time;
#[path = "../support/lan_test_websocket.rs"]
mod websocket;

mod app {
    pub(crate) mod fields {
        pub(crate) fn fields_from_pairs(
            pairs: Vec<(
                &'static str,
                ocentra_parent_agent_protocol::logging::LogFieldValue,
            )>,
        ) -> ocentra_parent_agent_protocol::logging::LogFields {
            crate::fields::fields_from_pairs(pairs)
        }
    }

    pub(crate) mod lan_pairing {
        pub(crate) type LanPairingRuntime = crate::lan_pairing::LanPairingRuntime;

        pub(crate) fn route_trust_state(
            target: Option<&ocentra_parent_agent_protocol::lan_pairing::LanSelectedRouteTarget>,
        ) -> ocentra_parent_agent_protocol::lan_pairing::LanPairingText {
            crate::lan_pairing_status::selection::route_trust_state(target)
        }
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

            pub(crate) fn passive_discovery_udp_sources() -> &'static [
                ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoverySource
            ]{
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

        pub(crate) const STREAM_LAN_RUNTIME_EVENT_CHAIN_FOR_HISTORY: fn(
            &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEventHistory,
            ocentra_parent_agent_protocol::lan_pairing::LanPairingText,
        ) -> LanRuntimeServiceStreamReport =
            crate::lan_runtime_stream_payload::stream_lan_runtime_event_chain_for_history;

        pub(crate) const LAN_RUNTIME_EVENT_CHAIN_STREAM_PAYLOAD:
            fn(&LanRuntimeServiceStreamReport) -> ocentra_parent_agent_protocol::logging::LogFields =
            crate::lan_runtime_stream_payload::lan_runtime_event_chain_stream_payload;
    }

    pub(crate) mod time {
        pub(crate) const TIMESTAMP_NOW: fn() -> String = crate::time::timestamp_now;
    }

    pub(crate) mod websocket {
        pub(crate) async fn handle_command_text_for_test(
            text: crate::test_text::TestText,
            lan_pairing: crate::lan_pairing::LanPairingRuntime,
            origin: Option<crate::test_text::TestText>,
        ) -> ocentra_parent_agent_protocol::transport::AgentEventEnvelope {
            crate::websocket::handle_command_text_for_test(text, lan_pairing, origin).await
        }
    }
}

#[path = "lan_pairing_household_device_spine_test_fixtures.rs"]
mod lan_pairing_household_device_spine_test_fixtures;
#[path = "lan_pairing_provider_selection_read_model_support.rs"]
mod lan_pairing_provider_selection_read_model;
#[path = "lan_pairing_test_assertions.rs"]
mod lan_pairing_test_assertions;
#[path = "lan_pairing_test_commands.rs"]
mod lan_pairing_test_commands;
#[path = "lan_pairing_test_multidevice_commands.rs"]
mod lan_pairing_test_multidevice_commands;
#[path = "../support/lan_runtime_test_support.rs"]
mod lan_runtime_test_support;

#[path = "lan_pairing_runtime_state.rs"]
mod lan_pairing_runtime_state_tests;
#[path = "lan_pairing_status_selection.rs"]
mod lan_pairing_status_selection_tests;
#[path = "lan_pairing.rs"]
mod lan_pairing_tests;

#[test]
fn lan_runtime_reports_ordered_local_network_change_triggers() {
    use ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryTriggerReason;
    use ocentra_lan_core::network_inventory::LanPassiveRuntimeLocalNetworkIdentity;

    let previous = LanPassiveRuntimeLocalNetworkIdentity {
        ip_address: Some("192.168.1.10".to_string()),
        network_interface: Some("Ethernet".to_string()),
        wifi_ssid: Some("old-wifi".to_string()),
        default_gateway: Some("192.168.1.1".to_string()),
    };
    let current = LanPassiveRuntimeLocalNetworkIdentity {
        ip_address: Some("192.168.1.11".to_string()),
        network_interface: Some("Wi-Fi".to_string()),
        wifi_ssid: Some("new-wifi".to_string()),
        default_gateway: Some("192.168.1.254".to_string()),
    };

    let triggers =
        crate::app::lan_pairing_runtime_state::passive_discovery::local_network_change_triggers(
            Some(&previous),
            &current,
        );

    assert_eq!(triggers.len(), 4);
    assert_eq!(
        triggers[0].reason,
        LanPassiveDiscoveryTriggerReason::InterfaceDown
    );
    assert_eq!(triggers[0].summary, "network interface down: Ethernet");
    assert_eq!(
        triggers[1].reason,
        LanPassiveDiscoveryTriggerReason::InterfaceUp
    );
    assert_eq!(triggers[1].summary, "network interface up: Wi-Fi");
    assert_eq!(
        triggers[2].reason,
        LanPassiveDiscoveryTriggerReason::IpAddressChanged
    );
    assert_eq!(
        triggers[2].summary,
        "ip address changed: 192.168.1.10 -> 192.168.1.11"
    );
    assert_eq!(
        triggers[3].reason,
        LanPassiveDiscoveryTriggerReason::DefaultGatewayChanged
    );
    assert_eq!(
        triggers[3].summary,
        "default gateway changed: 192.168.1.1 -> 192.168.1.254"
    );
    let same_interface = LanPassiveRuntimeLocalNetworkIdentity {
        network_interface: Some("Ethernet".to_string()),
        wifi_ssid: Some("new-wifi".to_string()),
        ..previous.clone()
    };
    let wifi_triggers =
        crate::app::lan_pairing_runtime_state::passive_discovery::local_network_change_triggers(
            Some(&previous),
            &same_interface,
        );
    assert_eq!(wifi_triggers.len(), 1);
    assert_eq!(
        wifi_triggers[0].reason,
        LanPassiveDiscoveryTriggerReason::WifiSsidChanged
    );
    assert_eq!(
        wifi_triggers[0].summary,
        "wifi ssid changed: old-wifi -> new-wifi"
    );
    assert!(
        crate::app::lan_pairing_runtime_state::passive_discovery::local_network_change_triggers(
            None, &current,
        )
        .is_empty()
    );
}

#[test]
fn lan_runtime_from_env_projects_configuration_into_read_model_and_status() {
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::lan_pairing::{DeviceRuntimeRole, DeviceRuntimeSurface};
    use ocentra_parent_agent_protocol::logging::LogFieldValue;
    use std::ffi::OsString;

    let _guard = test_require_ok::require_ok(
        LAN_RUNTIME_ENV_LOCK.lock(),
        "lan runtime env lock remains available",
    );

    let registry_path = std::env::temp_dir().join("lan-runtime-configured-test.json");
    let names = [
        constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH,
        constants::lan_pairing::DEVICE_SURFACE_ENV,
        constants::lan_pairing::DEVICE_ROLES_ENV,
        constants::lan_pairing::LAN_AI_PROVIDER_CAPABILITIES_ENV,
        constants::lan_pairing::LAN_AI_PROVIDER_OPT_IN_ENV,
    ];
    let previous: Vec<Option<OsString>> = names.iter().map(std::env::var_os).collect();
    std::env::set_var(
        constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH,
        &registry_path,
    );
    std::env::set_var(
        constants::lan_pairing::DEVICE_SURFACE_ENV,
        constants::value::DEVICE_RUNTIME_SURFACE_PARENT_DESKTOP,
    );
    std::env::set_var(
        constants::lan_pairing::DEVICE_ROLES_ENV,
        "parent-controller,ai-provider",
    );
    std::env::set_var(
        constants::lan_pairing::LAN_AI_PROVIDER_CAPABILITIES_ENV,
        "ocr,classification",
    );
    std::env::set_var(
        constants::lan_pairing::LAN_AI_PROVIDER_OPT_IN_ENV,
        constants::value::TRUE,
    );

    let runtime = crate::app::lan_pairing::LanPairingRuntime::from_env();
    let read_model = runtime.device_role_read_model();
    assert_eq!(read_model.surface, DeviceRuntimeSurface::ParentDesktop);
    assert_eq!(read_model.primary_role, DeviceRuntimeRole::ParentController);
    assert_eq!(read_model.roles.len(), 2);
    assert!(runtime.lan_ai_provider_available());
    assert_eq!(
        runtime.lan_ai_provider_capability_flags().0,
        "ocr,classification"
    );

    let status = crate::lan_pairing_status::pairing_status_event(
        &runtime,
        crate::lan_pairing_test_commands::status_command(
            ocentra_parent_agent_protocol::logging::LogFields::new(),
        ),
    );
    assert!(matches!(
        status.payload.get(constants::field::LAN_ADD_DEVICE_READ_MODEL),
        Some(LogFieldValue::String(value)) if !value.is_empty()
    ));
    assert_eq!(
        crate::lan_pairing_status::route_trust_state_for_selected_target(None).0,
        ""
    );

    for (name, value) in names.into_iter().zip(previous) {
        match value {
            Some(value) => std::env::set_var(name, value),
            None => std::env::remove_var(name),
        }
    }
    assert!(
        std::fs::remove_file(registry_path).is_ok(),
        "configured LAN registry test file should be removable"
    );
}

#[path = "../security/lan_api_boundary.rs"]
mod lan_api_boundary_tests;
#[path = "lan_pairing_browser_add_device_state.rs"]
mod lan_pairing_browser_add_device_state_tests;
#[path = "lan_pairing_browser_runtime.rs"]
mod lan_pairing_browser_runtime_tests;
#[path = "lan_pairing_household_device_spine.rs"]
mod lan_pairing_household_device_spine_tests;
#[path = "lan_pairing_provider_selection_read_model.rs"]
mod lan_pairing_provider_selection_read_model_tests;
#[path = "lan_pairing_status_get.rs"]
mod lan_pairing_status_get_tests;
#[path = "../concurrency/lan_replay_concurrency.rs"]
mod lan_replay_concurrency_tests;
#[path = "lan_runtime_stream.rs"]
mod lan_runtime_stream_tests;

mod lan_pairing_browser_add_device_scan_tests {
    use super::lan_pairing_browser_add_device_scan::*;
    use ocentra_lan_core::network_inventory::LanNetworkInventoryDevice;

    mod tests {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/unit/lan_pairing_browser_add_device_scan.rs"
        ));
    }
}

mod lan_pairing_browser_add_device_state_physical_lan_scan_tests {
    use super::lan_pairing_browser_add_device_state::physical_lan_scan::*;
    use super::lan_pairing_browser_add_device_state::scan_history::{
        recent_previous_scan_agent_truth_devices, scan_history_is_recent,
    };

    mod tests {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/unit/lan_pairing_browser_add_device_state_physical_lan_scan.rs"
        ));
    }
}

mod lan_pairing_browser_add_device_state_scan_history_tests {
    use super::lan_pairing::LanPairingRuntime;
    use super::lan_pairing_browser_add_device_state::scan_history::*;
    use crate::lan_runtime_test_support::load_scan_history_for_test as load_scan_history;
    use chrono::Utc;
    use ocentra_lan_core::network_inventory::{LanDiscoveryScanPlan, LanNetworkInventoryDevice};
    use std::fs;

    mod tests {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/unit/lan_pairing_browser_add_device_state_scan_history.rs"
        ));
    }
}
