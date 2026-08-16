use std::collections::{HashMap, HashSet};

use ocentra_app_game_core::app_game_child_ux_preference_preflight_types::AppGameChildUxPreferencePreflightStatus;
use ocentra_app_game_core::app_game_child_ux_provider_preflight_types::AppGameChildUxProviderPreflightStatus;
use ocentra_app_game_core::app_game_notification_preference_preflight_bridge_types::AppGameNotificationPreferencePreflightBridgeRow;
use ocentra_app_game_core::app_game_notification_provider_preflight_bridge_types::AppGameNotificationProviderPreflightBridgeRow;
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
use ocentra_parent_agent_protocol::AppGameNotificationReadinessReadModel;
use ocentra_parent_agent_protocol::AppGameNotificationReadinessRow;
use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::app_game::{APP_GAME_SCHEMA_VERSION, AppGameServiceReadModel};
use ocentra_parent_agent_protocol::app_game_authority_classifier::APP_GAME_CONTROL_ACTION_STATUS_ENFORCED;
use ocentra_parent_agent_protocol::app_game_notification_parent_surface_intent::{
    AppGameNotificationFamilyReference, AppGameNotificationParentSurfaceIntentOptions,
    AppGameNotificationParentSurfaceIntentReadModel,
    AppGameNotificationPreferenceStatusHandoffReadModel,
    AppGameNotificationPreferenceStatusHandoffRow,
    AppGameNotificationProviderStatusBoundaryEntry as ParentSurfaceProviderStatusEntry,
    AppGameNotificationProviderStatusHandoffReadModel, AppGameNotificationProviderStatusHandoffRow,
};
use ocentra_parent_agent_protocol::app_game_notification_status::{
    AppGameNotificationParentPreferenceState, AppGameNotificationPreferenceDeliveryResultState,
    AppGameNotificationPreferenceStatusEntry, AppGameNotificationPreferenceStatusReadModel,
    AppGameNotificationProviderChannel, AppGameNotificationQuietHoursDecision,
    AppGameNotificationStatusReadModels,
};
use ocentra_parent_agent_protocol::notification_provider_status_boundary::{
    V08_NOTIFICATION_PROVIDER_STATUS_BOUNDARY_SCHEMA_VERSION, V08NotificationEscalationReadiness,
    V08NotificationProviderDeliveryClaim, V08NotificationProviderStatus,
    V08NotificationProviderStatusBoundaryEntry, V08NotificationProviderStatusBoundaryReadModel,
    V08NotificationProviderStatusProofState, V08NotificationQuietHoursReadiness,
};

use super::evidence::{
    NotificationReadinessTextRef, app_game_boundary_row_count, approval_authority_refs,
    count_rows_with_state, evidence_claim_refs, manual_required_refs, platform_authority_row_count,
    policy_evidence_refs, push_evidence,
};
use super::scheduler_runtime::{
    VerifiedNotificationPreflight, load_verified_notification_preflight,
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
    let scheduler_evidence_invalid = match load_verified_notification_preflight() {
        Ok(Some(preflight)) => {
            if let Some(read_models) =
                notification_status_read_models_from_preflight(&preflight, generated_at)
            {
                return read_models;
            }
            true
        }
        Ok(None) => false,
        Err(_) => true,
    };

    notification_status_read_models_without_scheduler(
        rows,
        generated_at,
        scheduler_evidence_invalid,
    )
}

