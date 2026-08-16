use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::notification_provider_status_boundary::{
    V08NotificationEscalationReadiness, V08NotificationProviderDeliveryClaim,
    V08NotificationProviderStatus, V08NotificationProviderStatusBoundaryEntry,
    V08NotificationProviderStatusProofState, V08NotificationQuietHoursReadiness,
};

use crate::{
    app_game_child_ux_provider_preflight_types::{
        AppGameChildUxProviderPreflightRow, AppGameChildUxProviderPreflightStatus,
    },
    app_game_child_ux_provider_status_types::{
        AppGameChildUxProviderStatusHandoffRow, AppGameChildUxProviderStatusInput,
    },
};

const INVALID_SOURCE_FIELD: &str = "app_game.child_ux_provider_status.source";
const INVALID_CONTEXT_FIELD: &str = "app_game.child_ux_provider_status.context";
const SCHEMA_VERSION: &str = "v0.6";
const MANUAL_PAYLOAD_BOUNDARY: &str =
    "Provider delivery remains blocked until adapter credentials preferences and smoke proof exist.";
const UNAVAILABLE_PAYLOAD_BOUNDARY: &str =
    "Provider unavailable keeps delivery unclaimed and visible for manual review.";

pub fn build_app_game_child_ux_provider_status_handoff(
    input: AppGameChildUxProviderStatusInput,
) -> Result<AppGameChildUxProviderStatusHandoffRow, EventingError> {
    validate_context(&input)?;
    validate_preflight(&input.preflight_row)?;
    Ok(build_row(input))
}

fn build_row(input: AppGameChildUxProviderStatusInput) -> AppGameChildUxProviderStatusHandoffRow {
    let unavailable =
        input.preflight_row.status == AppGameChildUxProviderPreflightStatus::Unavailable;
    let boundary = V08NotificationProviderStatusBoundaryEntry {
        schema_version: SCHEMA_VERSION.to_owned(),
        status_entry_id: input.status_entry_id.as_str().to_owned(),
        provider_status: if unavailable {
            V08NotificationProviderStatus::Unavailable
        } else {
            V08NotificationProviderStatus::ManualRequired
        },
        status_proof_state: if unavailable {
            V08NotificationProviderStatusProofState::ProviderUnavailableContract
        } else {
            V08NotificationProviderStatusProofState::ManualActionRequired
        },
        quiet_hours_readiness: if unavailable {
            V08NotificationQuietHoursReadiness::Unavailable
        } else {
            V08NotificationQuietHoursReadiness::ManualRequired
        },
        escalation_readiness: if unavailable {
            V08NotificationEscalationReadiness::Unavailable
        } else {
            V08NotificationEscalationReadiness::ManualRequired
        },
        delivery_claim_state: if unavailable {
            V08NotificationProviderDeliveryClaim::NotImplemented
        } else {
            V08NotificationProviderDeliveryClaim::NotObserved
        },
        notification_intent_ref: input.notification_intent_ref.as_str().to_owned(),
        notification_status_ref: input.notification_status_ref.as_str().to_owned(),
        provider_attempt_ref: input.provider_attempt_ref.as_str().to_owned(),
        audit_refs: to_strings(&input.preflight_row.audit_refs),
        preference_refs: to_strings(&input.preference_refs),
        readiness_refs: to_strings(&input.preflight_row.adapter_requirement_refs),
        provider_receipt_refs: Vec::new(),
        manual_proof_requirements: to_strings(&input.preflight_row.manual_proof_requirements),
        minimal_payload_boundary: if unavailable {
            UNAVAILABLE_PAYLOAD_BOUNDARY.to_owned()
        } else {
            MANUAL_PAYLOAD_BOUNDARY.to_owned()
        },
        provider_delivery_implemented: false,
        provider_delivery_observed: false,
        delivered_notification_claimed: false,
        sensitive_provider_payload_claimed: false,
        provider_stores_child_evidence_claimed: false,
        last_checked_at: input.last_checked_at,
    };
    AppGameChildUxProviderStatusHandoffRow {
        handoff_row_id: input.handoff_row_id,
        source_preflight_row_id: input.preflight_row.preflight_row_id,
        source_preflight_status: input.preflight_row.status,
        source_scheduler_entry_id: input.preflight_row.source_scheduler_entry_id,
        source_local_outbox_record_ref: input.preflight_row.source_local_outbox_record_ref,
        source_provider_channel: input.preflight_row.provider_channel,
        provider_status_boundary_entry: boundary,
        manual_proof_requirements: input.preflight_row.manual_proof_requirements,
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

fn validate_context(input: &AppGameChildUxProviderStatusInput) -> Result<(), EventingError> {
    let refs = [
        input.handoff_row_id.as_str(),
        input.status_entry_id.as_str(),
        input.notification_intent_ref.as_str(),
        input.notification_status_ref.as_str(),
        input.provider_attempt_ref.as_str(),
        input.last_checked_at.as_str(),
    ];
    let invalid = refs.iter().any(|reference| reference.trim().is_empty())
        || input.preference_refs.is_empty()
        || input
            .preference_refs
            .iter()
            .any(|reference| reference.as_str().trim().is_empty());
    if invalid {
        return Err(invalid_value(
            INVALID_CONTEXT_FIELD,
            input.handoff_row_id.as_str(),
        ));
    }
    Ok(())
}

fn validate_preflight(row: &AppGameChildUxProviderPreflightRow) -> Result<(), EventingError> {
    let source_shape_valid = match row.status {
        AppGameChildUxProviderPreflightStatus::ProviderAdapterRequired => {
            row.source_local_outbox_record_ref.is_some()
                && row.source_outbox_file_ref.is_some()
                && row.local_data_path_ref.is_some()
                && row.provider_channel.is_some()
                && row.reason_code.is_some()
                && row.severity.is_some()
        }
        AppGameChildUxProviderPreflightStatus::ManualRequired
        | AppGameChildUxProviderPreflightStatus::Unavailable => {
            row.source_local_outbox_record_ref.is_none()
                && row.source_outbox_file_ref.is_none()
                && row.local_data_path_ref.is_none()
                && row.provider_channel.is_none()
                && row.reason_code.is_none()
                && row.severity.is_none()
        }
    };
    let invalid = !source_shape_valid
        || row.evidence_refs.is_empty()
        || row.policy_refs.is_empty()
        || row.audit_refs.is_empty()
        || row.adapter_requirement_refs.len() < 3
        || row.manual_proof_requirements.len() < 3
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

fn to_strings(
    values: &[ocentra_parent_agent_protocol::schema_domain_mirrors::notification::NotificationLocalOutboxReference],
) -> Vec<String> {
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
