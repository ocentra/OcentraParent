#[path = "fallback_preference.rs"]
mod preference;
#[path = "fallback_provider.rs"]
mod provider;

use ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationPreferenceStatusEntry;
use ocentra_parent_agent_protocol::notification_provider_status_boundary::V08NotificationProviderStatusBoundaryEntry;
use ocentra_parent_agent_protocol::AppGameNotificationReadinessRow;

pub(super) fn notification_provider_status_entry_without_scheduler(
    row: &AppGameNotificationReadinessRow,
    generated_at: &str,
    scheduler_evidence_invalid: bool,
) -> V08NotificationProviderStatusBoundaryEntry {
    provider::build(row, generated_at, scheduler_evidence_invalid)
}

pub(super) fn notification_preference_status_entry_without_scheduler(
    row: &AppGameNotificationReadinessRow,
    scheduler_evidence_invalid: bool,
) -> AppGameNotificationPreferenceStatusEntry {
    preference::build(row, scheduler_evidence_invalid)
}
