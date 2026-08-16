use ocentra_parent_agent_protocol::app_game_notification_parent_surface_intent::AppGameNotificationPreferenceStatusHandoffEntry as ParentSurfacePreferenceStatusEntry;
use ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationPreferenceStatusEntry;

use super::labels::{
    parent_preference_state_label, preference_delivery_result_label, provider_channel_label,
    quiet_hours_decision_label,
};

pub(super) fn preference_status_handoff_entry(
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
