use std::collections::{HashMap, HashSet};

use ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationPreferenceStatusReadModel;
use ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationStatusReadModels;
use ocentra_parent_agent_protocol::notification_provider_status_boundary::{
    V08NotificationProviderStatusBoundaryReadModel,
    V08_NOTIFICATION_PROVIDER_STATUS_BOUNDARY_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::AppGameNotificationReadinessRow;

use super::super::scheduler_runtime::{
    boundary::VerifiedNotificationPreflight, load_verified_notification_preflight,
};
use super::fallback_entries::{
    notification_preference_status_entry_without_scheduler,
    notification_provider_status_entry_without_scheduler,
};
use super::preflight_entries::{
    notification_preference_status_entry_from_preflight,
    notification_provider_status_entry_from_preflight,
};
use super::surface::notification_parent_surface_intent_read_model;

pub(super) fn notification_status_read_models<T: ToString>(
    rows: &[AppGameNotificationReadinessRow],
    generated_at: T,
) -> AppGameNotificationStatusReadModels {
    let generated_at = generated_at.to_string();
    let scheduler_evidence_invalid = match load_verified_notification_preflight() {
        Ok(Some(preflight)) => {
            if let Some(read_models) =
                notification_status_read_models_from_preflight(&preflight, &generated_at)
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
        &generated_at,
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
