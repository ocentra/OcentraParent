use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};
use ocentra_parent_agent_protocol::SocialAuditExplanationClaimBoundaries;
use ocentra_parent_agent_protocol::SocialAuditExplanationEntry;
use ocentra_parent_agent_protocol::SocialAuditExplanationEvidenceLink;
use ocentra_parent_agent_protocol::SocialAuditExplanationSnapshot;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_ACTION_ALLOW;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_ACTION_MANUAL_REVIEW;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_ACTION_PARENT_REVIEW;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_ACTION_WARN;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_AUDIENCE_PARENT;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_AUDIT_REF;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_CHILD_PROFILE_ID;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_DECISION_CANDIDATE_ONLY;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_DECISION_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_DECISION_PARENT_RECORDED;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_EVIDENCE_CONNECTOR_BOUNDARY;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_EVIDENCE_DECISION_MEMORY;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_EVIDENCE_MANUAL_GAP;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_EVIDENCE_NATIVE_CAPABILITY;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_EVIDENCE_PARENT_APPROVAL;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_EVIDENCE_POLICY_CANDIDATE;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_EVIDENCE_ROUTE_EVIDENCE;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_FAMILY_ID;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_PARENT_RULE_MATCH;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_SOCIAL_RISK_HIGH;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_VIDEO_SAFETY_RISK;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_POLICY_VERSION;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_REASON_CONNECTOR_BOUNDARY_LINKED;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_REASON_EVIDENCE_LINKED;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_REASON_MANUAL_REVIEW_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_REASON_MEMORY_LINKED;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_REASON_MISSING_RUNTIME_PROOF;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_REASON_NATIVE_APP_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_REASON_PARENT_DECISION_LINKED;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_REASON_POLICY_CANDIDATE_LINKED;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_REF_APPROVAL_DECISION;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_REF_APPROVAL_REQUEST;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_REF_CONNECTOR_BOUNDARY;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_REF_DECISION_MEMORY;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_REF_MANUAL_GAP;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_REF_NATIVE_CAPABILITY;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_SNAPSHOT_ID;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_STATUS_CONTRACT_ONLY;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_STATUS_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_STATUS_READY_FOR_PARENT;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_SUBJECT_ACCOUNT_APPROVAL;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_SUBJECT_CONNECTOR_BOUNDARY;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_SUBJECT_DECISION_MEMORY;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_SUBJECT_FEED_VIDEO_GATE;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_SUBJECT_MANUAL_REQUIRED_GAP;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_SUBJECT_NATIVE_APP_GAP;

#[path = "social_audit_explanation_read_model_payload/field_pairs.rs"]
mod field_pairs;

use crate::{
    event_builder::build_event, json_contract::serialize_json_string, time::timestamp_now,
};
use field_pairs::{
    field_pair, social_audit_explanation_fields_from_pairs, SocialAuditExplanationFieldKey,
    SocialAuditExplanationFieldPair, SocialAuditExplanationTextRef,
};

pub fn social_audit_explanation_read_model_from_service() -> SocialAuditExplanationSnapshot {
    SocialAuditExplanationSnapshot {
        schema_version: SOCIAL_AUDIT_EXPLANATION_SCHEMA_VERSION.to_string(),
        snapshot_id: SOCIAL_AUDIT_EXPLANATION_SNAPSHOT_ID.to_string(),
        family_id: SOCIAL_AUDIT_EXPLANATION_FAMILY_ID.to_string(),
        child_profile_id: SOCIAL_AUDIT_EXPLANATION_CHILD_PROFILE_ID.to_string(),
        captured_at: timestamp_now(),
        entries: vec![
            account_approval_entry(),
            feed_video_entry(),
            native_app_gap_entry(),
            connector_boundary_entry(),
            decision_memory_entry(),
            manual_gap_entry(),
        ],
        claim_boundaries: SocialAuditExplanationClaimBoundaries {
            runtime_audit_store: SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED.to_string(),
            rendered_explanation_ui: SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED.to_string(),
            notification_delivery: SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED.to_string(),
            raw_account_video_message_content: SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED
                .to_string(),
            connector_authorization: SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED.to_string(),
            native_app_control: SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED.to_string(),
            final_policy_decision: SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED.to_string(),
            enforcement: SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED.to_string(),
        },
    }
}

