use super::support::{test_event_for_type, TestText, OTHER_EVENT_TYPE, TEST_EVENT_TYPE};
use ocentra_eventing::contract_registry::EventContractRegistry;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{
    EventNamespace, EventType, SourceComponent, SubscriberId, TargetHandler,
};
use ocentra_eventing::topology::{
    EventTopologyEntry, EventTopologyFamilyVariant, EventTopologyManifest, EventTopologyPublisher,
    EventTopologyStatus, EventTopologySubscriber,
};
use serde_json::Value;

const NO_SUBSCRIBER_EVENT_TYPE: &str = "eventing.topology.no_subscriber";
const ACCEPTED_NO_PUBLISHER_EVENT_TYPE: &str = "eventing.topology.accepted_no_publisher";
const COVERED_PUBLISHER: &str = "covered-publisher";
const ORPHAN_PUBLISHER: &str = "orphan-publisher";
const COVERED_SUBSCRIBER: &str = "covered-subscriber";
const ACCEPTED_SUBSCRIBER: &str = "accepted-subscriber";
const TOPOLOGY_TARGET: &str = "topology-target";
const FAMILY_ID: &str = "eventing.topology.family";

#[test]
fn topology_manifest_classifies_covered_orphan_and_accepted_states() {
    let registry = topology_registry();
    let accepted_event = EventType::parse(ACCEPTED_NO_PUBLISHER_EVENT_TYPE)
        .expect_value("accepted event type parses");

    let manifest = EventTopologyManifest::from_registry(
        &registry,
        &[
            publisher(
                TestText(TEST_EVENT_TYPE.to_owned()),
                TestText(COVERED_PUBLISHER.to_owned()),
            ),
            publisher(
                TestText(NO_SUBSCRIBER_EVENT_TYPE.to_owned()),
                TestText(ORPHAN_PUBLISHER.to_owned()),
            ),
        ],
        &[
            subscriber(
                TestText(TEST_EVENT_TYPE.to_owned()),
                TestText(COVERED_SUBSCRIBER.to_owned()),
            ),
            subscriber(
                TestText(ACCEPTED_NO_PUBLISHER_EVENT_TYPE.to_owned()),
                TestText(ACCEPTED_SUBSCRIBER.to_owned()),
            ),
        ],
        &[
            family_variant(TestText(TEST_EVENT_TYPE.to_owned())),
            family_variant(TestText(NO_SUBSCRIBER_EVENT_TYPE.to_owned())),
        ],
        &[accepted_event],
    );

    assert_eq!(
        entry(&manifest, TestText(TEST_EVENT_TYPE.to_owned())).status,
        EventTopologyStatus::Covered
    );
    assert_eq!(
        entry(&manifest, TestText(NO_SUBSCRIBER_EVENT_TYPE.to_owned())).status,
        EventTopologyStatus::NoSubscriber
    );
    assert_eq!(
        entry(&manifest, TestText(OTHER_EVENT_TYPE.to_owned())).status,
        EventTopologyStatus::NoPublisher
    );
    assert_eq!(
        entry(
            &manifest,
            TestText(ACCEPTED_NO_PUBLISHER_EVENT_TYPE.to_owned())
        )
        .status,
        EventTopologyStatus::AcceptedOneSided
    );
    assert_eq!(
        manifest
            .unready_entries()
            .iter()
            .map(|entry| entry.contract.event_type.as_str())
            .collect::<Vec<_>>(),
        vec![OTHER_EVENT_TYPE, NO_SUBSCRIBER_EVENT_TYPE]
    );
}

#[test]
fn topology_manifest_records_family_variants_and_sorted_descriptors() {
    let registry = topology_registry();
    let manifest = EventTopologyManifest::from_registry(
        &registry,
        &[publisher(
            TestText(NO_SUBSCRIBER_EVENT_TYPE.to_owned()),
            TestText(ORPHAN_PUBLISHER.to_owned()),
        )],
        &[],
        &[
            family_variant(TestText(TEST_EVENT_TYPE.to_owned())),
            family_variant(TestText(NO_SUBSCRIBER_EVENT_TYPE.to_owned())),
        ],
        &[],
    );

    let event_types = manifest
        .entries()
        .iter()
        .map(|entry| entry.contract.event_type.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        event_types,
        vec![
            TEST_EVENT_TYPE,
            OTHER_EVENT_TYPE,
            ACCEPTED_NO_PUBLISHER_EVENT_TYPE,
            NO_SUBSCRIBER_EVENT_TYPE
        ]
    );
    assert_eq!(
        entry(&manifest, TestText(NO_SUBSCRIBER_EVENT_TYPE.to_owned())).families[0].as_str(),
        FAMILY_ID
    );
}

