use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LogFieldValue, LogFields,
    LogLevel, SocialAlertReportClaimBoundaries, SocialAlertReportDeviceRef,
    SocialAlertReportEvidenceRef, SocialAlertReportIntent, SocialAlertReportParentActionRef,
    SocialAlertReportParentActor, SocialAlertReportReadModelSnapshot,
    SOCIAL_ALERT_REPORT_ACTION_OPEN_PARENT_REVIEW, SOCIAL_ALERT_REPORT_ACTION_REVIEW_MANUALLY,
    SOCIAL_ALERT_REPORT_ADAPTER_NOT_DISPATCHED, SOCIAL_ALERT_REPORT_AUDIT_REF,
    SOCIAL_ALERT_REPORT_BODY_HIGH_RISK, SOCIAL_ALERT_REPORT_BODY_MANUAL_REQUIRED,
    SOCIAL_ALERT_REPORT_CAPABILITY_READY, SOCIAL_ALERT_REPORT_CHILD_PROFILE_ID,
    SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED, SOCIAL_ALERT_REPORT_CONTRACT_SCHEMA_VERSION,
    SOCIAL_ALERT_REPORT_DELIVERY_LOCAL_OUTBOX_ONLY, SOCIAL_ALERT_REPORT_DELIVERY_MANUAL_REQUIRED,
    SOCIAL_ALERT_REPORT_DEVICE_ID, SOCIAL_ALERT_REPORT_DEVICE_LABEL,
    SOCIAL_ALERT_REPORT_EVIDENCE_KIND_POLICY_DECISION, SOCIAL_ALERT_REPORT_EVIDENCE_MANUAL_GAP,
    SOCIAL_ALERT_REPORT_EVIDENCE_ROUTE_GATE, SOCIAL_ALERT_REPORT_EXPLANATION_EVENT_FEED_VIDEO_GATE,
    SOCIAL_ALERT_REPORT_EXPLANATION_EVENT_MANUAL_REQUIRED,
    SOCIAL_ALERT_REPORT_EXPLANATION_SNAPSHOT, SOCIAL_ALERT_REPORT_FAMILY_ID,
    SOCIAL_ALERT_REPORT_INTENT_HIGH_RISK, SOCIAL_ALERT_REPORT_INTENT_MANUAL_REQUIRED,
    SOCIAL_ALERT_REPORT_LOCAL_OUTBOX_REF, SOCIAL_ALERT_REPORT_MANUAL_PROOF_REQUIRED,
    SOCIAL_ALERT_REPORT_PANEL_FEED_VIDEO_GATES, SOCIAL_ALERT_REPORT_PANEL_MANUAL_REQUIRED_GAPS,
    SOCIAL_ALERT_REPORT_PARENT_ACTION_ID, SOCIAL_ALERT_REPORT_PARENT_ACTOR_ID,
    SOCIAL_ALERT_REPORT_PARENT_ACTOR_ROLE, SOCIAL_ALERT_REPORT_PAYLOAD_ALERT_ID,
    SOCIAL_ALERT_REPORT_PAYLOAD_EVIDENCE_REF, SOCIAL_ALERT_REPORT_PAYLOAD_EXPLANATION_REF,
    SOCIAL_ALERT_REPORT_PAYLOAD_FAMILY_DEVICE_SCOPE,
    SOCIAL_ALERT_REPORT_PAYLOAD_PARENT_ACTION_LINK_REF, SOCIAL_ALERT_REPORT_PAYLOAD_POLICY_REF,
    SOCIAL_ALERT_REPORT_PAYLOAD_REASON_CODE, SOCIAL_ALERT_REPORT_PAYLOAD_SEVERITY,
    SOCIAL_ALERT_REPORT_PLATFORM_ANDROID, SOCIAL_ALERT_REPORT_POLICY_HIGH_RISK,
    SOCIAL_ALERT_REPORT_POLICY_MANUAL_REQUIRED, SOCIAL_ALERT_REPORT_POLICY_VERSION,
    SOCIAL_ALERT_REPORT_PRIORITY_ATTENTION, SOCIAL_ALERT_REPORT_PRIORITY_URGENT,
    SOCIAL_ALERT_REPORT_PROVIDER_CHANNEL_IN_APP, SOCIAL_ALERT_REPORT_REASON_HIGH_RISK,
    SOCIAL_ALERT_REPORT_REASON_MANUAL_REQUIRED, SOCIAL_ALERT_REPORT_SCHEMA_VERSION,
    SOCIAL_ALERT_REPORT_SEVERITY_CRITICAL, SOCIAL_ALERT_REPORT_SEVERITY_WARNING,
    SOCIAL_ALERT_REPORT_STATUS_LOCAL_OUTBOX, SOCIAL_ALERT_REPORT_STATUS_MANUAL_REQUIRED,
    SOCIAL_ALERT_REPORT_TITLE_HIGH_RISK, SOCIAL_ALERT_REPORT_TITLE_MANUAL_REQUIRED,
};

use crate::{event_builder::build_event, fields::fields_from_pairs, time::timestamp_now};