fn notification_status_read_models_from_preflight(
    preflight: &VerifiedNotificationPreflight,
    generated_at: &str,
) -> Option<AppGameNotificationStatusReadModels> {
    let preference_rows = preflight
        .preference
        .rows
        .iter()
        .map(|row| (row.source_scheduler_bridge_record_id.as_str(), row))
        .collect::<HashMap<_, _>>();
    if preference_rows.len() != preflight.preference.rows.len()
        || preflight.provider.rows.len() != preflight.preference.rows.len()
    {
        return None;
    }
    let mut provider_entries = Vec::with_capacity(preflight.provider.rows.len());
    let mut preference_entries = Vec::with_capacity(preflight.provider.rows.len());
    let mut source_ids = HashSet::with_capacity(preflight.provider.rows.len());
    for provider_row in &preflight.provider.rows {
        if !source_ids.insert(provider_row.source_scheduler_bridge_record_id.clone()) {
            return None;
        }
        let preference_row =
            preference_rows.get(provider_row.source_scheduler_bridge_record_id.as_str())?;
        provider_entries.push(notification_provider_status_entry_from_preflight(
            provider_row,
            generated_at,
        ));
        preference_entries.push(notification_preference_status_entry_from_preflight(
            preference_row,
        ));
    }
    let source_provider_id = preflight.provider.bridge_id.clone();
    let source_preference_id = preflight.preference.bridge_id.clone();
    let provider_status_read_model = V08NotificationProviderStatusBoundaryReadModel {
        schema_version: V08_NOTIFICATION_PROVIDER_STATUS_BOUNDARY_SCHEMA_VERSION.to_string(),
        read_model_id: format!("app-game-provider-status:{source_provider_id}"),
        generated_at: generated_at.to_string(),
        source_read_model_ids: vec![source_provider_id],
        entries: provider_entries,
    };
    let preference_status_read_model = AppGameNotificationPreferenceStatusReadModel {
        schema_version:
            ocentra_parent_agent_protocol::app_game_notification_status::
                APP_GAME_NOTIFICATION_PREFERENCE_STATUS_SCHEMA_VERSION,
        read_model_id: format!("app-game-preference-status:{source_preference_id}"),
        generated_at: generated_at.to_string(),
        source_read_model_ids: vec![source_preference_id],
        entries: preference_entries,
    };
    let parent_surface_intent = notification_parent_surface_intent_read_model(
        &provider_status_read_model,
        &preference_status_read_model,
        generated_at,
    );
    Some(AppGameNotificationStatusReadModels {
        parent_surface_intent,
        provider_status_boundary: provider_status_read_model,
        preference_status: preference_status_read_model,
    })
}

fn notification_status_read_models_without_scheduler(
    rows: &[AppGameNotificationReadinessRow],
    generated_at: &str,
    scheduler_evidence_invalid: bool,
) -> AppGameNotificationStatusReadModels {
    let provider_entries = rows
        .iter()
        .map(|row| {
            notification_provider_status_entry_without_scheduler(
                row,
                generated_at,
                scheduler_evidence_invalid,
            )
        })
        .collect::<Vec<_>>();
    let preference_entries = rows
        .iter()
        .map(|row| {
            notification_preference_status_entry_without_scheduler(row, scheduler_evidence_invalid)
        })
        .collect::<Vec<_>>();
    let source_read_model_id = format!("app-game-notification-readiness:{generated_at}");
    let provider_status_read_model = V08NotificationProviderStatusBoundaryReadModel {
        schema_version: V08_NOTIFICATION_PROVIDER_STATUS_BOUNDARY_SCHEMA_VERSION.to_string(),
        read_model_id: format!("app-game-provider-status:{generated_at}"),
        generated_at: generated_at.to_string(),
        source_read_model_ids: vec![source_read_model_id.clone()],
        entries: provider_entries,
    };
    let preference_status_read_model = AppGameNotificationPreferenceStatusReadModel {
        schema_version:
            ocentra_parent_agent_protocol::app_game_notification_status::
                APP_GAME_NOTIFICATION_PREFERENCE_STATUS_SCHEMA_VERSION,
        read_model_id: format!("app-game-preference-status:{generated_at}"),
        generated_at: generated_at.to_string(),
        source_read_model_ids: vec![source_read_model_id],
        entries: preference_entries,
    };
    let parent_surface_intent = notification_parent_surface_intent_read_model(
        &provider_status_read_model,
        &preference_status_read_model,
        generated_at,
    );
    AppGameNotificationStatusReadModels {
        parent_surface_intent,
        provider_status_boundary: provider_status_read_model,
        preference_status: preference_status_read_model,
    }
}

