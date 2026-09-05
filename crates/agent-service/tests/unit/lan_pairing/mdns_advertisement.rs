use std::sync::Mutex;

use ocentra_lan_core::{
    lan_mdns_advertiser::{current_platform_support, LanMdnsPacketSink},
    lan_pairing::LanMdnsAdvertisementPlatformSupport,
};
use ocentra_parent_agent_protocol::{
    constants,
    logging::{LogFieldValue, LogFields},
};

use crate::test_text::TestText;

use crate::{
    app::{
        lan_pairing::LanPairingRuntime,
        lan_pairing_runtime_state::mdns_advertisement::LanMdnsAdvertisementSyncState,
        lan_pairing_status::pairing_status_event,
    },
    lan_pairing_test_commands::status_command,
    lan_runtime_test_support::{
        default_child_mdns_advertisement_fixture, LanChildMdnsAdvertisementFixture,
    },
    test_require_ok::require_ok,
    test_require_some::require_some,
};

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
    fn send(&self, packet: &[u8]) -> std::io::Result<()> {
        require_ok(self.packets.lock(), "packets").push(packet.to_vec());
        Ok(())
    }
}

fn packet_contains(packet: &[u8], text: &[u8]) -> bool {
    packet.windows(text.len()).any(|window| window == text)
}

#[test]
fn lan_pairing_runtime_builds_sanitized_mdns_advertisements_and_keeps_hint_only_state() {
    let runtime = LanPairingRuntime::empty();
    let lifecycle = LanPairingRuntime::mdns_advertisement_lifecycle(
        true,
        false,
        ocentra_lan_core::lan_pairing::LanMdnsAdvertisementPlatformSupport::Supported,
    );
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
        runtime.child_mdns_advertisement(default_child_mdns_advertisement_fixture(
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementLifecycleState::Degraded,
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementSupportState::Degraded,
        )),
        "child advertisement",
    );

    assert_eq!(lifecycle.lifecycle_action.as_str(), "start");
    assert!(lifecycle.hint_only);
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
        parent.pairing_state.as_str(),
        constants::value::LAN_PAIRING_UNPAIRED.into()
    );
    assert_eq!(
        child.pairing_state.as_str(),
        constants::value::LAN_PAIRING_UNPAIRED.into()
    );
    assert!(parent
        .txt_records
        .iter()
        .all(|record| !record.value.contains(' ') && !record.value.contains('@')));
    assert!(child
        .txt_records
        .iter()
        .all(|record| !record.value.contains(' ') && !record.value.contains('@')));
}

#[test]
fn unsupported_mdns_platform_reports_degraded_lifecycle() {
    let lifecycle = LanPairingRuntime::mdns_advertisement_lifecycle(
        true,
        true,
        ocentra_lan_core::lan_pairing::LanMdnsAdvertisementPlatformSupport::UnsupportedPlatform,
    );

    assert_eq!(lifecycle.lifecycle_action.as_str(), "degraded");
    assert!(lifecycle.hint_only);
}

#[test]
fn mdns_sync_updates_existing_advertisements_on_subsequent_pass() {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        Some(TestText::from_display("opaque-child-id")),
        TestText::from_display(constants::lan_pairing::PARENT_DEVICE_ID),
        TestText::from_display("sha256:family-1"),
        TestText::from_display(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK),
    );
    let sink = RecordingMdnsSink::default();
    let mut sync_state = LanMdnsAdvertisementSyncState::default();

    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            LanMdnsAdvertisementPlatformSupport::Supported,
            &sink,
        ),
        "initial sync succeeds",
    );
    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            LanMdnsAdvertisementPlatformSupport::Supported,
            &sink,
        ),
        "update sync succeeds",
    );

    let packets = sink.packets();

    assert_eq!(packets.len(), 4);
    assert!(packets
        .iter()
        .all(|packet| packet_contains(packet, b"lifecycle-state=start")
            || packet_contains(packet, b"lifecycle-state=update")));
    assert!(packets
        .iter()
        .skip(2)
        .all(|packet| packet_contains(packet, b"lifecycle-state=update")));
    let parent_instance = require_some(sync_state.parent.as_ref(), "parent instance");
    let child_instance = require_some(sync_state.child.as_ref(), "child instance");
    assert_eq!(
        require_some(packets.get(2), "parent update packet"),
        &ocentra_lan_core::lan_mdns_advertiser::encode_advertisement_packet(
            std::slice::from_ref(parent_instance),
            120,
        )
    );
    assert_eq!(
        require_some(packets.get(3), "child update packet"),
        &ocentra_lan_core::lan_mdns_advertiser::encode_advertisement_packet(
            std::slice::from_ref(child_instance),
            120,
        )
    );
}

