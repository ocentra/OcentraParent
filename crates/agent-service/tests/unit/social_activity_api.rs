#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

use std::primitive::str as TestStr;

#[path = "../support/log_payload.rs"]
mod log_payload;
#[path = "../support/test_text.rs"]
mod test_text;

#[path = "../support/command_dispatch_test_support.rs"]
pub mod test_support;

#[path = "../../src/activity_store_path.rs"]
mod activity_store_path;
#[path = "../../src/event_builder.rs"]
mod event_builder;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../../src/json_contract.rs"]
mod json_contract;
#[path = "../support/test_invariants.rs"]
mod test_invariants;
#[path = "../../src/time.rs"]
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
#[path = "../integration/social_alert_report_read_model_service_tests.rs"]
mod social_alert_report_read_model_service_tests;
#[path = "../integration/social_audit_explanation_read_model_payload_tests.rs"]
mod social_audit_explanation_read_model_payload_tests;
#[path = "../integration/social_audit_explanation_read_model_service_tests.rs"]
mod social_audit_explanation_read_model_service_tests;
#[path = "../integration/social_dashboard_read_model_payload_tests.rs"]
mod social_dashboard_read_model_payload_tests;
#[path = "../integration/social_dashboard_read_model_service_tests.rs"]
mod social_dashboard_read_model_service_tests;
#[path = "../integration/social_parent_notification_delivery_read_model_payload_tests.rs"]
mod social_parent_notification_delivery_read_model_payload_tests;
#[path = "../integration/social_parent_notification_delivery_read_model_service_tests.rs"]
mod social_parent_notification_delivery_read_model_service_tests;
#[path = "../integration/social_source_custody_mutation_payload_tests.rs"]
mod social_source_custody_mutation_payload_tests;
#[path = "../integration/social_source_custody_mutation_service_tests.rs"]
mod social_source_custody_mutation_service_tests;

#[cfg(test)]
mod clippy_linkage {
    use std::collections::BTreeMap;

    use super::*;
    use crate::test_invariants::{
        require_json_decode, require_log_string_field, require_ok, require_some,
        serialize_test_json,
    };
    use crate::test_text::{
        count_for_display, optional_log_string, test_ok, test_some, TestResult,
    };
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
    use ocentra_parent_agent_protocol::transport::{
        AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
        AgentPeerRole, AgentRoute,
    };
    use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

    #[tokio::test]
    async fn social_runtime_builders_are_linked() {
        let encoded = serialize_test_json(&serde_json::json!({
            "social_activity_api": true
        }));
        let decoded: serde_json::Value =
            require_json_decode(&encoded, "social_activity_api linkage json");
        assert!(require_some(
            decoded
                .get("social_activity_api")
                .and_then(|value| value.as_bool()),
            "social_activity_api linkage bool",
        ));
        let field = LogFieldValue::String(encoded);
        let text = require_log_string_field(Some(&field), "social_activity_api linkage field");
        let _: serde_json::Value =
            require_json_decode(text, "social_activity_api linkage field json");
        let _: () = require_ok(
            Ok::<(), std::io::Error>(()),
            "social_activity_api linkage ok",
        );
        let _ = crate::activity_store_path::activity_db_path();
        let _ = crate::activity_store_path::activity_journal_path();
        let _ = crate::activity_store_path::activity_journal_key_path();
        let _ = crate::event_builder::portal_peer();
        let _ = crate::json_contract::serialize_json_value(serde_json::json!({
            "social_activity_api": true
        }));
        let _: String = crate::time::timestamp_after_epoch_seconds(1, 0);
        let _: String = crate::time::timestamp_after_epoch_seconds(1, 1);

        let text = crate::test_text::TestText::from_display("social-activity");
        let mut counts = BTreeMap::new();
        let mut payload = LogFields::new();
        counts.insert(text.clone(), 1);
        payload.insert(
            constants::field::ACTIVITY_REPORT_ID.to_string(),
            LogFieldValue::String(text.to_string()),
        );

        let _: TestResult = Ok(());
        assert_eq!(text.as_bytes(), b"social-activity");
        assert_eq!(text.as_str(), "social-activity");
        assert_eq!(
            test_ok(Ok::<usize, std::io::Error>(1), "social_activity_api result")
                .unwrap_or_else(|_| std::process::abort()),
            1
        );
        assert_eq!(
            test_some(Some(2), "social_activity_api value")
                .unwrap_or_else(|_| std::process::abort()),
            2
        );
        assert_eq!(count_for_display(&counts, "social-activity"), 1);
        assert_eq!(
            optional_log_string(&payload, constants::field::ACTIVITY_REPORT_ID),
            Some(text)
        );

        let command = command_envelope("cmd-social-activity-clippy");
        assert_eq!(
            social_alert_report_read_model_payload::build_browser_social_alert_report_read_model_report(
                command.clone()
            )
            .await
            .event,
            AgentEventName::AgentBrowserSocialAlertReportReadModelReported
        );
        assert_eq!(
            social_alert_report_parent_surface_read_model_payload::build_browser_social_alert_report_parent_surface_read_model_report(
                command.clone()
            )
            .await
            .event,
            AgentEventName::AgentBrowserSocialAlertReportParentSurfaceReadModelReported
        );
        assert_eq!(
            social_audit_explanation_read_model_payload::build_browser_social_audit_explanation_read_model_report(
                command.clone()
            )
            .await
            .event,
            AgentEventName::AgentBrowserSocialAuditExplanationReadModelReported
        );
        assert_eq!(
            social_dashboard_read_model_payload::build_browser_social_dashboard_read_model_report(
                command.clone()
            )
            .await
            .event,
            AgentEventName::AgentBrowserSocialDashboardReadModelReported
        );
        assert_eq!(
            social_parent_notification_delivery_read_model_payload::build_browser_social_parent_notification_delivery_read_model_report(
                command.clone()
            )
            .await
            .event,
            AgentEventName::AgentBrowserSocialParentNotificationDeliveryReadModelReported
        );
        assert_eq!(
            social_source_custody_mutation_payload::build_browser_social_source_custody_mutation_report(
                command
            )
            .await
            .event,
            AgentEventName::AgentBrowserSocialSourceCustodyMutationApplied
        );
    }

    fn command_envelope(message_id: &TestStr) -> AgentCommandEnvelope {
        AgentCommandEnvelope {
            schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
            message_id: message_id.to_string(),
            sent_at: "2026-06-29T00:00:00Z".to_string(),
            source: AgentPeer {
                peer_id: constants::peer::PORTAL_DEV.to_string(),
                role: AgentPeerRole::Portal,
            },
            target: AgentMessageTarget {
                device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
                platform: "windows".to_string(),
                route: AgentRoute::Localhost,
            },
            command: AgentCommandName::AgentBrowserSocialDashboardReadModelGet,
            payload: LogFields::new(),
        }
    }
}