fn notification_parent_surface_intent_read_model(
    provider_read_model: &V08NotificationProviderStatusBoundaryReadModel,
    preference_read_model: &AppGameNotificationPreferenceStatusReadModel,
    generated_at: &str,
) -> Option<AppGameNotificationParentSurfaceIntentReadModel> {
    let provider_readiness_refs = provider_read_model
        .entries
        .iter()
        .filter_map(|entry| entry.readiness_refs.first().cloned())
        .collect::<HashSet<_>>();
    let preference_readiness_refs = preference_read_model
        .entries
        .iter()
        .map(|entry| entry.readiness_ref.clone())
        .filter(|reference| !reference.is_empty())
        .collect::<HashSet<_>>();
    if provider_readiness_refs.len() != provider_read_model.entries.len()
        || preference_readiness_refs.len() != preference_read_model.entries.len()
        || provider_readiness_refs != preference_readiness_refs
    {
        return None;
    }
    let family = AppGameNotificationFamilyReference {
        family_id: "app-game".to_string(),
    };
    let provider_handoff = AppGameNotificationProviderStatusHandoffReadModel {
        handoff_id: provider_read_model.read_model_id.clone(),
        family: family.clone(),
        rows: provider_read_model
            .entries
            .iter()
            .map(|entry| AppGameNotificationProviderStatusHandoffRow {
                handoff_row_id: entry.status_entry_id.clone(),
                source_scheduler_entry_ref: None,
                source_outbox_record_ref: None,
                provider_status_boundary_entry: ParentSurfaceProviderStatusEntry {
                    provider_status: provider_status_label(entry.provider_status),
                    notification_status_ref: entry.notification_status_ref.clone(),
                    audit_refs: entry.audit_refs.clone(),
                    manual_proof_requirements: entry.manual_proof_requirements.clone(),
                },
            })
            .collect(),
    };
    let mut preference_handoff = AppGameNotificationPreferenceStatusHandoffReadModel {
        handoff_id: preference_read_model.read_model_id.clone(),
        family,
        rows: preference_read_model
            .entries
            .iter()
            .map(|entry| AppGameNotificationPreferenceStatusHandoffRow {
                handoff_row_id: format!("app-game-preference-status-entry:{}", entry.readiness_ref),
                source_scheduler_entry_ref: None,
                source_outbox_record_ref: None,
                notification_preference_status_entry: preference_status_handoff_entry(entry),
            })
            .collect(),
    };
    if provider_handoff.family.family_id != preference_handoff.family.family_id
        || provider_handoff.rows.len() != preference_handoff.rows.len()
    {
        return None;
    }
    let mut readiness_refs = HashSet::with_capacity(provider_handoff.rows.len());
    let mut ordered_preference_rows = Vec::with_capacity(provider_handoff.rows.len());
    for provider_row in &provider_handoff.rows {
        let Some(readiness_ref) = provider_read_model
            .entries
            .iter()
            .find(|entry| entry.status_entry_id == provider_row.handoff_row_id)
            .and_then(|entry| entry.readiness_refs.first())
        else {
            return None;
        };
        if !readiness_refs.insert(readiness_ref.clone()) {
            return None;
        }
        let Some(preference_row) = preference_handoff.rows.iter().find(|row| {
            row.handoff_row_id == format!("app-game-preference-status-entry:{readiness_ref}")
        }) else {
            return None;
        };
        ordered_preference_rows.push(preference_row.clone());
    }
    preference_handoff.rows = ordered_preference_rows;
    let options = AppGameNotificationParentSurfaceIntentOptions {
        generated_at: generated_at.to_string(),
        intent_id: format!("app-game-parent-surface:{generated_at}"),
        source_contract_refs: vec![
            provider_read_model.read_model_id.clone(),
            preference_read_model.read_model_id.clone(),
        ],
    };
    ocentra_app_game_core::app_game_notification_parent_surface_intent::
        build_app_game_notification_parent_surface_intent_read_model(
            &options,
            &provider_handoff,
            &preference_handoff,
        )
        .ok()
}

fn provider_status_label(status: V08NotificationProviderStatus) -> String {
    match status {
        V08NotificationProviderStatus::Queued => "queued",
        V08NotificationProviderStatus::Delivered => "delivered",
        V08NotificationProviderStatus::Failed => "failed",
        V08NotificationProviderStatus::Unavailable => "unavailable",
        V08NotificationProviderStatus::ManualRequired => "manual-required",
    }
    .to_string()
}

