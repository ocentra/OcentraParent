use crate::support::{OptionTestExt as _, ResultTestExt as _};
use std::io;
use std::sync::Mutex;

use ocentra_eventing::error::EventingError;
use ocentra_lan_core::lan_mdns_advertiser::{
    child_instance, current_platform_support, derive_child_advertisement_id,
    derive_parent_advertisement_id, encode_advertisement_packet, parent_instance, send_goodbye,
    LanMdnsPacketSink,
};
use ocentra_lan_core::network_inventory::mdns_dns_sd::discovery_from_single_packet;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanChildMdnsAdvertisement, LanChildMdnsAdvertisementInput, LanMdnsAdvertisementLifecycleState,
    LanMdnsAdvertisementSupportState, LanPairingTrustState, LanParentMdnsAdvertisement,
};

macro_rules! assert_empty_value_error {
    ($result:expr, $expected_field:expr) => {
        assert!(
            matches!($result, Err(EventingError::EmptyValue { field }) if field == $expected_field),
            "expected EmptyValue error for field {}",
            $expected_field
        );
    };
}

#[test]
fn parent_instance_uses_hashed_service_label_and_contract_txt_records() {
    let advertisement = LanParentMdnsAdvertisement::new(
        derive_parent_advertisement_id("sha256:family-parent"),
        constants::lan_pairing::SCHEMA_VERSION_TEXT,
        "sha256:family-parent",
        LanPairingTrustState::Paired,
        LanMdnsAdvertisementLifecycleState::Start,
        LanMdnsAdvertisementSupportState::Supported,
    )
    .value_or_unreachable();

    let instance = parent_instance(&advertisement);

    assert!(instance.instance_name.starts_with("parent-"));
    assert!(instance.instance_name.ends_with(&format!(
        ".{}",
        constants::lan_pairing::MDNS_PARENT_SERVICE_TYPE
    )));
    assert!(instance.txt_records.iter().any(|record| record.key
        == constants::lan_pairing::MDNS_TXT_KEY_FAMILY_HASH
        && record.value == "sha256:family-parent"));
}

#[test]
fn child_instance_uses_hashed_service_label_and_opaque_txt_records() {
    let advertisement =
        LanChildMdnsAdvertisement::new(child_advertisement_input()).value_or_unreachable();

    let instance = child_instance(&advertisement);

    assert!(instance.instance_name.starts_with("child-"));
    assert!(instance.instance_name.ends_with(&format!(
        ".{}",
        constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE
    )));
    assert!(instance.txt_records.iter().any(|record| record.key
        == constants::lan_pairing::MDNS_TXT_KEY_OPAQUE_DEVICE_ID
        && record.value == "opaque-child-id"));
}

#[test]
fn parent_and_child_constructors_reject_missing_required_fields() {
    assert_empty_value_error!(
        LanParentMdnsAdvertisement::new(
            "",
            constants::lan_pairing::SCHEMA_VERSION_TEXT,
            "sha256:family-parent",
            LanPairingTrustState::Paired,
            LanMdnsAdvertisementLifecycleState::Start,
            LanMdnsAdvertisementSupportState::Supported,
        ),
        constants::lan_pairing::MDNS_ADVERTISEMENT_ID_FIELD
    );
    assert_empty_value_error!(
        LanParentMdnsAdvertisement::new(
            derive_parent_advertisement_id("sha256:family-parent"),
            "",
            "sha256:family-parent",
            LanPairingTrustState::Paired,
            LanMdnsAdvertisementLifecycleState::Start,
            LanMdnsAdvertisementSupportState::Supported,
        ),
        constants::lan_pairing::MDNS_PROTOCOL_VERSION_FIELD
    );
    assert_empty_value_error!(
        LanParentMdnsAdvertisement::new(
            derive_parent_advertisement_id("sha256:family-parent"),
            constants::lan_pairing::SCHEMA_VERSION_TEXT,
            "",
            LanPairingTrustState::Paired,
            LanMdnsAdvertisementLifecycleState::Start,
            LanMdnsAdvertisementSupportState::Supported,
        ),
        constants::lan_pairing::MDNS_FAMILY_HASH_FIELD
    );

    let mut input = child_advertisement_input();
    input.advertisement_id.clear();
    assert_empty_value_error!(
        LanChildMdnsAdvertisement::new(input),
        constants::lan_pairing::MDNS_ADVERTISEMENT_ID_FIELD
    );

    let mut input = child_advertisement_input();
    input.opaque_device_id.clear();
    assert_empty_value_error!(
        LanChildMdnsAdvertisement::new(input),
        constants::lan_pairing::MDNS_OPAQUE_DEVICE_ID_FIELD
    );

    let mut input = child_advertisement_input();
    input.protocol_version.clear();
    assert_empty_value_error!(
        LanChildMdnsAdvertisement::new(input),
        constants::lan_pairing::MDNS_PROTOCOL_VERSION_FIELD
    );

    let mut input = child_advertisement_input();
    input.family_hash.clear();
    assert_empty_value_error!(
        LanChildMdnsAdvertisement::new(input),
        constants::lan_pairing::MDNS_FAMILY_HASH_FIELD
    );

    let mut input = child_advertisement_input();
    input.platform.clear();
    assert_empty_value_error!(
        LanChildMdnsAdvertisement::new(input),
        constants::lan_pairing::MDNS_PLATFORM_FIELD
    );

    let mut input = child_advertisement_input();
    input.agent_version.clear();
    assert_empty_value_error!(
        LanChildMdnsAdvertisement::new(input),
        constants::lan_pairing::MDNS_AGENT_VERSION_FIELD
    );
}

