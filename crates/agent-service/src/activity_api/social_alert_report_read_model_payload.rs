use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields, LogLevel};
use ocentra_parent_agent_protocol::social_alert_report_read_model::{
    SocialAlertReportClaimBoundaries, SocialAlertReportDeviceRef, SocialAlertReportEvidenceRef,
    SocialAlertReportIntent, SocialAlertReportParentActionRef, SocialAlertReportParentActor,
    SocialAlertReportProviderStatusRow, SocialAlertReportReadModelSnapshot,
};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_ACTION_OPEN_PARENT_REVIEW;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_ACTION_REVIEW_MANUALLY;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_ADAPTER_NOT_DISPATCHED;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_AUDIT_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_BODY_HIGH_RISK;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_BODY_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_CAPABILITY_READY;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_CHILD_PROFILE_ID;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_CONTRACT_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_DELIVERY_LOCAL_OUTBOX_ONLY;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_DELIVERY_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_DEVICE_ID;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_DEVICE_LABEL;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_EVIDENCE_KIND_POLICY_DECISION;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_EVIDENCE_MANUAL_GAP;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_EVIDENCE_ROUTE_GATE;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_EXPLANATION_EVENT_FEED_VIDEO_GATE;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_EXPLANATION_EVENT_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_EXPLANATION_SNAPSHOT;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_FAMILY_ID;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_INTENT_HIGH_RISK;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_INTENT_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_LOCAL_OUTBOX_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_MANUAL_PROOF_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PANEL_FEED_VIDEO_GATES;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PANEL_MANUAL_REQUIRED_GAPS;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_ACTION_ID;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_ACTOR_ID;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_ACTOR_ROLE;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PAYLOAD_ALERT_ID;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PAYLOAD_EVIDENCE_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PAYLOAD_EXPLANATION_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PAYLOAD_FAMILY_DEVICE_SCOPE;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PAYLOAD_PARENT_ACTION_LINK_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PAYLOAD_POLICY_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PAYLOAD_REASON_CODE;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PAYLOAD_SEVERITY;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PLATFORM_ANDROID;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_POLICY_HIGH_RISK;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_POLICY_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_POLICY_VERSION;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PRIORITY_ATTENTION;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PRIORITY_URGENT;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PROVIDER_ADAPTER_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PROVIDER_ATTEMPT_HIGH_RISK;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PROVIDER_ATTEMPT_MANUAL;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PROVIDER_CHANNEL_IN_APP;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PROVIDER_CREDENTIALS_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PROVIDER_DELIVERY_NOT_OBSERVED;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PROVIDER_PREFLIGHT_ADAPTER_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PROVIDER_PREFLIGHT_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PROVIDER_SMOKE_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PROVIDER_STATUS_HIGH_RISK;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PROVIDER_STATUS_MANUAL;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PROVIDER_STATUS_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PROVIDER_STATUS_PROOF_MANUAL_ACTION;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_REASON_HIGH_RISK;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_REASON_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_SEVERITY_CRITICAL;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_SEVERITY_WARNING;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_STATUS_LOCAL_OUTBOX;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_STATUS_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_TITLE_HIGH_RISK;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_TITLE_MANUAL_REQUIRED;

