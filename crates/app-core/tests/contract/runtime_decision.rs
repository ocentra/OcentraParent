use ocentra_app_core::runtime_decision::{
    app_runtime_decision_recorded_event, AppCapabilityState, AppClassificationState,
    AppForegroundState, AppRuntimeDecisionRecordedEvent, AppRuntimeInput,
    APP_RUNTIME_DECISION_RECORDED_EVENT_TYPE, APP_RUNTIME_DECISION_SCHEMA_VERSION,
};
use ocentra_app_core::runtime_ids::{
    AppAggregateId, AppRuntimeDecisionId, APP_AGGREGATE_ID_PREFIX, APP_RUNTIME_DECISION_ID_PREFIX,
};
use ocentra_app_core::AppObservationIntent;
use ocentra_eventing::envelope::{DomainEvent, EventEnvelope, EventMetadata, EventSource};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{
    CorrelationId, EventCustody, EventId, RecordedAt, RuntimeInstanceId, RuntimeRole,
    SourceComponent, SourceService,
};
use serde::Deserialize;

const APP_RUNTIME_DECISION_CONTRACTS: &str =
    include_str!("fixtures/app-runtime-decision-contracts.json");
const APP_RUNTIME_DECISION_EVENT_ENVELOPE: &str =
    include_str!("fixtures/app-runtime-decision-event-envelope.json");

#[derive(Deserialize)]
struct RuntimeDecisionContracts {
    event_type: String,
    current_schema_version: u16,
    current_decisions: Vec<RuntimeDecisionContractCase>,
}

#[derive(Deserialize)]
struct RuntimeDecisionContractCase {
    input: AppRuntimeInput,
    decision: ocentra_app_core::runtime_decision::AppRuntimeDecision,
}

#[test]
fn background_inventory_state_records_decision_event_with_typed_contract() {
    let input = AppRuntimeInput {
        capability_state: AppCapabilityState::Supported,
        foreground_state: AppForegroundState::Background,
        classification_state: AppClassificationState::InventoryOnly,
    };
    let aggregate_id_result = AppAggregateId::parse("app.aggregate.child-device-1");
    assert!(
        aggregate_id_result.is_ok(),
        "aggregate id parses: {aggregate_id_result:?}"
    );
    let Ok(aggregate_id) = aggregate_id_result else {
        return;
    };
    let decision_id_result = AppRuntimeDecisionId::parse("app.runtime-decision-1");
    assert!(
        decision_id_result.is_ok(),
        "decision id parses: {decision_id_result:?}"
    );
    let Ok(decision_id) = decision_id_result else {
        return;
    };

    let recorded = app_runtime_decision_recorded_event(aggregate_id, decision_id, input);

    assert_eq!(
        recorded.decision.observation_intent,
        AppObservationIntent::InventoryObservationOnly
    );
    assert_eq!(
        recorded.decision.runtime_action_state,
        ocentra_app_core::runtime_decision::AppRuntimeActionState::RecordInventory
    );
    let contract_result = recorded.contract();
    assert!(
        contract_result.is_ok(),
        "app runtime contract parses: {contract_result:?}"
    );
    let Ok(contract) = contract_result else {
        return;
    };
    assert_eq!(
        contract.event_type.as_str(),
        APP_RUNTIME_DECISION_RECORDED_EVENT_TYPE
    );
    assert_eq!(
        contract.schema_version.value(),
        APP_RUNTIME_DECISION_SCHEMA_VERSION
    );

    assert_eq!(
        recorded.decision,
        ocentra_app_core::runtime_decision::evaluate_app_runtime(input)
    );
}

#[test]
fn current_rust_contract_matrix_exhaustively_matches_runtime_evaluation() {
    let contracts_result =
        serde_json::from_str::<RuntimeDecisionContracts>(APP_RUNTIME_DECISION_CONTRACTS);
    assert!(
        contracts_result.is_ok(),
        "Rust-owned app runtime contract matrix parses"
    );
    let Ok(contracts) = contracts_result else {
        return;
    };
    assert_eq!(
        contracts.event_type,
        APP_RUNTIME_DECISION_RECORDED_EVENT_TYPE
    );
    assert_eq!(
        contracts.current_schema_version,
        APP_RUNTIME_DECISION_SCHEMA_VERSION
    );
    assert_eq!(contracts.current_decisions.len(), 18);
    for case in contracts.current_decisions {
        assert_eq!(
            ocentra_app_core::runtime_decision::evaluate_app_runtime(case.input),
            case.decision
        );
    }
}

#[test]
fn rust_event_envelope_serializes_the_edge_decoder_contract_shape() {
    let input = AppRuntimeInput {
        capability_state: AppCapabilityState::Supported,
        foreground_state: AppForegroundState::Foreground,
        classification_state: AppClassificationState::InventoryOnly,
    };
    let event = app_runtime_decision_recorded_event(
        AppAggregateId::parse("app.aggregate.child-device-1").expect("aggregate id parses"),
        AppRuntimeDecisionId::parse("app.runtime-decision-1").expect("decision id parses"),
        input,
    );
    let metadata = EventMetadata::from_parts(
        EventId::parse("event-app-runtime-decision-1").expect("event id parses"),
        CorrelationId::parse("correlation-app-runtime-decision-1").expect("correlation id parses"),
        EventSource::new(
            EventCustody::parse("local-only").expect("custody parses"),
            RuntimeRole::parse("parent").expect("role parses"),
            SourceService::parse("app-core").expect("service parses"),
            SourceComponent::parse("runtime-decision").expect("component parses"),
            RuntimeInstanceId::parse("app-core-test").expect("instance id parses"),
        ),
        RecordedAt::parse("2026-07-23T00:00:00Z").expect("observed at parses"),
        None,
    );
    let envelope = EventEnvelope::from_event(event, metadata).expect("event envelope builds");
    let serialized = serde_json::to_value(envelope).expect("event envelope serializes");
    let expected = serde_json::from_str::<serde_json::Value>(APP_RUNTIME_DECISION_EVENT_ENVELOPE)
        .expect("Rust-owned event envelope fixture parses");
    assert_eq!(serialized, expected);
}

