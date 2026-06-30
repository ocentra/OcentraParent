use ocentra_parent_agent_protocol::*;

#[path = "contract/activity_memory_graph_tests.rs"]
mod activity_memory_graph_tests;
#[path = "contract/activity_query_tests.rs"]
mod activity_query_tests;
#[path = "contract/activity_surface_tests.rs"]
mod activity_surface_tests;
#[path = "contract/activity_tests.rs"]
mod activity_tests;
#[path = "contract/app_game_adapter_dispatch_preflight.rs"]
mod app_game_adapter_dispatch_preflight;
#[path = "contract/app_game_adapter_dispatch_result.rs"]
mod app_game_adapter_dispatch_result;
#[path = "contract/app_game_adapter_execution_readiness.rs"]
mod app_game_adapter_execution_readiness;
#[path = "contract/app_game_boundary_read_model_tests.rs"]
mod app_game_boundary_read_model_tests;
#[path = "contract/app_game_child_runtime_transport_receipt_tests.rs"]
mod app_game_child_runtime_transport_receipt_tests;
#[path = "contract/app_game_notification_readiness_tests.rs"]
mod app_game_notification_readiness_tests;
#[path = "contract/app_game_platform_proof_status_tests.rs"]
mod app_game_platform_proof_status_tests;
#[path = "contract/app_game_policy_readiness_tests.rs"]
mod app_game_policy_readiness_tests;
#[path = "contract/app_game_timer_parent_surface_read_model_tests.rs"]
mod app_game_timer_parent_surface_read_model_tests;
#[path = "contract/browser_inventory_tests.rs"]
mod browser_inventory_tests;
#[path = "contract/browser_managed_tests.rs"]
mod browser_managed_tests;
#[path = "contract/browser_read_model_tests.rs"]
mod browser_read_model_tests;
#[path = "contract/child_agent_event_tests.rs"]
mod child_agent_event_tests;
#[path = "contract/child_domain_runtime_events.rs"]
mod child_domain_runtime_events;
#[path = "contract/child_domain_runtime_tests.rs"]
mod child_domain_runtime_tests;
#[path = "contract/enforcement_audit_boundary_tests.rs"]
mod enforcement_audit_boundary_tests;
#[path = "contract/enforcement_broad_adapter_proof_tests.rs"]
mod enforcement_broad_adapter_proof_tests;
#[path = "contract/enforcement_browser_domain_adapter_proof_tests.rs"]
mod enforcement_browser_domain_adapter_proof_tests;
#[path = "contract/enforcement_cross_platform_capability_proof_tests.rs"]
mod enforcement_cross_platform_capability_proof_tests;
#[path = "contract/enforcement_integrity_runtime_audit_tests.rs"]
mod enforcement_integrity_runtime_audit_tests;
#[path = "contract/enforcement_os_adapter_product_proof_tests.rs"]
mod enforcement_os_adapter_product_proof_tests;
#[path = "contract/enforcement_readiness_tests.rs"]
mod enforcement_readiness_tests;
#[path = "contract/enforcement_supported_adapter_runtime_proof_tests.rs"]
mod enforcement_supported_adapter_runtime_proof_tests;
#[path = "contract/generated_non_lan_agent_protocol_fields.rs"]
mod generated_non_lan_agent_protocol_fields;
#[path = "contract/host_identity_tests.rs"]
mod host_identity_tests;
#[path = "contract/household_mesh.rs"]
mod household_mesh;
#[path = "contract/integrity_alert_status_bridge_tests.rs"]
mod integrity_alert_status_bridge_tests;
#[path = "contract/lan_pairing.rs"]
mod lan_pairing;
#[path = "contract/lan_pairing_browser_add_device_state.rs"]
mod lan_pairing_browser_add_device_state;
#[path = "contract/lan_pairing_browser_runtime.rs"]
mod lan_pairing_browser_runtime;
#[path = "contract/lan_pairing_device_roles.rs"]
mod lan_pairing_device_roles;
#[path = "contract/lan_pairing_household_proof.rs"]
mod lan_pairing_household_proof;
#[path = "contract/lan_pairing_provider_selection.rs"]
mod lan_pairing_provider_selection;
#[path = "contract/network_android_vpn_service_gate_status_tests.rs"]
mod network_android_vpn_service_gate_status_tests;
#[path = "contract/network_apple_network_extension_gate_status_tests.rs"]
mod network_apple_network_extension_gate_status_tests;
#[path = "contract/network_flow_tests.rs"]
mod network_flow_tests;
#[path = "contract/network_linux_nftables_lab_status_tests.rs"]
mod network_linux_nftables_lab_status_tests;
#[path = "contract/network_live_capture_status_tests.rs"]
mod network_live_capture_status_tests;
#[path = "contract/network_windows_firewall_lab_status_tests.rs"]
mod network_windows_firewall_lab_status_tests;
#[path = "contract/network_windows_wfp_gate_status_tests.rs"]
mod network_windows_wfp_gate_status_tests;
#[path = "contract/notification_provider_status_boundary_tests.rs"]
mod notification_provider_status_boundary_tests;
#[path = "contract/parent_controller_event_tests.rs"]
mod parent_controller_event_tests;
#[path = "contract/root_contract_shape_tests.rs"]
mod root_contract_shape_tests;
#[path = "contract/route_tests.rs"]
mod route_tests;
#[path = "contract/schema_domain_ai_wire.rs"]
mod schema_domain_ai_wire;
#[path = "contract/schema_domain_mirrors.rs"]
mod schema_domain_mirrors;
#[path = "contract/screen_evidence_tests.rs"]
mod screen_evidence_tests;
#[path = "contract/screen_settings_tests.rs"]
mod screen_settings_tests;
#[path = "contract/social_alert_report_read_model.rs"]
mod social_alert_report_read_model;
#[path = "contract/social_audit_explanation_read_model.rs"]
mod social_audit_explanation_read_model;
#[path = "contract/social_dashboard_read_model.rs"]
mod social_dashboard_read_model;
#[path = "contract/social_parent_notification_delivery_read_model.rs"]
mod social_parent_notification_delivery_read_model;
#[path = "contract/social_source_custody_mutation_tests.rs"]
mod social_source_custody_mutation_tests;
#[path = "contract/tracking_alert_evaluated_event.rs"]
mod tracking_alert_evaluated_event;
#[path = "contract/tracking_expected_place_state_evaluated_event.rs"]
mod tracking_expected_place_state_evaluated_event;
#[path = "contract/tracking_read_model.rs"]
mod tracking_read_model;
#[path = "contract/tracking_read_model_payload.rs"]
mod tracking_read_model_payload;
#[path = "contract/tracking_retention_settings_write_command.rs"]
mod tracking_retention_settings_write_command;
#[path = "contract/transport_lan_pairing.rs"]
mod transport_lan_pairing;
#[path = "contract/transport_policy_request_assistant_preview_confirm.rs"]
mod transport_policy_request_assistant_preview_confirm;
#[path = "contract/windows_adapter_artifact_gate_tests.rs"]
mod windows_adapter_artifact_gate_tests;
#[path = "contract/windows_adapter_artifact_ingestion_tests.rs"]
mod windows_adapter_artifact_ingestion_tests;
#[path = "contract/windows_adapter_capability_tests.rs"]
mod windows_adapter_capability_tests;
