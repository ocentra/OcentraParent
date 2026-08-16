use ocentra_parent_agent_protocol::app_game_notification_status::{
    AppGameNotificationParentPreferenceState, AppGameNotificationPreferenceDeliveryResultState,
    AppGameNotificationPreferenceStatusEntry, AppGameNotificationProviderChannel,
    AppGameNotificationQuietHoursDecision,
};
use ocentra_parent_agent_protocol::{
    AppGameNotificationReadinessRow, APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE,
};

use super::super::constants::*;

pub(super) fn build(
    row: &AppGameNotificationReadinessRow,
    scheduler_evidence_invalid: bool,
) -> AppGameNotificationPreferenceStatusEntry {
    let unavailable = row.readiness_state == APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE;
    let scheduler_evidence_ref =
        super::provider::scheduler_evidence_ref(unavailable, scheduler_evidence_invalid);
    AppGameNotificationPreferenceStatusEntry {
        readiness_ref: row.row_id.clone(),
        delivery_result_state: delivery_result_state(unavailable),
        parent_preference_state: parent_preference_state(unavailable),
        quiet_hours_decision: quiet_hours_decision(unavailable),
        provider_channel: AppGameNotificationProviderChannel::Unavailable,
        delivery_result_ref: format!("delivery-result-not-observed:{}", row.row_id),
        audit_refs: row.evidence_reference_ids.clone(),
        manual_proof_requirements: if unavailable {
            vec![
                MANUAL_PROVIDER_AVAILABILITY.to_string(),
                scheduler_evidence_ref.to_string(),
            ]
        } else {
            vec![
                MANUAL_PARENT_PREFERENCE.to_string(),
                MANUAL_NOTIFICATION_CHANNEL.to_string(),
                scheduler_evidence_ref.to_string(),
            ]
        },
    }
}

fn delivery_result_state(unavailable: bool) -> AppGameNotificationPreferenceDeliveryResultState {
    if unavailable {
        AppGameNotificationPreferenceDeliveryResultState::Unavailable
    } else {
        AppGameNotificationPreferenceDeliveryResultState::ManualRequired
    }
}

fn parent_preference_state(unavailable: bool) -> AppGameNotificationParentPreferenceState {
    if unavailable {
        AppGameNotificationParentPreferenceState::Unavailable
    } else {
        AppGameNotificationParentPreferenceState::ManualSetupRequired
    }
}

fn quiet_hours_decision(unavailable: bool) -> AppGameNotificationQuietHoursDecision {
    if unavailable {
        AppGameNotificationQuietHoursDecision::Unavailable
    } else {
        AppGameNotificationQuietHoursDecision::ManualRequired
    }
}
