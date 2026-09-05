use ocentra_parent_agent_protocol::activity_surface::ActivityReadModelState;
use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_CAPABILITY_STATUS_DEGRADED, APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED,
    APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED, APP_GAME_CAPABILITY_STATUS_STALE,
    APP_GAME_FOREGROUND_FOREGROUND, APP_GAME_PRODUCT_NATIVE_APP, APP_GAME_PRODUCT_NATIVE_GAME,
    APP_GAME_RUNTIME_RUNNING,
};
use ocentra_parent_agent_protocol::constants;

use crate::activity_surface_app_game_model_fixtures::app_game_service_model;
use crate::activity_surface_common_fixtures::{family_request, remote_device_request};
use crate::activity_surface_read_model_fixtures::{
    browser_read_model_fixture, network_read_model_fixture, recent_summary, screen_summary,
};

#[test]
fn screen_result_conversion_preserves_identity_state_and_evidence() {
    let mut results = screen_summary(1).results;
    assert_eq!(results.len(), 1);
    let row =
        crate::activity_surface_read_models::activity_screen_row_from_result(results.remove(0));

    assert_eq!(row.row_id, "screen-result-1");
    assert_eq!(row.label, "Screen summary");
    assert_eq!(
        row.device_id,
        constants::activity_surface::DEFAULT_DEVICE_ID
    );
    assert_eq!(row.state, ActivityReadModelState::Ready);
    assert_eq!(row.queue_job_id, "queue-job-1");
    assert_eq!(
        row.policy_decision_ref.as_deref(),
        Some("policy-decision-1")
    );
    assert_eq!(row.evidence.len(), 1);
    assert_eq!(row.evidence[0].evidence_id, "screen-evidence-1");
}

#[test]
fn screen_read_model_projects_ready_empty_unavailable_and_offline_states() {
    let ready = crate::activity_surface_read_models::screen_read_model(
        family_request(),
        Some(screen_summary(1)),
    );
    assert_eq!(ready.state, ActivityReadModelState::Ready);
    assert_eq!(ready.summary, "Screen summary");
    assert_eq!(ready.rows.len(), 1);
    assert_eq!(ready.rows[0].row_id, "screen-result-1");
    assert_eq!(
        ready.rows[0].device_id,
        constants::activity_surface::DEFAULT_DEVICE_ID
    );

    let empty = crate::activity_surface_read_models::screen_read_model(
        family_request(),
        Some(screen_summary(0)),
    );
    assert_eq!(empty.state, ActivityReadModelState::Empty);
    assert_eq!(empty.summary, constants::activity_surface::SUMMARY_EMPTY);
    assert!(empty.rows.is_empty());

    let unavailable =
        crate::activity_surface_read_models::screen_read_model(family_request(), None);
    assert_eq!(unavailable.state, ActivityReadModelState::Unavailable);

    let offline = crate::activity_surface_read_models::screen_read_model(
        remote_device_request(),
        Some(screen_summary(1)),
    );
    assert_eq!(offline.state, ActivityReadModelState::Offline);
}

#[test]
fn browser_and_network_read_models_project_ready_empty_unavailable_and_offline_states() {
    let browser_ready = crate::activity_surface_read_models::browser_read_model(
        family_request(),
        Some(browser_read_model_fixture(1)),
    );
    assert_eq!(browser_ready.state, ActivityReadModelState::Ready);
    assert_eq!(browser_ready.rows[0].domain_label, "example.com");

    let browser_empty = crate::activity_surface_read_models::browser_read_model(
        family_request(),
        Some(browser_read_model_fixture(0)),
    );
    assert_eq!(browser_empty.state, ActivityReadModelState::Empty);

    let browser_unavailable =
        crate::activity_surface_read_models::browser_read_model(family_request(), None);
    assert_eq!(
        browser_unavailable.state,
        ActivityReadModelState::Unavailable
    );

    let browser_offline = crate::activity_surface_read_models::browser_read_model(
        remote_device_request(),
        Some(browser_read_model_fixture(1)),
    );
    assert_eq!(browser_offline.state, ActivityReadModelState::Offline);

    let network_ready = crate::activity_surface_read_models::network_read_model(
        family_request(),
        Some(network_read_model_fixture(1)),
    );
    assert_eq!(network_ready.state, ActivityReadModelState::Ready);
    assert_eq!(network_ready.rows[0].destination_label, "api.example.com");
    assert_eq!(network_ready.rows[0].connection_count, 3);
    assert_eq!(network_ready.rows[0].total_bytes, 120);
    assert_eq!(
        network_ready.rows[0].evidence_digest.as_deref(),
        Some("network-digest-1")
    );

    let network_empty = crate::activity_surface_read_models::network_read_model(
        family_request(),
        Some(network_read_model_fixture(0)),
    );
    assert_eq!(network_empty.state, ActivityReadModelState::Empty);

    let network_unavailable =
        crate::activity_surface_read_models::network_read_model(family_request(), None);
    assert_eq!(
        network_unavailable.state,
        ActivityReadModelState::Unavailable
    );

    let network_offline = crate::activity_surface_read_models::network_read_model(
        remote_device_request(),
        Some(network_read_model_fixture(1)),
    );
    assert_eq!(network_offline.state, ActivityReadModelState::Offline);
}

