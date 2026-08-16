use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::social_alert_report_parent_surface_read_model::SocialAlertReportParentSurfaceReadModelSnapshot;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATE_MANUAL;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATE_UNAVAILABLE;

use super::social_alert_report_parent_surface_read_model_payload::{
    parent_surface_payload, request_social_alert_report_parent_surface_read_model_from_service,
    social_alert_report_parent_surface_read_model_from_service,
};

#[tokio::test]
async fn service_parent_surface_status_uses_local_eventing_request() {
    let read_model = request_social_alert_report_parent_surface_read_model_from_service()
        .await
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        read_model.schema_version,
        SOCIAL_ALERT_REPORT_PARENT_SURFACE_SCHEMA_VERSION
    );
    assert_eq!(read_model.rows.len(), 3);
    assert_eq!(read_model.manual_action_required_count, 2);
    assert_eq!(read_model.unavailable_visible_count, 1);
    assert_eq!(read_model.preference_setup_required_count, 2);
    assert!(!read_model.parent_notification_ui_rendered);
    assert!(!read_model.provider_delivery_runtime_claimed);
    assert!(!read_model.provider_credentials_claimed);
    assert!(!read_model.adapter_dispatch_claimed);
    assert!(!read_model.connector_native_runtime_claimed);
    assert!(!read_model.final_policy_execution_claimed);
    assert!(!read_model.enforcement_claimed);
    assert!(read_model
        .rows
        .iter()
        .all(|row| !row.adapter_dispatch_claimed && !row.parent_preference_mutation_claimed));
}

#[test]
fn parent_surface_payload_exposes_status_json_without_delivery_claims() {
    let read_model = social_alert_report_parent_surface_read_model_from_service();
    let payload = parent_surface_payload(&read_model);

    assert_eq!(
        payload.get(constants::field::CAPABILITY_STATUS),
        Some(&LogFieldValue::String(
            SOCIAL_ALERT_REPORT_PARENT_SURFACE_SCHEMA_VERSION.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::RETURNED),
        Some(&LogFieldValue::Number(3.0))
    );
    let json = payload
        .get(constants::field::BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_READ_MODEL)
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let LogFieldValue::String(json) = json else {
        std::process::abort();
    };
    let decoded: SocialAlertReportParentSurfaceReadModelSnapshot =
        serde_json::from_str(json).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        decoded
            .rows
            .iter()
            .map(|row| row.parent_surface_status.as_str())
            .collect::<Vec<_>>(),
        vec![
            SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATE_MANUAL,
            SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATE_MANUAL,
            SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATE_UNAVAILABLE,
        ]
    );
    assert!(!decoded.parent_notification_ui_rendered);
    assert!(!decoded.provider_delivery_runtime_claimed);
    assert!(!decoded.adapter_dispatch_claimed);
    assert!(!decoded.connector_native_runtime_claimed);
    assert!(!decoded.final_policy_execution_claimed);
    assert!(!decoded.enforcement_claimed);
}
