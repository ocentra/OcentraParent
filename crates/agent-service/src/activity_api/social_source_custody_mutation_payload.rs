use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};
use ocentra_parent_agent_protocol::SocialSourceCustodyMutationSnapshot;
use ocentra_parent_agent_protocol::SocialSourceCustodySettingsSnapshot;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_AUDIT_REF;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_CHILD_PROFILE_ID;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_DEVICE_ID;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_EVIDENCE_REF;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_MODE_REDACTED_REFS;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_MUTATION_ID;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_MUTATION_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_MUTATION_STATE_APPLIED;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_NO_CONNECTOR_API;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_NO_CONNECTOR_TOKEN;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_NO_ENFORCEMENT;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_NO_FINAL_POLICY;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_NO_RAW_MESSAGE;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_NO_RAW_VIDEO;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_NO_RUNTIME_CUSTODY_CLAIM;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_NO_RUNTIME_UI;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_NO_SCREENSHOT;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_PERMISSION_ENABLED;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_PRIVACY_EVIDENCE_ID;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_RETENTION_REDACTED_JOURNAL;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_SCOPE_MANAGED_BROWSER;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_SETTINGS_ID;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_USE_AI_CANDIDATE;
use ocentra_parent_agent_protocol::SOCIAL_SOURCE_CUSTODY_USE_PARENT_EXPLANATION;

use crate::{
    event_builder::build_event, fields::fields_from_pairs, json_contract::serialize_json_string,
    time::timestamp_now,
};

