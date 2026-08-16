use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::app_game_notification_readiness::{
    AppGameNotificationReadinessRow, APP_GAME_NOTIFICATION_READINESS_REASON_APPROVAL_REQUEST,
    APP_GAME_NOTIFICATION_READINESS_REASON_CAPABILITY_UNAVAILABLE,
    APP_GAME_NOTIFICATION_READINESS_REASON_MANUAL_REQUIRED,
    APP_GAME_NOTIFICATION_READINESS_REASON_SUSPICIOUS_UNKNOWN,
    APP_GAME_NOTIFICATION_READINESS_REASON_TIME_LIMIT_EXCEEDED,
    APP_GAME_NOTIFICATION_READINESS_STATE_MANUAL_REQUIRED,
    APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
    APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE,
};

use crate::app_game_notification_local_outbox_bridge_types::AppGameNotificationLocalOutboxBridgeOptions;

const INVALID_CONTEXT_FIELD: &str = "app_game.notification_local_outbox.context";
const INVALID_SOURCE_FIELD: &str = "app_game.notification_local_outbox.source";

pub(super) fn validate_options(
    options: &AppGameNotificationLocalOutboxBridgeOptions,
) -> Result<(), EventingError> {
    let empty = options.bridge_id.trim().is_empty()
        || options.generated_at.as_str().trim().is_empty()
        || options.family.family_id.trim().is_empty()
        || options.device.device_id.as_str().trim().is_empty()
        || options.parent_action.action_reference_id.trim().is_empty()
        || options.parent_action.actor.actor_id.trim().is_empty()
        || options.parent_action.policy_version.trim().is_empty()
        || options.parent_action.created_at.trim().is_empty()
        || options.outbox_root_ref.as_str().trim().is_empty()
        || options.outbox_file_ref.as_str().trim().is_empty()
        || options.local_data_path_ref.as_str().trim().is_empty()
        || refs_have_empty_value(&options.policy_refs)
        || refs_have_empty_value(&options.audit_refs);
    if empty || options.policy_refs.is_empty() || options.audit_refs.is_empty() {
        return Err(invalid(INVALID_CONTEXT_FIELD, &options.bridge_id));
    }
    Ok(())
}

pub(super) fn validate_source(row: &AppGameNotificationReadinessRow) -> Result<(), EventingError> {
    let empty = row.schema_version == 0
        || row.row_id.trim().is_empty()
        || row.reason.trim().is_empty()
        || row.readiness_state.trim().is_empty()
        || row.minimal_payload_ref.trim().is_empty()
        || row
            .evidence
            .iter()
            .any(|evidence| evidence.evidence_id.trim().is_empty());
    let coherent = match row.readiness_state.as_str() {
        APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT => {
            !row.evidence.is_empty()
                && matches!(
                    row.reason.as_str(),
                    APP_GAME_NOTIFICATION_READINESS_REASON_TIME_LIMIT_EXCEEDED
                        | APP_GAME_NOTIFICATION_READINESS_REASON_APPROVAL_REQUEST
                        | APP_GAME_NOTIFICATION_READINESS_REASON_SUSPICIOUS_UNKNOWN
                )
        }
        APP_GAME_NOTIFICATION_READINESS_STATE_MANUAL_REQUIRED => {
            row.reason == APP_GAME_NOTIFICATION_READINESS_REASON_MANUAL_REQUIRED
        }
        APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE => {
            row.reason == APP_GAME_NOTIFICATION_READINESS_REASON_CAPABILITY_UNAVAILABLE
        }
        _ => false,
    };
    if empty || !coherent {
        return Err(invalid(INVALID_SOURCE_FIELD, &row.row_id));
    }
    Ok(())
}

fn refs_have_empty_value(
    refs: &[ocentra_parent_agent_protocol::schema_domain_mirrors::notification::NotificationLocalOutboxReference],
) -> bool {
    refs.iter()
        .any(|reference| reference.as_str().trim().is_empty())
}

fn invalid(field: &'static str, value: &str) -> EventingError {
    EventingError::InvalidValue {
        field,
        value: value.to_owned(),
    }
}
