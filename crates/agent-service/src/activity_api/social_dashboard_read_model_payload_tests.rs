use ocentra_parent_agent_protocol::{
    constants, LogFieldValue, SocialDashboardUxSnapshot, SOCIAL_DASHBOARD_CAPABILITY_READY,
    SOCIAL_DASHBOARD_CLAIM_NOT_CLAIMED, SOCIAL_DASHBOARD_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    SOCIAL_DASHBOARD_PANEL_ACCOUNT_APPROVAL_QUEUE, SOCIAL_DASHBOARD_PANEL_FEED_VIDEO_GATES,
    SOCIAL_DASHBOARD_SCHEMA_VERSION,
};

use super::social_dashboard_read_model_payload::{
    social_dashboard_read_model_from_service, social_dashboard_read_model_payload,
};

#[test]
fn social_dashboard_payload_reports_six_honest_service_rows() {
    let read_model = social_dashboard_read_model_from_service();
    let payload = social_dashboard_read_model_payload(&read_model);
    let read_model_json = string_payload(
        &payload,
        constants::field::BROWSER_SOCIAL_DASHBOARD_READ_MODEL,
    );
    let decoded: SocialDashboardUxSnapshot =
        serde_json::from_str(read_model_json).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(decoded.schema_version, SOCIAL_DASHBOARD_SCHEMA_VERSION);
    assert_eq!(decoded.panels.len(), 6);
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
    payload: &'a ocentra_parent_agent_protocol::LogFields,
    field: &str,
) -> &'a str {
    match &payload[field] {
        LogFieldValue::String(text) => text,
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}

fn number_payload(payload: &ocentra_parent_agent_protocol::LogFields, field: &str) -> f64 {
    match &payload[field] {
        LogFieldValue::Number(value) => *value,
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}