#[test]
fn mdns_sync_broadcasts_real_parent_and_child_packets_when_runtime_has_context() {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        Some(TestText::from_display("opaque-child-id")),
        TestText::from_display(constants::lan_pairing::PARENT_DEVICE_ID),
        TestText::from_display("sha256:family-1"),
        TestText::from_display(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK),
    );
    let sink = RecordingMdnsSink::default();
    let mut sync_state = LanMdnsAdvertisementSyncState::default();

    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            LanMdnsAdvertisementPlatformSupport::Supported,
            &sink,
        ),
        "mdns sync succeeds",
    );

    let packets = sink.packets();

    assert_eq!(packets.len(), 2);
    assert!(packets
        .iter()
        .any(|packet| { packet_contains(packet, b"_ocentra-parent") }));
    assert!(packets
        .iter()
        .any(|packet| { packet_contains(packet, b"_ocentra-agent") }));
    let parent_instance = require_some(sync_state.parent.as_ref(), "parent instance");
    let child_instance = require_some(sync_state.child.as_ref(), "child instance");
    assert_eq!(
        require_some(packets.first(), "parent advertisement packet"),
        &ocentra_lan_core::lan_mdns_advertiser::encode_advertisement_packet(
            std::slice::from_ref(parent_instance),
            120,
        )
    );
    assert_eq!(
        require_some(packets.get(1), "child advertisement packet"),
        &ocentra_lan_core::lan_mdns_advertiser::encode_advertisement_packet(
            std::slice::from_ref(child_instance),
            120,
        )
    );
}

#[test]
fn mdns_sync_retracts_existing_advertisements_when_platform_support_becomes_degraded() {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        Some(TestText::from_display("opaque-child-id")),
        TestText::from_display(constants::lan_pairing::PARENT_DEVICE_ID),
        TestText::from_display("sha256:family-1"),
        TestText::from_display(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK),
    );
    let sink = RecordingMdnsSink::default();
    let mut sync_state = LanMdnsAdvertisementSyncState::default();

    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            LanMdnsAdvertisementPlatformSupport::Supported,
            &sink,
        ),
        "initial sync succeeds",
    );
    let parent_instance = require_some(sync_state.parent.clone(), "parent instance");
    let child_instance = require_some(sync_state.child.clone(), "child instance");
    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            LanMdnsAdvertisementPlatformSupport::Degraded,
            &sink,
        ),
        "degraded sync succeeds",
    );

    let packets = sink.packets();

    assert_eq!(packets.len(), 4);
    assert_eq!(
        require_some(packets.get(2), "parent goodbye"),
        &ocentra_lan_core::lan_mdns_advertiser::encode_advertisement_packet(
            std::slice::from_ref(&parent_instance),
            0,
        )
    );
    assert_eq!(
        require_some(packets.get(3), "child goodbye"),
        &ocentra_lan_core::lan_mdns_advertiser::encode_advertisement_packet(
            std::slice::from_ref(&child_instance),
            0,
        )
    );
    assert!(sync_state.parent.is_none());
    assert!(sync_state.child.is_none());
}

#[test]
fn mdns_sync_sends_goodbye_when_runtime_context_disappears() {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        Some(TestText::from_display("opaque-child-id")),
        TestText::from_display(constants::lan_pairing::PARENT_DEVICE_ID),
        TestText::from_display("sha256:family-1"),
        TestText::from_display(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK),
    );
    let sink = RecordingMdnsSink::default();
    let mut sync_state = LanMdnsAdvertisementSyncState::default();

    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            LanMdnsAdvertisementPlatformSupport::Supported,
            &sink,
        ),
        "initial sync succeeds",
    );
    require_ok(
        LanPairingRuntime::empty().sync_mdns_advertisements_with_sink(
            &mut sync_state,
            LanMdnsAdvertisementPlatformSupport::Supported,
            &sink,
        ),
        "goodbye sync succeeds",
    );

    assert_eq!(sink.packets().len(), 4);
    assert!(sync_state.parent.is_none());
    assert!(sync_state.child.is_none());
}

#[test]
fn mdns_sync_keeps_degraded_platform_manual_without_broadcasting_packets() {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        Some(TestText::from_display("opaque-child-id")),
        constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        "sha256:family-1".to_string(),
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
    );
    let sink = RecordingMdnsSink::default();
    let mut sync_state = LanMdnsAdvertisementSyncState::default();

    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            LanMdnsAdvertisementPlatformSupport::Degraded,
            &sink,
        ),
        "degraded sync succeeds",
    );

    assert!(sink.packets().is_empty());
    assert!(sync_state.parent.is_none());
    assert!(sync_state.child.is_none());
}

