#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../../src/event_builder/build.rs"]
mod event_builder;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../../src/json_contract/string.rs"]
mod json_contract;
#[path = "../support/log_payload.rs"]
mod log_payload;
#[path = "../support/lan_test_text.rs"]
mod test_text;
#[path = "../../src/time/now.rs"]
mod time;

#[path = "../../src/activity_api/social_alert_report_parent_surface_read_model_payload.rs"]
mod social_alert_report_parent_surface_read_model_payload;
#[path = "../../src/activity_api/social_alert_report_read_model_payload.rs"]
mod social_alert_report_read_model_payload;
#[path = "../../src/activity_api/social_audit_explanation_read_model_payload.rs"]
mod social_audit_explanation_read_model_payload;
#[path = "../../src/activity_api/social_dashboard_read_model_payload.rs"]
mod social_dashboard_read_model_payload;
#[path = "../../src/activity_api/social_parent_notification_delivery_read_model_payload.rs"]
mod social_parent_notification_delivery_read_model_payload;
#[path = "../../src/activity_api/social_source_custody_mutation_payload.rs"]
mod social_source_custody_mutation_payload;

#[path = "../integration/social_alert_report_parent_surface_read_model_payload_tests.rs"]
mod social_alert_report_parent_surface_read_model_payload_tests;
#[path = "../integration/social_alert_report_read_model_payload_tests.rs"]
mod social_alert_report_read_model_payload_tests;
#[path = "../integration/social_audit_explanation_read_model_payload_tests.rs"]
mod social_audit_explanation_read_model_payload_tests;
#[path = "../integration/social_dashboard_read_model_payload_tests.rs"]
mod social_dashboard_read_model_payload_tests;
#[path = "../integration/social_parent_notification_delivery_read_model_payload_tests.rs"]
mod social_parent_notification_delivery_read_model_payload_tests;
#[path = "social_report_envelope_tests.rs"]
mod social_report_envelope_tests;
#[path = "../integration/social_source_custody_mutation_payload_tests.rs"]
mod social_source_custody_mutation_payload_tests;
