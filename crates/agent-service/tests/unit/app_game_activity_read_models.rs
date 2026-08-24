#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../support/activity_capture_mod.rs"]
mod activity_capture;
#[path = "../support/activity_report_env_lock.rs"]
mod activity_report_env_lock;
#[path = "../support/app_game_policy_readiness_sources.rs"]
mod app_game_policy_readiness_sources;
#[path = "../support/test_invariants.rs"]
mod test_invariants;
#[path = "../support/command_dispatch_test_support.rs"]
pub mod test_support;

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
#[path = "../../src/activity_api/app_game_notification_readiness_payload.rs"]
mod app_game_notification_readiness_payload;
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
#[path = "app_game_adapter_dispatch_preflight_service_tests.rs"]
mod app_game_adapter_dispatch_preflight_service_tests;
#[path = "app_game_adapter_dispatch_result_payload_tests.rs"]
mod app_game_adapter_dispatch_result_payload_tests;
#[path = "app_game_adapter_dispatch_result_service_tests.rs"]
mod app_game_adapter_dispatch_result_service_tests;
#[path = "app_game_adapter_execution_readiness_payload_tests.rs"]
mod app_game_adapter_execution_readiness_payload_tests;
#[path = "app_game_adapter_execution_readiness_service_tests.rs"]
mod app_game_adapter_execution_readiness_service_tests;
#[path = "app_game_boundary_read_model_payload_tests.rs"]
mod app_game_boundary_read_model_payload_tests;
#[path = "app_game_boundary_read_model_service_tests.rs"]
mod app_game_boundary_read_model_service_tests;
#[path = "app_game_child_runtime_transport_receipt_payload_tests.rs"]
mod app_game_child_runtime_transport_receipt_payload_tests;
#[path = "app_game_child_runtime_transport_receipt_service_tests.rs"]
mod app_game_child_runtime_transport_receipt_service_tests;
#[path = "app_game_notification_readiness_payload_tests.rs"]
mod app_game_notification_readiness_payload_tests;
#[path = "app_game_notification_readiness_service_tests.rs"]
mod app_game_notification_readiness_service_tests;
#[path = "app_game_platform_proof_status_payload_tests.rs"]
mod app_game_platform_proof_status_payload_tests;
#[path = "app_game_platform_proof_status_service_tests.rs"]
mod app_game_platform_proof_status_service_tests;
#[path = "app_game_policy_readiness_payload_tests.rs"]
mod app_game_policy_readiness_payload_tests;
#[path = "app_game_policy_readiness_service_tests.rs"]
mod app_game_policy_readiness_service_tests;
#[path = "app_game_timer_parent_surface_payload_tests.rs"]
mod app_game_timer_parent_surface_payload_tests;
