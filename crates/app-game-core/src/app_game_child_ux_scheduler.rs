use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::schema_domain_mirrors::notification::{
    NotificationLocalOutboxDeliveryClaimState, NotificationLocalOutboxSchedulerRecord,
    NotificationLocalOutboxSchedulerState, NotificationLocalOutboxState,
};

use crate::app_game_child_ux_scheduler_types::{
    AppGameChildUxSchedulerInput, AppGameChildUxSchedulerRoute,
};

const INVALID_SOURCE_FIELD: &str = "app_game.child_ux_scheduler.source_record";
const INVALID_CONTEXT_FIELD: &str = "app_game.child_ux_scheduler.context";
const INVALID_RECORD_FIELD: &str = "app_game.child_ux_scheduler.scheduler_record";

pub fn build_app_game_child_ux_scheduler_route(
    input: AppGameChildUxSchedulerInput,
) -> Result<AppGameChildUxSchedulerRoute, EventingError> {
    validate_context(&input)?;
    validate_source(&input)?;
    if input.source_record.state != NotificationLocalOutboxState::QueuedLocal {
        return Ok(AppGameChildUxSchedulerRoute::Blocked {
            source_entry_id: input.source_record.entry_id,
            source_state: input.source_record.state,
        });
    }
    Ok(AppGameChildUxSchedulerRoute::DueLocal(Box::new(
        build_record(input),
    )))
}

pub(crate) fn validate_scheduler_record(
    record: &NotificationLocalOutboxSchedulerRecord,
) -> Result<(), EventingError> {
    let identity_invalid = record.scheduler_entry_id.as_str().trim().is_empty()
        || record.source_entry_id.as_str().trim().is_empty()
        || record.scheduler_decision_ref.as_str().trim().is_empty()
        || record.scheduler_artifact_ref.as_str().trim().is_empty()
        || record.source_outbox_file_ref.as_str().trim().is_empty()
        || record.local_data_path_ref.as_str().trim().is_empty()
        || record.scheduler_now_at.as_str().trim().is_empty()
        || record.scheduler_payload_preview.as_str().trim().is_empty();
    let state_invalid = record.source_state != NotificationLocalOutboxState::QueuedLocal
        || record.scheduler_state != NotificationLocalOutboxSchedulerState::DueLocal
        || record.next_attempt_at.as_ref().map(|value| value.as_str())
            != Some(record.scheduler_now_at.as_str());
    let unsupported_lifecycle = record.quiet_hours_window.is_some()
        || record.retry_window.is_some()
        || record.dead_letter_review_ref.is_some()
        || record.provider_receipt_ref.is_some()
        || !record.manual_proof_requirements.is_empty()
        || record.manual_action_required;
    let unsafe_claim = record.raw_child_evidence_included
        || record.raw_url_or_title_included
        || record.raw_message_text_included
        || record.screenshot_or_report_included
        || record.provider_delivery_attempted
        || record.provider_delivery_observed
        || record.provider_receipt_ingested
        || record.provider_credentials_stored
        || record.cloud_routing_claimed
        || record.parent_notification_ui_claimed
        || record.production_durable_outbox_storage_claimed
        || record.sensitive_provider_metadata_stored;
    if identity_invalid || state_invalid || unsupported_lifecycle || unsafe_claim {
        return Err(invalid_value(
            INVALID_RECORD_FIELD,
            record.scheduler_entry_id.as_str(),
        ));
    }
    Ok(())
}

