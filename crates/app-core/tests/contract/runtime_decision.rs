use ocentra_app_core::runtime_decision::{
    app_runtime_decision_recorded_event, AppCapabilityState, AppClassificationState,
    AppForegroundState, AppRuntimeInput, APP_RUNTIME_DECISION_RECORDED_EVENT_TYPE,
    APP_RUNTIME_DECISION_SCHEMA_VERSION,
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

fn fixture_event_source() -> Result<EventSource, EventingError> {
    Ok(EventSource::new(
        EventCustody::parse("local-only")?,
        RuntimeRole::parse("parent")?,
        SourceService::parse("app-core")?,
        SourceComponent::parse("runtime-decision")?,
        RuntimeInstanceId::parse("app-core-test")?,
    ))
}

#[test]
fn rust_event_envelope_serializes_the_edge_decoder_contract_shape() {
    let input = AppRuntimeInput {
        capability_state: AppCapabilityState::Supported,
        foreground_state: AppForegroundState::Foreground,
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
    let event = app_runtime_decision_recorded_event(aggregate_id, decision_id, input);
    let event_id_result = EventId::parse("event-app-runtime-decision-1");
    assert!(
        event_id_result.is_ok(),
        "event id parses: {event_id_result:?}"
    );
    let Ok(event_id) = event_id_result else {
        return;
    };
    let correlation_id_result = CorrelationId::parse("correlation-app-runtime-decision-1");
    assert!(
        correlation_id_result.is_ok(),
        "correlation id parses: {correlation_id_result:?}"
    );
    let Ok(correlation_id) = correlation_id_result else {
        return;
    };
    let observed_at_result = RecordedAt::parse("2026-07-23T00:00:00Z");
    assert!(
        observed_at_result.is_ok(),
        "observed at parses: {observed_at_result:?}"
    );
    let Ok(observed_at) = observed_at_result else {
        return;
    };
    let source_result = fixture_event_source();
    assert!(
        source_result.is_ok(),
        "fixture source parses: {source_result:?}"
    );
    let Ok(source) = source_result else {
        return;
    };
    let metadata = EventMetadata::from_parts(event_id, correlation_id, source, observed_at, None);
    let envelope_result = EventEnvelope::from_event(event, metadata);
    assert!(
        envelope_result.is_ok(),
        "event envelope builds: {envelope_result:?}"
    );
    let Ok(envelope) = envelope_result else {
        return;
    };
    let serialized_result = serde_json::to_value(envelope);
    assert!(
        serialized_result.is_ok(),
        "event envelope serializes: {serialized_result:?}"
    );
    let Ok(serialized) = serialized_result else {
        return;
    };
    let expected_result =
        serde_json::from_str::<serde_json::Value>(APP_RUNTIME_DECISION_EVENT_ENVELOPE);
    assert!(
        expected_result.is_ok(),
        "Rust-owned event envelope fixture parses: {expected_result:?}"
    );
    let Ok(expected) = expected_result else {
        return;
    };
    assert_eq!(serialized, expected);
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
