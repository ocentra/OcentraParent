#![allow(clippy::expect_used, clippy::panic)]

#[path = "contract/app_game_adapter_dispatch_preflight.rs"]
mod app_game_adapter_dispatch_preflight;
#[path = "contract/app_game_adapter_dispatch_result.rs"]
mod app_game_adapter_dispatch_result;
#[path = "contract/app_game_adapter_execution_readiness.rs"]
mod app_game_adapter_execution_readiness;
#[path = "contract/child_domain_runtime_events.rs"]
mod child_domain_runtime_events;
#[path = "contract/lan_pairing.rs"]
mod lan_pairing;
#[path = "contract/lan_pairing_browser_add_device_state.rs"]
mod lan_pairing_browser_add_device_state;
#[path = "contract/lan_pairing_browser_runtime.rs"]
mod lan_pairing_browser_runtime;
#[path = "contract/lan_pairing_household_proof.rs"]
mod lan_pairing_household_proof;
#[path = "contract/lan_pairing_provider_selection.rs"]
mod lan_pairing_provider_selection;
#[path = "contract/schema_domain_ai_wire.rs"]
mod schema_domain_ai_wire;
#[path = "contract/schema_domain_mirrors.rs"]
mod schema_domain_mirrors;
#[path = "contract/social_alert_report_read_model.rs"]
mod social_alert_report_read_model;
#[path = "contract/social_audit_explanation_read_model.rs"]
mod social_audit_explanation_read_model;
#[path = "contract/social_dashboard_read_model.rs"]
mod social_dashboard_read_model;
#[path = "contract/social_parent_notification_delivery_read_model.rs"]
mod social_parent_notification_delivery_read_model;
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
