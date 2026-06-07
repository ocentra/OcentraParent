use ocentra_parent_agent_protocol::{
    constants, LogFieldValue, SocialAlertReportReadModelSnapshot,
    SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED, SOCIAL_ALERT_REPORT_DELIVERY_LOCAL_OUTBOX_ONLY,
    SOCIAL_ALERT_REPORT_INTENT_HIGH_RISK, SOCIAL_ALERT_REPORT_INTENT_MANUAL_REQUIRED,
};

use super::social_alert_report_read_model_payload::{
    social_alert_report_read_model_from_service, social_alert_report_read_model_payload,
};

#[test]
fn social_alert_report_payload_reports_honest_service_rows() {
    let read_model = social_alert_report_read_model_from_service();
    let payload = social_alert_report_read_model_payload(&read_model);
    let decoded: SocialAlertReportReadModelSnapshot = string_payload(
        &payload,
        constants::field::BROWSER_SOCIAL_ALERT_REPORT_READ_MODEL,
    );

    assert_eq!(decoded.intents.len(), 2);
    assert_eq!(
        decoded.intents[0].intent_kind,
        SOCIAL_ALERT_REPORT_INTENT_HIGH_RISK
    );
    assert_eq!(
        decoded.intents[0].delivery_claim_state,
        SOCIAL_ALERT_REPORT_DELIVERY_LOCAL_OUTBOX_ONLY
    );
    assert_eq!(
        decoded.intents[1].intent_kind,
        SOCIAL_ALERT_REPORT_INTENT_MANUAL_REQUIRED
    );
    assert!(!decoded.intents[0].provider_delivery_attempted);
    assert!(!decoded.intents[0].parent_notification_ui_claimed);
    assert!(!decoded.intents[0].final_policy_decision_claimed);
    assert!(!decoded.intents[0].enforcement_claimed);
    assert_eq!(
        decoded.claim_boundaries.provider_delivery,
        SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED
    );
}

fn string_payload<T>(payload: &ocentra_parent_agent_protocol::LogFields, field: &str) -> T
where
    T: serde::de::DeserializeOwned,
{
    match &payload[field] {
        LogFieldValue::String(text) => {
            serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}
