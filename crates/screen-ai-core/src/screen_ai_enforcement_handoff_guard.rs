use serde::{Deserialize, Serialize};

pub const SCREEN_AI_ENFORCEMENT_HANDOFF_ACCEPTED_EVENT_TYPE: &str =
    "screen.enforcement.handoff.guard.accepted";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenAiEnforcementHandoffConfidenceState {
    High,
    Medium,
    Low,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ScreenAiEnforcementHandoffMode {
    DryRun,
    ManualRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PolicyAction {
    TimeLimit,
    Block,
    Warn,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PolicyTargetType {
    Category,
    App,
    Url,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EnforcementHandoffState {
    Disabled,
    HandedOff,
    ManualRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvidenceReferenceKind {
    QueryStoreSummary,
    LocalAiResult,
    JournalEvent,
    ActivityEvent,
    PolicyDecision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentEvidenceReference {
    pub evidence_reference_id: String,
    pub kind: EvidenceReferenceKind,
    pub observed_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyDecision {
    pub schema_version: String,
    pub decision_id: String,
    pub action: PolicyAction,
    pub reason_codes: Vec<String>,
    pub evidence_references: Vec<ParentEvidenceReference>,
    pub rule_ids: Vec<String>,
    pub local_ai_result_id: Option<String>,
    pub dry_run: bool,
    pub enforcement_handoff_state: EnforcementHandoffState,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyTarget {
    pub target_id: String,
    pub target_type: PolicyTargetType,
    pub target_value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyRuleActor {
    pub actor_id: String,
    pub role: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyRule {
    pub rule_id: String,
    pub target: PolicyTarget,
    pub action: PolicyAction,
    pub schedule_id: Option<String>,
    pub priority: i32,
    pub reason_code: String,
    pub created_by: PolicyRuleActor,
    pub enabled: bool,
    pub effective_from: Option<String>,
    pub effective_until: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenAiEnforcementHandoffAuditEvent {
    pub audit_event_id: String,
    pub event_type: String,
    pub emitted_at: String,
    pub evidence_reference: ParentEvidenceReference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenAiEnforcementHandoffInputMaterial {
    pub summary_reference: ParentEvidenceReference,
    pub local_ai_result_reference: ParentEvidenceReference,
    pub audit_reference: ParentEvidenceReference,
    pub raw_pixels_included: bool,
    pub raw_model_text_included: bool,
    pub raw_screenshot_retained: bool,
    pub local_ai_authority_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenAiEnforcementHandoffGuardInput {
    pub schema_version: String,
    pub payload_id: String,
    pub generated_at: String,
    pub source_policy_decision: PolicyDecision,
    pub parent_policy_rule: PolicyRule,
    pub requested_action: PolicyAction,
    pub confidence_state: ScreenAiEnforcementHandoffConfidenceState,
    pub handoff_mode: ScreenAiEnforcementHandoffMode,
    pub input_material: ScreenAiEnforcementHandoffInputMaterial,
    pub audit_event: ScreenAiEnforcementHandoffAuditEvent,
    pub claim_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenAiEnforcementHandoffGuardPayload {
    pub schema_version: String,
    pub payload_id: String,
    pub generated_at: String,
    pub source_policy_decision: PolicyDecision,
    pub parent_policy_rule: PolicyRule,
    pub requested_action: PolicyAction,
    pub confidence_state: ScreenAiEnforcementHandoffConfidenceState,
    pub handoff_mode: ScreenAiEnforcementHandoffMode,
    pub summary_reference: ParentEvidenceReference,
    pub local_ai_result_reference: ParentEvidenceReference,
    pub audit_reference: ParentEvidenceReference,
    pub audit_event: ScreenAiEnforcementHandoffAuditEvent,
    pub raw_pixels_included: bool,
    pub raw_model_text_included: bool,
    pub raw_screenshot_retained: bool,
    pub local_ai_authority_claimed: bool,
    pub claim_boundary: String,
}

pub fn screen_ai_policy_decision_is_guarded(input: &ScreenAiEnforcementHandoffGuardInput) -> bool {
    input.source_policy_decision.dry_run
        && input.source_policy_decision.enforcement_handoff_state
            != EnforcementHandoffState::HandedOff
        && input.source_policy_decision.local_ai_result_id.is_some()
        && input
            .source_policy_decision
            .rule_ids
            .contains(&input.parent_policy_rule.rule_id)
        && input.parent_policy_rule.enabled
        && input.requested_action == input.source_policy_decision.action
}

pub fn screen_ai_handoff_material_matches_decision(
    input: &ScreenAiEnforcementHandoffGuardInput,
) -> bool {
    evidence_includes(
        &input.source_policy_decision.evidence_references,
        &input.input_material.summary_reference,
    ) && evidence_includes(
        &input.source_policy_decision.evidence_references,
        &input.input_material.local_ai_result_reference,
    ) && evidence_includes(
        &input.source_policy_decision.evidence_references,
        &input.input_material.audit_reference,
    ) && input.input_material.summary_reference.kind == EvidenceReferenceKind::QueryStoreSummary
        && input.input_material.local_ai_result_reference.kind
            == EvidenceReferenceKind::LocalAiResult
        && input.input_material.audit_reference.kind == EvidenceReferenceKind::JournalEvent
        && !input.input_material.raw_pixels_included
        && !input.input_material.raw_model_text_included
        && !input.input_material.raw_screenshot_retained
        && !input.input_material.local_ai_authority_claimed
}

pub fn screen_ai_enforcement_handoff_input_is_ready(
    input: &ScreenAiEnforcementHandoffGuardInput,
) -> bool {
    screen_ai_policy_decision_is_guarded(input)
        && screen_ai_handoff_material_matches_decision(input)
        && input.audit_event.evidence_reference.evidence_reference_id
            == input.input_material.audit_reference.evidence_reference_id
        && input.audit_event.evidence_reference.kind == EvidenceReferenceKind::JournalEvent
        && input.audit_event.event_type == SCREEN_AI_ENFORCEMENT_HANDOFF_ACCEPTED_EVENT_TYPE
}

pub fn screen_ai_enforcement_handoff_payload_is_honest(
    payload: &ScreenAiEnforcementHandoffGuardPayload,
) -> bool {
    let input = ScreenAiEnforcementHandoffGuardInput {
        schema_version: payload.schema_version.clone(),
        payload_id: payload.payload_id.clone(),
        generated_at: payload.generated_at.clone(),
        source_policy_decision: payload.source_policy_decision.clone(),
        parent_policy_rule: payload.parent_policy_rule.clone(),
        requested_action: payload.requested_action.clone(),
        confidence_state: payload.confidence_state.clone(),
        handoff_mode: payload.handoff_mode.clone(),
        input_material: ScreenAiEnforcementHandoffInputMaterial {
            summary_reference: payload.summary_reference.clone(),
            local_ai_result_reference: payload.local_ai_result_reference.clone(),
            audit_reference: payload.audit_reference.clone(),
            raw_pixels_included: payload.raw_pixels_included,
            raw_model_text_included: payload.raw_model_text_included,
            raw_screenshot_retained: payload.raw_screenshot_retained,
            local_ai_authority_claimed: payload.local_ai_authority_claimed,
        },
        audit_event: payload.audit_event.clone(),
        claim_boundary: payload.claim_boundary.clone(),
    };

    screen_ai_policy_decision_is_guarded(&input)
        && !payload.raw_pixels_included
        && !payload.raw_model_text_included
        && !payload.raw_screenshot_retained
        && !payload.local_ai_authority_claimed
        && payload.audit_event.evidence_reference.evidence_reference_id
            == payload.audit_reference.evidence_reference_id
        && payload.audit_event.evidence_reference.kind == EvidenceReferenceKind::JournalEvent
        && payload.audit_event.event_type == SCREEN_AI_ENFORCEMENT_HANDOFF_ACCEPTED_EVENT_TYPE
}

pub fn build_screen_ai_enforcement_handoff_guard_payload(
    input: &ScreenAiEnforcementHandoffGuardInput,
) -> ScreenAiEnforcementHandoffGuardPayload {
    ScreenAiEnforcementHandoffGuardPayload {
        schema_version: input.schema_version.clone(),
        payload_id: input.payload_id.clone(),
        generated_at: input.generated_at.clone(),
        source_policy_decision: input.source_policy_decision.clone(),
        parent_policy_rule: input.parent_policy_rule.clone(),
        requested_action: input.requested_action.clone(),
        confidence_state: input.confidence_state.clone(),
        handoff_mode: input.handoff_mode.clone(),
        summary_reference: input.input_material.summary_reference.clone(),
        local_ai_result_reference: input.input_material.local_ai_result_reference.clone(),
        audit_reference: input.input_material.audit_reference.clone(),
        audit_event: input.audit_event.clone(),
        raw_pixels_included: false,
        raw_model_text_included: false,
        raw_screenshot_retained: false,
        local_ai_authority_claimed: false,
        claim_boundary: input.claim_boundary.clone(),
    }
}

fn evidence_includes(
    evidence_references: &[ParentEvidenceReference],
    expected_reference: &ParentEvidenceReference,
) -> bool {
    evidence_references.iter().any(|reference| {
        reference.evidence_reference_id == expected_reference.evidence_reference_id
            && reference.kind == expected_reference.kind
    })
}

pub fn screen_ai_enforcement_handoff_guard_generated_typescript() -> String {
    include_str!("screen_ai_enforcement_handoff_guard.ts.txt").to_string()
}