#[test]
fn topology_manifest_renders_deterministic_markdown() {
    let registry = topology_registry();
    let manifest = EventTopologyManifest::from_registry(
        &registry,
        &[publisher(
            TestText(TEST_EVENT_TYPE.to_owned()),
            TestText(COVERED_PUBLISHER.to_owned()),
        )],
        &[subscriber(
            TestText(TEST_EVENT_TYPE.to_owned()),
            TestText(COVERED_SUBSCRIBER.to_owned()),
        )],
        &[family_variant(TestText(TEST_EVENT_TYPE.to_owned()))],
        &[],
    );

    let markdown = manifest.render_markdown();
    let lines = markdown.lines().collect::<Vec<_>>();
    assert_eq!(lines.first().copied(), Some("# Event Topology Manifest"));
    assert!(lines.iter().any(|line| {
        *line == "| Event Type | Schema Version | Publishers | Subscribers | Families | Status | Rust Type |"
    }));
    assert!(lines.iter().any(|line| {
        *line
            == "| eventing.test.observed | 1 | covered-publisher | covered-subscriber -> topology-target | eventing.topology.family | covered | contract::support::TestEvent |"
    }));
    assert!(lines.iter().any(|line| {
        *line
            == "| eventing.test.other | 1 | none | none | none | no-publisher | contract::support::TestEvent |"
    }));
}

#[test]
fn topology_manifest_serializes_canonical_eventing_entry_keys() {
    let registry = topology_registry();
    let manifest = EventTopologyManifest::from_registry(
        &registry,
        &[publisher(
            TestText(TEST_EVENT_TYPE.to_owned()),
            TestText(COVERED_PUBLISHER.to_owned()),
        )],
        &[subscriber(
            TestText(TEST_EVENT_TYPE.to_owned()),
            TestText(COVERED_SUBSCRIBER.to_owned()),
        )],
        &[family_variant(TestText(TEST_EVENT_TYPE.to_owned()))],
        &[],
    );

    let manifest_json = serde_json::to_value(&manifest).expect_value("manifest serializes");
    let entry = manifest_entry(&manifest_json, TestText(TEST_EVENT_TYPE.to_owned()))
        .as_object()
        .expect_value("manifest entry");
    let subscriber_target = entry["subscribers"][0]
        .as_object()
        .expect_value("subscriber target object");

    assert_eq!(entry["contract"]["eventType"], Value::from(TEST_EVENT_TYPE));
    assert_eq!(entry["contract"]["schemaVersion"], Value::from(1));
    assert_eq!(
        entry["rustType"],
        Value::from("contract::support::TestEvent")
    );
    assert!(entry.get("rust_type").is_none());
    assert_eq!(
        subscriber_target.get("subscriberId"),
        Some(&Value::from(COVERED_SUBSCRIBER))
    );
    assert_eq!(
        subscriber_target.get("targetHandler"),
        Some(&Value::from(TOPOLOGY_TARGET))
    );
}

fn topology_registry() -> EventContractRegistry {
    let mut registry = EventContractRegistry::new();
    registry
        .register_event(&test_event_for_type(
            TestText("covered".to_owned()),
            TestText(TEST_EVENT_TYPE.to_owned()),
        ))
        .expect_value("covered registers");
    registry
        .register_event(&test_event_for_type(
            TestText("other".to_owned()),
            TestText(OTHER_EVENT_TYPE.to_owned()),
        ))
        .expect_value("other registers");
    registry
        .register_event(&test_event_for_type(
            TestText("accepted".to_owned()),
            TestText(ACCEPTED_NO_PUBLISHER_EVENT_TYPE.to_owned()),
        ))
        .expect_value("accepted registers");
    registry
        .register_event(&test_event_for_type(
            TestText("orphan".to_owned()),
            TestText(NO_SUBSCRIBER_EVENT_TYPE.to_owned()),
        ))
        .expect_value("orphan registers");
    registry
}

fn publisher(event_type: TestText, component: TestText) -> EventTopologyPublisher {
    EventTopologyPublisher {
        event_type: EventType::parse(event_type.0).expect_value("publisher event type parses"),
        source_component: SourceComponent::parse(component.0)
            .expect_value("publisher component parses"),
    }
}

fn subscriber(event_type: TestText, subscriber_id: TestText) -> EventTopologySubscriber {
    EventTopologySubscriber {
        event_type: EventType::parse(event_type.0).expect_value("subscriber event type parses"),
        subscriber_id: SubscriberId::parse(subscriber_id.0).expect_value("subscriber id parses"),
        target_handler: TargetHandler::parse(TOPOLOGY_TARGET).expect_value("target parses"),
    }
}

fn family_variant(event_type: TestText) -> EventTopologyFamilyVariant {
    EventTopologyFamilyVariant {
        family: EventNamespace::parse(FAMILY_ID).expect_value("family parses"),
        event_type: EventType::parse(event_type.0).expect_value("family event type parses"),
    }
}

fn entry(manifest: &EventTopologyManifest, event_type: TestText) -> &EventTopologyEntry {
    let event_type = event_type.0;
    manifest
        .entries()
        .iter()
        .find(|entry| entry.contract.event_type.as_str() == event_type)
        .expect_value("topology entry exists")
}

fn manifest_entry(manifest_json: &Value, event_type: TestText) -> &Value {
    let event_type = event_type.0;
    manifest_json["entries"]
        .as_array()
        .expect_value("entries array")
        .iter()
        .find(|entry| entry["contract"]["eventType"] == event_type)
        .expect_value("manifest entry exists")
}
