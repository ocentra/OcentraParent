use std::collections::HashSet;

use ocentra_parent_agent_protocol::app_game_notification_parent_surface_intent::{
    AppGameNotificationFamilyReference, AppGameNotificationParentSurfaceIntentOptions,
    AppGameNotificationParentSurfaceIntentReadModel,
    AppGameNotificationPreferenceStatusHandoffReadModel,
    AppGameNotificationPreferenceStatusHandoffRow,
    AppGameNotificationProviderStatusBoundaryEntry as ParentSurfaceProviderStatusEntry,
    AppGameNotificationProviderStatusHandoffReadModel, AppGameNotificationProviderStatusHandoffRow,
};
use ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationPreferenceStatusReadModel;
use ocentra_parent_agent_protocol::notification_provider_status_boundary::V08NotificationProviderStatusBoundaryReadModel;

use super::constants::FAMILY_ID;
use super::labels::provider_status_label;
use super::status_entries::preference_status_handoff_entry;

pub(super) fn notification_parent_surface_intent_read_model(
    provider_read_model: &V08NotificationProviderStatusBoundaryReadModel,
    preference_read_model: &AppGameNotificationPreferenceStatusReadModel,
    generated_at: &str,
) -> Option<AppGameNotificationParentSurfaceIntentReadModel> {
    if !matching_readiness_refs(provider_read_model, preference_read_model) {
        return None;
    }
    let family = AppGameNotificationFamilyReference {
        family_id: FAMILY_ID.to_string(),
    };
    let provider_handoff = provider_handoff(provider_read_model, family.clone());
    let mut preference_handoff = preference_handoff(preference_read_model, family);
    if provider_handoff.rows.len() != preference_handoff.rows.len() {
        return None;
    }
    preference_handoff.rows =
        ordered_preference_rows(provider_read_model, &provider_handoff, &preference_handoff)?;
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

fn matching_readiness_refs(
    provider_read_model: &V08NotificationProviderStatusBoundaryReadModel,
    preference_read_model: &AppGameNotificationPreferenceStatusReadModel,
) -> bool {
    let provider_refs = provider_read_model
        .entries
        .iter()
        .filter_map(|entry| entry.readiness_refs.first().cloned())
        .collect::<HashSet<_>>();
    let preference_refs = preference_read_model
        .entries
        .iter()
        .map(|entry| entry.readiness_ref.clone())
        .filter(|reference| !reference.is_empty())
        .collect::<HashSet<_>>();
    provider_refs.len() == provider_read_model.entries.len()
        && preference_refs.len() == preference_read_model.entries.len()
        && provider_refs == preference_refs
}

fn provider_handoff(
    read_model: &V08NotificationProviderStatusBoundaryReadModel,
    family: AppGameNotificationFamilyReference,
) -> AppGameNotificationProviderStatusHandoffReadModel {
    AppGameNotificationProviderStatusHandoffReadModel {
        handoff_id: read_model.read_model_id.clone(),
        family,
        rows: read_model
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
    }
}

fn preference_handoff(
    read_model: &AppGameNotificationPreferenceStatusReadModel,
    family: AppGameNotificationFamilyReference,
) -> AppGameNotificationPreferenceStatusHandoffReadModel {
    AppGameNotificationPreferenceStatusHandoffReadModel {
        handoff_id: read_model.read_model_id.clone(),
        family,
        rows: read_model
            .entries
            .iter()
            .map(|entry| AppGameNotificationPreferenceStatusHandoffRow {
                handoff_row_id: format!("app-game-preference-status-entry:{}", entry.readiness_ref),
                source_scheduler_entry_ref: None,
                source_outbox_record_ref: None,
                notification_preference_status_entry: preference_status_handoff_entry(entry),
            })
            .collect(),
    }
}

fn ordered_preference_rows(
    provider_read_model: &V08NotificationProviderStatusBoundaryReadModel,
    provider_handoff: &AppGameNotificationProviderStatusHandoffReadModel,
    preference_handoff: &AppGameNotificationPreferenceStatusHandoffReadModel,
) -> Option<Vec<AppGameNotificationPreferenceStatusHandoffRow>> {
    let mut readiness_refs = HashSet::with_capacity(provider_handoff.rows.len());
    let mut ordered_rows = Vec::with_capacity(provider_handoff.rows.len());
    for provider_row in &provider_handoff.rows {
        let readiness_ref = provider_read_model
            .entries
            .iter()
            .find(|entry| entry.status_entry_id == provider_row.handoff_row_id)
            .and_then(|entry| entry.readiness_refs.first())?;
        if !readiness_refs.insert(readiness_ref.clone()) {
            return None;
        }
        let preference_row = preference_handoff.rows.iter().find(|row| {
            row.handoff_row_id == format!("app-game-preference-status-entry:{readiness_ref}")
        })?;
        ordered_rows.push(preference_row.clone());
    }
    Some(ordered_rows)
}
