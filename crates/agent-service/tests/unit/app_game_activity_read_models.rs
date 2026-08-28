#![forbid(unsafe_code)]

extern crate self as ocentra_parent_agent_service;

#[path = "../support/activity_capture_mod.rs"]
mod activity_capture;
#[path = "../support/activity_report_env_lock.rs"]
mod activity_report_env_lock;
#[path = "../support/activity_surface_app_game_boundary_fixtures.rs"]
mod activity_surface_app_game_boundary_fixtures;
#[path = "../support/activity_surface_app_game_model_fixtures.rs"]
mod activity_surface_app_game_model_fixtures;
#[path = "../support/activity_surface_common_fixtures.rs"]
mod activity_surface_common_fixtures;
#[path = "../support/app_game_policy_readiness_sources.rs"]
mod app_game_policy_readiness_sources;
#[path = "../support/test_invariants.rs"]
mod test_invariants;
mod test_text {
    use std::{
        fmt::{self, Display},
        primitive::str as TestStr,
        string::String as TestString,
    };

    #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
    pub(crate) struct TestText(pub(crate) TestString);

    impl TestText {
        pub(crate) fn from_display(value: impl Display) -> Self {
            Self(value.to_string())
        }
    }

    impl AsRef<TestStr> for TestText {
        fn as_ref(&self) -> &TestStr {
            self.0.as_str()
        }
    }

    impl AsRef<[u8]> for TestText {
        fn as_ref(&self) -> &[u8] {
            self.0.as_bytes()
        }
    }

    impl Display for TestText {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(formatter)
        }
    }

    impl std::error::Error for TestText {}
}

#[path = "../../src/activity_payload.rs"]
mod activity_payload;
mod activity_api {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) struct ActivityEventId(pub(crate) &'static str);
}
#[path = "../../src/activity_api/activity_store_error_event.rs"]
mod activity_store_error_event;
#[path = "../../src/activity_store_path.rs"]
mod activity_store_path;
#[path = "../../src/activity_surface_read_model_states.rs"]
mod activity_surface_read_model_states;
#[path = "../../src/activity_surface_read_models.rs"]
mod activity_surface_read_models;
#[path = "../../src/activity_surface_store.rs"]
mod activity_surface_store;
#[path = "../support/dev_log.rs"]
mod dev_log;
#[path = "../../src/enforcement_api.rs"]
mod enforcement_api;
#[path = "../../src/enforcement_browser_domain_adapter_app_control_proof_states.rs"]
mod enforcement_browser_domain_adapter_app_control_proof_states;
#[path = "../../src/enforcement_browser_domain_adapter_proof_read_model.rs"]
mod enforcement_browser_domain_adapter_proof_read_model;
#[path = "../../src/enforcement_capability.rs"]
mod enforcement_capability;
#[path = "../../src/enforcement_cross_platform_capability_proof_read_model.rs"]
mod enforcement_cross_platform_capability_proof_read_model;
#[path = "../../src/enforcement_os_adapter_product_proof_read_model.rs"]
mod enforcement_os_adapter_product_proof_read_model;
#[path = "../../src/enforcement_payload.rs"]
mod enforcement_payload;
#[path = "../../src/enforcement_policy_dispatch_read_model.rs"]
mod enforcement_policy_dispatch_read_model;
#[path = "../../src/enforcement_timer_state_file.rs"]
mod enforcement_timer_state_file;
#[path = "../../src/enforcement_timer_state_path.rs"]
mod enforcement_timer_state_path;
#[path = "../../src/event_builder.rs"]
mod event_builder;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../../src/host_identity_read_model.rs"]
mod host_identity_read_model;
#[path = "../../src/json_contract.rs"]
mod json_contract;
#[path = "../../src/time.rs"]
mod time;
#[path = "../../src/windows_adapter_artifact_gate_read_model.rs"]
mod windows_adapter_artifact_gate_read_model;
#[path = "../../src/windows_adapter_capability_read_model.rs"]
mod windows_adapter_capability_read_model;

