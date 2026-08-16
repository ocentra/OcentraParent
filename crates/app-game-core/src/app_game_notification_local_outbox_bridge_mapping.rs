use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::activity::policy::{
    ParentEvidenceReference, ParentEvidenceReferenceKind,
};
use ocentra_parent_agent_protocol::app_game_notification_readiness::AppGameNotificationReadinessRow;
use ocentra_parent_agent_protocol::schema_domain_mirrors::notification::{
    NotificationLocalOutboxDeliveryClaimState, NotificationLocalOutboxEntryId,
    NotificationLocalOutboxMinimalAlertEnvelope, NotificationLocalOutboxPayloadPreview,
    NotificationLocalOutboxRecord, NotificationLocalOutboxReference, NotificationLocalOutboxState,
};

use crate::app_game_notification_local_outbox_bridge_classification::classify;
use crate::app_game_notification_local_outbox_bridge_types::{
    AppGameNotificationLocalOutboxBridgeOptions, AppGameNotificationLocalOutboxBridgeRow,
    AppGameNotificationLocalOutboxBridgeStatus,
};

const ENTRY_ID_PREFIX: &str = "app-game-notification-outbox";
const ALERT_REF_PREFIX: &str = "app-game-notification-alert";
const BRIDGE_RECORD_PREFIX: &str = "app-game-notification-outbox-bridge";
const INVALID_SOURCE_FIELD: &str = "app_game.notification_local_outbox.source";

pub(super) fn bridge_row(
    options: &AppGameNotificationLocalOutboxBridgeOptions,
    source: AppGameNotificationReadinessRow,
) -> Result<AppGameNotificationLocalOutboxBridgeRow, EventingError> {
    let classification = classify(&source).ok_or_else(|| EventingError::InvalidValue {
        field: INVALID_SOURCE_FIELD,
        value: source.row_id.clone(),
    })?;
    let status = classification.status;
    let bridge_record_id = format!(
        "{BRIDGE_RECORD_PREFIX}:{}:{}",
        options.bridge_id, source.row_id
    );
    let outbox_record = (status == AppGameNotificationLocalOutboxBridgeStatus::Linked)
        .then(|| outbox_record(options, &source, classification));
    let blocked_reason_refs = if outbox_record.is_some() {
        Vec::new()
    } else {
        blocked_refs(&source)
    };
    Ok(AppGameNotificationLocalOutboxBridgeRow {
        bridge_record_id,
        status,
        source,
        outbox_record,
        blocked_reason_refs,
    })
}

fn outbox_record(
    options: &AppGameNotificationLocalOutboxBridgeOptions,
    row: &AppGameNotificationReadinessRow,
    classification: crate::app_game_notification_local_outbox_bridge_classification::NotificationOutboxClassification,
) -> NotificationLocalOutboxRecord {
    NotificationLocalOutboxRecord {
        entry_id: NotificationLocalOutboxEntryId::from(format!(
            "{ENTRY_ID_PREFIX}:{}:{}",
            options.bridge_id, row.row_id
        )),
        state: NotificationLocalOutboxState::QueuedLocal,
        envelope: NotificationLocalOutboxMinimalAlertEnvelope {
            alert_ref: NotificationLocalOutboxReference::from(format!(
                "{ALERT_REF_PREFIX}:{}:{}",
                options.bridge_id, row.row_id
            )),
            family: options.family.clone(),
            device: options.device.clone(),
            parent_action: options.parent_action.clone(),
            severity: classification.severity,
            reason_code: classification.reason_code,
            provider_channel: options.provider_channel.clone(),
            evidence_refs: row
                .evidence
                .iter()
                .map(|evidence| ParentEvidenceReference {
                    evidence_reference_id: evidence.evidence_id.clone(),
                    kind: ParentEvidenceReferenceKind::ActivityEvent,
                    observed_at: options.generated_at.as_str().to_owned(),
                })
                .collect(),
            policy_refs: options.policy_refs.clone(),
            audit_refs: options.audit_refs.clone(),
            payload_template_ref: row.minimal_payload_ref.as_str().into(),
            provider_payload_preview: NotificationLocalOutboxPayloadPreview::from(
                row.minimal_payload_ref.as_str(),
            ),
            sensitive_detail_minimized: true,
            raw_child_evidence_included: false,
            raw_url_or_title_included: false,
            raw_message_text_included: false,
            screenshot_or_report_included: false,
        },
        outbox_file_ref: options.outbox_file_ref.clone(),
        local_data_path_ref: options.local_data_path_ref.clone(),
        delivery_claim_state: NotificationLocalOutboxDeliveryClaimState::LocalOutboxOnly,
        visible_after_at: None,
        retry_attempt_count: 0,
        quiet_hours_ref: None,
        retry_policy_ref: None,
        dead_letter_ref: None,
        provider_receipt_ref: None,
        manual_proof_requirements: Vec::new(),
        manual_action_required: false,
        provider_delivery_attempted: false,
        provider_delivery_observed: false,
        provider_receipt_ingested: false,
        provider_credentials_stored: false,
        cloud_routing_claimed: false,
        parent_notification_ui_claimed: false,
        sensitive_provider_metadata_stored: false,
    }
}

fn blocked_refs(row: &AppGameNotificationReadinessRow) -> Vec<NotificationLocalOutboxReference> {
    let mut refs = row
        .evidence_reference_ids
        .iter()
        .map(|reference| NotificationLocalOutboxReference::from(reference.as_str()))
        .collect::<Vec<_>>();
    if refs.is_empty() {
        refs.push(row.minimal_payload_ref.as_str().into());
    }
    refs
}
