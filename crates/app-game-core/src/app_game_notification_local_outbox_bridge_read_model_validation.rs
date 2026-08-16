use std::collections::HashSet;

use ocentra_eventing::error::EventingError;

use crate::app_game_notification_local_outbox_bridge_types::{
    AppGameNotificationLocalOutboxBridgeReadModel, AppGameNotificationLocalOutboxBridgeRow,
    AppGameNotificationLocalOutboxBridgeStatus,
};

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
        .any(|row| !source_row_is_honest(source, row));
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
    match row.status {
        AppGameNotificationLocalOutboxBridgeStatus::Linked => {
            row.outbox_record.as_ref().is_some_and(|record| {
                row.blocked_reason_refs.is_empty()
                    && record.envelope.family == source.family
                    && record.envelope.policy_refs == source.policy_refs
                    && record.envelope.audit_refs == source.audit_refs
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
