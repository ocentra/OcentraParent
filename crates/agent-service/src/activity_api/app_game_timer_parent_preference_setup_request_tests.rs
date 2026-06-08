use std::fs::remove_file;

use ocentra_parent_agent_core::ActivityStore;
use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget,
    AgentPeer, AgentPeerRole, AgentRoute, AppGameTimerParentPreferenceSetupRequest,
    AppGameTimerParentPreferenceSetupRequestResult, LogFieldValue, LogFields,
    AGENT_PROTOCOL_SCHEMA_VERSION, APP_GAME_CONTROL_ACTION_STATUS_MANUAL_REQUIRED,
    APP_GAME_CONTROL_PERSISTENCE_REPLAYABLE,
};

use crate::{
    activity_api::app_game_timer_parent_preference_setup_request::build_activity_app_game_timer_parent_preference_setup_request_report_for_store_path,
    lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test,
};

#[tokio::test]
async fn app_game_timer_parent_preference_setup_request_command_returns_accepted_boundary_result() {
    let body =
        serde_json::to_string(&command_envelope()).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
    let result = request_payload(
        &event.payload[constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST],
    );

    assert_eq!(
        event.event,
        AgentEventName::AgentActivityAppGameTimerParentPreferenceSetupRequested
    );
    assert_eq!(
        result.schema_version,
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_REQUEST_SCHEMA_VERSION
    );
    assert_eq!(
        result.request_status,
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_REQUEST_ACCEPTED
    );
    assert_eq!(
        result.action_result_reference_id,
        constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX
    );
    assert_eq!(
        result.action_result_reference_ids,
        vec![
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string(),
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX.to_string()
        ]
    );
    assert!(result.command_boundary_claimed);
    assert!(result.action_result_handoff_claimed);
    assert!(
        result.action_result_persistence_status
            == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_PERSISTED
            || result.action_result_persistence_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_UNAVAILABLE
    );
    assert_eq!(
        result.parent_preference_mutation_receipt_id,
        parent_preference_mutation_receipt_id()
    );
    assert_eq!(
        result.parent_preference_mutation_receipt_ids,
        vec![
            parent_preference_mutation_receipt_id(),
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string(),
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX.to_string()
        ]
    );
    assert!(
        result.parent_preference_mutation_receipt_status
            == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_PERSISTED
            || result.parent_preference_mutation_receipt_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_UNAVAILABLE
    );
    assert_child_runtime_delivery_handoff_boundary(&result);
    assert_child_runtime_delivery_queue_boundary(&result);
    assert_child_runtime_delivery_dispatch_boundary(&result);
    assert_child_runtime_delivery_receipt_requirement_boundary(&result);
    assert_no_delivery_or_platform_claims(&result);
}

