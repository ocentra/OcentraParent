use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::schema_domain_mirrors::notification::NotificationLocalOutboxSchedulerState;

use crate::app_game_child_ux_preference_preflight_types::{
    AppGameChildUxPreferencePreflightInput, AppGameChildUxPreferencePreflightRow,
    AppGameChildUxPreferencePreflightStatus,
};

const INVALID_SOURCE_FIELD: &str = "app_game.child_ux_preference_preflight.source";
const INVALID_REQUIREMENTS_FIELD: &str = "app_game.child_ux_preference_preflight.requirements";

pub fn build_app_game_child_ux_preference_preflight(
    input: AppGameChildUxPreferencePreflightInput,
) -> Result<AppGameChildUxPreferencePreflightRow, EventingError> {
    validate_requirements(&input)?;
    validate_sources(&input)?;
    Ok(build_row(input))
}

fn build_row(
    input: AppGameChildUxPreferencePreflightInput,
) -> AppGameChildUxPreferencePreflightRow {
    let status = status_for(&input.scheduler_record.scheduler_state);
    let preference_required =
        status == AppGameChildUxPreferencePreflightStatus::ParentPreferenceRequired;
    let parent_preference_requirement_ref = input.parent_preference_requirement_ref;
    let notification_frequency_requirement_ref = input.notification_frequency_requirement_ref;
    let quiet_hours_requirement_ref = input.quiet_hours_requirement_ref;
    let manual_proof_requirements = vec![
        parent_preference_requirement_ref.clone(),
        notification_frequency_requirement_ref.clone(),
        quiet_hours_requirement_ref.clone(),
    ];
    AppGameChildUxPreferencePreflightRow {
        preflight_row_id: input.preflight_row_id,
        source_scheduler_entry_id: input.scheduler_record.scheduler_entry_id,
        source_scheduler_state: input.scheduler_record.scheduler_state,
        status,
        source_local_outbox_record_ref: preference_required
            .then_some(input.source_outbox_record.entry_id),
        source_outbox_file_ref: preference_required
            .then_some(input.source_outbox_record.outbox_file_ref),
        local_data_path_ref: preference_required
            .then_some(input.source_outbox_record.local_data_path_ref),
        scheduler_decision_ref: input.scheduler_record.scheduler_decision_ref,
        scheduler_artifact_ref: input.scheduler_record.scheduler_artifact_ref,
        provider_channel: preference_required.then_some(input.scheduler_record.provider_channel),
        reason_code: preference_required.then_some(input.scheduler_record.reason_code),
        severity: preference_required.then_some(input.scheduler_record.severity),
        evidence_refs: input.source_outbox_record.envelope.evidence_refs,
        policy_refs: input.source_outbox_record.envelope.policy_refs,
        audit_refs: input.source_outbox_record.envelope.audit_refs,
        parent_preference_requirement_refs: preference_required
            .then_some(parent_preference_requirement_ref)
            .into_iter()
            .collect(),
        notification_frequency_requirement_refs: preference_required
            .then_some(notification_frequency_requirement_ref)
            .into_iter()
            .collect(),
        quiet_hours_requirement_refs: preference_required
            .then_some(quiet_hours_requirement_ref)
            .into_iter()
            .collect(),
        manual_proof_requirements,
        parent_preference_mutation_runtime_claimed: false,
        parent_frequency_control_ui_claimed: false,
        quiet_hours_timer_runtime_claimed: false,
        provider_delivery_runtime_claimed: false,
        provider_receipt_ingestion_claimed: false,
        provider_credentials_claimed: false,
        cloud_routing_claimed: false,
        parent_notification_ui_claimed: false,
        child_delivery_claimed: false,
        adapter_dispatch_claimed: false,
        platform_enforcement_claimed: false,
    }
}

fn status_for(
    state: &NotificationLocalOutboxSchedulerState,
) -> AppGameChildUxPreferencePreflightStatus {
    match state {
        NotificationLocalOutboxSchedulerState::DueLocal => {
            AppGameChildUxPreferencePreflightStatus::ParentPreferenceRequired
        }
        NotificationLocalOutboxSchedulerState::ManualRequired
        | NotificationLocalOutboxSchedulerState::HeldQuietHours
        | NotificationLocalOutboxSchedulerState::RetryWindowScheduled
        | NotificationLocalOutboxSchedulerState::ReceiptRequired => {
            AppGameChildUxPreferencePreflightStatus::ManualRequired
        }
        NotificationLocalOutboxSchedulerState::DeadLetterReview => {
            AppGameChildUxPreferencePreflightStatus::Unavailable
        }
    }
}

fn validate_requirements(
    input: &AppGameChildUxPreferencePreflightInput,
) -> Result<(), EventingError> {
    let refs = [
        input.preflight_row_id.as_str(),
        input.parent_preference_requirement_ref.as_str(),
        input.notification_frequency_requirement_ref.as_str(),
        input.quiet_hours_requirement_ref.as_str(),
    ];
    let empty = refs.iter().any(|reference| reference.trim().is_empty());
    let requirements = &refs[1..];
    let duplicate = requirements
        .iter()
        .enumerate()
        .any(|(index, reference)| requirements[index + 1..].contains(reference));
    if empty || duplicate {
        return Err(invalid_value(
            INVALID_REQUIREMENTS_FIELD,
            input.preflight_row_id.as_str(),
        ));
    }
    Ok(())
}

fn validate_sources(input: &AppGameChildUxPreferencePreflightInput) -> Result<(), EventingError> {
    let scheduler = &input.scheduler_record;
    let source = &input.source_outbox_record;
    let envelope = &source.envelope;
    let identity_mismatch = scheduler.source_entry_id != source.entry_id
        || scheduler.source_state != source.state
        || scheduler.source_outbox_file_ref != source.outbox_file_ref
        || scheduler.local_data_path_ref != source.local_data_path_ref
        || scheduler.provider_channel != envelope.provider_channel
        || scheduler.reason_code != envelope.reason_code
        || scheduler.severity != envelope.severity;
    let missing_evidence = envelope.evidence_refs.is_empty()
        || envelope.policy_refs.is_empty()
        || envelope.audit_refs.is_empty();
    let unsafe_claim = !scheduler.parent_owned_artifact_written
        || scheduler.provider_delivery_attempted
        || scheduler.provider_delivery_observed
        || scheduler.provider_receipt_ingested
        || scheduler.provider_credentials_stored
        || scheduler.cloud_routing_claimed
        || scheduler.parent_notification_ui_claimed
        || scheduler.production_durable_outbox_storage_claimed
        || scheduler.sensitive_provider_metadata_stored
        || source.provider_delivery_attempted
        || source.provider_delivery_observed
        || source.provider_receipt_ingested
        || source.provider_credentials_stored
        || source.cloud_routing_claimed
        || source.parent_notification_ui_claimed
        || source.sensitive_provider_metadata_stored;
    if identity_mismatch || missing_evidence || unsafe_claim {
        return Err(invalid_value(
            INVALID_SOURCE_FIELD,
            scheduler.scheduler_entry_id.as_str(),
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