#[path = "../../src/activity_api/app_game_adapter_dispatch_execute_payload.rs"]
mod app_game_adapter_dispatch_execute_payload;
#[path = "../../src/activity_api/app_game_adapter_dispatch_preflight_payload.rs"]
mod app_game_adapter_dispatch_preflight_payload;
#[path = "../../src/activity_api/app_game_adapter_dispatch_result_fields.rs"]
mod app_game_adapter_dispatch_result_fields;
#[path = "../../src/activity_api/app_game_adapter_dispatch_result_payload.rs"]
mod app_game_adapter_dispatch_result_payload;
#[path = "../../src/activity_api/app_game_adapter_execution_readiness_payload.rs"]
mod app_game_adapter_execution_readiness_payload;
#[path = "../../src/activity_api/app_game_adapter_host_capabilities.rs"]
mod app_game_adapter_host_capabilities;
#[path = "../../src/activity_api/app_game_adapter_host_capabilities_linux.rs"]
mod app_game_adapter_host_capabilities_linux;
#[path = "../../src/activity_api/app_game_adapter_host_capabilities_paths.rs"]
mod app_game_adapter_host_capabilities_paths;
#[path = "../../src/activity_api/app_game_boundary_read_model_payload.rs"]
mod app_game_boundary_read_model_payload;
#[path = "../../src/activity_api/app_game_boundary_read_model_payload_rows.rs"]
mod app_game_boundary_read_model_payload_rows;
#[path = "../../src/activity_api/app_game_child_runtime_transport_receipt_payload.rs"]
mod app_game_child_runtime_transport_receipt_payload;
#[path = "../../src/app_game_dispatch_evidence.rs"]
mod app_game_dispatch_evidence;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight.rs"]
mod app_game_linux_docker_host_preflight;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_cleanup.rs"]
mod app_game_linux_docker_host_preflight_cleanup;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_cleanup_owner.rs"]
mod app_game_linux_docker_host_preflight_cleanup_owner;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_cleanup_process.rs"]
mod app_game_linux_docker_host_preflight_cleanup_process;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_cleanup_worker.rs"]
mod app_game_linux_docker_host_preflight_cleanup_worker;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_group.rs"]
mod app_game_linux_docker_host_preflight_group;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_output.rs"]
mod app_game_linux_docker_host_preflight_output;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_path_security.rs"]
mod app_game_linux_docker_host_preflight_path_security;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_paths.rs"]
mod app_game_linux_docker_host_preflight_paths;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_process.rs"]
mod app_game_linux_docker_host_preflight_process;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_state.rs"]
mod app_game_linux_docker_host_preflight_state;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_supervisor.rs"]
mod app_game_linux_docker_host_preflight_supervisor;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_wait.rs"]
mod app_game_linux_docker_host_preflight_wait;
#[path = "../../src/activity_api/app_game_notification_readiness_payload.rs"]
mod app_game_notification_readiness_payload;
#[path = "../../src/activity_api/app_game_platform_probe_cache.rs"]
mod app_game_platform_probe_cache;
#[path = "../../src/activity_api/app_game_platform_proof_status_payload.rs"]
mod app_game_platform_proof_status_payload;
#[path = "../../src/activity_api/app_game_policy_readiness_payload.rs"]
mod app_game_policy_readiness_payload;
#[path = "../../src/activity_api/app_game_timer_parent_surface_action_results.rs"]
mod app_game_timer_parent_surface_action_results;
#[path = "../../src/activity_api/app_game_timer_parent_surface_payload.rs"]
mod app_game_timer_parent_surface_payload;