type FieldPair = (&'static str, LogFieldValue);

pub fn social_alert_report_read_model_from_service() -> SocialAlertReportReadModelSnapshot {
    let generated_at = timestamp_now();
    SocialAlertReportReadModelSnapshot {
        schema_version: SOCIAL_ALERT_REPORT_SCHEMA_VERSION.to_string(),
        family_id: SOCIAL_ALERT_REPORT_FAMILY_ID.to_string(),
        child_profile_id: SOCIAL_ALERT_REPORT_CHILD_PROFILE_ID.to_string(),
        generated_at: generated_at.clone(),
        intents: vec![
            high_risk_intent(&generated_at),
            manual_required_intent(&generated_at),
        ],
        claim_boundaries: SocialAlertReportClaimBoundaries {
            provider_delivery: SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED.to_string(),
            report_delivery: SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED.to_string(),
            parent_notification_ui: SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED.to_string(),
            final_policy_decision: SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED.to_string(),
            enforcement: SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED.to_string(),
        },
    }
}

pub fn social_alert_report_read_model_payload(
    read_model: &SocialAlertReportReadModelSnapshot,
) -> LogFields {
    fields_from_pairs(read_model_pairs(read_model))
}

pub async fn build_browser_social_alert_report_read_model_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let read_model = social_alert_report_read_model_from_service();
    build_event(
        constants::event_id::BROWSER_SOCIAL_ALERT_REPORT_READ_MODEL_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentBrowserSocialAlertReportReadModelReported,
        LogLevel::Info,
        social_alert_report_read_model_payload(&read_model),
        None,
    )
}

