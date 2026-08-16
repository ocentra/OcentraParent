use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::schema_domain_mirrors::notification::{
    NotificationLocalOutboxReference, V3NotificationProviderChannel,
};

use crate::{
    app_game_child_ux_preference_preflight_types::{
        AppGameChildUxPreferencePreflightRow, AppGameChildUxPreferencePreflightStatus,
    },
    app_game_child_ux_preference_status_types::{
        AppGameChildUxPreferenceStatusHandoffRow, AppGameChildUxPreferenceStatusInput,
    },
    app_game_notification_parent_surface_intent::{
        AppGameNotificationPreferenceStatusEntry, AppGameNotificationPreferenceStatusHandoffRow,
    },
};

const INVALID_SOURCE_FIELD: &str = "app_game.child_ux_preference_status.source";
const INVALID_CONTEXT_FIELD: &str = "app_game.child_ux_preference_status.context";

pub fn build_app_game_child_ux_preference_status_handoff(
    input: AppGameChildUxPreferenceStatusInput,
) -> Result<AppGameChildUxPreferenceStatusHandoffRow, EventingError> {
    validate_context(&input)?;
    validate_preflight(&input.preflight_row)?;
    build_row(input)
}

fn build_row(
    input: AppGameChildUxPreferenceStatusInput,
) -> Result<AppGameChildUxPreferenceStatusHandoffRow, EventingError> {
    let provider_channel = input
        .preflight_row
        .provider_channel
        .clone()
        .ok_or_else(|| {
            invalid_value(
                INVALID_SOURCE_FIELD,
                input.preflight_row.preflight_row_id.as_str(),
            )
        })?;
    let reason_code = input.preflight_row.reason_code.clone().ok_or_else(|| {
        invalid_value(
            INVALID_SOURCE_FIELD,
            input.preflight_row.preflight_row_id.as_str(),
        )
    })?;
    let downstream = build_downstream_handoff(&input, &provider_channel);
    Ok(AppGameChildUxPreferenceStatusHandoffRow {
        handoff_row_id: input.handoff_row_id,
        source_preflight_row_id: input.preflight_row.preflight_row_id,
        source_preflight_status: input.preflight_row.status,
        source_scheduler_entry_id: input.preflight_row.source_scheduler_entry_id,
        source_local_outbox_record_ref: input.preflight_row.source_local_outbox_record_ref,
        source_provider_channel: provider_channel,
        source_reason_code: reason_code,
        preference_status_handoff_row: downstream,
        notification_rule_ref: input.notification_rule_ref,
        notification_intent_ref: input.notification_intent_ref,
        delivery_attempt_ref: input.delivery_attempt_ref,
        retry_policy_ref: input.retry_policy_ref,
        quiet_hours_policy_ref: input.quiet_hours_policy_ref,
        escalation_policy_ref: input.escalation_policy_ref,
        parent_preference_ref: input.parent_preference_ref,
        evidence_refs: input.preflight_row.evidence_refs,
        policy_refs: input.preflight_row.policy_refs,
        audit_refs: input.preflight_row.audit_refs,
        manual_proof_requirements: input.preflight_row.manual_proof_requirements,
        last_checked_at: input.last_checked_at,
        parent_preference_mutation_runtime_claimed: false,
        parent_frequency_control_ui_claimed: false,
        quiet_hours_timer_runtime_claimed: false,
        retry_execution_runtime_claimed: false,
        provider_delivery_runtime_claimed: false,
        provider_receipt_ingestion_claimed: false,
        provider_credentials_claimed: false,
        cloud_routing_claimed: false,
        parent_notification_ui_claimed: false,
        child_delivery_claimed: false,
        adapter_dispatch_claimed: false,
        platform_enforcement_claimed: false,
    })
}