fn preference_status_handoff_entry(
    entry: &AppGameNotificationPreferenceStatusEntry,
) -> ParentSurfacePreferenceStatusEntry {
    ParentSurfacePreferenceStatusEntry {
        delivery_result_state: preference_delivery_result_label(entry.delivery_result_state),
        parent_preference_state: parent_preference_state_label(entry.parent_preference_state),
        quiet_hours_decision: quiet_hours_decision_label(entry.quiet_hours_decision),
        provider_channel: provider_channel_label(entry.provider_channel),
        delivery_result_ref: entry.delivery_result_ref.clone(),
        audit_refs: entry.audit_refs.clone(),
        manual_proof_requirements: entry.manual_proof_requirements.clone(),
    }
}

type ParentSurfacePreferenceStatusEntry =
    ocentra_parent_agent_protocol::app_game_notification_parent_surface_intent::
        AppGameNotificationPreferenceStatusHandoffEntry;

fn preference_delivery_result_label(
    status: AppGameNotificationPreferenceDeliveryResultState,
) -> String {
    match status {
        AppGameNotificationPreferenceDeliveryResultState::NotSent => "not-sent",
        AppGameNotificationPreferenceDeliveryResultState::ManualRequired => "manual-required",
        AppGameNotificationPreferenceDeliveryResultState::Unavailable => "unavailable",
    }
    .to_string()
}

fn parent_preference_state_label(status: AppGameNotificationParentPreferenceState) -> String {
    match status {
        AppGameNotificationParentPreferenceState::ChannelDisabled => "channel-disabled",
        AppGameNotificationParentPreferenceState::ManualSetupRequired => "manual-setup-required",
        AppGameNotificationParentPreferenceState::Unavailable => "unavailable",
    }
    .to_string()
}

fn quiet_hours_decision_label(status: AppGameNotificationQuietHoursDecision) -> String {
    match status {
        AppGameNotificationQuietHoursDecision::Allow => "allow",
        AppGameNotificationQuietHoursDecision::ManualRequired => "manual-required",
        AppGameNotificationQuietHoursDecision::Unavailable => "unavailable",
    }
    .to_string()
}

fn provider_channel_label(status: AppGameNotificationProviderChannel) -> String {
    match status {
        AppGameNotificationProviderChannel::Unavailable => "unavailable",
    }
    .to_string()
}

fn notification_provider_status_entry_from_preflight(
    row: &AppGameNotificationProviderPreflightBridgeRow,
    generated_at: &str,
) -> V08NotificationProviderStatusBoundaryEntry {
    let unavailable = row.status == AppGameChildUxProviderPreflightStatus::Unavailable;
    let preflight = row.preflight_row.as_ref();
    let row_id = row.preflight_bridge_record_id.clone();
    let audit_refs = preflight
        .map(|value| refs_to_strings(&value.audit_refs))
        .unwrap_or_default();
    let manual_proof_requirements = preflight
        .map(|value| refs_to_strings(&value.manual_proof_requirements))
        .filter(|refs| !refs.is_empty())
        .unwrap_or_else(|| refs_to_strings(&row.blocked_reason_refs));
    V08NotificationProviderStatusBoundaryEntry {
        schema_version: V08_NOTIFICATION_PROVIDER_STATUS_BOUNDARY_SCHEMA_VERSION.to_string(),
        status_entry_id: format!("app-game-provider-status-entry:{row_id}"),
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
        notification_intent_ref: preflight
            .map(|value| value.preflight_row_id.to_string())
            .unwrap_or_else(|| format!("scheduler-preflight-unavailable:{row_id}")),
        notification_status_ref: format!("notification-status:{row_id}"),
        provider_attempt_ref: format!("provider-attempt-not-observed:{row_id}"),
        audit_refs,
        preference_refs: preflight
            .map(|value| refs_to_strings(&value.policy_refs))
            .unwrap_or_default(),
        readiness_refs: vec![row_id.clone()],
        provider_receipt_refs: Vec::new(),
        manual_proof_requirements,
        minimal_payload_boundary:
            "Verified WP59 scheduler and WP61 preflight refs only; provider delivery is not implemented."
                .to_string(),
        provider_delivery_implemented: false,
        provider_delivery_observed: false,
        delivered_notification_claimed: false,
        sensitive_provider_payload_claimed: false,
        provider_stores_child_evidence_claimed: false,
        last_checked_at: generated_at.to_string(),
    }
}

