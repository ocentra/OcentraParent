use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LogFieldValue, LogFields,
    LogLevel, SocialAuditExplanationClaimBoundaries, SocialAuditExplanationEntry,
    SocialAuditExplanationEvidenceLink, SocialAuditExplanationSnapshot,
    SOCIAL_AUDIT_EXPLANATION_ACTION_ALLOW, SOCIAL_AUDIT_EXPLANATION_ACTION_MANUAL_REVIEW,
    SOCIAL_AUDIT_EXPLANATION_ACTION_PARENT_REVIEW, SOCIAL_AUDIT_EXPLANATION_ACTION_WARN,
    SOCIAL_AUDIT_EXPLANATION_AUDIENCE_PARENT, SOCIAL_AUDIT_EXPLANATION_AUDIT_REF,
    SOCIAL_AUDIT_EXPLANATION_CHILD_PROFILE_ID, SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED,
    SOCIAL_AUDIT_EXPLANATION_DECISION_CANDIDATE_ONLY,
    SOCIAL_AUDIT_EXPLANATION_DECISION_MANUAL_REQUIRED,
    SOCIAL_AUDIT_EXPLANATION_DECISION_PARENT_RECORDED,
    SOCIAL_AUDIT_EXPLANATION_EVIDENCE_CONNECTOR_BOUNDARY,
    SOCIAL_AUDIT_EXPLANATION_EVIDENCE_DECISION_MEMORY,
    SOCIAL_AUDIT_EXPLANATION_EVIDENCE_MANUAL_GAP,
    SOCIAL_AUDIT_EXPLANATION_EVIDENCE_NATIVE_CAPABILITY,
    SOCIAL_AUDIT_EXPLANATION_EVIDENCE_PARENT_APPROVAL,
    SOCIAL_AUDIT_EXPLANATION_EVIDENCE_POLICY_CANDIDATE,
    SOCIAL_AUDIT_EXPLANATION_EVIDENCE_ROUTE_EVIDENCE, SOCIAL_AUDIT_EXPLANATION_FAMILY_ID,
    SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_MANUAL_REQUIRED,
    SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_PARENT_RULE_MATCH,
    SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_SOCIAL_RISK_HIGH,
    SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_VIDEO_SAFETY_RISK,
    SOCIAL_AUDIT_EXPLANATION_POLICY_VERSION,
    SOCIAL_AUDIT_EXPLANATION_REASON_CONNECTOR_BOUNDARY_LINKED,
    SOCIAL_AUDIT_EXPLANATION_REASON_EVIDENCE_LINKED,
    SOCIAL_AUDIT_EXPLANATION_REASON_MANUAL_REVIEW_REQUIRED,
    SOCIAL_AUDIT_EXPLANATION_REASON_MEMORY_LINKED,
    SOCIAL_AUDIT_EXPLANATION_REASON_MISSING_RUNTIME_PROOF,
    SOCIAL_AUDIT_EXPLANATION_REASON_NATIVE_APP_MANUAL_REQUIRED,
    SOCIAL_AUDIT_EXPLANATION_REASON_PARENT_DECISION_LINKED,
    SOCIAL_AUDIT_EXPLANATION_REASON_POLICY_CANDIDATE_LINKED,
    SOCIAL_AUDIT_EXPLANATION_REF_APPROVAL_DECISION, SOCIAL_AUDIT_EXPLANATION_REF_APPROVAL_REQUEST,
    SOCIAL_AUDIT_EXPLANATION_REF_CONNECTOR_BOUNDARY, SOCIAL_AUDIT_EXPLANATION_REF_DECISION_MEMORY,
    SOCIAL_AUDIT_EXPLANATION_REF_MANUAL_GAP, SOCIAL_AUDIT_EXPLANATION_REF_NATIVE_CAPABILITY,
    SOCIAL_AUDIT_EXPLANATION_SCHEMA_VERSION, SOCIAL_AUDIT_EXPLANATION_SNAPSHOT_ID,
    SOCIAL_AUDIT_EXPLANATION_STATUS_CONTRACT_ONLY, SOCIAL_AUDIT_EXPLANATION_STATUS_MANUAL_REQUIRED,
    SOCIAL_AUDIT_EXPLANATION_STATUS_READY_FOR_PARENT,
    SOCIAL_AUDIT_EXPLANATION_SUBJECT_ACCOUNT_APPROVAL,
    SOCIAL_AUDIT_EXPLANATION_SUBJECT_CONNECTOR_BOUNDARY,
    SOCIAL_AUDIT_EXPLANATION_SUBJECT_DECISION_MEMORY,
    SOCIAL_AUDIT_EXPLANATION_SUBJECT_FEED_VIDEO_GATE,
    SOCIAL_AUDIT_EXPLANATION_SUBJECT_MANUAL_REQUIRED_GAP,
    SOCIAL_AUDIT_EXPLANATION_SUBJECT_NATIVE_APP_GAP,
};

