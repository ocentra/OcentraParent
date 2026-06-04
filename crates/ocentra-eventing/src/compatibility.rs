use serde::{Deserialize, Serialize};

const MATRIX_TITLE: &str = "# Eventing Compatibility Matrix";
const MATRIX_HEADER: &str =
    "| Semantic Id | Source Semantic | Rust Surface | Status | Proof | Note |";
const MATRIX_SEPARATOR: &str = "| --- | --- | --- | --- | --- | --- |";
const CELL_ESCAPE_TARGET: &str = "|";
const CELL_ESCAPE_REPLACEMENT: &str = "\\|";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EventCompatibilityStatus {
    Compatible,
    IntentionalDeviation,
    ManualRequired,
}

impl EventCompatibilityStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Compatible => "compatible",
            Self::IntentionalDeviation => "intentional-deviation",
            Self::ManualRequired => "manual-required",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventCompatibilityEntry {
    pub semantic_id: String,
    pub source_semantic: String,
    pub rust_surface: String,
    pub status: EventCompatibilityStatus,
    pub proof_artifact: String,
    pub compatibility_note: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventCompatibilityMatrix {
    entries: Vec<EventCompatibilityEntry>,
}

impl EventCompatibilityMatrix {
    pub fn ocentra_games_lineage() -> Self {
        let entries = LINEAGE_ROWS
            .iter()
            .map(EventCompatibilityEntry::from_row)
            .collect::<Vec<_>>();
        Self { entries }
    }

    pub fn entries(&self) -> &[EventCompatibilityEntry] {
        &self.entries
    }

    pub fn entry(&self, semantic_id: &str) -> Option<&EventCompatibilityEntry> {
        self.entries
            .iter()
            .find(|entry| entry.semantic_id == semantic_id)
    }

    pub fn compatible_entries(&self) -> Vec<&EventCompatibilityEntry> {
        self.entries_by_status(EventCompatibilityStatus::Compatible)
    }

    pub fn intentional_deviations(&self) -> Vec<&EventCompatibilityEntry> {
        self.entries_by_status(EventCompatibilityStatus::IntentionalDeviation)
    }

    pub fn manual_required_entries(&self) -> Vec<&EventCompatibilityEntry> {
        self.entries_by_status(EventCompatibilityStatus::ManualRequired)
    }

    pub fn render_markdown(&self) -> String {
        let mut markdown = String::from(MATRIX_TITLE);
        markdown.push_str("\n\n");
        markdown.push_str(MATRIX_HEADER);
        markdown.push('\n');
        markdown.push_str(MATRIX_SEPARATOR);
        markdown.push('\n');
        for entry in &self.entries {
            markdown.push_str("| ");
            markdown.push_str(&escape_cell(&entry.semantic_id));
            markdown.push_str(" | ");
            markdown.push_str(&escape_cell(&entry.source_semantic));
            markdown.push_str(" | ");
            markdown.push_str(&escape_cell(&entry.rust_surface));
            markdown.push_str(" | ");
            markdown.push_str(entry.status.as_str());
            markdown.push_str(" | ");
            markdown.push_str(&escape_cell(&entry.proof_artifact));
            markdown.push_str(" | ");
            markdown.push_str(&escape_cell(&entry.compatibility_note));
            markdown.push_str(" |\n");
        }
        markdown
    }

    fn entries_by_status(&self, status: EventCompatibilityStatus) -> Vec<&EventCompatibilityEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.status == status)
            .collect()
    }
}

impl EventCompatibilityEntry {
    fn from_row(row: &CompatibilityRow) -> Self {
        Self {
            semantic_id: String::from(row.semantic_id),
            source_semantic: String::from(row.source_semantic),
            rust_surface: String::from(row.rust_surface),
            status: row.status,
            proof_artifact: String::from(row.proof_artifact),
            compatibility_note: String::from(row.compatibility_note),
        }
    }
}

struct CompatibilityRow {
    semantic_id: &'static str,
    source_semantic: &'static str,
    rust_surface: &'static str,
    status: EventCompatibilityStatus,
    proof_artifact: &'static str,
    compatibility_note: &'static str,
}

