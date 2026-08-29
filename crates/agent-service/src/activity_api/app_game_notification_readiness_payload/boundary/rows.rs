use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::app_game::{AppGameServiceReadModel, APP_GAME_SCHEMA_VERSION};
use ocentra_parent_agent_protocol::AppGameNotificationReadinessReadModel;
use ocentra_parent_agent_protocol::AppGameNotificationReadinessRow;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_SUSPICIOUS_UNKNOWN;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_UNAVAILABLE;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_REASON_CAPABILITY_UNAVAILABLE;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_REASON_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_REASON_SUSPICIOUS_UNKNOWN;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_STATE_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_STATUS_NO_ROWS;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_STATUS_PARTIAL;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_STATUS_READY;

use super::super::super::app_game_policy_readiness_sources::{
    unknown_review_refs, unknown_review_row_count,
};
use super::super::evidence::{
    app_game_boundary_row_count, count_rows_with_state, manual_required_refs,
    platform_authority_row_count, NotificationReadinessTextRef,
};

pub(super) fn app_game_notification_readiness_from_service_model(
    model: AppGameServiceReadModel,
) -> AppGameNotificationReadinessReadModel {
    let rows = notification_rows(&model);
    let returned = rows.len() as u64;
    let ready_intent_count = count_rows_with_state(
        &rows,
        NotificationReadinessTextRef(APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT),
    );
    let manual_required_count = count_rows_with_state(
        &rows,
        NotificationReadinessTextRef(APP_GAME_NOTIFICATION_READINESS_STATE_MANUAL_REQUIRED),
    );
    let unavailable_count = count_rows_with_state(
        &rows,
        NotificationReadinessTextRef(APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE),
    );
    AppGameNotificationReadinessReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: model.generated_at,
        custody_label: ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        capability_status: notification_readiness_status(ready_intent_count, unavailable_count)
            .0
            .to_string(),
        returned,
        ready_intent_count,
        manual_required_count,
        unavailable_count,
        provider_delivery_claimed: false,
        provider_receipt_ingestion_claimed: false,
        local_outbox_runtime_claimed: false,
        scheduler_runtime_claimed: false,
        adapter_dispatch_claimed: false,
        parent_ui_claimed: false,
        child_delivery_claimed: false,
        rows,
    }
}

fn notification_rows(model: &AppGameServiceReadModel) -> Vec<AppGameNotificationReadinessRow> {
    let mut rows = Vec::new();
    let policy_ready = policy_evaluation_ready(model);
    let unknown_review_count = unknown_review_row_count(model);
    if unknown_review_count > 0 {
        rows.push(notification_row(
            NotificationReadinessTextRef(APP_GAME_NOTIFICATION_READINESS_REASON_SUSPICIOUS_UNKNOWN),
            NotificationReadinessTextRef(APP_GAME_NOTIFICATION_READINESS_STATE_MANUAL_REQUIRED),
            unknown_review_count,
            NotificationReadinessTextRef(
                APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_SUSPICIOUS_UNKNOWN,
            ),
            unknown_review_refs(model),
        ));
    }
    if !policy_ready || model.ai_classifier_result_rows.is_empty() {
        rows.push(notification_row(
            NotificationReadinessTextRef(APP_GAME_NOTIFICATION_READINESS_REASON_MANUAL_REQUIRED),
            NotificationReadinessTextRef(APP_GAME_NOTIFICATION_READINESS_STATE_MANUAL_REQUIRED),
            manual_required_count(model),
            NotificationReadinessTextRef(
                APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_MANUAL_REQUIRED,
            ),
            manual_required_refs(model),
        ));
    }
    if app_game_boundary_row_count(model) == 0 {
        rows.push(notification_row(
            NotificationReadinessTextRef(
                APP_GAME_NOTIFICATION_READINESS_REASON_CAPABILITY_UNAVAILABLE,
            ),
            NotificationReadinessTextRef(APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE),
            0,
            NotificationReadinessTextRef(
                APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_UNAVAILABLE,
            ),
            Vec::new(),
        ));
    }
    rows
}

fn notification_row(
    reason: NotificationReadinessTextRef<'static>,
    readiness_state: NotificationReadinessTextRef<'static>,
    row_count: u64,
    minimal_payload_ref: NotificationReadinessTextRef<'static>,
    evidence: Vec<ActivityEvidenceRef>,
) -> AppGameNotificationReadinessRow {
    AppGameNotificationReadinessRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id: reason.0.to_string(),
        reason: reason.0.to_string(),
        readiness_state: readiness_state.0.to_string(),
        row_count,
        minimal_payload_ref: minimal_payload_ref.0.to_string(),
        evidence_reference_ids: evidence.iter().map(|row| row.evidence_id.clone()).collect(),
        evidence,
    }
}

fn notification_readiness_status(
    ready_intent_count: u64,
    unavailable_count: u64,
) -> NotificationReadinessTextRef<'static> {
    if ready_intent_count == 0 && unavailable_count > 0 {
        NotificationReadinessTextRef(APP_GAME_NOTIFICATION_READINESS_STATUS_NO_ROWS)
    } else if ready_intent_count >= 3 && unavailable_count == 0 {
        NotificationReadinessTextRef(APP_GAME_NOTIFICATION_READINESS_STATUS_READY)
    } else {
        NotificationReadinessTextRef(APP_GAME_NOTIFICATION_READINESS_STATUS_PARTIAL)
    }
}

fn policy_evaluation_ready(model: &AppGameServiceReadModel) -> bool {
    !model.evidence_claim_rows.is_empty()
        && !model.identity_rows.is_empty()
        && !model.approval_authority_rows.is_empty()
        && platform_authority_row_count(model) > 0
}

fn manual_required_count(model: &AppGameServiceReadModel) -> u64 {
    let mut count = 0;
    if model.identity_rows.is_empty() {
        count += 1;
    }
    if model.approval_authority_rows.is_empty() {
        count += 1;
    }
    if platform_authority_row_count(model) == 0 {
        count += 1;
    }
    if model.ai_classifier_result_rows.is_empty() {
        count += 1;
    }
    count
}
