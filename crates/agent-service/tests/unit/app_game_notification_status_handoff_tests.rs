use ocentra_parent_agent_protocol::app_game_notification_parent_surface_intent::AppGameNotificationParentSurfaceIntentReadModel;
use ocentra_parent_agent_protocol::app_game_notification_status::{
    AppGameNotificationParentPreferenceState, AppGameNotificationPreferenceDeliveryResultState,
    AppGameNotificationQuietHoursDecision, AppGameNotificationStatusReadModels,
};
use ocentra_parent_agent_protocol::notification_provider_status_boundary::V08NotificationProviderStatus;

use super::app_game_notification_readiness_payload_tests::service_model;
use super::app_game_notification_readiness_report::app_game_notification_readiness_report_from_service_model_with_activity_db_path;
use super::app_game_notification_status_handoff_fixture::NotificationStatusFixture;

#[test]
fn persisted_scheduler_handoff_projects_paired_provider_preference_and_parent_status(
) -> Result<(), Box<dyn std::error::Error>> {
    let fixture = NotificationStatusFixture::persisted()?;
    let report = app_game_notification_readiness_report_from_service_model_with_activity_db_path(
        service_model(),
        fixture.activity_db_path(),
    );
    let status = report.status_read_models;

    assert_source_models(&status, fixture.scheduler_bridge_id());
    assert_status_mappings(&status);
    assert_preserved_references(&status, &fixture)?;
    let parent_surface = status.parent_surface_intent.as_ref().ok_or_else(|| {
        invalid_fixture("verified paired rows must produce a parent-surface intent")
    })?;
    assert_parent_surface(parent_surface, &status);
    Ok(())
}

fn assert_source_models(status: &AppGameNotificationStatusReadModels, scheduler_bridge_id: &str) {
    assert_eq!(
        status.provider_status_boundary.source_read_model_ids,
        vec![format!("service-provider:{scheduler_bridge_id}")]
    );
    assert_eq!(
        status.preference_status.source_read_model_ids,
        vec![format!("service-preference:{scheduler_bridge_id}")]
    );
}

fn assert_status_mappings(status: &AppGameNotificationStatusReadModels) {
    assert_eq!(
        status
            .provider_status_boundary
            .entries
            .iter()
            .map(|entry| entry.provider_status)
            .collect::<Vec<_>>(),
        vec![
            V08NotificationProviderStatus::ManualRequired,
            V08NotificationProviderStatus::ManualRequired,
            V08NotificationProviderStatus::Unavailable,
        ]
    );
    assert_eq!(
        status
            .preference_status
            .entries
            .iter()
            .map(|entry| entry.delivery_result_state)
            .collect::<Vec<_>>(),
        vec![
            AppGameNotificationPreferenceDeliveryResultState::ManualRequired,
            AppGameNotificationPreferenceDeliveryResultState::ManualRequired,
            AppGameNotificationPreferenceDeliveryResultState::Unavailable,
        ]
    );
    assert_eq!(
        status
            .preference_status
            .entries
            .iter()
            .map(|entry| entry.parent_preference_state)
            .collect::<Vec<_>>(),
        vec![
            AppGameNotificationParentPreferenceState::ManualSetupRequired,
            AppGameNotificationParentPreferenceState::ManualSetupRequired,
            AppGameNotificationParentPreferenceState::Unavailable,
        ]
    );
    assert_eq!(
        status
            .preference_status
            .entries
            .iter()
            .map(|entry| entry.quiet_hours_decision)
            .collect::<Vec<_>>(),
        vec![
            AppGameNotificationQuietHoursDecision::ManualRequired,
            AppGameNotificationQuietHoursDecision::ManualRequired,
            AppGameNotificationQuietHoursDecision::Unavailable,
        ]
    );
}

fn assert_preserved_references(
    status: &AppGameNotificationStatusReadModels,
    fixture: &NotificationStatusFixture,
) -> Result<(), std::io::Error> {
    let scheduled_record = fixture.scheduled_record()?;
    let provider_entry = status
        .provider_status_boundary
        .entries
        .first()
        .ok_or_else(|| invalid_fixture("provider status must retain the scheduled row"))?;
    let preference_entry = status
        .preference_status
        .entries
        .first()
        .ok_or_else(|| invalid_fixture("preference status must retain the scheduled row"))?;
    assert_eq!(provider_entry.audit_refs, vec!["audit-65"]);
    assert_eq!(preference_entry.audit_refs, vec!["audit-65"]);
    assert_eq!(
        preference_entry.delivery_result_ref,
        scheduled_record.scheduler_artifact_ref.to_string()
    );
    assert_eq!(
        provider_entry.readiness_refs,
        vec![preference_entry.readiness_ref.clone()]
    );
    assert_eq!(provider_entry.manual_proof_requirements.len(), 3);
    assert_eq!(preference_entry.manual_proof_requirements.len(), 3);
    Ok(())
}

fn assert_parent_surface(
    parent_surface: &AppGameNotificationParentSurfaceIntentReadModel,
    status: &AppGameNotificationStatusReadModels,
) {
    assert_eq!(parent_surface.rows.len(), 3);
    assert_eq!(parent_surface.manual_action_required_count, 2);
    assert_eq!(parent_surface.unavailable_visible_count, 1);
    assert_eq!(parent_surface.history_visible_count, 3);
    assert_eq!(parent_surface.preference_setup_required_count, 3);
    assert_eq!(
        parent_surface.source_contract_refs,
        vec![
            status.provider_status_boundary.read_model_id.clone(),
            status.preference_status.read_model_id.clone(),
        ]
    );
    assert_eq!(
        [
            parent_surface.parent_notification_ui_rendered,
            parent_surface.parent_preference_ui_rendered,
            parent_surface.parent_frequency_control_ui_rendered,
            parent_surface.provider_delivery_runtime_claimed,
            parent_surface.provider_receipt_ingestion_claimed,
            parent_surface.provider_credentials_claimed,
            parent_surface.cloud_routing_claimed,
            parent_surface.child_delivery_claimed,
            parent_surface.production_runtime_claimed,
            parent_surface.production_durable_outbox_storage_claimed,
            parent_surface.adapter_dispatch_claimed,
        ],
        [false; 11]
    );
}

fn invalid_fixture(message: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, message)
}
