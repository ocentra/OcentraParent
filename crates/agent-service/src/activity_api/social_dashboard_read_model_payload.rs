use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};
use ocentra_parent_agent_protocol::SocialDashboardClaimBoundaries;
use ocentra_parent_agent_protocol::SocialDashboardPanel;
use ocentra_parent_agent_protocol::SocialDashboardUxSnapshot;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_ACTION_MANUAL_REVIEW;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_ACTION_OPEN_PARENT_APPROVAL;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_ACTION_REVIEW_CONNECTOR_BOUNDARY;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_ACTION_REVIEW_FEED_GATE;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_ACTION_REVIEW_MEMORY_ENTRY;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_ACTION_REVIEW_NATIVE_CAPABILITY;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_ACTION_REVIEW_SETTINGS_CUSTODY;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_CAPABILITY_READY;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_CHILD_PROFILE_ID;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_CLAIM_NOT_CLAIMED;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_CUSTODY_CHILD_DEVICE_QUERY_STORE;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_EVIDENCE_ACCOUNT_APPROVAL_QUEUE;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_EVIDENCE_CONNECTOR_BOUNDARIES;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_EVIDENCE_DECISION_MEMORY;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_EVIDENCE_FEED_VIDEO_GATES;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_EVIDENCE_MANUAL_REQUIRED_GAPS;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_EVIDENCE_NATIVE_APP_CAPABILITY;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_EVIDENCE_SETTINGS_CUSTODY;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_FAMILY_ID;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_PANEL_ACCOUNT_APPROVAL_QUEUE;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_PANEL_CONNECTOR_BOUNDARIES;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_PANEL_DECISION_MEMORY;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_PANEL_FEED_VIDEO_GATES;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_PANEL_MANUAL_REQUIRED_GAPS;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_PANEL_NATIVE_APP_CAPABILITY;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_PANEL_SETTINGS_CUSTODY;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_REASON_CONNECTOR_BOUNDARY_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_REASON_FEED_VIDEO_GATE_CANDIDATE;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_REASON_MEMORY_CONTRACT_ONLY;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_REASON_NATIVE_APP_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_REASON_PARENT_REVIEW_NEEDED;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_REASON_PLATFORM_PROOF_GAP;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_REASON_SETTINGS_CUSTODY_RUNTIME_GAP;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_SEVERITY_INFO;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_SEVERITY_WARNING;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_STATUS_CONTRACT_ONLY;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_STATUS_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_DASHBOARD_STATUS_READY_FOR_REVIEW;

use crate::{
    event_builder::build_event, fields::fields_from_pairs, json_contract::serialize_json_string,
    time::timestamp_now,
};