#[test]
fn encoded_packet_contains_service_enumeration_service_type_and_txt_values() {
    let parent = LanParentMdnsAdvertisement::new(
        derive_parent_advertisement_id("sha256:family-parent"),
        constants::lan_pairing::SCHEMA_VERSION_TEXT,
        "sha256:family-parent",
        LanPairingTrustState::Paired,
        LanMdnsAdvertisementLifecycleState::Start,
        LanMdnsAdvertisementSupportState::Supported,
    )
    .value_or_unreachable();
    let child = LanChildMdnsAdvertisement::new(child_advertisement_input()).value_or_unreachable();

    let packet =
        encode_advertisement_packet(&[parent_instance(&parent), child_instance(&child)], 120);

    assert!(packet_contains(&packet, b"_services"));
    assert!(packet_contains(&packet, b"_ocentra-parent"));
    assert!(packet_contains(&packet, b"_ocentra-agent"));
    assert!(packet_contains(&packet, b"protocol-version=v0.9"));
    assert!(packet_contains(&packet, b"pairing-state=paired"));
    assert!(packet_contains(&packet, b"pairing-state=unpaired"));
    assert!(packet_contains(&packet, b"lifecycle-state=start"));
    assert!(packet_contains(&packet, b"lifecycle-state=update"));
    assert!(packet_contains(&packet, b"confirmation-state=hint-only"));
    assert!(packet_contains(
        &packet,
        b"opaque-device-id=opaque-child-id"
    ));
    assert!(packet_contains(&packet, b"platform=windows"));
    assert!(packet_contains(&packet, b"agent-version=1.2.3"));
}

#[test]
fn encoded_parent_and_child_packets_round_trip_through_mdns_discovery_contracts() {
    let parent = LanParentMdnsAdvertisement::new(
        derive_parent_advertisement_id("sha256:family-parent"),
        constants::lan_pairing::SCHEMA_VERSION_TEXT,
        "sha256:family-parent",
        LanPairingTrustState::Paired,
        LanMdnsAdvertisementLifecycleState::Start,
        LanMdnsAdvertisementSupportState::Supported,
    )
    .value_or_unreachable();
    let child = LanChildMdnsAdvertisement::new(child_advertisement_input()).value_or_unreachable();
    let packet =
        encode_advertisement_packet(&[parent_instance(&parent), child_instance(&child)], 120);

    assert!(packet_contains(
        &packet,
        format!(
            "{}={}",
            constants::lan_pairing::MDNS_ADVERTISEMENT_ID_FIELD,
            parent.advertisement_id
        )
        .as_bytes()
    ));
    assert!(packet_contains(
        &packet,
        format!(
            "{}={}",
            constants::lan_pairing::MDNS_ADVERTISEMENT_ID_FIELD,
            child.advertisement_id
        )
        .as_bytes()
    ));

    let discovery = discovery_from_single_packet(&packet).value_or_unreachable();
    let parsed_parent = discovery
        .service_instances
        .iter()
        .find(|instance| {
            instance.service_type == constants::lan_pairing::MDNS_PARENT_SERVICE_TYPE
        })
        .and_then(|instance| instance.parent_advertisement.as_ref())
        .value_or_unreachable();
    let parsed_child = discovery
        .service_instances
        .iter()
        .find(|instance| {
            instance.service_type == constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE
        })
        .and_then(|instance| instance.child_advertisement.as_ref())
        .value_or_unreachable();

    assert_eq!(parsed_parent, &parent);
    assert_eq!(parsed_child, &child);
    assert_eq!(
        parsed_parent.confirmation_state,
        ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementConfirmationState::HintOnly
    );
    assert_eq!(
        parsed_child.confirmation_state,
        ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementConfirmationState::HintOnly
    );
}

#[test]
fn current_platform_support_is_honest_for_the_current_host_family() {
    let support = current_platform_support();

    match std::env::consts::OS {
        "windows" | "linux" | "macos" => assert_eq!(
            support,
            ocentra_lan_core::lan_pairing::LanMdnsAdvertisementPlatformSupport::Supported
        ),
        "android" | "ios" => assert_eq!(
            support,
            ocentra_lan_core::lan_pairing::LanMdnsAdvertisementPlatformSupport::Degraded
        ),
        _ => assert_eq!(
            support,
            ocentra_lan_core::lan_pairing::LanMdnsAdvertisementPlatformSupport::UnsupportedPlatform
        ),
    }
}