#[test]
fn app_use_read_model_projects_recent_summary_and_remote_device_paths() {
    let ready = crate::activity_surface_read_models::app_use::app_use_read_model(
        family_request(),
        Some(recent_summary(2)),
    );
    assert_eq!(ready.state, ActivityReadModelState::Ready);
    assert_eq!(ready.rows[0].row_id, "recent-event-1");
    assert_eq!(ready.rows[0].app_name, "Recent App");
    assert_eq!(ready.rows[0].launch_count, 2);

    let empty = crate::activity_surface_read_models::app_use::app_use_read_model(
        family_request(),
        Some(recent_summary(0)),
    );
    assert_eq!(empty.state, ActivityReadModelState::Empty);

    let unavailable = crate::activity_surface_read_models::app_use::app_use_read_model(
        family_request(),
        None::<ocentra_parent_agent_protocol::activity_query::ActivityRecentSummary>,
    );
    assert_eq!(unavailable.state, ActivityReadModelState::Unavailable);

    let offline = crate::activity_surface_read_models::app_use::app_use_read_model(
        remote_device_request(),
        Some(recent_summary(2)),
    );
    assert_eq!(offline.state, ActivityReadModelState::Offline);
}

#[test]
fn app_use_read_model_exercises_shared_boundary_and_source_status_helpers() {
    let service_model = app_game_service_model();

    let app_use = crate::activity_surface_read_models::app_use::app_use_read_model(
        family_request(),
        Some(service_model),
    );
    assert_eq!(app_use.state, ActivityReadModelState::Ready);
    assert_eq!(app_use.rows[0].app_name, "Ocentra Parent App");
    assert_eq!(
        app_use.rows[0].state,
        ActivityReadModelState::PermissionRequired
    );
    assert_eq!(app_use.rows[0].product_kind, APP_GAME_PRODUCT_NATIVE_APP);
    assert_eq!(app_use.rows[0].launch_count, 5);
    assert_eq!(app_use.rows[0].source_status_rows.len(), 3);
    assert_eq!(app_use.rows[0].evidence_claim_row_count, 1);
    assert_eq!(app_use.rows[0].identity_row_count, 1);
    assert_eq!(app_use.rows[0].approval_authority_row_count, 0);
    assert_eq!(app_use.rows[0].approval_action_result_row_count, 0);
    assert_eq!(app_use.rows[0].platform_authority_matrix_count, 0);
    assert_eq!(app_use.rows[0].ai_classifier_result_row_count, 0);
    assert!(app_use.rows[0]
        .evidence
        .iter()
        .any(|evidence| evidence.evidence_id == "app-evidence-claim-1"));
}

#[test]
fn games_read_model_exercises_shared_boundary_and_source_status_helpers() {
    let service_model = app_game_service_model();
    let games = crate::activity_surface_read_models::games::games_read_model(
        family_request(),
        Some(service_model),
    );
    assert_eq!(games.state, ActivityReadModelState::Ready);
    assert_eq!(games.rows[0].display_name, "game.exe");
    assert_eq!(games.rows[0].product_kind, APP_GAME_PRODUCT_NATIVE_GAME);
    assert_eq!(
        games.rows[0].state,
        ActivityReadModelState::PermissionRequired
    );
    assert_eq!(games.rows[0].launcher_row_count, 1);
    assert_eq!(games.rows[0].daily_rollup_count, 1);
    assert_eq!(games.rows[0].total_ms, 4200);
    assert_eq!(games.rows[0].session_count, 2);
    assert_eq!(games.rows[0].source_status_rows.len(), 4);
    assert!(games.rows[0]
        .evidence
        .iter()
        .any(|evidence| evidence.evidence_id == "app-identity-1"));
}

#[test]
fn app_game_read_models_preserve_fail_closed_capability_states() {
    for capability_status in [
        APP_GAME_CAPABILITY_STATUS_STALE,
        APP_GAME_CAPABILITY_STATUS_DEGRADED,
        APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED,
        APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED,
    ] {
        let mut model = app_game_service_model();
        model.capability_status = capability_status.to_string();
        let app_use = crate::activity_surface_read_models::app_use::app_use_read_model(
            family_request(),
            Some(model.clone()),
        );
        let games = crate::activity_surface_read_models::games::games_read_model(
            family_request(),
            Some(model),
        );

        assert_eq!(app_use.rows[0].capability_status, capability_status);
        assert_eq!(games.rows[0].capability_status, capability_status);
        assert_eq!(app_use.rows[0].runtime_state, APP_GAME_RUNTIME_RUNNING);
        assert_eq!(
            games.rows[0].foreground_state,
            APP_GAME_FOREGROUND_FOREGROUND
        );
    }
}
