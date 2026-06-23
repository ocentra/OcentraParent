use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::SocialDashboardUxSnapshot;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_CAPABILITY_READY;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_CLAIM_NOT_CLAIMED;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_CUSTODY_CHILD_DEVICE_QUERY_STORE;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_PANEL_ACCOUNT_APPROVAL_QUEUE;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_PANEL_FEED_VIDEO_GATES;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_PANEL_SETTINGS_CUSTODY;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_REASON_SETTINGS_CUSTODY_RUNTIME_GAP;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_SCHEMA_VERSION;

use super::social_dashboard_read_model_payload::{
    social_dashboard_read_model_from_service, social_dashboard_read_model_payload,
};

#[test]
fn social_dashboard_payload_reports_seven_honest_service_rows() {
    let read_model = social_dashboard_read_model_from_service();
    let payload = social_dashboard_read_model_payload(&read_model);
    let read_model_json = string_payload(
        &payload,
        constants::field::BROWSER_SOCIAL_DASHBOARD_READ_MODEL,
    );
    let decoded: SocialDashboardUxSnapshot = serde_json::from_str(read_model_json)
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(decoded.schema_version, SOCIAL_DASHBOARD_SCHEMA_VERSION);
    assert_eq!(decoded.panels.len(), 7);
    assert_eq!(
        decoded.panels[0].panel_kind,
        SOCIAL_DASHBOARD_PANEL_ACCOUNT_APPROVAL_QUEUE
    );
    assert_eq!(
        decoded.panels[1].panel_kind,
        SOCIAL_DASHBOARD_PANEL_FEED_VIDEO_GATES
    );
    assert!(!decoded.panels[1].policy_decision_claimed);
    assert!(!decoded.panels[1].enforcement_claimed);
    assert_eq!(
        decoded.panels[5].panel_kind,
        SOCIAL_DASHBOARD_PANEL_SETTINGS_CUSTODY
    );
    assert!(decoded.panels[5]
        .reasons
        .contains(&SOCIAL_DASHBOARD_REASON_SETTINGS_CUSTODY_RUNTIME_GAP.to_string()));
    assert!(!decoded.panels[5].policy_decision_claimed);
    assert!(!decoded.panels[5].enforcement_claimed);
    assert_eq!(
        decoded.claim_boundaries.enforcement,
        SOCIAL_DASHBOARD_CLAIM_NOT_CLAIMED
    );
    assert_eq!(
        string_payload(&payload, constants::field::CUSTODY_LABEL),
        SOCIAL_DASHBOARD_CUSTODY_CHILD_DEVICE_QUERY_STORE
    );
    assert_eq!(
        string_payload(&payload, constants::field::CAPABILITY_STATUS),
        SOCIAL_DASHBOARD_CAPABILITY_READY
    );
    assert_eq!(
        number_payload(&payload, constants::field::RETURNED),
        decoded.panels.len() as f64
    );
}

fn string_payload<'a>(
    payload: &'a ocentra_parent_agent_protocol::logging::LogFields,
    field: &str,
) -> &'a str {
    match &payload[field] {
        LogFieldValue::String(text) => text,
        _ => std::process::abort(),
    }
}

fn number_payload(payload: &ocentra_parent_agent_protocol::logging::LogFields, field: &str) -> f64 {
    match &payload[field] {
        LogFieldValue::Number(value) => *value,
        _ => std::process::abort(),
    }
}