fn read_model_pairs(read_model: &SocialAlertReportReadModelSnapshot) -> Vec<FieldPair> {
    vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(SOCIAL_ALERT_REPORT_CAPABILITY_READY.to_string()),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.intents.len() as f64),
        ),
        (
            constants::field::BROWSER_SOCIAL_ALERT_REPORT_READ_MODEL,
            LogFieldValue::String(
                serde_json::to_string(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
    ]
}

fn high_risk_intent(created_at: &str) -> SocialAlertReportIntent {
    intent(
        SOCIAL_ALERT_REPORT_INTENT_HIGH_RISK,
        SOCIAL_ALERT_REPORT_STATUS_LOCAL_OUTBOX,
        SOCIAL_ALERT_REPORT_PRIORITY_URGENT,
        SOCIAL_ALERT_REPORT_SEVERITY_CRITICAL,
        SOCIAL_ALERT_REPORT_REASON_HIGH_RISK,
        SOCIAL_ALERT_REPORT_TITLE_HIGH_RISK,
        SOCIAL_ALERT_REPORT_BODY_HIGH_RISK,
        SOCIAL_ALERT_REPORT_ACTION_OPEN_PARENT_REVIEW,
        SOCIAL_ALERT_REPORT_PANEL_FEED_VIDEO_GATES,
        SOCIAL_ALERT_REPORT_EXPLANATION_EVENT_FEED_VIDEO_GATE,
        SOCIAL_ALERT_REPORT_EVIDENCE_ROUTE_GATE,
        SOCIAL_ALERT_REPORT_POLICY_HIGH_RISK,
        SOCIAL_ALERT_REPORT_DELIVERY_LOCAL_OUTBOX_ONLY,
        Some(SOCIAL_ALERT_REPORT_LOCAL_OUTBOX_REF.to_string()),
        Some(parent_action_ref(created_at)),
        Vec::new(),
        created_at,
    )
}

fn manual_required_intent(created_at: &str) -> SocialAlertReportIntent {
    intent(
        SOCIAL_ALERT_REPORT_INTENT_MANUAL_REQUIRED,
        SOCIAL_ALERT_REPORT_STATUS_MANUAL_REQUIRED,
        SOCIAL_ALERT_REPORT_PRIORITY_ATTENTION,
        SOCIAL_ALERT_REPORT_SEVERITY_WARNING,
        SOCIAL_ALERT_REPORT_REASON_MANUAL_REQUIRED,
        SOCIAL_ALERT_REPORT_TITLE_MANUAL_REQUIRED,
        SOCIAL_ALERT_REPORT_BODY_MANUAL_REQUIRED,
        SOCIAL_ALERT_REPORT_ACTION_REVIEW_MANUALLY,
        SOCIAL_ALERT_REPORT_PANEL_MANUAL_REQUIRED_GAPS,
        SOCIAL_ALERT_REPORT_EXPLANATION_EVENT_MANUAL_REQUIRED,
        SOCIAL_ALERT_REPORT_EVIDENCE_MANUAL_GAP,
        SOCIAL_ALERT_REPORT_POLICY_MANUAL_REQUIRED,
        SOCIAL_ALERT_REPORT_DELIVERY_MANUAL_REQUIRED,
        None,
        None,
        vec![SOCIAL_ALERT_REPORT_MANUAL_PROOF_REQUIRED.to_string()],
        created_at,
    )
}

#[allow(clippy::too_many_arguments)]
fn intent(
    intent_kind: &'static str,
    intent_status: &'static str,
    priority: &'static str,
    severity: &'static str,
    reason: &'static str,
    title: &'static str,
    body: &'static str,
    action: &'static str,
    dashboard_panel: &'static str,
    explanation_event: &'static str,
    evidence_ref: &'static str,
    policy_ref: &'static str,
    delivery_claim: &'static str,
    local_outbox: Option<String>,
    parent_action: Option<SocialAlertReportParentActionRef>,
    manual_requirements: Vec<String>,
    created_at: &str,
) -> SocialAlertReportIntent {
    SocialAlertReportIntent {
        schema_version: SOCIAL_ALERT_REPORT_CONTRACT_SCHEMA_VERSION.to_string(),
        alert_report_intent_id: intent_kind.to_string(),
        intent_kind: intent_kind.to_string(),
        intent_status: intent_status.to_string(),
        priority: priority.to_string(),
        severity: severity.to_string(),
        device: device_ref(),
        notification_reason_code: reason.to_string(),
        provider_channel_preference: SOCIAL_ALERT_REPORT_PROVIDER_CHANNEL_IN_APP.to_string(),
        parent_title_token: title.to_string(),
        parent_body_token: body.to_string(),
        parent_action_token: action.to_string(),
        dashboard_panel_refs: vec![dashboard_panel.to_string()],
        explanation_snapshot_ref: SOCIAL_ALERT_REPORT_EXPLANATION_SNAPSHOT.to_string(),
        explanation_event_refs: vec![explanation_event.to_string()],
        evidence_references: vec![evidence_reference(evidence_ref, created_at)],
        policy_refs: vec![policy_ref.to_string()],
        audit_refs: vec![SOCIAL_ALERT_REPORT_AUDIT_REF.to_string()],
        parent_report_ref: None,
        parent_action_ref: parent_action,
        local_outbox_record_ref: local_outbox,
        provider_attempt_refs: Vec::new(),
        provider_receipt_refs: Vec::new(),
        manual_proof_requirements: manual_requirements,
        minimal_payload_fields: minimal_payload_fields(),
        delivery_claim_state: delivery_claim.to_string(),
        raw_account_data_included: false,
        raw_video_content_included: false,
        raw_message_content_included: false,
        screenshot_included: false,
        provider_delivery_attempted: false,
        provider_delivery_observed: false,
        provider_receipt_ingested: false,
        cloud_routing_claimed: false,
        parent_notification_ui_claimed: false,
        report_delivery_claimed: false,
        final_policy_decision_claimed: false,
        enforcement_claimed: false,
        adapter_dispatch_state: SOCIAL_ALERT_REPORT_ADAPTER_NOT_DISPATCHED.to_string(),
        adapter_action_claimed: false,
        created_at: created_at.to_string(),
    }
}

fn device_ref() -> SocialAlertReportDeviceRef {
    SocialAlertReportDeviceRef {
        device_id: SOCIAL_ALERT_REPORT_DEVICE_ID.to_string(),
        child_profile_id: SOCIAL_ALERT_REPORT_CHILD_PROFILE_ID.to_string(),
        label: SOCIAL_ALERT_REPORT_DEVICE_LABEL.to_string(),
        platform: SOCIAL_ALERT_REPORT_PLATFORM_ANDROID.to_string(),
    }
}

fn evidence_reference(
    evidence_ref: &'static str,
    observed_at: &str,
) -> SocialAlertReportEvidenceRef {
    SocialAlertReportEvidenceRef {
        evidence_reference_id: evidence_ref.to_string(),
        kind: SOCIAL_ALERT_REPORT_EVIDENCE_KIND_POLICY_DECISION.to_string(),
        observed_at: observed_at.to_string(),
    }
}

fn parent_action_ref(created_at: &str) -> SocialAlertReportParentActionRef {
    SocialAlertReportParentActionRef {
        action_reference_id: SOCIAL_ALERT_REPORT_PARENT_ACTION_ID.to_string(),
        actor: SocialAlertReportParentActor {
            actor_id: SOCIAL_ALERT_REPORT_PARENT_ACTOR_ID.to_string(),
            role: SOCIAL_ALERT_REPORT_PARENT_ACTOR_ROLE.to_string(),
        },
        policy_version: SOCIAL_ALERT_REPORT_POLICY_VERSION.to_string(),
        created_at: created_at.to_string(),
    }
}

fn minimal_payload_fields() -> Vec<String> {
    vec![
        SOCIAL_ALERT_REPORT_PAYLOAD_ALERT_ID.to_string(),
        SOCIAL_ALERT_REPORT_PAYLOAD_FAMILY_DEVICE_SCOPE.to_string(),
        SOCIAL_ALERT_REPORT_PAYLOAD_SEVERITY.to_string(),
        SOCIAL_ALERT_REPORT_PAYLOAD_REASON_CODE.to_string(),
        SOCIAL_ALERT_REPORT_PAYLOAD_EVIDENCE_REF.to_string(),
        SOCIAL_ALERT_REPORT_PAYLOAD_POLICY_REF.to_string(),
        SOCIAL_ALERT_REPORT_PAYLOAD_EXPLANATION_REF.to_string(),
        SOCIAL_ALERT_REPORT_PAYLOAD_PARENT_ACTION_LINK_REF.to_string(),
    ]
}
