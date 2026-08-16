use std::io;
use std::sync::Mutex;

use ocentra_lan_core::lan_mdns_advertiser::{encode_advertisement_packet, LanMdnsPacketSink};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;

#[path = "../support/test_invariants.rs"]
mod test_invariants;

#[macro_use]
#[path = "../support/lan_root_harness.rs"]
mod lan_root_harness;
declare_lan_root_harness!();
#[path = "../unit/lan_pairing_test_commands.rs"]
mod lan_pairing_test_commands;
use crate::app::lan_pairing::LanPairingRuntime;
use crate::app::lan_pairing_runtime_state::mdns_advertisement::LanMdnsAdvertisementSyncState;
use crate::lan_pairing::LanPairingRegistryPersistence;
use crate::lan_pairing_runtime_state::mdns_advertisement::spawn_lan_mdns_advertisement_runtime;
use crate::lan_pairing_runtime_state::passive_discovery::spawn_lan_passive_discovery_runtime;
use crate::lan_pairing_test_commands::{
    health_command, health_command_for_target, paired_runtime, status_command,
};
use crate::lan_runtime_test_support::{
    default_child_mdns_advertisement_fixture, LanChildMdnsAdvertisementFixture,
};
use crate::test_invariants::{
    require_json_decode, require_log_string_field, require_ok, require_some,
};
use crate::time::{timestamp_after_epoch_seconds, timestamp_from_epoch_seconds};

static ENV_LOCK: Mutex<()> = Mutex::new(());

#[path = "lan_pairing_runtime_state/passive_discovery.rs"]
mod passive_discovery_tests;

#[test]
fn from_env_defaults_to_local_json_registry_path() {
    let _guard = require_ok(ENV_LOCK.lock(), "lan runtime env lock remains available");
    let previous_registry_path =
        std::env::var_os(constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH);
    let previous_child_device_id =
        std::env::var_os(constants::lan_pairing::LOCAL_CHILD_DEVICE_ID_ENV);
    std::env::remove_var(constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH);
    std::env::set_var(
        constants::lan_pairing::LOCAL_CHILD_DEVICE_ID_ENV,
        "Child Device 01",
    );

    let runtime = LanPairingRuntime::from_env();

    assert_eq!(
        runtime.persistence_mode(),
        constants::value::LAN_PERSISTENCE_LOCAL_JSON_REGISTRY.into()
    );
    assert!(matches!(
        runtime.persistence,
        LanPairingRegistryPersistence::LocalJsonRegistry(_)
    ));
    if let LanPairingRegistryPersistence::LocalJsonRegistry(path) = &runtime.persistence {
        assert_eq!(
            path,
            &std::env::temp_dir().join("ocentra-parent-lan-registry-child-device-01.json")
        );
    }

    match previous_registry_path {
        Some(value) => {
            std::env::set_var(constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH, value)
        }
        None => std::env::remove_var(constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH),
    }
    match previous_child_device_id {
        Some(value) => std::env::set_var(constants::lan_pairing::LOCAL_CHILD_DEVICE_ID_ENV, value),
        None => std::env::remove_var(constants::lan_pairing::LOCAL_CHILD_DEVICE_ID_ENV),
    }
}

#[test]
fn from_env_respects_explicit_registry_path_override() {
    let _guard = require_ok(ENV_LOCK.lock(), "lan runtime env lock remains available");
    let previous_registry_path =
        std::env::var_os(constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH);
    let explicit_path = std::env::temp_dir().join("ocentra-parent-lan-registry-override.json");
    std::env::set_var(
        constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH,
        explicit_path.as_os_str(),
    );

    let runtime = LanPairingRuntime::from_env();

    assert!(matches!(
        runtime.persistence,
        LanPairingRegistryPersistence::LocalJsonRegistry(_)
    ));
    if let LanPairingRegistryPersistence::LocalJsonRegistry(path) = &runtime.persistence {
        assert_eq!(path, &explicit_path);
    }

    match previous_registry_path {
        Some(value) => {
            std::env::set_var(constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH, value)
        }
        None => std::env::remove_var(constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH),
    }
}

