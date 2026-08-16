use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::app_game::{AppGameServiceReadModel, APP_GAME_SCHEMA_VERSION};
use ocentra_parent_agent_protocol::app_game_authority_classifier::APP_GAME_CONTROL_ACTION_STATUS_ENFORCED;
use ocentra_parent_agent_protocol::app_game_notification_status::{
    AppGameNotificationParentPreferenceState, AppGameNotificationPreferenceDeliveryResultState,
    AppGameNotificationPreferenceStatusEntry, AppGameNotificationPreferenceStatusReadModel,
    AppGameNotificationProviderChannel, AppGameNotificationQuietHoursDecision,
    AppGameNotificationStatusReadModels,
};
use ocentra_parent_agent_protocol::notification_provider_status_boundary::{
    V08NotificationEscalationReadiness, V08NotificationProviderDeliveryClaim,
    V08NotificationProviderStatus, V08NotificationProviderStatusBoundaryEntry,
    V08NotificationProviderStatusBoundaryReadModel, V08NotificationProviderStatusProofState,
    V08NotificationQuietHoursReadiness, V08_NOTIFICATION_PROVIDER_STATUS_BOUNDARY_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::AppGameNotificationReadinessReadModel;
use ocentra_parent_agent_protocol::AppGameNotificationReadinessRow;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_CUSTODY_CHILD_DEVICE_QUERY_STORE;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_APPROVAL_REQUEST;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_SUSPICIOUS_UNKNOWN;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_TIME_LIMIT;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_UNAVAILABLE;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_REASON_APPROVAL_REQUEST;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_REASON_CAPABILITY_UNAVAILABLE;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_REASON_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_REASON_SUSPICIOUS_UNKNOWN;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_REASON_TIME_LIMIT_EXCEEDED;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_STATE_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_STATUS_NO_ROWS;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_STATUS_PARTIAL;
use ocentra_parent_agent_protocol::APP_GAME_NOTIFICATION_READINESS_STATUS_READY;

use super::evidence::{
    app_game_boundary_row_count, approval_authority_refs, count_rows_with_state,
    evidence_claim_refs, manual_required_refs, platform_authority_row_count, policy_evidence_refs,
    push_evidence, NotificationReadinessTextRef,
};

pub(super) fn app_game_notification_readiness_from_service_model(
    model: AppGameServiceReadModel,
    local_outbox_runtime_claimed: bool,
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
    let adapter_dispatch_claimed = model
        .approval_action_result_rows
        .iter()
        .any(|row| row.result_status == APP_GAME_CONTROL_ACTION_STATUS_ENFORCED);

    AppGameNotificationReadinessReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: model.generated_at,
        custody_label: APP_GAME_NOTIFICATION_READINESS_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        capability_status: notification_readiness_status(ready_intent_count, unavailable_count)
            .0
            .to_string(),
        returned,
        ready_intent_count,
        manual_required_count,
        unavailable_count,
        provider_delivery_claimed: false,
        provider_receipt_ingestion_claimed: false,
        local_outbox_runtime_claimed,
        scheduler_runtime_claimed: false,
        adapter_dispatch_claimed,
        parent_ui_claimed: false,
        child_delivery_claimed: false,
        rows,
    }
}

pub(super) fn notification_status_read_models(
    rows: &[AppGameNotificationReadinessRow],
    generated_at: &str,
) -> AppGameNotificationStatusReadModels {
    let provider_entries = rows
        .iter()
        .map(|row| notification_provider_status_entry(row, generated_at))
        .collect::<Vec<_>>();
    let preference_entries = rows
        .iter()
        .map(|row| notification_preference_status_entry(row))
        .collect::<Vec<_>>();
    let source_read_model_id = format!("app-game-notification-readiness:{generated_at}");
    AppGameNotificationStatusReadModels {
        provider_status_boundary: V08NotificationProviderStatusBoundaryReadModel {
            schema_version: V08_NOTIFICATION_PROVIDER_STATUS_BOUNDARY_SCHEMA_VERSION.to_string(),
            read_model_id: format!("app-game-provider-status:{generated_at}"),
            generated_at: generated_at.to_string(),
            source_read_model_ids: vec![source_read_model_id.clone()],
            entries: provider_entries,
        },
        preference_status: AppGameNotificationPreferenceStatusReadModel {
            schema_version:
                ocentra_parent_agent_protocol::app_game_notification_status::
                    APP_GAME_NOTIFICATION_PREFERENCE_STATUS_SCHEMA_VERSION,
            read_model_id: format!("app-game-preference-status:{generated_at}"),
            generated_at: generated_at.to_string(),
            source_read_model_ids: vec![source_read_model_id],
            entries: preference_entries,
        },
    }
}