#[test]
fn serde_rejects_unknown_fields_inside_the_runtime_contract() {
    let input = AppRuntimeInput {
        capability_state: AppCapabilityState::Supported,
        foreground_state: AppForegroundState::Background,
        classification_state: AppClassificationState::InventoryOnly,
    };
    let event = app_runtime_decision_recorded_event(
        AppAggregateId::parse("app.aggregate.child-device-1").expect("aggregate id parses"),
        AppRuntimeDecisionId::parse("app.runtime-decision-1").expect("decision id parses"),
        input,
    );
    let mut payload = serde_json::to_value(event).expect("event serializes");
    payload["input"]["display_name"] = serde_json::json!("Chat Client");

    let decoded = serde_json::from_value::<AppRuntimeDecisionRecordedEvent>(payload);
    assert!(decoded.is_err(), "unknown nested fields must be rejected");
}

#[test]
fn serde_rejects_decisions_that_do_not_match_the_observed_runtime_input() {
    let input = AppRuntimeInput {
        capability_state: AppCapabilityState::Supported,
        foreground_state: AppForegroundState::Background,
        classification_state: AppClassificationState::InventoryOnly,
    };
    let event = app_runtime_decision_recorded_event(
        AppAggregateId::parse("app.aggregate.child-device-1").expect("aggregate id parses"),
        AppRuntimeDecisionId::parse("app.runtime-decision-1").expect("decision id parses"),
        input,
    );
    let mut payload = serde_json::to_value(event).expect("event serializes");
    payload["decision"]["policy_handoff_state"] = serde_json::json!("publish");

    let decoded = serde_json::from_value::<AppRuntimeDecisionRecordedEvent>(payload);
    assert!(
        decoded.is_err(),
        "a caller-supplied decision cannot override the runtime matrix"
    );
}

#[test]
fn serde_rejects_the_legacy_inventory_only_decision_without_a_version_gate() {
    let input = AppRuntimeInput {
        capability_state: AppCapabilityState::Supported,
        foreground_state: AppForegroundState::Foreground,
        classification_state: AppClassificationState::InventoryOnly,
    };
    let event = app_runtime_decision_recorded_event(
        AppAggregateId::parse("app.aggregate.child-device-1").expect("aggregate id parses"),
        AppRuntimeDecisionId::parse("app.runtime-decision-1").expect("decision id parses"),
        input,
    );
    let mut payload = serde_json::to_value(event).expect("event serializes");
    payload["decision"]["runtime_action_state"] = serde_json::json!("record-foreground");

    let decoded = serde_json::from_value::<AppRuntimeDecisionRecordedEvent>(payload);
    assert!(
        decoded.is_err(),
        "the stale v1 inventory-only decision cannot pass as a current event"
    );
}

#[test]
fn runtime_ids_reject_noncanonical_or_empty_suffixes() {
    let invalid_aggregate = AppAggregateId::parse("child-device-1");
    assert!(matches!(
        invalid_aggregate,
        Err(EventingError::InvalidValue {
            field: "app.aggregate_id",
            ..
        })
    ));

    let empty_aggregate_suffix = AppAggregateId::parse(APP_AGGREGATE_ID_PREFIX);
    assert!(matches!(
        empty_aggregate_suffix,
        Err(EventingError::InvalidValue {
            field: "app.aggregate_id",
            ..
        })
    ));

    let invalid_decision = AppRuntimeDecisionId::parse("decision-1");
    assert!(matches!(
        invalid_decision,
        Err(EventingError::InvalidValue {
            field: "app.runtime_decision_id",
            ..
        })
    ));

    let empty_decision_suffix = AppRuntimeDecisionId::parse(APP_RUNTIME_DECISION_ID_PREFIX);
    assert!(matches!(
        empty_decision_suffix,
        Err(EventingError::InvalidValue {
            field: "app.runtime_decision_id",
            ..
        })
    ));

    let display_name_suffix = AppAggregateId::parse("app.aggregate.Chat Client");
    assert!(matches!(
        display_name_suffix,
        Err(EventingError::InvalidValue {
            field: "app.aggregate_id",
            ..
        })
    ));
}

#[test]
fn schema_v1_runtime_decision_payload_deserializes_legacy_opaque_ids() {
    let input = AppRuntimeInput {
        capability_state: AppCapabilityState::Supported,
        foreground_state: AppForegroundState::Background,
        classification_state: AppClassificationState::InventoryOnly,
    };
    let current = app_runtime_decision_recorded_event(
        AppAggregateId::parse("app.aggregate.child-device-1").expect("aggregate id parses"),
        AppRuntimeDecisionId::parse("app.runtime-decision-1").expect("decision id parses"),
        input,
    );
    let mut legacy_payload = serde_json::to_value(current).expect("event serializes");
    legacy_payload["aggregate_id"] = serde_json::json!("child-device-1");
    legacy_payload["decision_id"] = serde_json::json!("decision-1");

    let decoded = serde_json::from_value::<AppRuntimeDecisionRecordedEvent>(legacy_payload)
        .expect("schema-v1 runtime decision payload deserializes");

    assert_eq!(decoded.aggregate_id.as_str(), "child-device-1");
    assert_eq!(decoded.decision_id.as_str(), "decision-1");
    assert_eq!(decoded.input, input);
}