#[derive(Clone, Debug, PartialEq)]
struct FieldPairs(Vec<(&'static str, LogFieldValue)>);

#[derive(Clone, Debug, PartialEq, Eq)]
struct RequestedAtText(String);

#[derive(Clone, Debug, PartialEq, Eq)]
struct ClaimLabels(Vec<String>);

pub fn social_source_custody_mutation_from_command(
    command: &AgentCommandEnvelope,
) -> SocialSourceCustodyMutationSnapshot {
    let requested_at = requested_at(command);
    let applied_at: String = timestamp_now();
    SocialSourceCustodyMutationSnapshot {
        schema_version: SOCIAL_SOURCE_CUSTODY_MUTATION_SCHEMA_VERSION.to_string(),
        mutation_id: SOCIAL_SOURCE_CUSTODY_MUTATION_ID.to_string(),
        requested_at: requested_at.0,
        applied_at: applied_at.clone(),
        mutation_state: SOCIAL_SOURCE_CUSTODY_MUTATION_STATE_APPLIED.to_string(),
        settings: SocialSourceCustodySettingsSnapshot {
            schema_version: 1,
            settings_id: SOCIAL_SOURCE_CUSTODY_SETTINGS_ID.to_string(),
            generated_at: applied_at,
            child_profile_ref: SOCIAL_SOURCE_CUSTODY_CHILD_PROFILE_ID.to_string(),
            device_id: SOCIAL_SOURCE_CUSTODY_DEVICE_ID.to_string(),
            source_privacy_evidence_ids: vec![SOCIAL_SOURCE_CUSTODY_PRIVACY_EVIDENCE_ID.to_string()],
            evidence_refs: vec![SOCIAL_SOURCE_CUSTODY_EVIDENCE_REF.to_string()],
            setting_scope: SOCIAL_SOURCE_CUSTODY_SCOPE_MANAGED_BROWSER.to_string(),
            permission_state: SOCIAL_SOURCE_CUSTODY_PERMISSION_ENABLED.to_string(),
            custody_mode: SOCIAL_SOURCE_CUSTODY_MODE_REDACTED_REFS.to_string(),
            retention_mode: SOCIAL_SOURCE_CUSTODY_RETENTION_REDACTED_JOURNAL.to_string(),
            permitted_downstream_uses: vec![
                SOCIAL_SOURCE_CUSTODY_USE_AI_CANDIDATE.to_string(),
                SOCIAL_SOURCE_CUSTODY_USE_PARENT_EXPLANATION.to_string(),
            ],
            disabled_use_reasons: Vec::new(),
            parent_review_refs: Vec::new(),
            connector_authorization_refs: Vec::new(),
            manual_proof_requirements: Vec::new(),
            no_claim_labels: no_claim_labels().0,
            raw_message_content_allowed: false,
            raw_video_content_allowed: false,
            screenshot_custody_allowed: false,
            connector_token_stored: false,
            connector_api_called: false,
            runtime_settings_ui_claimed: false,
            runtime_custody_mutation_claimed: false,
            final_policy_decision_claimed: false,
            enforcement_claimed: false,
        },
        evidence_refs: vec![SOCIAL_SOURCE_CUSTODY_EVIDENCE_REF.to_string()],
        audit_refs: vec![SOCIAL_SOURCE_CUSTODY_AUDIT_REF.to_string()],
        service_mutation_executed: true,
        runtime_custody_mutation_applied: true,
        raw_content_custody_claimed: false,
        connector_api_called: false,
        final_policy_decision_claimed: false,
        enforcement_claimed: false,
        product_claim_ready: false,
    }
}

pub fn social_source_custody_mutation_payload(
    mutation: &SocialSourceCustodyMutationSnapshot,
) -> LogFields {
    fields_from_pairs(mutation_pairs(mutation).0)
}

pub async fn build_browser_social_source_custody_mutation_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let mutation = social_source_custody_mutation_from_command(&command);
    build_event(
        constants::event_id::BROWSER_SOCIAL_SOURCE_CUSTODY_MUTATION_APPLIED,
        &command.message_id,
        command.source,
        AgentEventName::AgentBrowserSocialSourceCustodyMutationApplied,
        LogLevel::Info,
        social_source_custody_mutation_payload(&mutation),
        None,
    )
}

fn mutation_pairs(mutation: &SocialSourceCustodyMutationSnapshot) -> FieldPairs {
    FieldPairs(vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(mutation.applied_at.clone()),
        ),
        (
            constants::field::CUSTODY_LABEL,
            LogFieldValue::String(SOCIAL_SOURCE_CUSTODY_RETENTION_REDACTED_JOURNAL.to_string()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(SOCIAL_SOURCE_CUSTODY_MUTATION_STATE_APPLIED.to_string()),
        ),
        (
            constants::field::BROWSER_SOCIAL_SOURCE_CUSTODY_MUTATION,
            LogFieldValue::String(serialize_json_string(mutation).0),
        ),
    ])
}

fn requested_at(command: &AgentCommandEnvelope) -> RequestedAtText {
    match command.payload.get(constants::field::REQUESTED_AT) {
        Some(LogFieldValue::String(value)) if !value.is_empty() => RequestedAtText(value.clone()),
        _ => RequestedAtText(command.sent_at.clone()),
    }
}

fn no_claim_labels() -> ClaimLabels {
    ClaimLabels(vec![
        SOCIAL_SOURCE_CUSTODY_NO_RAW_MESSAGE.to_string(),
        SOCIAL_SOURCE_CUSTODY_NO_RAW_VIDEO.to_string(),
        SOCIAL_SOURCE_CUSTODY_NO_SCREENSHOT.to_string(),
        SOCIAL_SOURCE_CUSTODY_NO_CONNECTOR_TOKEN.to_string(),
        SOCIAL_SOURCE_CUSTODY_NO_CONNECTOR_API.to_string(),
        SOCIAL_SOURCE_CUSTODY_NO_RUNTIME_UI.to_string(),
        SOCIAL_SOURCE_CUSTODY_NO_RUNTIME_CUSTODY_CLAIM.to_string(),
        SOCIAL_SOURCE_CUSTODY_NO_FINAL_POLICY.to_string(),
        SOCIAL_SOURCE_CUSTODY_NO_ENFORCEMENT.to_string(),
    ])
}