use crate::{event_builder::build_event, fields::fields_from_pairs, time::timestamp_now};

type FieldPair = (&'static str, LogFieldValue);

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
    fields_from_pairs(read_model_pairs(read_model))
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

fn read_model_pairs(read_model: &SocialAuditExplanationSnapshot) -> Vec<FieldPair> {
    vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.captured_at.clone()),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.entries.len() as f64),
        ),
        (
            constants::field::BROWSER_SOCIAL_AUDIT_EXPLANATION_READ_MODEL,
            LogFieldValue::String(
                serde_json::to_string(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
    ]
}

fn account_approval_entry() -> SocialAuditExplanationEntry {
    entry(
        SOCIAL_AUDIT_EXPLANATION_SUBJECT_ACCOUNT_APPROVAL,
        SOCIAL_AUDIT_EXPLANATION_STATUS_READY_FOR_PARENT,
        SOCIAL_AUDIT_EXPLANATION_DECISION_PARENT_RECORDED,
        Some(SOCIAL_AUDIT_EXPLANATION_POLICY_VERSION),
        SOCIAL_AUDIT_EXPLANATION_ACTION_PARENT_REVIEW,
        vec![SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_PARENT_RULE_MATCH],
        vec![
            SOCIAL_AUDIT_EXPLANATION_REASON_EVIDENCE_LINKED,
            SOCIAL_AUDIT_EXPLANATION_REASON_POLICY_CANDIDATE_LINKED,
            SOCIAL_AUDIT_EXPLANATION_REASON_PARENT_DECISION_LINKED,
        ],
        vec![
            evidence_link(SOCIAL_AUDIT_EXPLANATION_EVIDENCE_POLICY_CANDIDATE),
            evidence_link(SOCIAL_AUDIT_EXPLANATION_EVIDENCE_PARENT_APPROVAL),
        ],
        OptionalRefs {
            parent_approval_request_ref: Some(SOCIAL_AUDIT_EXPLANATION_REF_APPROVAL_REQUEST),
            parent_approval_decision_ref: Some(SOCIAL_AUDIT_EXPLANATION_REF_APPROVAL_DECISION),
            ..OptionalRefs::default()
        },
    )
}

fn feed_video_entry() -> SocialAuditExplanationEntry {
    entry(
        SOCIAL_AUDIT_EXPLANATION_SUBJECT_FEED_VIDEO_GATE,
        SOCIAL_AUDIT_EXPLANATION_STATUS_READY_FOR_PARENT,
        SOCIAL_AUDIT_EXPLANATION_DECISION_CANDIDATE_ONLY,
        Some(SOCIAL_AUDIT_EXPLANATION_POLICY_VERSION),
        SOCIAL_AUDIT_EXPLANATION_ACTION_WARN,
        vec![
            SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_SOCIAL_RISK_HIGH,
            SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_VIDEO_SAFETY_RISK,
        ],
        vec![
            SOCIAL_AUDIT_EXPLANATION_REASON_EVIDENCE_LINKED,
            SOCIAL_AUDIT_EXPLANATION_REASON_POLICY_CANDIDATE_LINKED,
        ],
        vec![
            evidence_link(SOCIAL_AUDIT_EXPLANATION_EVIDENCE_ROUTE_EVIDENCE),
            evidence_link(SOCIAL_AUDIT_EXPLANATION_EVIDENCE_POLICY_CANDIDATE),
        ],
        OptionalRefs::default(),
    )
}

fn native_app_gap_entry() -> SocialAuditExplanationEntry {
    manual_entry(
        SOCIAL_AUDIT_EXPLANATION_SUBJECT_NATIVE_APP_GAP,
        SOCIAL_AUDIT_EXPLANATION_EVIDENCE_NATIVE_CAPABILITY,
        vec![
            SOCIAL_AUDIT_EXPLANATION_REASON_NATIVE_APP_MANUAL_REQUIRED,
            SOCIAL_AUDIT_EXPLANATION_REASON_MISSING_RUNTIME_PROOF,
        ],
        OptionalRefs {
            native_capability_ref: Some(SOCIAL_AUDIT_EXPLANATION_REF_NATIVE_CAPABILITY),
            ..OptionalRefs::default()
        },
    )
}

fn connector_boundary_entry() -> SocialAuditExplanationEntry {
    manual_entry(
        SOCIAL_AUDIT_EXPLANATION_SUBJECT_CONNECTOR_BOUNDARY,
        SOCIAL_AUDIT_EXPLANATION_EVIDENCE_CONNECTOR_BOUNDARY,
        vec![
            SOCIAL_AUDIT_EXPLANATION_REASON_CONNECTOR_BOUNDARY_LINKED,
            SOCIAL_AUDIT_EXPLANATION_REASON_MANUAL_REVIEW_REQUIRED,
        ],
        OptionalRefs {
            connector_boundary_ref: Some(SOCIAL_AUDIT_EXPLANATION_REF_CONNECTOR_BOUNDARY),
            ..OptionalRefs::default()
        },
    )
}

fn decision_memory_entry() -> SocialAuditExplanationEntry {
    entry(
        SOCIAL_AUDIT_EXPLANATION_SUBJECT_DECISION_MEMORY,
        SOCIAL_AUDIT_EXPLANATION_STATUS_CONTRACT_ONLY,
        SOCIAL_AUDIT_EXPLANATION_DECISION_CANDIDATE_ONLY,
        Some(SOCIAL_AUDIT_EXPLANATION_POLICY_VERSION),
        SOCIAL_AUDIT_EXPLANATION_ACTION_ALLOW,
        vec![SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_PARENT_RULE_MATCH],
        vec![
            SOCIAL_AUDIT_EXPLANATION_REASON_MEMORY_LINKED,
            SOCIAL_AUDIT_EXPLANATION_REASON_EVIDENCE_LINKED,
        ],
        vec![evidence_link(
            SOCIAL_AUDIT_EXPLANATION_EVIDENCE_DECISION_MEMORY,
        )],
        OptionalRefs {
            decision_memory_ref: Some(SOCIAL_AUDIT_EXPLANATION_REF_DECISION_MEMORY),
            ..OptionalRefs::default()
        },
    )
}

fn manual_gap_entry() -> SocialAuditExplanationEntry {
    manual_entry(
        SOCIAL_AUDIT_EXPLANATION_SUBJECT_MANUAL_REQUIRED_GAP,
        SOCIAL_AUDIT_EXPLANATION_EVIDENCE_MANUAL_GAP,
        vec![SOCIAL_AUDIT_EXPLANATION_REASON_MANUAL_REVIEW_REQUIRED],
        OptionalRefs {
            manual_required_ref: Some(SOCIAL_AUDIT_EXPLANATION_REF_MANUAL_GAP),
            ..OptionalRefs::default()
        },
    )
}

fn manual_entry(
    subject_kind: &'static str,
    evidence_kind: &'static str,
    explanation_reasons: Vec<&'static str>,
    refs: OptionalRefs,
) -> SocialAuditExplanationEntry {
    entry(
        subject_kind,
        SOCIAL_AUDIT_EXPLANATION_STATUS_MANUAL_REQUIRED,
        SOCIAL_AUDIT_EXPLANATION_DECISION_MANUAL_REQUIRED,
        None,
        SOCIAL_AUDIT_EXPLANATION_ACTION_MANUAL_REVIEW,
        vec![SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_MANUAL_REQUIRED],
        explanation_reasons,
        vec![evidence_link(evidence_kind)],
        refs,
    )
}

fn entry(
    subject_kind: &'static str,
    status: &'static str,
    decision_state: &'static str,
    policy_version_ref: Option<&'static str>,
    action_candidate: &'static str,
    policy_reason_codes: Vec<&'static str>,
    explanation_reasons: Vec<&'static str>,
    evidence_links: Vec<SocialAuditExplanationEvidenceLink>,
    refs: OptionalRefs,
) -> SocialAuditExplanationEntry {
    SocialAuditExplanationEntry {
        event_id: subject_kind.to_string(),
        subject_kind: subject_kind.to_string(),
        status: status.to_string(),
        decision_state: decision_state.to_string(),
        audience: SOCIAL_AUDIT_EXPLANATION_AUDIENCE_PARENT.to_string(),
        policy_version_ref: policy_version_ref.map(str::to_string),
        action_candidate: action_candidate.to_string(),
        policy_reason_codes: policy_reason_codes
            .into_iter()
            .map(str::to_string)
            .collect(),
        explanation_reasons: explanation_reasons
            .into_iter()
            .map(str::to_string)
            .collect(),
        evidence_links,
        audit_refs: vec![SOCIAL_AUDIT_EXPLANATION_AUDIT_REF.to_string()],
        parent_approval_request_ref: refs.parent_approval_request_ref.map(str::to_string),
        parent_approval_decision_ref: refs.parent_approval_decision_ref.map(str::to_string),
        decision_memory_ref: refs.decision_memory_ref.map(str::to_string),
        connector_boundary_ref: refs.connector_boundary_ref.map(str::to_string),
        native_capability_ref: refs.native_capability_ref.map(str::to_string),
        manual_required_ref: refs.manual_required_ref.map(str::to_string),
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

fn evidence_link(evidence_kind: &'static str) -> SocialAuditExplanationEvidenceLink {
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
