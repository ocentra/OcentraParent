use ocentra_parent_agent_protocol::{
    constants, LogFieldValue, SocialParentNotificationDeliveryReadinessSnapshot,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_MANUAL_REQUIRED,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_REPORT_READY,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_UNAVAILABLE,
};

use super::social_parent_notification_delivery_read_model_payload::{
    request_social_parent_notification_delivery_read_model_from_service,
    social_parent_notification_delivery_read_model_from_service,
    social_parent_notification_delivery_read_model_payload,
};

#[test]
fn social_parent_notification_delivery_payload_reports_honest_service_rows() {
    let read_model = social_parent_notification_delivery_read_model_from_service();
    let payload = social_parent_notification_delivery_read_model_payload(&read_model);
    let decoded: SocialParentNotificationDeliveryReadinessSnapshot = string_payload(
        &payload,
        constants::field::BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_READ_MODEL,
    );

    assert_eq!(decoded.rows.len(), 3);
    assert_eq!(decoded.parent_report_status_ready_count, 1);
    assert_eq!(decoded.manual_required_count, 1);
    assert_eq!(decoded.unavailable_count, 1);
    assert_eq!(
        decoded.rows[0].notification_delivery_readiness_state,
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_REPORT_READY
    );
    assert_eq!(
        decoded.rows[1].notification_delivery_readiness_state,
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_MANUAL_REQUIRED
    );
    assert_eq!(
        decoded.rows[2].notification_delivery_readiness_state,
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_UNAVAILABLE
    );
    assert!(decoded.rows[0].parent_report_ref.is_some());
    assert!(decoded.rows[0].report_artifact_ref.is_some());
    assert!(decoded.rows[0].report_receipt_ref.is_some());
    assert!(decoded.rows[1].parent_report_ref.is_none());
    assert!(!decoded.rows[1].manual_proof_requirements.is_empty());
    assert!(decoded.rows[2].parent_report_ref.is_none());
    assert!(!decoded.rows[2].manual_proof_requirements.is_empty());
    assert!(!decoded.parent_notification_ui_delivery_claimed);
    assert!(!decoded.external_runtime_report_delivery_claimed);
    assert!(!decoded.final_policy_execution_claimed);
    assert!(!decoded.enforcement_claimed);
    assert!(decoded
        .rows
        .iter()
        .all(|row| !row.parent_notification_ui_delivered && !row.provider_delivery_attempted));
}

#[tokio::test]
async fn social_parent_notification_delivery_event_request_matches_service_projection() {
    let direct = social_parent_notification_delivery_read_model_from_service();
    let evented = request_social_parent_notification_delivery_read_model_from_service()
        .await
        .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(evented.rows.len(), direct.rows.len());
    assert_eq!(
        evented.parent_report_status_ready_count,
        direct.parent_report_status_ready_count
    );
    assert_eq!(evented.manual_required_count, direct.manual_required_count);
    assert_eq!(evented.unavailable_count, direct.unavailable_count);
    assert_eq!(evented.non_claims, direct.non_claims);
    assert!(!evented.parent_notification_ui_delivery_claimed);
    assert!(!evented.external_runtime_report_delivery_claimed);
    assert!(!evented.final_policy_execution_claimed);
    assert!(!evented.enforcement_claimed);
    assert!(evented.rows.iter().all(|row| {
        !row.parent_notification_ui_delivered
            && !row.provider_delivery_attempted
            && !row.provider_receipt_ingested
            && !row.final_policy_decision_claimed
            && !row.enforcement_claimed
    }));
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