#[test]
fn mdns_sync_retracts_stale_state_when_mdns_context_is_invalid() {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        Some(TestText::from_display("opaque-child-id")),
        TestText::from_display(constants::lan_pairing::PARENT_DEVICE_ID),
        TestText::from_display("sha256:family broken"),
        TestText::from_display(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK),
    );
    let sink = RecordingMdnsSink::default();
    let parent_instance = ocentra_lan_core::lan_mdns_advertiser::LanMdnsAdvertisementInstance {
        service_type: constants::lan_pairing::MDNS_PARENT_SERVICE_TYPE.to_string(),
        instance_name: "parent-stale._ocentra-parent._tcp.local".to_string(),
        txt_records: Vec::new(),
    };
    let child_instance = ocentra_lan_core::lan_mdns_advertiser::LanMdnsAdvertisementInstance {
        service_type: constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE.to_string(),
        instance_name: "child-stale._ocentra-agent._tcp.local".to_string(),
        txt_records: Vec::new(),
    };
    let mut sync_state = LanMdnsAdvertisementSyncState {
        parent: Some(parent_instance.clone()),
        child: Some(child_instance.clone()),
    };

    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            LanMdnsAdvertisementPlatformSupport::Supported,
            &sink,
        ),
        "invalid context sync succeeds without advertising",
    );

    assert_eq!(
        sink.packets(),
        vec![
            ocentra_lan_core::lan_mdns_advertiser::encode_advertisement_packet(
                std::slice::from_ref(&parent_instance),
                0,
            ),
            ocentra_lan_core::lan_mdns_advertiser::encode_advertisement_packet(
                std::slice::from_ref(&child_instance),
                0,
            ),
        ]
    );
    assert!(sync_state.parent.is_none());
    assert!(sync_state.child.is_none());
}

#[tokio::test]
async fn lan_pairing_status_surface_reports_live_mdns_support_from_runtime_context() {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        Some(TestText::from_display(
            constants::lan_pairing::CHILD_DEVICE_ID,
        )),
        TestText::from_display(constants::lan_pairing::PARENT_DEVICE_ID),
        TestText::from_display("sha256:family-1"),
        TestText::from_display(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK),
    );
    let expected =
        LanPairingRuntime::mdns_advertisement_lifecycle(true, false, current_platform_support());
    let event = pairing_status_event(&runtime, status_command(LogFields::new()));

    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_MDNS_ADVERTISEMENT_LIFECYCLE),
        Some(&LogFieldValue::String(
            expected.lifecycle_action.as_str().to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_MDNS_ADVERTISEMENT_SUPPORT),
        Some(&LogFieldValue::String(
            expected.platform_support.as_str().to_string()
        ))
    );
}

#[test]
fn lan_pairing_runtime_rejects_sensitive_mdns_txt_atoms() {
    let runtime = LanPairingRuntime::empty();

    assert!(runtime
        .parent_mdns_advertisement(
            "parent family name",
            constants::lan_pairing::SCHEMA_VERSION_TEXT,
            "sha256:family-1",
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementLifecycleState::Start,
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementSupportState::Supported,
        )
        .is_err());
    assert!(runtime
        .child_mdns_advertisement(LanChildMdnsAdvertisementFixture {
            opaque_device_id: "child@example.com".to_string(),
            ..default_child_mdns_advertisement_fixture(
                ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementLifecycleState::Start,
                ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementSupportState::Supported,
            )
        })
        .is_err());
    assert!(runtime
        .child_mdns_advertisement(LanChildMdnsAdvertisementFixture {
            platform: "Windows Laptop".to_string(),
            ..default_child_mdns_advertisement_fixture(
                ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementLifecycleState::Start,
                ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementSupportState::Supported,
            )
        })
        .is_err());
}

#[test]
fn lan_pairing_runtime_rejects_invalid_mdns_protocol_versions() {
    let runtime = LanPairingRuntime::empty();

    assert!(runtime
        .parent_mdns_advertisement(
            "sha256:parent-family-1",
            " ",
            "sha256:family-1",
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementLifecycleState::Start,
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementSupportState::Supported,
        )
        .is_err());
    assert!(runtime
        .child_mdns_advertisement(LanChildMdnsAdvertisementFixture {
            protocol_version: " ".to_string(),
            ..default_child_mdns_advertisement_fixture(
                ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementLifecycleState::Start,
                ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementSupportState::Supported,
            )
        })
        .is_err());
}
