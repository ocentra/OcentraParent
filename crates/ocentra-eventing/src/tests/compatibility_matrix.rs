use crate::{EventCompatibilityMatrix, EventCompatibilityStatus};

const CLASS_CONTRACTS: &str = "class-backed-contracts";
const EVENT_METADATA: &str = "event-args-metadata";
const TARGET_ROUTING: &str = "target-handler-routing";
const SUBSCRIBE_UNSUBSCRIBE: &str = "subscribe-unsubscribe";
const SYNC_ASYNC_PUBLISH: &str = "sync-and-async-publish";
const REGISTRAR_DISPOSE: &str = "registrar-dispose";
const OPERATION_DEFERRED: &str = "operation-result-deferred";
const QUEUE_RETRY_TIMEOUT: &str = "queue-retry-timeout";
const DUPLICATE_GUARD: &str = "in-flight-duplicate-guard";
const ISOLATED_TEST_BUS: &str = "isolated-test-bus";
const REPUBLISH_OVERRIDE: &str = "payload-republish-override";
const DISPOSAL_CALLBACKS: &str = "payload-disposal-callbacks";
const BROKER_DELIVERY: &str = "broker-backed-delivery";

#[test]
fn compatibility_matrix_covers_games_lineage_semantics() {
    let matrix = EventCompatibilityMatrix::ocentra_games_lineage();
    let semantic_ids = matrix
        .entries()
        .iter()
        .map(|entry| entry.semantic_id())
        .collect::<Vec<_>>();

    assert_eq!(
        semantic_ids,
        vec![
            CLASS_CONTRACTS,
            EVENT_METADATA,
            TARGET_ROUTING,
            SUBSCRIBE_UNSUBSCRIBE,
            SYNC_ASYNC_PUBLISH,
            REGISTRAR_DISPOSE,
            OPERATION_DEFERRED,
            QUEUE_RETRY_TIMEOUT,
            DUPLICATE_GUARD,
            ISOLATED_TEST_BUS,
            REPUBLISH_OVERRIDE,
            DISPOSAL_CALLBACKS,
            BROKER_DELIVERY,
        ]
    );
    assert_eq!(matrix.compatible_entries().len(), 9);
    assert_eq!(matrix.intentional_deviations().len(), 3);
    assert_eq!(matrix.manual_required_entries().len(), 1);
}

#[test]
fn compatibility_matrix_marks_deviations_and_manual_required_scope() {
    let matrix = EventCompatibilityMatrix::ocentra_games_lineage();

    assert_eq!(
        matrix
            .entry(REPUBLISH_OVERRIDE)
            .expect("republish entry")
            .status(),
        EventCompatibilityStatus::IntentionalDeviation
    );
    assert_eq!(
        matrix
            .entry(DISPOSAL_CALLBACKS)
            .expect("disposal entry")
            .status(),
        EventCompatibilityStatus::IntentionalDeviation
    );
    assert_eq!(
        matrix
            .entry(BROKER_DELIVERY)
            .expect("broker entry")
            .status(),
        EventCompatibilityStatus::ManualRequired
    );
    for entry in matrix.entries() {
        assert!(!entry.source_semantic().is_empty());
        assert!(!entry.rust_surface().is_empty());
        assert!(!entry.proof_artifact().is_empty());
        assert!(!entry.compatibility_note().is_empty());
    }
}

#[test]
fn compatibility_matrix_renders_deterministic_markdown() {
    let matrix = EventCompatibilityMatrix::ocentra_games_lineage();
    let markdown = matrix.render_markdown();

    assert!(markdown.starts_with("# Eventing Compatibility Matrix"));
    assert!(markdown
        .contains("| Semantic Id | Source Semantic | Rust Surface | Status | Proof | Note |"));
    assert!(markdown.contains("| class-backed-contracts | Class-backed contracts expose canonical static event types | Payload-derived DomainEvent::contract plus EventContractRegistry descriptors | intentional-deviation |"));
    assert!(markdown.contains("| payload-republish-override | Payload-carried republish or force override | Explicit idempotency rejection; constrained override remains unclaimed | intentional-deviation |"));
    assert!(markdown.contains("| broker-backed-delivery | Cross-process or broker-backed event delivery | Stored envelope transport boundary is not yet broker-backed | manual-required |"));
}