use crate::{
    event_builder::build_event, fields::fields_from_pairs, json_contract::serialize_json_string,
    time::timestamp_now,
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct SocialAlertReportText(String);

#[derive(Clone, Debug, Eq, PartialEq)]
struct SocialAlertReportTextList(Vec<String>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SocialAlertReportFieldName(&'static str);

impl<T> From<T> for SocialAlertReportText
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<T> From<Vec<T>> for SocialAlertReportTextList
where
    T: Into<String>,
{
    fn from(value: Vec<T>) -> Self {
        Self(value.into_iter().map(Into::into).collect())
    }
}

pub fn social_alert_report_read_model_from_service() -> SocialAlertReportReadModelSnapshot {
    let generated_at: String = timestamp_now();
    SocialAlertReportReadModelSnapshot {
        schema_version: SOCIAL_ALERT_REPORT_SCHEMA_VERSION.to_string(),
        family_id: SOCIAL_ALERT_REPORT_FAMILY_ID.to_string(),
        child_profile_id: SOCIAL_ALERT_REPORT_CHILD_PROFILE_ID.to_string(),
        generated_at: generated_at.clone(),
        intents: vec![
            high_risk_intent(&generated_at),
            manual_required_intent(&generated_at),
        ],
        provider_status_rows: vec![
            high_risk_provider_status_row(&generated_at),
            manual_required_provider_status_row(&generated_at),
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
    fields_from_pairs(vec![
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
            LogFieldValue::String(serialize_json_string(read_model).0),
        ),
    ])
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

fn high_risk_provider_status_row(
    last_checked_at: impl Into<SocialAlertReportText>,
) -> SocialAlertReportProviderStatusRow {
    provider_status_row(
        SocialAlertReportFieldName(SOCIAL_ALERT_REPORT_PROVIDER_STATUS_HIGH_RISK),
        SocialAlertReportFieldName(SOCIAL_ALERT_REPORT_INTENT_HIGH_RISK),
        SocialAlertReportFieldName(SOCIAL_ALERT_REPORT_PROVIDER_PREFLIGHT_ADAPTER_REQUIRED),
        SocialAlertReportFieldName(SOCIAL_ALERT_REPORT_PROVIDER_ATTEMPT_HIGH_RISK),
        vec![
            SOCIAL_ALERT_REPORT_PROVIDER_ADAPTER_REQUIRED.to_string(),
            SOCIAL_ALERT_REPORT_PROVIDER_CREDENTIALS_REQUIRED.to_string(),
            SOCIAL_ALERT_REPORT_PROVIDER_SMOKE_REQUIRED.to_string(),
        ],
        vec![
            SOCIAL_ALERT_REPORT_PROVIDER_ADAPTER_REQUIRED.to_string(),
            SOCIAL_ALERT_REPORT_PROVIDER_CREDENTIALS_REQUIRED.to_string(),
            SOCIAL_ALERT_REPORT_PROVIDER_SMOKE_REQUIRED.to_string(),
        ],
        last_checked_at,
    )
}

fn manual_required_provider_status_row(
    last_checked_at: impl Into<SocialAlertReportText>,
) -> SocialAlertReportProviderStatusRow {
    provider_status_row(
        SocialAlertReportFieldName(SOCIAL_ALERT_REPORT_PROVIDER_STATUS_MANUAL),
        SocialAlertReportFieldName(SOCIAL_ALERT_REPORT_INTENT_MANUAL_REQUIRED),
        SocialAlertReportFieldName(SOCIAL_ALERT_REPORT_PROVIDER_PREFLIGHT_MANUAL_REQUIRED),
        SocialAlertReportFieldName(SOCIAL_ALERT_REPORT_PROVIDER_ATTEMPT_MANUAL),
        vec![SOCIAL_ALERT_REPORT_MANUAL_PROOF_REQUIRED.to_string()],
        vec![SOCIAL_ALERT_REPORT_MANUAL_PROOF_REQUIRED.to_string()],
        last_checked_at,
    )
}

fn provider_status_row(
    status_entry_id: SocialAlertReportFieldName,
    source_intent_ref: SocialAlertReportFieldName,
    source_preflight_status: SocialAlertReportFieldName,
    provider_attempt_ref: SocialAlertReportFieldName,
    readiness_refs: impl Into<SocialAlertReportTextList>,
    manual_proof_requirements: impl Into<SocialAlertReportTextList>,
    last_checked_at: impl Into<SocialAlertReportText>,
) -> SocialAlertReportProviderStatusRow {
    let readiness_refs = readiness_refs.into();
    let manual_proof_requirements = manual_proof_requirements.into();
    let last_checked_at = last_checked_at.into();
    SocialAlertReportProviderStatusRow {
        status_entry_id: status_entry_id.0.to_string(),
        source_intent_ref: source_intent_ref.0.to_string(),
        source_preflight_status: source_preflight_status.0.to_string(),
        provider_status: SOCIAL_ALERT_REPORT_PROVIDER_STATUS_MANUAL_REQUIRED.to_string(),
        status_proof_state: SOCIAL_ALERT_REPORT_PROVIDER_STATUS_PROOF_MANUAL_ACTION.to_string(),
        delivery_claim_state: SOCIAL_ALERT_REPORT_PROVIDER_DELIVERY_NOT_OBSERVED.to_string(),
        provider_attempt_ref: provider_attempt_ref.0.to_string(),
        readiness_refs: readiness_refs.0,
        provider_receipt_refs: Vec::new(),
        manual_proof_requirements: manual_proof_requirements.0,
        provider_delivery_implemented: false,
        provider_delivery_observed: false,
        delivered_notification_claimed: false,
        sensitive_provider_payload_claimed: false,
        provider_stores_child_evidence_claimed: false,
        last_checked_at: last_checked_at.0,
    }
}

fn high_risk_intent(created_at: impl Into<SocialAlertReportText>) -> SocialAlertReportIntent {
    let created_at = created_at.into();
    SocialAlertReportIntent {
        schema_version: SOCIAL_ALERT_REPORT_CONTRACT_SCHEMA_VERSION.to_string(),
        alert_report_intent_id: SOCIAL_ALERT_REPORT_INTENT_HIGH_RISK.to_string(),
        intent_kind: SOCIAL_ALERT_REPORT_INTENT_HIGH_RISK.to_string(),
        intent_status: SOCIAL_ALERT_REPORT_STATUS_LOCAL_OUTBOX.to_string(),
        priority: SOCIAL_ALERT_REPORT_PRIORITY_URGENT.to_string(),
        severity: SOCIAL_ALERT_REPORT_SEVERITY_CRITICAL.to_string(),
        device: device_ref(),
        notification_reason_code: SOCIAL_ALERT_REPORT_REASON_HIGH_RISK.to_string(),
        provider_channel_preference: SOCIAL_ALERT_REPORT_PROVIDER_CHANNEL_IN_APP.to_string(),
        parent_title_token: SOCIAL_ALERT_REPORT_TITLE_HIGH_RISK.to_string(),
        parent_body_token: SOCIAL_ALERT_REPORT_BODY_HIGH_RISK.to_string(),
        parent_action_token: SOCIAL_ALERT_REPORT_ACTION_OPEN_PARENT_REVIEW.to_string(),
        dashboard_panel_refs: vec![SOCIAL_ALERT_REPORT_PANEL_FEED_VIDEO_GATES.to_string()],
        explanation_snapshot_ref: SOCIAL_ALERT_REPORT_EXPLANATION_SNAPSHOT.to_string(),
        explanation_event_refs: vec![
            SOCIAL_ALERT_REPORT_EXPLANATION_EVENT_FEED_VIDEO_GATE.to_string()
        ],
        evidence_references: vec![evidence_reference(
            SocialAlertReportFieldName(SOCIAL_ALERT_REPORT_EVIDENCE_ROUTE_GATE),
            created_at.0.as_str(),
        )],
        policy_refs: vec![SOCIAL_ALERT_REPORT_POLICY_HIGH_RISK.to_string()],
        audit_refs: vec![SOCIAL_ALERT_REPORT_AUDIT_REF.to_string()],
        parent_report_ref: None,
        parent_action_ref: Some(parent_action_ref(created_at.0.as_str())),
        local_outbox_record_ref: Some(SOCIAL_ALERT_REPORT_LOCAL_OUTBOX_REF.to_string()),
        provider_attempt_refs: Vec::new(),
        provider_receipt_refs: Vec::new(),
        manual_proof_requirements: Vec::new(),
        minimal_payload_fields: minimal_payload_fields().0,
        delivery_claim_state: SOCIAL_ALERT_REPORT_DELIVERY_LOCAL_OUTBOX_ONLY.to_string(),
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
        created_at: created_at.0,
    }
}

fn manual_required_intent(created_at: impl Into<SocialAlertReportText>) -> SocialAlertReportIntent {
    let created_at = created_at.into();
    SocialAlertReportIntent {
        schema_version: SOCIAL_ALERT_REPORT_CONTRACT_SCHEMA_VERSION.to_string(),
        alert_report_intent_id: SOCIAL_ALERT_REPORT_INTENT_MANUAL_REQUIRED.to_string(),
        intent_kind: SOCIAL_ALERT_REPORT_INTENT_MANUAL_REQUIRED.to_string(),
        intent_status: SOCIAL_ALERT_REPORT_STATUS_MANUAL_REQUIRED.to_string(),
        priority: SOCIAL_ALERT_REPORT_PRIORITY_ATTENTION.to_string(),
        severity: SOCIAL_ALERT_REPORT_SEVERITY_WARNING.to_string(),
        device: device_ref(),
        notification_reason_code: SOCIAL_ALERT_REPORT_REASON_MANUAL_REQUIRED.to_string(),
        provider_channel_preference: SOCIAL_ALERT_REPORT_PROVIDER_CHANNEL_IN_APP.to_string(),
        parent_title_token: SOCIAL_ALERT_REPORT_TITLE_MANUAL_REQUIRED.to_string(),
        parent_body_token: SOCIAL_ALERT_REPORT_BODY_MANUAL_REQUIRED.to_string(),
        parent_action_token: SOCIAL_ALERT_REPORT_ACTION_REVIEW_MANUALLY.to_string(),
        dashboard_panel_refs: vec![SOCIAL_ALERT_REPORT_PANEL_MANUAL_REQUIRED_GAPS.to_string()],
        explanation_snapshot_ref: SOCIAL_ALERT_REPORT_EXPLANATION_SNAPSHOT.to_string(),
        explanation_event_refs: vec![
            SOCIAL_ALERT_REPORT_EXPLANATION_EVENT_MANUAL_REQUIRED.to_string()
        ],
        evidence_references: vec![evidence_reference(
            SocialAlertReportFieldName(SOCIAL_ALERT_REPORT_EVIDENCE_MANUAL_GAP),
            created_at.0.as_str(),
        )],
        policy_refs: vec![SOCIAL_ALERT_REPORT_POLICY_MANUAL_REQUIRED.to_string()],
        audit_refs: vec![SOCIAL_ALERT_REPORT_AUDIT_REF.to_string()],
        parent_report_ref: None,
        parent_action_ref: None,
        local_outbox_record_ref: None,
        provider_attempt_refs: Vec::new(),
        provider_receipt_refs: Vec::new(),
        manual_proof_requirements: vec![SOCIAL_ALERT_REPORT_MANUAL_PROOF_REQUIRED.to_string()],
        minimal_payload_fields: minimal_payload_fields().0,
        delivery_claim_state: SOCIAL_ALERT_REPORT_DELIVERY_MANUAL_REQUIRED.to_string(),
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
        created_at: created_at.0,
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
    evidence_ref: SocialAlertReportFieldName,
    observed_at: impl Into<SocialAlertReportText>,
) -> SocialAlertReportEvidenceRef {
    let observed_at = observed_at.into();
    SocialAlertReportEvidenceRef {
        evidence_reference_id: evidence_ref.0.to_string(),
        kind: SOCIAL_ALERT_REPORT_EVIDENCE_KIND_POLICY_DECISION.to_string(),
        observed_at: observed_at.0,
    }
}

fn parent_action_ref(
    created_at: impl Into<SocialAlertReportText>,
) -> SocialAlertReportParentActionRef {
    let created_at = created_at.into();
    SocialAlertReportParentActionRef {
        action_reference_id: SOCIAL_ALERT_REPORT_PARENT_ACTION_ID.to_string(),
        actor: SocialAlertReportParentActor {
            actor_id: SOCIAL_ALERT_REPORT_PARENT_ACTOR_ID.to_string(),
            role: SOCIAL_ALERT_REPORT_PARENT_ACTOR_ROLE.to_string(),
        },
        policy_version: SOCIAL_ALERT_REPORT_POLICY_VERSION.to_string(),
        created_at: created_at.0,
    }
}

fn minimal_payload_fields() -> SocialAlertReportTextList {
    SocialAlertReportTextList(vec![
        SOCIAL_ALERT_REPORT_PAYLOAD_ALERT_ID.to_string(),
        SOCIAL_ALERT_REPORT_PAYLOAD_FAMILY_DEVICE_SCOPE.to_string(),
        SOCIAL_ALERT_REPORT_PAYLOAD_SEVERITY.to_string(),
        SOCIAL_ALERT_REPORT_PAYLOAD_REASON_CODE.to_string(),
        SOCIAL_ALERT_REPORT_PAYLOAD_EVIDENCE_REF.to_string(),
        SOCIAL_ALERT_REPORT_PAYLOAD_POLICY_REF.to_string(),
        SOCIAL_ALERT_REPORT_PAYLOAD_EXPLANATION_REF.to_string(),
        SOCIAL_ALERT_REPORT_PAYLOAD_PARENT_ACTION_LINK_REF.to_string(),
    ])
}
