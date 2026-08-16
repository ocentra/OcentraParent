use ocentra_app_game_core::app_game_child_ux_preference_preflight_types::AppGameChildUxPreferencePreflightStatus;
use ocentra_app_game_core::app_game_child_ux_provider_preflight_types::AppGameChildUxProviderPreflightStatus;
use ocentra_app_game_core::app_game_notification_preference_preflight_bridge_types::AppGameNotificationPreferencePreflightBridgeRow;
use ocentra_app_game_core::app_game_notification_provider_preflight_bridge_types::AppGameNotificationProviderPreflightBridgeRow;
use ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationPreferenceStatusEntry;
use ocentra_parent_agent_protocol::notification_provider_status_boundary::{
    V08NotificationEscalationReadiness, V08NotificationProviderDeliveryClaim,
    V08NotificationProviderStatus, V08NotificationProviderStatusBoundaryEntry,
    V08NotificationProviderStatusProofState, V08NotificationQuietHoursReadiness,
    V08_NOTIFICATION_PROVIDER_STATUS_BOUNDARY_SCHEMA_VERSION,
};

use super::constants::{delivery_result_unobserved, provider_preview_boundary};
pub(super) fn notification_provider_status_entry_from_preflight(
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
        minimal_payload_boundary: provider_preview_boundary(),
        provider_delivery_implemented: false,
        provider_delivery_observed: false,
        delivered_notification_claimed: false,
        sensitive_provider_payload_claimed: false,
        provider_stores_child_evidence_claimed: false,
        last_checked_at: generated_at.to_string(),
    }
}

pub(super) fn notification_preference_status_entry_from_preflight(
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
            ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationPreferenceDeliveryResultState::Unavailable
        } else {
            ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationPreferenceDeliveryResultState::ManualRequired
        },
        parent_preference_state: if unavailable {
            ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationParentPreferenceState::Unavailable
        } else {
            ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationParentPreferenceState::ManualSetupRequired
        },
        quiet_hours_decision: if unavailable {
            ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationQuietHoursDecision::Unavailable
        } else {
            ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationQuietHoursDecision::ManualRequired
        },
        provider_channel: ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationProviderChannel::Unavailable,
        delivery_result_ref: preflight
            .map(|value| value.scheduler_artifact_ref.to_string())
            .unwrap_or_else(delivery_result_unobserved),
        audit_refs: preflight
            .map(|value| refs_to_strings(&value.audit_refs))
            .unwrap_or_default(),
        manual_proof_requirements,
    }
}

fn refs_to_strings<T: ToString>(refs: &[T]) -> Vec<String> {
    refs.iter().map(ToString::to_string).collect()
}