fn notification_provider_status_entry(
    row: &AppGameNotificationReadinessRow,
    generated_at: &str,
) -> V08NotificationProviderStatusBoundaryEntry {
    let unavailable = row.readiness_state == APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE;
    V08NotificationProviderStatusBoundaryEntry {
        schema_version: V08_NOTIFICATION_PROVIDER_STATUS_BOUNDARY_SCHEMA_VERSION.to_string(),
        status_entry_id: format!("app-game-provider-status-entry:{}", row.row_id),
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
        delivery_claim_state: V08NotificationProviderDeliveryClaim::NotImplemented,
        notification_intent_ref: row.row_id.clone(),
        notification_status_ref: format!("notification-status:{}", row.row_id),
        provider_attempt_ref: format!("provider-attempt-not-observed:{}", row.row_id),
        audit_refs: row.evidence_reference_ids.clone(),
        preference_refs: Vec::new(),
        readiness_refs: vec![row.row_id.clone()],
        provider_receipt_refs: Vec::new(),
        manual_proof_requirements: if unavailable {
            vec!["manual-proof:provider-availability".to_string()]
        } else {
            vec![
                "manual-proof:provider-credentials".to_string(),
                "manual-proof:provider-delivery-receipt".to_string(),
            ]
        },
        minimal_payload_boundary: row.minimal_payload_ref.clone(),
        provider_delivery_implemented: false,
        provider_delivery_observed: false,
        delivered_notification_claimed: false,
        sensitive_provider_payload_claimed: false,
        provider_stores_child_evidence_claimed: false,
        last_checked_at: generated_at.to_string(),
    }
}

fn notification_preference_status_entry(
    row: &AppGameNotificationReadinessRow,
) -> AppGameNotificationPreferenceStatusEntry {
    let unavailable = row.readiness_state == APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE;
    AppGameNotificationPreferenceStatusEntry {
        delivery_result_state: if unavailable {
            AppGameNotificationPreferenceDeliveryResultState::Unavailable
        } else {
            AppGameNotificationPreferenceDeliveryResultState::ManualRequired
        },
        parent_preference_state: if unavailable {
            AppGameNotificationParentPreferenceState::Unavailable
        } else {
            AppGameNotificationParentPreferenceState::ManualSetupRequired
        },
        quiet_hours_decision: if unavailable {
            AppGameNotificationQuietHoursDecision::Unavailable
        } else {
            AppGameNotificationQuietHoursDecision::ManualRequired
        },
        provider_channel: AppGameNotificationProviderChannel::Unavailable,
        delivery_result_ref: format!("delivery-result-not-observed:{}", row.row_id),
        audit_refs: row.evidence_reference_ids.clone(),
        manual_proof_requirements: if unavailable {
            vec!["manual-proof:provider-availability".to_string()]
        } else {
            vec![
                "manual-proof:parent-preference".to_string(),
                "manual-proof:notification-channel".to_string(),
            ]
        },
    }
}

fn notification_rows(model: &AppGameServiceReadModel) -> Vec<AppGameNotificationReadinessRow> {
    let mut rows = Vec::new();
    let policy_ready = policy_evaluation_ready(model);
    let policy_evidence = policy_evidence_refs(model);
    let approval_evidence = approval_authority_refs(model);
    if policy_ready {
        rows.push(notification_row(
            NotificationReadinessTextRef(
                APP_GAME_NOTIFICATION_READINESS_REASON_TIME_LIMIT_EXCEEDED,
            ),
            NotificationReadinessTextRef(
                APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
            ),
            policy_evidence.len() as u64,
            NotificationReadinessTextRef(
                APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_TIME_LIMIT,
            ),
            policy_evidence.clone(),
        ));
    }
    if !policy_evidence.is_empty() && !approval_evidence.is_empty() {
        let mut evidence = policy_evidence;
        push_evidence(&mut evidence, approval_evidence);
        rows.push(notification_row(
            NotificationReadinessTextRef(APP_GAME_NOTIFICATION_READINESS_REASON_APPROVAL_REQUEST),
            NotificationReadinessTextRef(
                APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
            ),
            evidence.len() as u64,
            NotificationReadinessTextRef(
                APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_APPROVAL_REQUEST,
            ),
            evidence,
        ));
    }
    if !model.evidence_claim_rows.is_empty() {
        rows.push(notification_row(
            NotificationReadinessTextRef(APP_GAME_NOTIFICATION_READINESS_REASON_SUSPICIOUS_UNKNOWN),
            NotificationReadinessTextRef(
                APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
            ),
            model.evidence_claim_rows.len() as u64,
            NotificationReadinessTextRef(
                APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_SUSPICIOUS_UNKNOWN,
            ),
            evidence_claim_refs(model),
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
