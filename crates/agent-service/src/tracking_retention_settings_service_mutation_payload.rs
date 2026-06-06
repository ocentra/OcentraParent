use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LogFieldValue, LogFields,
    LogLevel, TrackingRetentionSettingsMutationRequest, TrackingRetentionSettingsMutationResult,
    TRACKING_RETENTION_SETTINGS_MUTATION_REJECTION_INVALID_REQUEST,
    TRACKING_RETENTION_SETTINGS_MUTATION_STATE_ACCEPTED,
    TRACKING_RETENTION_SETTINGS_MUTATION_STATE_REJECTED,
};

use crate::{event_builder::build_event, fields::fields_from_pairs};

pub async fn build_tracking_retention_settings_mutation_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let payload = tracking_retention_settings_mutation_payload(&command);
    build_event(
        constants::event_id::ACTIVITY_TRACKING_RETENTION_SETTINGS_MUTATION_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentActivityTrackingRetentionSettingsMutationReported,
        LogLevel::Info,
        payload,
        None,
    )
}

fn tracking_retention_settings_mutation_payload(command: &AgentCommandEnvelope) -> LogFields {
    fields_from_pairs(vec![(
        constants::field::ACTIVITY_TRACKING_RETENTION_SETTINGS_MUTATION,
        LogFieldValue::String(mutation_result_json(&mutation_result(command))),
    )])
}

fn mutation_result(command: &AgentCommandEnvelope) -> TrackingRetentionSettingsMutationResult {
    match parse_request(command) {
        Some(request) if valid_request(&request) => accepted_result(request),
        _ => rejected_result(command),
    }
}

fn parse_request(
    command: &AgentCommandEnvelope,
) -> Option<TrackingRetentionSettingsMutationRequest> {
    match command
        .payload
        .get(constants::field::ACTIVITY_TRACKING_RETENTION_SETTINGS_MUTATION)
    {
        Some(LogFieldValue::String(value)) => serde_json::from_str(value).ok(),
        _ => None,
    }
}

fn valid_request(request: &TrackingRetentionSettingsMutationRequest) -> bool {
    !request.request_id.trim().is_empty()
        && !request.intent_id.trim().is_empty()
        && !request.settings_kind.trim().is_empty()
        && !request.write_action.trim().is_empty()
        && !request.requested_value.trim().is_empty()
        && !request.evidence_reference_ids.is_empty()
        && !request.source_read_model_proof_refs.is_empty()
        && !request.writer_boundary_proof_refs.is_empty()
        && !request.audit_refs.is_empty()
}

fn accepted_result(
    request: TrackingRetentionSettingsMutationRequest,
) -> TrackingRetentionSettingsMutationResult {
    TrackingRetentionSettingsMutationResult {
        request_id: request.request_id.clone(),
        mutation_id: mutation_id(&request.request_id),
        intent_id: request.intent_id,
        settings_kind: request.settings_kind,
        write_action: request.write_action,
        requested_value: request.requested_value,
        mutation_state: TRACKING_RETENTION_SETTINGS_MUTATION_STATE_ACCEPTED.to_string(),
        rejection_reason: None,
        service_mutation_executed: true,
        durable_persistence_claimed: false,
        portal_ui_claimed: false,
        platform_runtime_claimed: false,
        child_device_delivery_claimed: false,
        provider_delivery_claimed: false,
        notification_receipt_claimed: false,
        physical_device_claimed: false,
        authority_claimed: false,
        product_claim_ready: false,
        evidence_reference_ids: request.evidence_reference_ids,
        source_read_model_proof_refs: request.source_read_model_proof_refs,
        writer_boundary_proof_refs: request.writer_boundary_proof_refs,
        audit_refs: request.audit_refs,
    }
}

fn rejected_result(command: &AgentCommandEnvelope) -> TrackingRetentionSettingsMutationResult {
    TrackingRetentionSettingsMutationResult {
        request_id: command.message_id.clone(),
        mutation_id: mutation_id(&command.message_id),
        intent_id: constants::event_id::COMMAND_REJECTED.to_string(),
        settings_kind: constants::activity_subject_kind::RETENTION.to_string(),
        write_action: constants::event_id::COMMAND_REJECTED.to_string(),
        requested_value: constants::event_id::COMMAND_REJECTED.to_string(),
        mutation_state: TRACKING_RETENTION_SETTINGS_MUTATION_STATE_REJECTED.to_string(),
        rejection_reason: Some(
            TRACKING_RETENTION_SETTINGS_MUTATION_REJECTION_INVALID_REQUEST.to_string(),
        ),
        service_mutation_executed: false,
        durable_persistence_claimed: false,
        portal_ui_claimed: false,
        platform_runtime_claimed: false,
        child_device_delivery_claimed: false,
        provider_delivery_claimed: false,
        notification_receipt_claimed: false,
        physical_device_claimed: false,
        authority_claimed: false,
        product_claim_ready: false,
        evidence_reference_ids: Vec::new(),
        source_read_model_proof_refs: Vec::new(),
        writer_boundary_proof_refs: Vec::new(),
        audit_refs: Vec::new(),
    }
}

fn mutation_id(request_id: &str) -> String {
    let mut id = String::from(request_id);
    id.push(constants::delimiter::HYPHEN);
    id.push_str(constants::field::EVENT_REF);
    id
}

fn mutation_result_json(result: &TrackingRetentionSettingsMutationResult) -> String {
    serde_json::to_string(result).expect(constants::error::AGENT_EVENT_SERIALIZES)
}