pub fn social_audit_explanation_read_model_payload(
    read_model: &SocialAuditExplanationSnapshot,
) -> LogFields {
    social_audit_explanation_fields_from_pairs(read_model_pairs(read_model))
}

pub async fn build_browser_social_audit_explanation_read_model_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let read_model = social_audit_explanation_read_model_from_service();
    build_event(
        constants::event_id::BROWSER_SOCIAL_AUDIT_EXPLANATION_READ_MODEL_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentBrowserSocialAuditExplanationReadModelReported,
        LogLevel::Info,
        social_audit_explanation_read_model_payload(&read_model),
        None,
    )
}

fn read_model_pairs(
    read_model: &SocialAuditExplanationSnapshot,
) -> Vec<SocialAuditExplanationFieldPair> {
    vec![
        field_pair(
            &SocialAuditExplanationFieldKey(constants::field::GENERATED_AT),
            LogFieldValue::String(read_model.captured_at.clone()),
        ),
        field_pair(
            &SocialAuditExplanationFieldKey(constants::field::RETURNED),
            LogFieldValue::Number(read_model.entries.len() as f64),
        ),
        field_pair(
            &SocialAuditExplanationFieldKey(
                constants::field::BROWSER_SOCIAL_AUDIT_EXPLANATION_READ_MODEL,
            ),
            LogFieldValue::String(serialize_json_string(read_model).0),
        ),
    ]
}

fn account_approval_entry() -> SocialAuditExplanationEntry {
    entry(EntryInput {
        subject_kind: SOCIAL_AUDIT_EXPLANATION_SUBJECT_ACCOUNT_APPROVAL,
        status: SOCIAL_AUDIT_EXPLANATION_STATUS_READY_FOR_PARENT,
        decision_state: SOCIAL_AUDIT_EXPLANATION_DECISION_PARENT_RECORDED,
        policy_version_ref: Some(SOCIAL_AUDIT_EXPLANATION_POLICY_VERSION),
        action_candidate: SOCIAL_AUDIT_EXPLANATION_ACTION_PARENT_REVIEW,
        policy_reason_codes: vec![SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_PARENT_RULE_MATCH],
        explanation_reasons: vec![
            SOCIAL_AUDIT_EXPLANATION_REASON_EVIDENCE_LINKED,
            SOCIAL_AUDIT_EXPLANATION_REASON_POLICY_CANDIDATE_LINKED,
            SOCIAL_AUDIT_EXPLANATION_REASON_PARENT_DECISION_LINKED,
        ],
        evidence_links: vec![
            evidence_link(SocialAuditExplanationTextRef(
                SOCIAL_AUDIT_EXPLANATION_EVIDENCE_POLICY_CANDIDATE,
            )),
            evidence_link(SocialAuditExplanationTextRef(
                SOCIAL_AUDIT_EXPLANATION_EVIDENCE_PARENT_APPROVAL,
            )),
        ],
        refs: OptionalRefs {
            parent_approval_request_ref: Some(SOCIAL_AUDIT_EXPLANATION_REF_APPROVAL_REQUEST),
            parent_approval_decision_ref: Some(SOCIAL_AUDIT_EXPLANATION_REF_APPROVAL_DECISION),
            ..OptionalRefs::default()
        },
    })
}

fn feed_video_entry() -> SocialAuditExplanationEntry {
    entry(EntryInput {
        subject_kind: SOCIAL_AUDIT_EXPLANATION_SUBJECT_FEED_VIDEO_GATE,
        status: SOCIAL_AUDIT_EXPLANATION_STATUS_READY_FOR_PARENT,
        decision_state: SOCIAL_AUDIT_EXPLANATION_DECISION_CANDIDATE_ONLY,
        policy_version_ref: Some(SOCIAL_AUDIT_EXPLANATION_POLICY_VERSION),
        action_candidate: SOCIAL_AUDIT_EXPLANATION_ACTION_WARN,
        policy_reason_codes: vec![
            SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_SOCIAL_RISK_HIGH,
            SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_VIDEO_SAFETY_RISK,
        ],
        explanation_reasons: vec![
            SOCIAL_AUDIT_EXPLANATION_REASON_EVIDENCE_LINKED,
            SOCIAL_AUDIT_EXPLANATION_REASON_POLICY_CANDIDATE_LINKED,
        ],
        evidence_links: vec![
            evidence_link(SocialAuditExplanationTextRef(
                SOCIAL_AUDIT_EXPLANATION_EVIDENCE_ROUTE_EVIDENCE,
            )),
            evidence_link(SocialAuditExplanationTextRef(
                SOCIAL_AUDIT_EXPLANATION_EVIDENCE_POLICY_CANDIDATE,
            )),
        ],
        refs: OptionalRefs::default(),
    })
}