fn build_downstream_handoff(
    input: &AppGameChildUxPreferenceStatusInput,
    provider_channel: &V3NotificationProviderChannel,
) -> AppGameNotificationPreferenceStatusHandoffRow {
    let unavailable =
        input.preflight_row.status == AppGameChildUxPreferencePreflightStatus::Unavailable;
    let entry = AppGameNotificationPreferenceStatusEntry {
        delivery_result_state: if unavailable {
            "not-sent".to_owned()
        } else {
            "manual-required".to_owned()
        },
        parent_preference_state: if unavailable {
            "channel-disabled".to_owned()
        } else {
            "manual-setup-required".to_owned()
        },
        quiet_hours_decision: if unavailable {
            "allow".to_owned()
        } else {
            "manual-required".to_owned()
        },
        provider_channel: provider_channel_name(provider_channel).to_owned(),
        delivery_result_ref: input.delivery_result_ref.as_str().to_owned(),
        audit_refs: reference_strings(&input.preflight_row.audit_refs),
        manual_proof_requirements: reference_strings(
            &input.preflight_row.manual_proof_requirements,
        ),
    };
    AppGameNotificationPreferenceStatusHandoffRow {
        handoff_row_id: input.handoff_row_id.as_str().to_owned(),
        source_scheduler_entry_ref: Some(
            input
                .preflight_row
                .source_scheduler_entry_id
                .as_str()
                .to_owned(),
        ),
        source_outbox_record_ref: input
            .preflight_row
            .source_local_outbox_record_ref
            .as_ref()
            .map(|reference| reference.as_str().to_owned()),
        notification_preference_status_entry: entry,
    }
}

fn validate_context(input: &AppGameChildUxPreferenceStatusInput) -> Result<(), EventingError> {
    let refs = [
        input.handoff_row_id.as_str(),
        input.notification_rule_ref.as_str(),
        input.notification_intent_ref.as_str(),
        input.delivery_attempt_ref.as_str(),
        input.delivery_result_ref.as_str(),
        input.retry_policy_ref.as_str(),
        input.quiet_hours_policy_ref.as_str(),
        input.escalation_policy_ref.as_str(),
        input.parent_preference_ref.as_str(),
        input.last_checked_at.as_str(),
    ];
    let duplicate_refs = refs[..9]
        .iter()
        .enumerate()
        .any(|(index, reference)| refs[index + 1..9].contains(reference));
    if refs.iter().any(|reference| reference.trim().is_empty()) || duplicate_refs {
        return Err(invalid_value(
            INVALID_CONTEXT_FIELD,
            input.handoff_row_id.as_str(),
        ));
    }
    Ok(())
}

fn validate_preflight(row: &AppGameChildUxPreferencePreflightRow) -> Result<(), EventingError> {
    let ready = row.status == AppGameChildUxPreferencePreflightStatus::ParentPreferenceRequired;
    let source_shape_valid = if ready {
        row.source_local_outbox_record_ref.is_some()
            && row.source_outbox_file_ref.is_some()
            && row.local_data_path_ref.is_some()
            && row.parent_preference_requirement_refs.len() == 1
            && row.notification_frequency_requirement_refs.len() == 1
            && row.quiet_hours_requirement_refs.len() == 1
    } else {
        row.source_local_outbox_record_ref.is_none()
            && row.source_outbox_file_ref.is_none()
            && row.local_data_path_ref.is_none()
            && row.parent_preference_requirement_refs.is_empty()
            && row.notification_frequency_requirement_refs.is_empty()
            && row.quiet_hours_requirement_refs.is_empty()
    };
    let invalid = !source_shape_valid
        || row.provider_channel.is_none()
        || row.reason_code.is_none()
        || row.severity.is_none()
        || row.evidence_refs.is_empty()
        || row.policy_refs.is_empty()
        || row.audit_refs.is_empty()
        || row.manual_proof_requirements.len() < 3
        || row.parent_preference_mutation_runtime_claimed
        || row.parent_frequency_control_ui_claimed
        || row.quiet_hours_timer_runtime_claimed
        || row.provider_delivery_runtime_claimed
        || row.provider_receipt_ingestion_claimed
        || row.provider_credentials_claimed
        || row.cloud_routing_claimed
        || row.parent_notification_ui_claimed
        || row.child_delivery_claimed
        || row.adapter_dispatch_claimed
        || row.platform_enforcement_claimed;
    if invalid {
        return Err(invalid_value(
            INVALID_SOURCE_FIELD,
            row.preflight_row_id.as_str(),
        ));
    }
    Ok(())
}

fn provider_channel_name(channel: &V3NotificationProviderChannel) -> &'static str {
    match channel {
        V3NotificationProviderChannel::Push => "push",
        V3NotificationProviderChannel::Email => "email",
        V3NotificationProviderChannel::Sms => "sms",
        V3NotificationProviderChannel::Whatsapp => "whatsapp",
        V3NotificationProviderChannel::InApp => "in-app",
    }
}

fn reference_strings(values: &[NotificationLocalOutboxReference]) -> Vec<String> {
    values
        .iter()
        .map(|value| value.as_str().to_owned())
        .collect()
}

fn invalid_value(field: &'static str, value: &str) -> EventingError {
    EventingError::InvalidValue {
        field,
        value: value.to_owned(),
    }
}