#[tokio::test]
async fn lan_pairing_runtime_builds_hint_only_parent_and_child_mdns_advertisements() {
    let runtime = paired_runtime().await;
    let parent = require_ok(
        runtime.parent_mdns_advertisement(
            "sha256:parent-family-1",
            constants::lan_pairing::SCHEMA_VERSION_TEXT,
            "sha256:family-1",
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementLifecycleState::Start,
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementSupportState::Supported,
        ),
        "parent advertisement",
    );
    let child = require_ok(
        runtime.child_mdns_advertisement(LanChildMdnsAdvertisementFixture {
            opaque_device_id: "opaque-child-id".to_string(),
            ..default_child_mdns_advertisement_fixture(
                ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementLifecycleState::Update,
                ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementSupportState::Degraded,
            )
        }),
        "child advertisement",
    );

    assert_eq!(
        parent.service_type,
        constants::lan_pairing::MDNS_PARENT_SERVICE_TYPE
    );
    assert_eq!(
        child.service_type,
        constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE
    );
    assert_eq!(
        parent.confirmation_state.as_str(),
        constants::lan_pairing::MDNS_TXT_VALUE_HINT_ONLY.into()
    );
    assert_eq!(
        child.confirmation_state.as_str(),
        constants::lan_pairing::MDNS_TXT_VALUE_HINT_ONLY.into()
    );
    assert_eq!(
        parent.protocol_version,
        constants::lan_pairing::SCHEMA_VERSION_TEXT.to_string()
    );
    assert_eq!(
        child.protocol_version,
        constants::lan_pairing::SCHEMA_VERSION_TEXT.to_string()
    );
    assert_eq!(parent.family_hash, "sha256:family-1");
    assert_eq!(child.family_hash, "sha256:family-1");
    assert_eq!(child.opaque_device_id, "opaque-child-id");
    assert_eq!(child.platform, constants::lan_pairing::PLATFORM_WINDOWS);
    assert_eq!(child.agent_version, "1.2.3");
    assert_eq!(parent.txt_records.len(), 7);
    assert_eq!(child.txt_records.len(), 10);
    assert!(parent
        .txt_records
        .iter()
        .all(|record| !record.value.contains(' ') && !record.value.contains('@')));
    assert!(child
        .txt_records
        .iter()
        .all(|record| !record.value.contains(' ') && !record.value.contains('@')));
}

#[tokio::test]
async fn lan_pairing_runtime_syncs_mdns_advertisements_and_retracts_on_degraded_platform() {
    let mut runtime = paired_runtime().await;
    runtime.local_child_device_id = Some("opaque-child-id".to_string());
    runtime.signed_child_agent_parent_device_id =
        Some(constants::lan_pairing::PARENT_DEVICE_ID.to_string());
    runtime.signed_child_agent_family_hash = Some("sha256:family-1".to_string());
    runtime.signed_child_agent_route_id =
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string();
    let sink = RecordingMdnsSink::default();
    let mut sync_state = LanMdnsAdvertisementSyncState::default();

    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            ocentra_lan_core::lan_pairing::LanMdnsAdvertisementPlatformSupport::Supported,
            &sink,
        ),
        "initial sync succeeds",
    );
    let first_parent = require_some(sync_state.parent.clone(), "parent instance");
    let first_child = require_some(sync_state.child.clone(), "child instance");
    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            ocentra_lan_core::lan_pairing::LanMdnsAdvertisementPlatformSupport::Supported,
            &sink,
        ),
        "update sync succeeds",
    );
    let second_parent = require_some(sync_state.parent.clone(), "parent instance");
    let second_child = require_some(sync_state.child.clone(), "child instance");
    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            ocentra_lan_core::lan_pairing::LanMdnsAdvertisementPlatformSupport::Degraded,
            &sink,
        ),
        "degraded sync succeeds",
    );

    assert_eq!(
        sink.packets(),
        vec![
            encode_advertisement_packet(std::slice::from_ref(&first_parent), 120),
            encode_advertisement_packet(std::slice::from_ref(&first_child), 120),
            encode_advertisement_packet(std::slice::from_ref(&second_parent), 120),
            encode_advertisement_packet(std::slice::from_ref(&second_child), 120),
            encode_advertisement_packet(std::slice::from_ref(&second_parent), 0),
            encode_advertisement_packet(std::slice::from_ref(&second_child), 0),
        ]
    );
    assert!(sync_state.parent.is_none());
    assert!(sync_state.child.is_none());
}

#[derive(Default)]
struct RecordingMdnsSink {
    packets: Mutex<Vec<Vec<u8>>>,
}

impl RecordingMdnsSink {
    fn packets(&self) -> Vec<Vec<u8>> {
        require_ok(self.packets.lock(), "packets").clone()
    }
}

impl LanMdnsPacketSink for RecordingMdnsSink {
    fn send(&self, packet: &[u8]) -> io::Result<()> {
        require_ok(self.packets.lock(), "packets").push(packet.to_vec());
        Ok(())
    }
}

#[tokio::test]
async fn lan_pairing_runtime_state_helpers_are_used_by_real_flow() {
    let runtime = paired_runtime().await;
    let status = crate::app::lan_pairing_status::pairing_status_event(
        &runtime,
        status_command(LogFields::new()),
    );
    let pairing_state = require_log_string_field(
        status.payload.get(constants::field::LAN_PAIRING_STATE),
        "pairing state",
    );
    assert_eq!(pairing_state, "paired");

    let decoded: serde_json::Value =
        require_json_decode(br#"{"status":"paired"}"#, "status payload");
    assert_eq!(decoded["status"], "paired");

    let _ = health_command(LogFields::new());
    let _ = health_command_for_target("child-device", LogFields::new());
    assert_eq!(
        crate::app::lan_pairing::route_trust_state(runtime.selected_target().as_ref()),
        "paired".into()
    );

    let timestamp: String = timestamp_from_epoch_seconds(0);
    let later_timestamp: String = timestamp_after_epoch_seconds(0, 1);
    assert!(timestamp < later_timestamp);
    assert_eq!(
        crate::lan_pairing_status::route_trust_state_for_selected_target(
            runtime.selected_target().as_ref(),
        ),
        "paired".into()
    );

    spawn_lan_mdns_advertisement_runtime(runtime.clone());
    spawn_lan_passive_discovery_runtime(runtime);
}