fn native_app_gap_entry() -> SocialAuditExplanationEntry {
    manual_entry(
        &SocialAuditExplanationTextRef(SOCIAL_AUDIT_EXPLANATION_SUBJECT_NATIVE_APP_GAP),
        &SocialAuditExplanationTextRef(SOCIAL_AUDIT_EXPLANATION_EVIDENCE_NATIVE_CAPABILITY),
        vec![
            SocialAuditExplanationTextRef(
                SOCIAL_AUDIT_EXPLANATION_REASON_NATIVE_APP_MANUAL_REQUIRED,
            ),
            SocialAuditExplanationTextRef(SOCIAL_AUDIT_EXPLANATION_REASON_MISSING_RUNTIME_PROOF),
        ],
        OptionalRefs {
            native_capability_ref: Some(SOCIAL_AUDIT_EXPLANATION_REF_NATIVE_CAPABILITY),
            ..OptionalRefs::default()
        },
    )
}

fn connector_boundary_entry() -> SocialAuditExplanationEntry {
    manual_entry(
        &SocialAuditExplanationTextRef(SOCIAL_AUDIT_EXPLANATION_SUBJECT_CONNECTOR_BOUNDARY),
        &SocialAuditExplanationTextRef(SOCIAL_AUDIT_EXPLANATION_EVIDENCE_CONNECTOR_BOUNDARY),
        vec![
            SocialAuditExplanationTextRef(
                SOCIAL_AUDIT_EXPLANATION_REASON_CONNECTOR_BOUNDARY_LINKED,
            ),
            SocialAuditExplanationTextRef(SOCIAL_AUDIT_EXPLANATION_REASON_MANUAL_REVIEW_REQUIRED),
        ],
        OptionalRefs {
            connector_boundary_ref: Some(SOCIAL_AUDIT_EXPLANATION_REF_CONNECTOR_BOUNDARY),
            ..OptionalRefs::default()
        },
    )
}

fn decision_memory_entry() -> SocialAuditExplanationEntry {
    entry(EntryInput {
        subject_kind: SOCIAL_AUDIT_EXPLANATION_SUBJECT_DECISION_MEMORY,
        status: SOCIAL_AUDIT_EXPLANATION_STATUS_CONTRACT_ONLY,
        decision_state: SOCIAL_AUDIT_EXPLANATION_DECISION_CANDIDATE_ONLY,
        policy_version_ref: Some(SOCIAL_AUDIT_EXPLANATION_POLICY_VERSION),
        action_candidate: SOCIAL_AUDIT_EXPLANATION_ACTION_ALLOW,
        policy_reason_codes: vec![SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_PARENT_RULE_MATCH],
        explanation_reasons: vec![
            SOCIAL_AUDIT_EXPLANATION_REASON_MEMORY_LINKED,
            SOCIAL_AUDIT_EXPLANATION_REASON_EVIDENCE_LINKED,
        ],
        evidence_links: vec![evidence_link(SocialAuditExplanationTextRef(
            SOCIAL_AUDIT_EXPLANATION_EVIDENCE_DECISION_MEMORY,
        ))],
        refs: OptionalRefs {
            decision_memory_ref: Some(SOCIAL_AUDIT_EXPLANATION_REF_DECISION_MEMORY),
            ..OptionalRefs::default()
        },
    })
}

fn manual_gap_entry() -> SocialAuditExplanationEntry {
    manual_entry(
        &SocialAuditExplanationTextRef(SOCIAL_AUDIT_EXPLANATION_SUBJECT_MANUAL_REQUIRED_GAP),
        &SocialAuditExplanationTextRef(SOCIAL_AUDIT_EXPLANATION_EVIDENCE_MANUAL_GAP),
        vec![SocialAuditExplanationTextRef(
            SOCIAL_AUDIT_EXPLANATION_REASON_MANUAL_REVIEW_REQUIRED,
        )],
        OptionalRefs {
            manual_required_ref: Some(SOCIAL_AUDIT_EXPLANATION_REF_MANUAL_GAP),
            ..OptionalRefs::default()
        },
    )
}

