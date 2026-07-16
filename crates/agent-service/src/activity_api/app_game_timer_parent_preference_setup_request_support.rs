#[path = "app_game_timer_parent_preference_setup_request_support/refs.rs"]
mod refs;

use self::refs::{setup_request_refs, SetupRequestRefs};
use super::AppGameTimerSetupStorePath;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};
use ocentra_parent_agent_protocol::AppGameTimerParentPreferenceSetupRequest;
use ocentra_parent_agent_protocol::AppGameTimerParentPreferenceSetupRequestResult;

use super::super::app_game_timer_parent_preference_setup_request_persistence::persist_setup_handoff;
use super::super::app_game_timer_parent_preference_setup_request_status::apply_persisted_setup_statuses;

use crate::{event_builder::build_event, fields::fields_from_pairs, time::timestamp_now};

struct AppGameTimerSetupFieldPairs(Vec<(&'static str, LogFieldValue)>);

struct SetupResultJson(String);

pub(super) async fn build_activity_app_game_timer_parent_preference_setup_request_report_for_store_path(
    command: AgentCommandEnvelope,
    store_path: AppGameTimerSetupStorePath,
) -> AgentEventEnvelope {
    let mut result = app_game_timer_parent_preference_setup_request_from_command(&command);
    if persist_setup_handoff(&command, &result, store_path).await {
        apply_persisted_setup_statuses(&mut result);
    }
    build_event(
        constants::event_id::ACTIVITY_APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUESTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentActivityAppGameTimerParentPreferenceSetupRequested,
        LogLevel::Info,
        app_game_timer_parent_preference_setup_request_payload(&result),
        None,
    )
}

pub(super) fn app_game_timer_parent_preference_setup_request_from_command(
    command: &AgentCommandEnvelope,
) -> AppGameTimerParentPreferenceSetupRequestResult {
    let request = request_from_command(command);
    let refs = setup_request_refs(&request);
    setup_request_result(request, refs)
}

macro_rules! setup_request_result_defaults {
    () => {
        AppGameTimerParentPreferenceSetupRequestResult {
            schema_version: String::new(),
            request_id: String::new(),
            requested_at: String::new(),
            accepted_at: String::new(),
            request_status: String::new(),
            parent_surface_intent_reference_id: String::new(),
            parent_preference_setup_reference_id: String::new(),
            request_reference_ids: Vec::new(),
            action_result_reference_id: String::new(),
            action_result_reference_ids: Vec::new(),
            action_result_persistence_status: String::new(),
            parent_preference_mutation_receipt_id: String::new(),
            parent_preference_mutation_receipt_ids: Vec::new(),
            parent_preference_mutation_receipt_status: String::new(),
            parent_preference_mutation_receipt_claimed: false,
            child_runtime_delivery_handoff_id: String::new(),
            child_runtime_delivery_handoff_ids: Vec::new(),
            child_runtime_delivery_handoff_status: String::new(),
            child_runtime_delivery_handoff_claimed: false,
            child_runtime_delivery_queue_id: String::new(),
            child_runtime_delivery_queue_ids: Vec::new(),
            child_runtime_delivery_queue_status: String::new(),
            child_runtime_delivery_queue_claimed: false,
            child_runtime_delivery_dispatch_id: String::new(),
            child_runtime_delivery_dispatch_ids: Vec::new(),
            child_runtime_delivery_dispatch_status: String::new(),
            child_runtime_delivery_dispatch_claimed: false,
            child_runtime_delivery_receipt_requirement_id: String::new(),
            child_runtime_delivery_receipt_requirement_ids: Vec::new(),
            child_runtime_delivery_receipt_requirement_status: String::new(),
            child_runtime_delivery_receipt_requirement_claimed: false,
            child_runtime_delivery_receipt_pending_id: String::new(),
            child_runtime_delivery_receipt_pending_ids: Vec::new(),
            child_runtime_delivery_receipt_pending_status: String::new(),
            child_runtime_delivery_receipt_pending_claimed: false,
            child_runtime_delivery_receipt_ingested_id: String::new(),
            child_runtime_delivery_receipt_ingested_ids: Vec::new(),
            child_runtime_delivery_receipt_ingested_status: String::new(),
            child_runtime_delivery_receipt_ingested_claimed: false,
            durable_outbox_record_id: String::new(),
            durable_outbox_record_ids: Vec::new(),
            durable_outbox_status: String::new(),
            provider_delivery_readiness_id: String::new(),
            provider_delivery_readiness_ids: Vec::new(),
            provider_delivery_readiness_status: String::new(),
            provider_delivery_readiness_claimed: false,
            provider_delivery_attempt_id: String::new(),
            provider_delivery_attempt_ids: Vec::new(),
            provider_delivery_attempt_status: String::new(),
            provider_delivery_attempt_claimed: false,
            provider_delivery_adapter_requirement_id: String::new(),
            provider_delivery_adapter_requirement_ids: Vec::new(),
            provider_delivery_adapter_requirement_status: String::new(),
            provider_delivery_adapter_requirement_claimed: false,
            provider_delivery_credential_requirement_id: String::new(),
            provider_delivery_credential_requirement_ids: Vec::new(),
            provider_delivery_credential_requirement_status: String::new(),
            provider_delivery_credential_requirement_claimed: false,
            provider_delivery_queue_id: String::new(),
            provider_delivery_queue_ids: Vec::new(),
            provider_delivery_queue_status: String::new(),
            provider_delivery_queue_claimed: false,
            provider_delivery_receipt_requirement_id: String::new(),
            provider_delivery_receipt_requirement_ids: Vec::new(),
            provider_delivery_receipt_requirement_status: String::new(),
            provider_delivery_receipt_requirement_claimed: false,
            provider_delivery_receipt_pending_id: String::new(),
            provider_delivery_receipt_pending_ids: Vec::new(),
            provider_delivery_receipt_pending_status: String::new(),
            provider_delivery_receipt_pending_claimed: false,
            provider_delivery_receipt_ingested_id: String::new(),
            provider_delivery_receipt_ingested_ids: Vec::new(),
            provider_delivery_receipt_ingested_status: String::new(),
            provider_delivery_receipt_ingested_claimed: false,
            command_boundary_claimed: false,
            action_result_handoff_claimed: false,
            action_result_persistence_claimed: false,
            parent_preference_mutation_claimed: false,
            notification_rule_mutation_claimed: false,
            provider_delivery_claimed: false,
            provider_receipt_ingestion_claimed: false,
            child_runtime_delivery_claimed: false,
            durable_outbox_claimed: false,
            adapter_dispatch_claimed: false,
            broad_blocking_claimed: false,
            platform_enforcement_claimed: false,
            raw_private_source_rows_claimed: false,
            raw_target_values_claimed: false,
            private_diagnostics_claimed: false,
        }
    };
}

fn setup_request_result(
    request: AppGameTimerParentPreferenceSetupRequest,
    refs: SetupRequestRefs,
) -> AppGameTimerParentPreferenceSetupRequestResult {
    let parent_preference_setup_reference_id = request.parent_preference_setup_reference_id.clone();
    let unavailable = constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_UNAVAILABLE;
    let schema_version = constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_REQUEST_SCHEMA_VERSION;
    let request_status = constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_REQUEST_ACCEPTED;
    let setup_value = |value: &str| value.to_string();
    AppGameTimerParentPreferenceSetupRequestResult {
        schema_version: setup_value(schema_version),
        request_id: request.request_id,
        requested_at: request.requested_at,
        accepted_at: timestamp_now(),
        request_status: setup_value(request_status),
        parent_surface_intent_reference_id: request.parent_surface_intent_reference_id,
        parent_preference_setup_reference_id: parent_preference_setup_reference_id.clone(),
        request_reference_ids: request.request_reference_ids,
        action_result_reference_id: parent_preference_setup_reference_id,
        action_result_reference_ids: refs.action_result_reference_ids,
        action_result_persistence_status: setup_value(unavailable),
        parent_preference_mutation_receipt_id: refs.parent_preference_mutation_receipt_id,
        parent_preference_mutation_receipt_ids: refs.parent_preference_mutation_receipt_ids,
        parent_preference_mutation_receipt_status: setup_value(unavailable),
        child_runtime_delivery_handoff_id: refs.child_runtime_delivery_handoff_id,
        child_runtime_delivery_handoff_ids: refs.child_runtime_delivery_handoff_ids,
        child_runtime_delivery_handoff_status: setup_value(unavailable),
        child_runtime_delivery_queue_id: refs.child_runtime_delivery_queue_id,
        child_runtime_delivery_queue_ids: refs.child_runtime_delivery_queue_ids,
        child_runtime_delivery_queue_status: setup_value(unavailable),
        child_runtime_delivery_dispatch_id: refs.child_runtime_delivery_dispatch_id,
        child_runtime_delivery_dispatch_ids: refs.child_runtime_delivery_dispatch_ids,
        child_runtime_delivery_dispatch_status: setup_value(unavailable),
        child_runtime_delivery_receipt_requirement_id: refs
            .child_runtime_delivery_receipt_requirement_id,
        child_runtime_delivery_receipt_requirement_ids: refs
            .child_runtime_delivery_receipt_requirement_ids,
        child_runtime_delivery_receipt_requirement_status: setup_value(unavailable),
        child_runtime_delivery_receipt_pending_id: refs.child_runtime_delivery_receipt_pending_id,
        child_runtime_delivery_receipt_pending_ids: refs.child_runtime_delivery_receipt_pending_ids,
        child_runtime_delivery_receipt_pending_status: setup_value(unavailable),
        child_runtime_delivery_receipt_ingested_id: refs.child_runtime_delivery_receipt_ingested_id,
        child_runtime_delivery_receipt_ingested_ids: refs
            .child_runtime_delivery_receipt_ingested_ids,
        child_runtime_delivery_receipt_ingested_status: setup_value(unavailable),
        durable_outbox_record_id: refs.durable_outbox_record_id,
        durable_outbox_record_ids: refs.durable_outbox_record_ids,
        durable_outbox_status: setup_value(unavailable),
        provider_delivery_readiness_id: refs.provider_delivery_readiness_id,
        provider_delivery_readiness_ids: refs.provider_delivery_readiness_ids,
        provider_delivery_readiness_status: setup_value(unavailable),
        provider_delivery_attempt_id: refs.provider_delivery_attempt_id,
        provider_delivery_attempt_ids: refs.provider_delivery_attempt_ids,
        provider_delivery_attempt_status: setup_value(unavailable),
        provider_delivery_adapter_requirement_id: refs.provider_delivery_adapter_requirement_id,
        provider_delivery_adapter_requirement_ids: refs.provider_delivery_adapter_requirement_ids,
        provider_delivery_adapter_requirement_status: setup_value(unavailable),
        provider_delivery_credential_requirement_id: refs
            .provider_delivery_credential_requirement_id,
        provider_delivery_credential_requirement_ids: refs
            .provider_delivery_credential_requirement_ids,
        provider_delivery_credential_requirement_status: setup_value(unavailable),
        provider_delivery_queue_id: refs.provider_delivery_queue_id,
        provider_delivery_queue_ids: refs.provider_delivery_queue_ids,
        provider_delivery_queue_status: setup_value(unavailable),
        provider_delivery_receipt_requirement_id: refs.provider_delivery_receipt_requirement_id,
        provider_delivery_receipt_requirement_ids: refs.provider_delivery_receipt_requirement_ids,
        provider_delivery_receipt_requirement_status: setup_value(unavailable),
        provider_delivery_receipt_pending_id: refs.provider_delivery_receipt_pending_id,
        provider_delivery_receipt_pending_ids: refs.provider_delivery_receipt_pending_ids,
        provider_delivery_receipt_pending_status: setup_value(unavailable),
        provider_delivery_receipt_ingested_id: refs.provider_delivery_receipt_ingested_id,
        provider_delivery_receipt_ingested_ids: refs.provider_delivery_receipt_ingested_ids,
        provider_delivery_receipt_ingested_status: setup_value(unavailable),
        command_boundary_claimed: true,
        action_result_handoff_claimed: true,
        ..setup_request_result_defaults!()
    }
}

pub(super) fn app_game_timer_parent_preference_setup_request_payload(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> LogFields {
    fields_from_pairs(result_pairs(result).0)
}

fn result_pairs(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> AppGameTimerSetupFieldPairs {
    AppGameTimerSetupFieldPairs(vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(result.accepted_at.clone()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(result.request_status.clone()),
        ),
        (
            constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST,
            LogFieldValue::String(serialized_setup_request_result(result).0),
        ),
    ])
}

fn request_from_command(
    command: &AgentCommandEnvelope,
) -> AppGameTimerParentPreferenceSetupRequest {
    match command
        .payload
        .get(constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST)
    {
        Some(LogFieldValue::String(value)) if !value.is_empty() => {
            match serde_json::from_str(value) {
                Ok(request) => request,
                Err(_error) => default_request_from_command(command),
            }
        }
        _ => default_request_from_command(command),
    }
}

fn default_request_from_command(
    command: &AgentCommandEnvelope,
) -> AppGameTimerParentPreferenceSetupRequest {
    AppGameTimerParentPreferenceSetupRequest {
        request_id: command.message_id.clone(),
        requested_at: command.sent_at.clone(),
        parent_surface_intent_reference_id: command.message_id.clone(),
        parent_preference_setup_reference_id: command.message_id.clone(),
        request_reference_ids: vec![command.message_id.clone()],
    }
}

fn serialized_setup_request_result(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> SetupResultJson {
    match serde_json::to_string(result) {
        Ok(json) => SetupResultJson(json),
        Err(_error) => SetupResultJson(constants::value::EMPTY.to_string()),
    }
}
