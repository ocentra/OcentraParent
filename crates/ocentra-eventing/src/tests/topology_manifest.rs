use crate::{
    tests::fixtures::{test_event_for_type, OTHER_EVENT_TYPE, TEST_EVENT_TYPE},
    EventContractRegistry, EventNamespace, EventTopologyFamilyVariant, EventTopologyManifest,
    EventTopologyPublisher, EventTopologyStatus, EventTopologySubscriber, EventType,
    SourceComponent, SubscriberId, TargetHandler,
};

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
    let accepted_event =
        EventType::parse(ACCEPTED_NO_PUBLISHER_EVENT_TYPE).expect("accepted event type parses");

    let manifest = EventTopologyManifest::from_registry(
        &registry,
        &[
            publisher(TEST_EVENT_TYPE, COVERED_PUBLISHER),
            publisher(NO_SUBSCRIBER_EVENT_TYPE, ORPHAN_PUBLISHER),
        ],
        &[
            subscriber(TEST_EVENT_TYPE, COVERED_SUBSCRIBER),
            subscriber(ACCEPTED_NO_PUBLISHER_EVENT_TYPE, ACCEPTED_SUBSCRIBER),
        ],
        &[
            family_variant(TEST_EVENT_TYPE),
            family_variant(NO_SUBSCRIBER_EVENT_TYPE),
        ],
        &[accepted_event],
    );

    assert_eq!(
        entry(&manifest, TEST_EVENT_TYPE).status,
        EventTopologyStatus::Covered
    );
    assert_eq!(
        entry(&manifest, NO_SUBSCRIBER_EVENT_TYPE).status,
        EventTopologyStatus::NoSubscriber
    );
    assert_eq!(
        entry(&manifest, OTHER_EVENT_TYPE).status,
        EventTopologyStatus::NoPublisher
    );
    assert_eq!(
        entry(&manifest, ACCEPTED_NO_PUBLISHER_EVENT_TYPE).status,
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
        &[publisher(NO_SUBSCRIBER_EVENT_TYPE, ORPHAN_PUBLISHER)],
        &[],
        &[
            family_variant(TEST_EVENT_TYPE),
            family_variant(NO_SUBSCRIBER_EVENT_TYPE),
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
        entry(&manifest, NO_SUBSCRIBER_EVENT_TYPE).families[0].as_str(),
        FAMILY_ID
    );
}

#[test]
fn topology_manifest_renders_deterministic_markdown() {
    let registry = topology_registry();
    let manifest = EventTopologyManifest::from_registry(
        &registry,
        &[publisher(TEST_EVENT_TYPE, COVERED_PUBLISHER)],
        &[subscriber(TEST_EVENT_TYPE, COVERED_SUBSCRIBER)],
        &[family_variant(TEST_EVENT_TYPE)],
        &[],
    );

    let markdown = manifest.render_markdown();
    assert!(markdown.starts_with("# Event Topology Manifest"));
    assert!(markdown.contains("| Event Type | Schema Version | Publishers | Subscribers | Families | Status | Rust Type |"));
    assert!(markdown.contains("| eventing.test.observed | 1 | covered-publisher | covered-subscriber -> topology-target | eventing.topology.family | covered |"));
    assert!(markdown.contains("| eventing.test.other | 1 | none | none | none | no-publisher |"));
}

fn topology_registry() -> EventContractRegistry {
    let mut registry = EventContractRegistry::new();
    registry
        .register_event(&test_event_for_type("covered", TEST_EVENT_TYPE))
        .expect("covered registers");
    registry
        .register_event(&test_event_for_type("other", OTHER_EVENT_TYPE))
        .expect("other registers");
    registry
        .register_event(&test_event_for_type(
            "accepted",
            ACCEPTED_NO_PUBLISHER_EVENT_TYPE,
        ))
        .expect("accepted registers");
    registry
        .register_event(&test_event_for_type("orphan", NO_SUBSCRIBER_EVENT_TYPE))
        .expect("orphan registers");
    registry
}

fn publisher(event_type: &str, component: &str) -> EventTopologyPublisher {
    EventTopologyPublisher {
        event_type: EventType::parse(event_type).expect("publisher event type parses"),
        source_component: SourceComponent::parse(component).expect("publisher component parses"),
    }
}

fn subscriber(event_type: &str, subscriber_id: &str) -> EventTopologySubscriber {
    EventTopologySubscriber {
        event_type: EventType::parse(event_type).expect("subscriber event type parses"),
        subscriber_id: SubscriberId::parse(subscriber_id).expect("subscriber id parses"),
        target_handler: TargetHandler::parse(TOPOLOGY_TARGET).expect("target parses"),
    }
}

fn family_variant(event_type: &str) -> EventTopologyFamilyVariant {
    EventTopologyFamilyVariant {
        family: EventNamespace::parse(FAMILY_ID).expect("family parses"),
        event_type: EventType::parse(event_type).expect("family event type parses"),
    }
}

fn entry<'a>(
    manifest: &'a EventTopologyManifest,
    event_type: &str,
) -> &'a crate::EventTopologyEntry {
    manifest
        .entries()
        .iter()
        .find(|entry| entry.contract.event_type.as_str() == event_type)
        .expect("topology entry exists")
}
