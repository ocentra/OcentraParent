use ocentra_app_core::runtime_decision::{
    app_runtime_decision_recorded_event, AppCapabilityState, AppClassificationState,
    AppForegroundState, AppRuntimeInput, APP_RUNTIME_DECISION_RECORDED_EVENT_TYPE,
    APP_RUNTIME_DECISION_SCHEMA_VERSION,
};
use ocentra_app_core::runtime_ids::{
    AppAggregateId, AppRuntimeDecisionId, APP_AGGREGATE_ID_PREFIX, APP_RUNTIME_DECISION_ID_PREFIX,
};
use ocentra_app_core::AppObservationIntent;
use ocentra_eventing::envelope::DomainEvent;
use ocentra_eventing::error::EventingError;

const APP_RUNTIME_DECISION_GOLDEN: &str = include_str!(
    "../../../../packages/schema-domain/tests/fixtures/app-runtime-decision-recorded-event.json"
);

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

    let actual_golden = serde_json::json!({
        "event_type": contract.event_type.as_str(),
        "schema_version": contract.schema_version.value(),
        "payload": recorded,
    });
    let expected_golden_result =
        serde_json::from_str::<serde_json::Value>(APP_RUNTIME_DECISION_GOLDEN);
    assert!(
        expected_golden_result.is_ok(),
        "shared app runtime golden parses: {expected_golden_result:?}"
    );
    let Ok(expected_golden) = expected_golden_result else {
        return;
    };
    assert_eq!(actual_golden, expected_golden);
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
}