type FieldPair = (&'static str, LogFieldValue);

pub fn social_dashboard_read_model_from_service() -> SocialDashboardUxSnapshot {
    SocialDashboardUxSnapshot {
        schema_version: SOCIAL_DASHBOARD_SCHEMA_VERSION.to_string(),
        family_id: SOCIAL_DASHBOARD_FAMILY_ID.to_string(),
        child_profile_id: SOCIAL_DASHBOARD_CHILD_PROFILE_ID.to_string(),
        generated_at: timestamp_now(),
        panels: social_dashboard_panels(),
        claim_boundaries: SocialDashboardClaimBoundaries {
            rendered_portal_ui: SOCIAL_DASHBOARD_CLAIM_NOT_CLAIMED.to_string(),
            notification_delivery: SOCIAL_DASHBOARD_CLAIM_NOT_CLAIMED.to_string(),
            runtime_data_fetch: SOCIAL_DASHBOARD_CLAIM_NOT_CLAIMED.to_string(),
            policy_decision: SOCIAL_DASHBOARD_CLAIM_NOT_CLAIMED.to_string(),
            native_app_control: SOCIAL_DASHBOARD_CLAIM_NOT_CLAIMED.to_string(),
            connector_authorization: SOCIAL_DASHBOARD_CLAIM_NOT_CLAIMED.to_string(),
            enforcement: SOCIAL_DASHBOARD_CLAIM_NOT_CLAIMED.to_string(),
        },
    }
}

fn social_dashboard_panels() -> Vec<SocialDashboardPanel> {
    vec![
        panel(
            SOCIAL_DASHBOARD_PANEL_ACCOUNT_APPROVAL_QUEUE,
            SOCIAL_DASHBOARD_STATUS_READY_FOR_REVIEW,
            SOCIAL_DASHBOARD_ACTION_OPEN_PARENT_APPROVAL,
            SOCIAL_DASHBOARD_SEVERITY_INFO,
            0,
            SOCIAL_DASHBOARD_EVIDENCE_ACCOUNT_APPROVAL_QUEUE,
            SOCIAL_DASHBOARD_REASON_PARENT_REVIEW_NEEDED,
        ),
        panel(
            SOCIAL_DASHBOARD_PANEL_FEED_VIDEO_GATES,
            SOCIAL_DASHBOARD_STATUS_READY_FOR_REVIEW,
            SOCIAL_DASHBOARD_ACTION_REVIEW_FEED_GATE,
            SOCIAL_DASHBOARD_SEVERITY_INFO,
            1,
            SOCIAL_DASHBOARD_EVIDENCE_FEED_VIDEO_GATES,
            SOCIAL_DASHBOARD_REASON_FEED_VIDEO_GATE_CANDIDATE,
        ),
        panel(
            SOCIAL_DASHBOARD_PANEL_NATIVE_APP_CAPABILITY,
            SOCIAL_DASHBOARD_STATUS_MANUAL_REQUIRED,
            SOCIAL_DASHBOARD_ACTION_REVIEW_NATIVE_CAPABILITY,
            SOCIAL_DASHBOARD_SEVERITY_WARNING,
            2,
            SOCIAL_DASHBOARD_EVIDENCE_NATIVE_APP_CAPABILITY,
            SOCIAL_DASHBOARD_REASON_NATIVE_APP_MANUAL_REQUIRED,
        ),
        panel(
            SOCIAL_DASHBOARD_PANEL_CONNECTOR_BOUNDARIES,
            SOCIAL_DASHBOARD_STATUS_MANUAL_REQUIRED,
            SOCIAL_DASHBOARD_ACTION_REVIEW_CONNECTOR_BOUNDARY,
            SOCIAL_DASHBOARD_SEVERITY_WARNING,
            3,
            SOCIAL_DASHBOARD_EVIDENCE_CONNECTOR_BOUNDARIES,
            SOCIAL_DASHBOARD_REASON_CONNECTOR_BOUNDARY_MANUAL_REQUIRED,
        ),
        panel(
            SOCIAL_DASHBOARD_PANEL_DECISION_MEMORY,
            SOCIAL_DASHBOARD_STATUS_CONTRACT_ONLY,
            SOCIAL_DASHBOARD_ACTION_REVIEW_MEMORY_ENTRY,
            SOCIAL_DASHBOARD_SEVERITY_INFO,
            4,
            SOCIAL_DASHBOARD_EVIDENCE_DECISION_MEMORY,
            SOCIAL_DASHBOARD_REASON_MEMORY_CONTRACT_ONLY,
        ),
        panel(
            SOCIAL_DASHBOARD_PANEL_SETTINGS_CUSTODY,
            SOCIAL_DASHBOARD_STATUS_MANUAL_REQUIRED,
            SOCIAL_DASHBOARD_ACTION_REVIEW_SETTINGS_CUSTODY,
            SOCIAL_DASHBOARD_SEVERITY_WARNING,
            5,
            SOCIAL_DASHBOARD_EVIDENCE_SETTINGS_CUSTODY,
            SOCIAL_DASHBOARD_REASON_SETTINGS_CUSTODY_RUNTIME_GAP,
        ),
        panel(
            SOCIAL_DASHBOARD_PANEL_MANUAL_REQUIRED_GAPS,
            SOCIAL_DASHBOARD_STATUS_MANUAL_REQUIRED,
            SOCIAL_DASHBOARD_ACTION_MANUAL_REVIEW,
            SOCIAL_DASHBOARD_SEVERITY_WARNING,
            6,
            SOCIAL_DASHBOARD_EVIDENCE_MANUAL_REQUIRED_GAPS,
            SOCIAL_DASHBOARD_REASON_PLATFORM_PROOF_GAP,
        ),
    ]
}

pub fn social_dashboard_read_model_payload(read_model: &SocialDashboardUxSnapshot) -> LogFields {
    fields_from_pairs(read_model_pairs(read_model))
}

pub async fn build_browser_social_dashboard_read_model_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let read_model = social_dashboard_read_model_from_service();
    build_event(
        constants::event_id::BROWSER_SOCIAL_DASHBOARD_READ_MODEL_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentBrowserSocialDashboardReadModelReported,
        LogLevel::Info,
        social_dashboard_read_model_payload(&read_model),
        None,
    )
}

fn read_model_pairs(read_model: &SocialDashboardUxSnapshot) -> Vec<FieldPair> {
    vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        (
            constants::field::CUSTODY_LABEL,
            LogFieldValue::String(SOCIAL_DASHBOARD_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(SOCIAL_DASHBOARD_CAPABILITY_READY.to_string()),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.panels.len() as f64),
        ),
        (
            constants::field::BROWSER_SOCIAL_DASHBOARD_READ_MODEL,
            LogFieldValue::String(serialize_json_string(read_model)),
        ),
    ]
}

fn panel(
    panel_kind: &'static str,
    status: &'static str,
    primary_action: &'static str,
    severity: &'static str,
    sort_order: u64,
    evidence_ref: &'static str,
    reason: &'static str,
) -> SocialDashboardPanel {
    SocialDashboardPanel {
        panel_id: panel_kind.to_string(),
        panel_kind: panel_kind.to_string(),
        status: status.to_string(),
        primary_action: primary_action.to_string(),
        severity: severity.to_string(),
        sort_order,
        source_evidence_refs: vec![evidence_ref.to_string()],
        reasons: vec![reason.to_string()],
        rendered_ui_claimed: false,
        notification_claimed: false,
        runtime_data_fetch_claimed: false,
        policy_decision_claimed: false,
        native_app_control_claimed: false,
        connector_authorization_claimed: false,
        enforcement_claimed: false,
    }
}