#[path = "app_game_activity_read_models_clippy_linkage_tests.rs"]
mod app_game_activity_read_models_clippy_linkage_tests;
#[path = "app_game_adapter_dispatch_preflight_payload_tests.rs"]
mod app_game_adapter_dispatch_preflight_payload_tests;
#[path = "app_game_adapter_dispatch_result_payload_tests.rs"]
mod app_game_adapter_dispatch_result_payload_tests;
#[path = "app_game_adapter_dispatch_result_service_tests.rs"]
mod app_game_adapter_dispatch_result_service_tests;
#[path = "app_game_adapter_execution_readiness_payload_tests.rs"]
mod app_game_adapter_execution_readiness_payload_tests;
#[path = "app_game_adapter_host_capabilities.rs"]
mod app_game_adapter_host_capabilities_tests;
#[path = "app_game_boundary_read_model_payload_tests.rs"]
mod app_game_boundary_read_model_payload_tests;
#[path = "app_game_child_runtime_transport_receipt_payload_tests.rs"]
mod app_game_child_runtime_transport_receipt_payload_tests;
#[path = "app_game_linux_capture_readiness.rs"]
mod app_game_linux_capture_readiness;
#[path = "app_game_linux_docker_host_preflight_cleanup_tests.rs"]
mod app_game_linux_docker_host_preflight_cleanup_tests;
#[path = "app_game_linux_docker_host_preflight_parser_tests.rs"]
mod app_game_linux_docker_host_preflight_parser_tests;
#[path = "app_game_linux_docker_host_preflight_path_security_tests.rs"]
mod app_game_linux_docker_host_preflight_path_security_tests;
#[path = "app_game_linux_docker_host_preflight.rs"]
mod app_game_linux_docker_host_preflight_tests;
#[path = "app_game_linux_source_preflight.rs"]
mod app_game_linux_source_preflight;
#[path = "app_game_notification_readiness_payload_tests.rs"]
mod app_game_notification_readiness_payload_tests;
#[path = "app_game_platform_probe_cache_tests.rs"]
mod app_game_platform_probe_cache_tests;
#[path = "app_game_platform_proof_status_payload_tests.rs"]
mod app_game_platform_proof_status_payload_tests;
#[path = "app_game_platform_proof_status_route_rejection_tests.rs"]
mod app_game_platform_proof_status_route_rejection_tests;
#[path = "app_game_policy_readiness_payload_tests.rs"]
mod app_game_policy_readiness_payload_tests;
#[path = "app_game_timer_parent_surface_payload_tests.rs"]
mod app_game_timer_parent_surface_payload_tests;
#[test]
fn app_game_service_read_models_project_typed_app_and_game_rows() {
    let request = activity_surface_common_fixtures::family_request();
    let app_use = activity_surface_read_models::app_use::app_use_read_model(
        request.clone(),
        Some(activity_surface_app_game_model_fixtures::app_game_service_model()),
    );
    let games = activity_surface_read_models::games::games_read_model(
        request,
        Some(activity_surface_app_game_model_fixtures::app_game_service_model()),
    );

    assert_eq!(
        app_use.state,
        ocentra_parent_agent_protocol::activity_surface::ActivityReadModelState::Ready
    );
    assert_eq!(app_use.rows.len(), 1);
    assert_eq!(
        app_use.rows[0].state,
        ocentra_parent_agent_protocol::activity_surface::ActivityReadModelState::PermissionRequired
    );
    assert_eq!(app_use.rows[0].app_name, "Ocentra Parent App");
    assert_eq!(app_use.rows[0].product_kind, "nativeApp");
    assert_eq!(app_use.rows[0].launch_count, 5);
    assert_eq!(app_use.rows[0].source_status_rows.len(), 3);
    assert!(app_use.rows[0]
        .evidence
        .iter()
        .any(|evidence| evidence.evidence_id == "app-evidence-claim-1"));

    assert_eq!(
        games.state,
        ocentra_parent_agent_protocol::activity_surface::ActivityReadModelState::Ready
    );
    assert_eq!(games.rows.len(), 1);
    assert_eq!(
        games.rows[0].state,
        ocentra_parent_agent_protocol::activity_surface::ActivityReadModelState::PermissionRequired
    );
    assert_eq!(games.rows[0].display_name, "game.exe");
    assert_eq!(games.rows[0].product_kind, "nativeGame");
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
fn app_game_service_read_models_preserve_fail_closed_capability_states() {
    for capability_status in [
        ocentra_parent_agent_protocol::app_game::APP_GAME_CAPABILITY_STATUS_STALE,
        ocentra_parent_agent_protocol::app_game::APP_GAME_CAPABILITY_STATUS_DEGRADED,
        ocentra_parent_agent_protocol::app_game::APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED,
        ocentra_parent_agent_protocol::app_game::APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED,
    ] {
        let mut model = activity_surface_app_game_model_fixtures::app_game_service_model();
        model.capability_status = capability_status.to_string();
        let request = activity_surface_common_fixtures::family_request();
        let app_use = activity_surface_read_models::app_use::app_use_read_model(
            request.clone(),
            Some(model.clone()),
        );
        let games = activity_surface_read_models::games::games_read_model(request, Some(model));

        assert_eq!(app_use.rows[0].capability_status, capability_status);
        assert_eq!(games.rows[0].capability_status, capability_status);
        assert_eq!(
            app_use.rows[0].runtime_state,
            ocentra_parent_agent_protocol::app_game::APP_GAME_RUNTIME_RUNNING
        );
        assert_eq!(
            games.rows[0].foreground_state,
            ocentra_parent_agent_protocol::app_game::APP_GAME_FOREGROUND_FOREGROUND
        );
    }
}

#[test]
fn app_game_service_read_models_keep_remote_device_requests_offline() {
    let request = activity_surface_common_fixtures::remote_device_request();
    let model = activity_surface_app_game_model_fixtures::app_game_service_model();
    let app_use = activity_surface_read_models::app_use::app_use_read_model(
        request.clone(),
        Some(model.clone()),
    );
    let games = activity_surface_read_models::games::games_read_model(request, Some(model));

    assert_eq!(
        app_use.state,
        ocentra_parent_agent_protocol::activity_surface::ActivityReadModelState::Offline
    );
    assert!(app_use.rows.is_empty());
    assert_eq!(
        games.state,
        ocentra_parent_agent_protocol::activity_surface::ActivityReadModelState::Offline
    );
    assert!(games.rows.is_empty());
}
