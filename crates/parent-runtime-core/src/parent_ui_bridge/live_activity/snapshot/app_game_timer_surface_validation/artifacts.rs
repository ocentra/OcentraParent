use std::collections::HashSet;

use ocentra_parent_agent_protocol::app_game::APP_GAME_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::app_game_timer_parent_surface_read_model::{
    AppGameTimerParentSurfaceChildUxLocalArtifactRecord, AppGameTimerParentSurfaceReadModel,
    APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP,
    APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
};

pub(super) fn valid(read_model: &AppGameTimerParentSurfaceReadModel) -> bool {
    let records = &read_model.child_ux_local_handoff_artifact_records;
    let mut artifact_ids = HashSet::<String>::new();
    counts_match(read_model, records)
        && records
            .iter()
            .zip(
                read_model
                    .child_ux_local_handoff_artifact_reference_ids
                    .iter(),
            )
            .all(|pair| valid_record_pair(pair, &mut artifact_ids))
}

fn counts_match(
    read_model: &AppGameTimerParentSurfaceReadModel,
    records: &[AppGameTimerParentSurfaceChildUxLocalArtifactRecord],
) -> bool {
    read_model.child_ux_local_handoff_artifact_record_count == records.len() as u64
        && read_model.child_ux_local_handoff_artifact_record_count
            == read_model
                .child_ux_local_handoff_artifact_reference_ids
                .len() as u64
        && read_model.child_ux_local_handoff_artifact_record_count
            == read_model.child_ux_handoff_ready_count
        && read_model.child_ux_local_handoff_artifact_skipped_count
            == read_model.child_ux_handoff_blocked_count
}

fn valid_record_pair(
    pair: (
        &AppGameTimerParentSurfaceChildUxLocalArtifactRecord,
        &String,
    ),
    artifact_ids: &mut HashSet<String>,
) -> bool {
    let (record, reference_id) = pair;
    valid_identity(record)
        && valid_references(record)
        && reference_id == &record.artifact_reference_id
        && artifact_ids.insert(record.artifact_reference_id.clone())
}

fn valid_identity(record: &AppGameTimerParentSurfaceChildUxLocalArtifactRecord) -> bool {
    record.schema_version == APP_GAME_SCHEMA_VERSION
        && !record.source_result_id.trim().is_empty()
        && !record.artifact_reference_id.trim().is_empty()
        && matches!(
            record.target_domain.as_str(),
            APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP
                | APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME
        )
}

fn valid_references(record: &AppGameTimerParentSurfaceChildUxLocalArtifactRecord) -> bool {
    !record.child_reason_reference_ids.is_empty()
        && !record.child_status_reference_ids.is_empty()
        && record
            .child_reason_reference_ids
            .iter()
            .all(|id| !id.trim().is_empty())
        && record
            .child_status_reference_ids
            .iter()
            .all(|id| !id.trim().is_empty())
        && !has_duplicate_refs(&record.child_reason_reference_ids)
        && !has_duplicate_refs(&record.child_status_reference_ids)
}

fn has_duplicate_refs(refs: &[String]) -> bool {
    let mut seen = HashSet::new();
    refs.iter()
        .any(|reference| !seen.insert(reference.as_str()))
}

pub(super) fn claims_are_clear(read_model: &AppGameTimerParentSurfaceReadModel) -> bool {
    artifact_claims_are_clear(read_model)
        && intent_claims_are_clear(read_model)
        && preference_claims_are_clear(read_model)
}

fn artifact_claims_are_clear(read_model: &AppGameTimerParentSurfaceReadModel) -> bool {
    read_model
        .child_ux_local_handoff_artifact_records
        .iter()
        .all(|record| {
            !record.child_delivery_claimed
                && !record.notification_delivery_claimed
                && !record.adapter_dispatch_claimed
                && !record.platform_enforcement_claimed
                && !record.raw_private_source_rows_included
        })
}

fn intent_claims_are_clear(read_model: &AppGameTimerParentSurfaceReadModel) -> bool {
    read_model
        .child_ux_parent_surface_intent_records
        .iter()
        .all(|record| {
            !record.sensitive_detail_included
                && !record.parent_notification_ui_rendered
                && !record.parent_preference_mutation_claimed
                && !record.provider_delivery_claimed
                && !record.child_delivery_claimed
                && !record.adapter_dispatch_claimed
                && !record.platform_enforcement_claimed
                && !record.raw_private_source_rows_included
        })
}

fn preference_claims_are_clear(read_model: &AppGameTimerParentSurfaceReadModel) -> bool {
    read_model
        .child_ux_parent_preference_setup_records
        .iter()
        .all(|record| {
            !record.parent_preference_ui_rendered
                && !record.parent_frequency_control_ui_rendered
                && !record.parent_preference_mutation_claimed
                && !record.notification_rule_mutation_claimed
                && !record.provider_delivery_claimed
                && !record.child_delivery_claimed
                && !record.adapter_dispatch_claimed
                && !record.platform_enforcement_claimed
                && !record.raw_private_source_rows_included
        })
}