fn notification_preference_status_entry_from_preflight(
    row: &AppGameNotificationPreferencePreflightBridgeRow,
) -> AppGameNotificationPreferenceStatusEntry {
    let unavailable = row.status == AppGameChildUxPreferencePreflightStatus::Unavailable;
    let preflight = row.preflight_row.as_ref();
    let readiness_ref = row.preflight_bridge_record_id.clone();
    let manual_proof_requirements = preflight
        .map(|value| refs_to_strings(&value.manual_proof_requirements))
        .filter(|refs| !refs.is_empty())
        .unwrap_or_else(|| refs_to_strings(&row.blocked_reason_refs));
    AppGameNotificationPreferenceStatusEntry {
        readiness_ref,
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
        delivery_result_ref: preflight
            .map(|value| value.scheduler_artifact_ref.to_string())
            .unwrap_or_else(|| {
                "delivery-result-not-observed:scheduler-preflight-unavailable".to_string()
            }),
        audit_refs: preflight
            .map(|value| refs_to_strings(&value.audit_refs))
            .unwrap_or_default(),
        manual_proof_requirements,
    }
}

fn refs_to_strings<T: ToString>(refs: &[T]) -> Vec<String> {
    refs.iter().map(ToString::to_string).collect()
}

fn notification_provider_status_entry_without_scheduler(
    row: &AppGameNotificationReadinessRow,
    generated_at: &str,
    scheduler_evidence_invalid: bool,
) -> V08NotificationProviderStatusBoundaryEntry {
    let unavailable = row.readiness_state == APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE;
    let scheduler_evidence_ref = if scheduler_evidence_invalid {
        if unavailable {
            "unavailable:invalid-scheduler-evidence"
        } else {
            "manual-required:invalid-scheduler-evidence"
        }
    } else if unavailable {
        "unavailable:scheduler-evidence-unavailable"
    } else {
        "manual-required:scheduler-evidence-unavailable"
    };
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
            vec![
                "manual-proof:provider-availability".to_string(),
                scheduler_evidence_ref.to_string(),
            ]
        } else {
            vec![
                "manual-proof:provider-credentials".to_string(),
                "manual-proof:provider-delivery-receipt".to_string(),
                scheduler_evidence_ref.to_string(),
            ]
        },
        minimal_payload_boundary:
            "No verified WP59 scheduler evidence; status is manual-required or unavailable only."
                .to_string(),
        provider_delivery_implemented: false,
        provider_delivery_observed: false,
        delivered_notification_claimed: false,
        sensitive_provider_payload_claimed: false,
        provider_stores_child_evidence_claimed: false,
        last_checked_at: generated_at.to_string(),
    }
}

fn notification_preference_status_entry_without_scheduler(
    row: &AppGameNotificationReadinessRow,
    scheduler_evidence_invalid: bool,
) -> AppGameNotificationPreferenceStatusEntry {
    let unavailable = row.readiness_state == APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE;
    let scheduler_evidence_ref = if scheduler_evidence_invalid {
        if unavailable {
            "unavailable:invalid-scheduler-evidence"
        } else {
            "manual-required:invalid-scheduler-evidence"
        }
    } else if unavailable {
        "unavailable:scheduler-evidence-unavailable"
    } else {
        "manual-required:scheduler-evidence-unavailable"
    };
    AppGameNotificationPreferenceStatusEntry {
        readiness_ref: row.row_id.clone(),
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
            vec![
                "manual-proof:provider-availability".to_string(),
                scheduler_evidence_ref.to_string(),
            ]
        } else {
            vec![
                "manual-proof:parent-preference".to_string(),
                "manual-proof:notification-channel".to_string(),
                scheduler_evidence_ref.to_string(),
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