#[test]
fn lifecycle_decision_covers_start_update_stop_and_degraded_paths() {
    use ocentra_lan_core::lan_pairing::{
        evaluate_lan_mdns_advertisement_lifecycle, LanMdnsAdvertisementLifecycleAction,
        LanMdnsAdvertisementLifecycleInput, LanMdnsAdvertisementPlatformSupport,
    };

    assert_eq!(
        evaluate_lan_mdns_advertisement_lifecycle(LanMdnsAdvertisementLifecycleInput {
            desired_present: true,
            running: false,
            platform_support: LanMdnsAdvertisementPlatformSupport::Supported,
        })
        .lifecycle_action,
        LanMdnsAdvertisementLifecycleAction::Start
    );
    assert_eq!(
        evaluate_lan_mdns_advertisement_lifecycle(LanMdnsAdvertisementLifecycleInput {
            desired_present: true,
            running: true,
            platform_support: LanMdnsAdvertisementPlatformSupport::Supported,
        })
        .lifecycle_action,
        LanMdnsAdvertisementLifecycleAction::Update
    );
    assert_eq!(
        evaluate_lan_mdns_advertisement_lifecycle(LanMdnsAdvertisementLifecycleInput {
            desired_present: false,
            running: true,
            platform_support: LanMdnsAdvertisementPlatformSupport::Supported,
        })
        .lifecycle_action,
        LanMdnsAdvertisementLifecycleAction::Stop
    );
    assert_eq!(
        evaluate_lan_mdns_advertisement_lifecycle(LanMdnsAdvertisementLifecycleInput {
            desired_present: true,
            running: true,
            platform_support: LanMdnsAdvertisementPlatformSupport::Degraded,
        })
        .lifecycle_action,
        LanMdnsAdvertisementLifecycleAction::Degraded
    );
    assert_eq!(
        evaluate_lan_mdns_advertisement_lifecycle(LanMdnsAdvertisementLifecycleInput {
            desired_present: true,
            running: false,
            platform_support: LanMdnsAdvertisementPlatformSupport::UnsupportedPlatform,
        })
        .lifecycle_action,
        LanMdnsAdvertisementLifecycleAction::Degraded
    );
}

#[test]
fn goodbye_packets_match_the_zero_ttl_advertisement_shape_and_empty_inputs_noop() {
    let advertisement = LanParentMdnsAdvertisement::new(
        derive_parent_advertisement_id("sha256:family-parent"),
        constants::lan_pairing::SCHEMA_VERSION_TEXT,
        "sha256:family-parent",
        LanPairingTrustState::Paired,
        LanMdnsAdvertisementLifecycleState::Start,
        LanMdnsAdvertisementSupportState::Supported,
    )
    .value_or_unreachable();
    let instance = parent_instance(&advertisement);
    let expected = encode_advertisement_packet(std::slice::from_ref(&instance), 0);
    let sink = RecordingMdnsSink::default();

    send_goodbye(std::slice::from_ref(&instance), &sink).value_or_unreachable();
    send_goodbye(&[], &sink).value_or_unreachable();

    assert_eq!(sink.packets(), vec![expected]);
}

#[derive(Default)]
struct RecordingMdnsSink {
    packets: Mutex<Vec<Vec<u8>>>,
}

impl RecordingMdnsSink {
    fn packets(&self) -> Vec<Vec<u8>> {
        self.packets.lock().value_or_unreachable().clone()
    }
}

impl LanMdnsPacketSink for RecordingMdnsSink {
    fn send(&self, packet: &[u8]) -> io::Result<()> {
        self.packets
            .lock()
            .value_or_unreachable()
            .push(packet.to_vec());
        Ok(())
    }
}

fn child_advertisement_input() -> LanChildMdnsAdvertisementInput {
    LanChildMdnsAdvertisementInput {
        advertisement_id: derive_child_advertisement_id("sha256:family-parent", "opaque-child-id"),
        opaque_device_id: "opaque-child-id".to_string(),
        protocol_version: constants::lan_pairing::SCHEMA_VERSION_TEXT.to_string(),
        family_hash: "sha256:family-parent".to_string(),
        platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
        agent_version: "1.2.3".to_string(),
        pairing_state: LanPairingTrustState::Unpaired,
        lifecycle_state: LanMdnsAdvertisementLifecycleState::Update,
        support_state: LanMdnsAdvertisementSupportState::Degraded,
    }
}

fn packet_contains(packet: &[u8], text: &[u8]) -> bool {
    packet.windows(text.len()).any(|window| window == text)
}