fn build_record(input: AppGameChildUxSchedulerInput) -> NotificationLocalOutboxSchedulerRecord {
    let source = input.source_record;
    NotificationLocalOutboxSchedulerRecord {
        scheduler_entry_id: input.scheduler_entry_id,
        source_entry_id: source.entry_id,
        source_state: source.state,
        scheduler_state: NotificationLocalOutboxSchedulerState::DueLocal,
        reason_code: source.envelope.reason_code,
        provider_channel: source.envelope.provider_channel,
        severity: source.envelope.severity,
        scheduler_decision_ref: input.scheduler_decision_ref,
        scheduler_artifact_ref: input.scheduler_artifact_ref,
        source_outbox_file_ref: source.outbox_file_ref,
        local_data_path_ref: source.local_data_path_ref,
        scheduler_now_at: input.scheduler_now_at.clone(),
        next_attempt_at: Some(input.scheduler_now_at),
        quiet_hours_window: None,
        retry_window: None,
        dead_letter_review_ref: None,
        provider_receipt_ref: None,
        manual_proof_requirements: Vec::new(),
        manual_action_required: false,
        parent_owned_artifact_written: false,
        raw_child_evidence_included: false,
        raw_url_or_title_included: false,
        raw_message_text_included: false,
        screenshot_or_report_included: false,
        provider_delivery_attempted: false,
        provider_delivery_observed: false,
        provider_receipt_ingested: false,
        provider_credentials_stored: false,
        cloud_routing_claimed: false,
        parent_notification_ui_claimed: false,
        production_durable_outbox_storage_claimed: false,
        sensitive_provider_metadata_stored: false,
        scheduler_payload_preview: input.scheduler_payload_preview,
    }
}

fn validate_context(input: &AppGameChildUxSchedulerInput) -> Result<(), EventingError> {
    let invalid = input.scheduler_entry_id.as_str().trim().is_empty()
        || input.scheduler_decision_ref.as_str().trim().is_empty()
        || input.scheduler_artifact_ref.as_str().trim().is_empty()
        || input.scheduler_now_at.as_str().trim().is_empty()
        || input.scheduler_payload_preview.as_str().trim().is_empty();
    if invalid {
        return Err(invalid_value(
            INVALID_CONTEXT_FIELD,
            input.scheduler_entry_id.as_str(),
        ));
    }
    Ok(())
}

fn validate_source(input: &AppGameChildUxSchedulerInput) -> Result<(), EventingError> {
    let source = &input.source_record;
    let envelope = &source.envelope;
    let unsafe_claim = source.provider_delivery_attempted
        || source.provider_delivery_observed
        || source.provider_receipt_ingested
        || source.provider_credentials_stored
        || source.cloud_routing_claimed
        || source.parent_notification_ui_claimed
        || source.sensitive_provider_metadata_stored
        || envelope.raw_child_evidence_included
        || envelope.raw_url_or_title_included
        || envelope.raw_message_text_included
        || envelope.screenshot_or_report_included
        || !envelope.sensitive_detail_minimized;
    let dishonest_queue = source.state == NotificationLocalOutboxState::QueuedLocal
        && (source.delivery_claim_state
            != NotificationLocalOutboxDeliveryClaimState::LocalOutboxOnly
            || source.manual_action_required
            || source.visible_after_at.is_some()
            || source.retry_attempt_count != 0
            || source.quiet_hours_ref.is_some()
            || source.retry_policy_ref.is_some()
            || source.dead_letter_ref.is_some()
            || source.provider_receipt_ref.is_some()
            || !source.manual_proof_requirements.is_empty());
    let empty_identity = source.entry_id.as_str().trim().is_empty()
        || source.outbox_file_ref.as_str().trim().is_empty()
        || source.local_data_path_ref.as_str().trim().is_empty()
        || envelope.alert_ref.as_str().trim().is_empty()
        || envelope.evidence_refs.is_empty()
        || envelope.policy_refs.is_empty()
        || envelope.audit_refs.is_empty();
    if unsafe_claim || dishonest_queue || empty_identity {
        return Err(invalid_value(
            INVALID_SOURCE_FIELD,
            source.entry_id.as_str(),
        ));
    }
    Ok(())
}

fn invalid_value(field: &'static str, value: &str) -> EventingError {
    EventingError::InvalidValue {
        field,
        value: value.to_owned(),
    }
}
