use std::collections::HashSet;

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::schema_domain_mirrors::notification::{
    NotificationLocalOutboxDeliveryClaimState, NotificationLocalOutboxState,
};

use crate::app_game_notification_local_outbox_bridge_types::{
    AppGameNotificationLocalOutboxBridgeReadModel, AppGameNotificationLocalOutboxBridgeRow,
    AppGameNotificationLocalOutboxBridgeStatus,
};
use crate::app_game_notification_local_outbox_bridge_validation::validate_source;

const BRIDGE_RECORD_PREFIX: &str = "app-game-notification-outbox-bridge";
const ENTRY_ID_PREFIX: &str = "app-game-notification-outbox";
const ALERT_REF_PREFIX: &str = "app-game-notification-alert";

pub(super) fn validate_app_game_notification_local_outbox_bridge_read_model(
    source: &AppGameNotificationLocalOutboxBridgeReadModel,
    invalid_field: &'static str,
) -> Result<(), EventingError> {
    let honest_counts = source.linked_record_count
        == count_rows(source, AppGameNotificationLocalOutboxBridgeStatus::Linked)
        && source.manual_required_count
            == count_rows(
                source,
                AppGameNotificationLocalOutboxBridgeStatus::ManualRequired,
            )
        && source.unavailable_count
            == count_rows(
                source,
                AppGameNotificationLocalOutboxBridgeStatus::Unavailable,
            );
    let unsafe_claim = source.provider_delivery_runtime_claimed
        || source.provider_receipt_ingestion_claimed
        || source.scheduler_runtime_claimed
        || source.cloud_routing_claimed
        || source.parent_notification_ui_claimed
        || source.child_delivery_claimed
        || source.adapter_dispatch_claimed;
    let dishonest_row = source
        .rows
        .iter()
        .any(|row| validate_source(&row.source).is_err() || !source_row_is_honest(source, row));
    let mut identities = HashSet::new();
    let duplicate_identity = source
        .rows
        .iter()
        .any(|row| !identities.insert(row.bridge_record_id.as_str()));
    let missing_context = source.schema_version == 0
        || source.bridge_id.trim().is_empty()
        || source.generated_at.as_str().trim().is_empty()
        || source.family.family_id.trim().is_empty()
        || source.outbox_root_ref.as_str().trim().is_empty()
        || source.policy_refs.is_empty()
        || source.audit_refs.is_empty()
        || refs_have_empty_value(&source.policy_refs)
        || refs_have_empty_value(&source.audit_refs);
    if missing_context || !honest_counts || unsafe_claim || dishonest_row || duplicate_identity {
        return Err(EventingError::InvalidValue {
            field: invalid_field,
            value: source.bridge_id.clone(),
        });
    }
    Ok(())
}

fn source_row_is_honest(
    source: &AppGameNotificationLocalOutboxBridgeReadModel,
    row: &AppGameNotificationLocalOutboxBridgeRow,
) -> bool {
    let expected_bridge_record_id = format!(
        "{BRIDGE_RECORD_PREFIX}:{}:{}",
        source.bridge_id, row.source.row_id
    );
    if row.bridge_record_id != expected_bridge_record_id {
        return false;
    }
    match row.status {
        AppGameNotificationLocalOutboxBridgeStatus::Linked => {
            row.outbox_record.as_ref().is_some_and(|record| {
                record.entry_id.as_str()
                    == format!(
                        "{ENTRY_ID_PREFIX}:{}:{}",
                        source.bridge_id, row.source.row_id
                    )
                    && record.envelope.alert_ref.as_str()
                        == format!(
                            "{ALERT_REF_PREFIX}:{}:{}",
                            source.bridge_id, row.source.row_id
                        )
                    && row.blocked_reason_refs.is_empty()
                    && record.envelope.family == source.family
                    && record.envelope.policy_refs == source.policy_refs
                    && record.envelope.audit_refs == source.audit_refs
                    && record.state == NotificationLocalOutboxState::QueuedLocal
                    && record.delivery_claim_state
                        == NotificationLocalOutboxDeliveryClaimState::LocalOutboxOnly
                    && !record.outbox_file_ref.as_str().trim().is_empty()
                    && !record.local_data_path_ref.as_str().trim().is_empty()
                    && record.visible_after_at.is_none()
                    && record.retry_attempt_count == 0
                    && record.quiet_hours_ref.is_none()
                    && record.retry_policy_ref.is_none()
                    && record.dead_letter_ref.is_none()
                    && record.provider_receipt_ref.is_none()
                    && record.manual_proof_requirements.is_empty()
                    && !record.manual_action_required
                    && !record.provider_delivery_attempted
                    && !record.provider_delivery_observed
                    && !record.provider_receipt_ingested
                    && !record.provider_credentials_stored
                    && !record.cloud_routing_claimed
                    && !record.parent_notification_ui_claimed
                    && !record.sensitive_provider_metadata_stored
                    && record.envelope.sensitive_detail_minimized
                    && !record.envelope.raw_child_evidence_included
                    && !record.envelope.raw_url_or_title_included
                    && !record.envelope.raw_message_text_included
                    && !record.envelope.screenshot_or_report_included
            })
        }
        AppGameNotificationLocalOutboxBridgeStatus::ManualRequired
        | AppGameNotificationLocalOutboxBridgeStatus::Unavailable => {
            row.outbox_record.is_none() && !row.blocked_reason_refs.is_empty()
        }
    }
}

fn count_rows(
    source: &AppGameNotificationLocalOutboxBridgeReadModel,
    status: AppGameNotificationLocalOutboxBridgeStatus,
) -> u64 {
    source
        .rows
        .iter()
        .filter(|row| row.status == status)
        .count() as u64
}

fn refs_have_empty_value(
    refs: &[ocentra_parent_agent_protocol::schema_domain_mirrors::notification::NotificationLocalOutboxReference],
) -> bool {
    refs.iter()
        .any(|reference| reference.as_str().trim().is_empty())
}