#[tokio::test]
async fn app_game_timer_parent_preference_setup_request_persists_action_result_row() {
    let store_path = temp_path(constants::activity_store::TEST_STORE_SUFFIX);
    cleanup_path(&store_path);

    let event =
        build_activity_app_game_timer_parent_preference_setup_request_report_for_store_path(
            command_envelope(),
            store_path.clone(),
        )
        .await;
    let result = request_payload(
        &event.payload[constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST],
    );

    let store = ActivityStore::open(&store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
    let model = store
        .app_game_service_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    let status = store
        .status()
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    cleanup_path(&store_path);

    assert_persisted_setup_result(&result);
    assert_eq!(status.events_stored, 6);
    assert_persisted_action_result_model(&model);
}

fn assert_persisted_setup_result(result: &AppGameTimerParentPreferenceSetupRequestResult) {
    assert_eq!(
        result.action_result_persistence_status,
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_PERSISTED
    );
    assert!(result.action_result_persistence_claimed);
    assert_eq!(
        result.parent_preference_mutation_receipt_id,
        parent_preference_mutation_receipt_id()
    );
    assert_eq!(
        result.parent_preference_mutation_receipt_status,
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_PERSISTED
    );
    assert!(result.parent_preference_mutation_receipt_claimed);
    assert_eq!(
        result.child_runtime_delivery_handoff_id,
        child_runtime_delivery_handoff_id()
    );
    assert_eq!(
        result.child_runtime_delivery_handoff_status,
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_READY
    );
    assert!(result.child_runtime_delivery_handoff_claimed);
    assert_eq!(
        result.child_runtime_delivery_queue_id,
        child_runtime_delivery_queue_id()
    );
    assert_eq!(
        result.child_runtime_delivery_queue_status,
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_QUEUED
    );
    assert!(result.child_runtime_delivery_queue_claimed);
    assert_eq!(
        result.child_runtime_delivery_dispatch_id,
        child_runtime_delivery_dispatch_id()
    );
    assert_eq!(
        result.child_runtime_delivery_dispatch_status,
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_READY
    );
    assert!(result.child_runtime_delivery_dispatch_claimed);
    assert_eq!(
        result.child_runtime_delivery_receipt_requirement_id,
        child_runtime_delivery_receipt_requirement_id()
    );
    assert_eq!(
        result.child_runtime_delivery_receipt_requirement_status,
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_REQUIRED
    );
    assert!(result.child_runtime_delivery_receipt_requirement_claimed);
    assert_no_delivery_or_platform_claims(result);
}

fn assert_persisted_action_result_model(
    model: &ocentra_parent_agent_protocol::AppGameServiceReadModel,
) {
    assert_eq!(model.approval_action_result_returned, 1);
    assert_eq!(
        model.approval_action_result_rows[0].result_id,
        constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX
    );
    assert_eq!(
        model.approval_action_result_rows[0].result_status,
        APP_GAME_CONTROL_ACTION_STATUS_MANUAL_REQUIRED
    );
    assert_eq!(
        model.approval_action_result_rows[0]
            .decision
            .persistence_state,
        APP_GAME_CONTROL_PERSISTENCE_REPLAYABLE
    );
    assert!(model.approval_action_result_rows[0]
        .enforcement_result
        .is_none());
}

fn assert_child_runtime_delivery_handoff_boundary(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) {
    assert_eq!(
        result.child_runtime_delivery_handoff_id,
        child_runtime_delivery_handoff_id()
    );
    assert_eq!(
        result.child_runtime_delivery_handoff_ids,
        vec![
            child_runtime_delivery_handoff_id(),
            parent_preference_mutation_receipt_id(),
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string(),
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX.to_string(),
        ]
    );
    assert!(
        result.child_runtime_delivery_handoff_status
            == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_READY
            || result.child_runtime_delivery_handoff_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_UNAVAILABLE
    );
}

fn assert_child_runtime_delivery_queue_boundary(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) {
    assert_eq!(
        result.child_runtime_delivery_queue_id,
        child_runtime_delivery_queue_id()
    );
    assert_eq!(
        result.child_runtime_delivery_queue_ids,
        vec![
            child_runtime_delivery_queue_id(),
            child_runtime_delivery_handoff_id(),
            parent_preference_mutation_receipt_id(),
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string(),
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX.to_string(),
        ]
    );
    assert!(
        result.child_runtime_delivery_queue_status
            == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_QUEUED
            || result.child_runtime_delivery_queue_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_UNAVAILABLE
    );
}

fn assert_child_runtime_delivery_dispatch_boundary(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) {
    assert_eq!(
        result.child_runtime_delivery_dispatch_id,
        child_runtime_delivery_dispatch_id()
    );
    assert_eq!(
        result.child_runtime_delivery_dispatch_ids,
        vec![
            child_runtime_delivery_dispatch_id(),
            child_runtime_delivery_queue_id(),
            child_runtime_delivery_handoff_id(),
            parent_preference_mutation_receipt_id(),
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string(),
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX.to_string(),
        ]
    );
    assert!(
        result.child_runtime_delivery_dispatch_status
            == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_READY
            || result.child_runtime_delivery_dispatch_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_UNAVAILABLE
    );
}

fn assert_child_runtime_delivery_receipt_requirement_boundary(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) {
    assert_eq!(
        result.child_runtime_delivery_receipt_requirement_id,
        child_runtime_delivery_receipt_requirement_id()
    );
    assert_eq!(
        result.child_runtime_delivery_receipt_requirement_ids,
        vec![
            child_runtime_delivery_receipt_requirement_id(),
            child_runtime_delivery_dispatch_id(),
            child_runtime_delivery_queue_id(),
            child_runtime_delivery_handoff_id(),
            parent_preference_mutation_receipt_id(),
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string(),
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX.to_string(),
        ]
    );
    assert!(
        result.child_runtime_delivery_receipt_requirement_status
            == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_REQUIRED
            || result.child_runtime_delivery_receipt_requirement_status
                == constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_UNAVAILABLE
    );
}

fn assert_no_delivery_or_platform_claims(result: &AppGameTimerParentPreferenceSetupRequestResult) {
    assert!(!result.parent_preference_mutation_claimed);
    assert!(!result.notification_rule_mutation_claimed);
    assert!(!result.provider_delivery_claimed);
    assert!(!result.provider_receipt_ingestion_claimed);
    assert!(!result.child_runtime_delivery_claimed);
    assert!(!result.durable_outbox_claimed);
    assert!(!result.adapter_dispatch_claimed);
    assert!(!result.broad_blocking_claimed);
    assert!(!result.platform_enforcement_claimed);
    assert!(!result.raw_private_source_rows_claimed);
    assert!(!result.raw_target_values_claimed);
    assert!(!result.private_diagnostics_claimed);
}

fn parent_preference_mutation_receipt_id() -> String {
    let mut receipt_id =
        constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string();
    receipt_id.push(constants::delimiter::HYPHEN);
    receipt_id.push_str(constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_SUFFIX);
    receipt_id
}

fn child_runtime_delivery_handoff_id() -> String {
    let mut handoff_id =
        constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string();
    handoff_id.push(constants::delimiter::HYPHEN);
    handoff_id.push_str(
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_SUFFIX,
    );
    handoff_id
}

fn child_runtime_delivery_queue_id() -> String {
    let mut queue_id =
        constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string();
    queue_id.push(constants::delimiter::HYPHEN);
    queue_id.push_str(
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_SUFFIX,
    );
    queue_id
}

fn child_runtime_delivery_dispatch_id() -> String {
    let mut dispatch_id =
        constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string();
    dispatch_id.push(constants::delimiter::HYPHEN);
    dispatch_id.push_str(
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_SUFFIX,
    );
    dispatch_id
}

fn child_runtime_delivery_receipt_requirement_id() -> String {
    let mut receipt_requirement_id =
        constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string();
    receipt_requirement_id.push(constants::delimiter::HYPHEN);
    receipt_requirement_id.push_str(
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_REQUIREMENT_SUFFIX,
    );
    receipt_requirement_id
}

fn command_envelope() -> AgentCommandEnvelope {
    let request = AppGameTimerParentPreferenceSetupRequest {
        request_id: constants::event_id::ACTIVITY_APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUESTED
            .to_string(),
        requested_at: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT
            .to_string(),
        parent_surface_intent_reference_id:
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX.to_string(),
        parent_preference_setup_reference_id:
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string(),
        request_reference_ids: vec![
            constants::value::APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX.to_string(),
            constants::value::APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX.to_string(),
        ],
    };
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST.to_string(),
        LogFieldValue::String(
            serde_json::to_string(&request).expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
    );
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::ACTIVITY_APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUESTED
            .to_string(),
        sent_at: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform:
                ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
                    .to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentActivityAppGameTimerParentPreferenceSetupRequest,
        payload,
    }
}

fn request_payload(value: &LogFieldValue) -> AppGameTimerParentPreferenceSetupRequestResult {
    match value {
        LogFieldValue::String(text) => {
            serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}

fn temp_path(suffix: &str) -> std::path::PathBuf {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix);
    name.push(constants::delimiter::HYPHEN);
    name.push_str(constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST);

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::activity_store::FILE_EXTENSION);
    path
}

fn cleanup_path(path: &std::path::PathBuf) {
    let _ = remove_file(path);
    let mut wal_path = path.clone();
    wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(wal_path);
    let mut shm_path = path.clone();
    shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(shm_path);
}