fn manual_entry(
    subject_kind: &SocialAuditExplanationTextRef,
    evidence_kind: &SocialAuditExplanationTextRef,
    explanation_reasons: Vec<SocialAuditExplanationTextRef>,
    refs: OptionalRefs,
) -> SocialAuditExplanationEntry {
    entry(EntryInput {
        subject_kind: subject_kind.0,
        status: SOCIAL_AUDIT_EXPLANATION_STATUS_MANUAL_REQUIRED,
        decision_state: SOCIAL_AUDIT_EXPLANATION_DECISION_MANUAL_REQUIRED,
        policy_version_ref: None,
        action_candidate: SOCIAL_AUDIT_EXPLANATION_ACTION_MANUAL_REVIEW,
        policy_reason_codes: vec![SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_MANUAL_REQUIRED],
        explanation_reasons: explanation_reasons
            .into_iter()
            .map(|reason| reason.0)
            .collect(),
        evidence_links: vec![evidence_link(SocialAuditExplanationTextRef(
            evidence_kind.0,
        ))],
        refs,
    })
}

struct EntryInput {
    subject_kind: &'static str,
    status: &'static str,
    decision_state: &'static str,
    policy_version_ref: Option<&'static str>,
    action_candidate: &'static str,
    policy_reason_codes: Vec<&'static str>,
    explanation_reasons: Vec<&'static str>,
    evidence_links: Vec<SocialAuditExplanationEvidenceLink>,
    refs: OptionalRefs,
}

fn entry(input: EntryInput) -> SocialAuditExplanationEntry {
    SocialAuditExplanationEntry {
        event_id: input.subject_kind.to_string(),
        subject_kind: input.subject_kind.to_string(),
        status: input.status.to_string(),
        decision_state: input.decision_state.to_string(),
        audience: SOCIAL_AUDIT_EXPLANATION_AUDIENCE_PARENT.to_string(),
        policy_version_ref: input.policy_version_ref.map(str::to_string),
        action_candidate: input.action_candidate.to_string(),
        policy_reason_codes: input
            .policy_reason_codes
            .into_iter()
            .map(str::to_string)
            .collect(),
        explanation_reasons: input
            .explanation_reasons
            .into_iter()
            .map(str::to_string)
            .collect(),
        evidence_links: input.evidence_links,
        audit_refs: vec![SOCIAL_AUDIT_EXPLANATION_AUDIT_REF.to_string()],
        parent_approval_request_ref: input.refs.parent_approval_request_ref.map(str::to_string),
        parent_approval_decision_ref: input.refs.parent_approval_decision_ref.map(str::to_string),
        decision_memory_ref: input.refs.decision_memory_ref.map(str::to_string),
        connector_boundary_ref: input.refs.connector_boundary_ref.map(str::to_string),
        native_capability_ref: input.refs.native_capability_ref.map(str::to_string),
        manual_required_ref: input.refs.manual_required_ref.map(str::to_string),
        runtime_audit_store_claimed: false,
        rendered_explanation_ui_claimed: false,
        notification_delivered_claimed: false,
        raw_account_data_included: false,
        raw_video_content_included: false,
        raw_message_content_included: false,
        connector_authorization_claimed: false,
        native_app_control_claimed: false,
        final_policy_decision_claimed: false,
        enforcement_claimed: false,
    }
}

fn evidence_link(
    SocialAuditExplanationTextRef(evidence_kind): SocialAuditExplanationTextRef,
) -> SocialAuditExplanationEvidenceLink {
    SocialAuditExplanationEvidenceLink {
        evidence_kind: evidence_kind.to_string(),
        evidence_ref: evidence_kind.to_string(),
    }
}

#[derive(Default)]
struct OptionalRefs {
    parent_approval_request_ref: Option<&'static str>,
    parent_approval_decision_ref: Option<&'static str>,
    decision_memory_ref: Option<&'static str>,
    connector_boundary_ref: Option<&'static str>,
    native_capability_ref: Option<&'static str>,
    manual_required_ref: Option<&'static str>,
}