const LINEAGE_ROWS: &[CompatibilityRow] = &[
    CompatibilityRow {
        semantic_id: "class-backed-contracts",
        source_semantic: "Class-backed contracts expose canonical static event types",
        rust_surface: "DomainEvent::contract plus EventContractRegistry descriptors",
        status: EventCompatibilityStatus::Compatible,
        proof_artifact: "output/eventing-plan-proof/72-contract-registry/proof-summary.json",
        compatibility_note: "Event type and schema version are validated before registration",
    },
    CompatibilityRow {
        semantic_id: "event-args-metadata",
        source_semantic: "EventArgsBase carries unique id, timestamp, and target handler",
        rust_surface: "EventMetadata and EventEnvelope carry event id, observed_at, and target",
        status: EventCompatibilityStatus::Compatible,
        proof_artifact: "test-results/eventing-network-runtime-proof/proof.json",
        compatibility_note: "Metadata is part of the typed envelope and stored boundary",
    },
    CompatibilityRow {
        semantic_id: "target-handler-routing",
        source_semantic: "Target handler can constrain delivery to one subscriber",
        rust_surface: "EventMetadata.target_handler filters EventSubscriber target handlers",
        status: EventCompatibilityStatus::Compatible,
        proof_artifact: "test-results/eventing-network-runtime-proof/proof.json",
        compatibility_note: "Wrong-target subscribers are reported as not invoked",
    },
    CompatibilityRow {
        semantic_id: "subscribe-unsubscribe",
        source_semantic: "EventBus subscribe, subscribeAsync, and unsubscribe",
        rust_surface: "EventBus::subscribe, subscribe_with_handle, and SubscriptionHandle",
        status: EventCompatibilityStatus::Compatible,
        proof_artifact: "output/eventing-plan-proof/14-24-runtime-lifecycle/proof-summary.json",
        compatibility_note: "Async handlers are first-class and unsubscribe is idempotent",
    },
    CompatibilityRow {
        semantic_id: "sync-and-async-publish",
        source_semantic: "EventBus publish and publishAsync",
        rust_surface: "publish, publish_and_wait, and publish_detached",
        status: EventCompatibilityStatus::Compatible,
        proof_artifact: "output/eventing-plan-proof/14-24-runtime-lifecycle/proof-summary.json",
        compatibility_note: "Detached publish returns an observable join report",
    },
    CompatibilityRow {
        semantic_id: "registrar-dispose",
        source_semantic: "EventRegistrar owns scoped subscriptions and dispose",
        rust_surface: "EventRegistrar subscribe, dispose, Drop cleanup, and disposed guard",
        status: EventCompatibilityStatus::Compatible,
        proof_artifact: "output/eventing-plan-proof/14-24-runtime-lifecycle/proof-summary.json",
        compatibility_note: "Dispose removes owned subscriptions and rejects new ones",
    },
    CompatibilityRow {
        semantic_id: "operation-result-deferred",
        source_semantic: "OperationResult and OperationDeferred request/response flow",
        rust_surface: "RequestEvent::Response, EventResponseContract, and RequestRegistry",
        status: EventCompatibilityStatus::Compatible,
        proof_artifact: "output/eventing-plan-proof/31-35-request-response/proof-summary.json",
        compatibility_note: "Local completion is validated and separated from durable results",
    },
    CompatibilityRow {
        semantic_id: "queue-retry-timeout",
        source_semantic: "Queueing, retry, TTL, max queue, and timeout semantics",
        rust_surface: "EventQueuePolicy, HandlerExecutionPolicy, and ManualEventClock",
        status: EventCompatibilityStatus::Compatible,
        proof_artifact: "output/eventing-plan-proof/71-manual-clock/proof-summary.json",
        compatibility_note: "Deterministic manual clock proof avoids long wall-clock sleeps",
    },
    CompatibilityRow {
        semantic_id: "in-flight-duplicate-guard",
        source_semantic: "In-flight duplicate guard prevents repeated work",
        rust_surface: "IdempotencyKey queue and in-flight duplicate registry",
        status: EventCompatibilityStatus::Compatible,
        proof_artifact: "output/eventing-plan-proof/25-30-queue-policy/proof-summary.json",
        compatibility_note: "Concurrent duplicate publish rejects while the first is active",
    },
    CompatibilityRow {
        semantic_id: "isolated-test-bus",
        source_semantic: "Isolated test bus and clear lifecycle",
        rust_surface: "EventBus::new, ManualEventClock, EventRecorder, and clear_for_test",
        status: EventCompatibilityStatus::Compatible,
        proof_artifact: "output/eventing-plan-proof/74-lifecycle-clear/proof-summary.json",
        compatibility_note: "Test clear is explicit and does not create a production singleton",
    },
    CompatibilityRow {
        semantic_id: "payload-republish-override",
        source_semantic: "Payload-carried republish or force override",
        rust_surface: "Explicit idempotency rejection; constrained override remains unclaimed",
        status: EventCompatibilityStatus::IntentionalDeviation,
        proof_artifact: "output/eventing-plan-proof/73-duplicate-subscriber/proof-summary.json",
        compatibility_note: "Future override must be a typed policy with reason and report",
    },
    CompatibilityRow {
        semantic_id: "payload-disposal-callbacks",
        source_semantic: "Event payload disposal callbacks or resource handles",
        rust_surface: "Immutable payload facts; local handles stay in registries",
        status: EventCompatibilityStatus::IntentionalDeviation,
        proof_artifact: "output/eventing-plan-proof/66-76-source-safety/proof-summary.json",
        compatibility_note: "Payloads cannot carry deferred, cancellation, or cleanup handles",
    },
    CompatibilityRow {
        semantic_id: "broker-backed-delivery",
        source_semantic: "Cross-process or broker-backed event delivery",
        rust_surface: "Stored envelope transport boundary is not yet broker-backed",
        status: EventCompatibilityStatus::ManualRequired,
        proof_artifact: "docs/plans/network-plan/workpacks/README.md#workpack-45",
        compatibility_note: "Broker delivery is P6 and cannot redefine local dispatch semantics",
    },
];

fn escape_cell(value: &str) -> String {
    value.replace(CELL_ESCAPE_TARGET, CELL_ESCAPE_REPLACEMENT)
}
