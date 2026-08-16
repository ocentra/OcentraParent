use ocentra_network_evidence::{
    dns::types::NetworkEvidenceGrade,
    policy::{
        map_network_evidence_grade_to_policy, NetworkEvidencePolicyAction,
        NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingError,
        NetworkEvidencePolicyMappingInput, NetworkEvidencePolicyMode,
    },
};
use ocentra_parent_agent_protocol::activity::policy::{PolicyAction, PolicyDecision};
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyPreviewNetworkEvidenceMapping;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::policy_constants as policy;

use crate::activity_store_policy_preview_fields::string_field;
use crate::activity_store_policy_preview_rows::PolicyPreviewStoreRow;

const EVIDENCE_GRADE_TABLE: [NetworkEvidenceGrade; 8] = [
    NetworkEvidenceGrade::D,
    NetworkEvidenceGrade::D,
    NetworkEvidenceGrade::D,
    NetworkEvidenceGrade::D,
    NetworkEvidenceGrade::D,
    NetworkEvidenceGrade::D,
    NetworkEvidenceGrade::C,
    NetworkEvidenceGrade::B,
];

const EVIDENCE_GRADE_PROTOCOL_RULES: &[(NetworkEvidenceGrade, &str)] = &[
    (NetworkEvidenceGrade::A, policy::NETWORK_EVIDENCE_GRADE_A),
    (NetworkEvidenceGrade::B, policy::NETWORK_EVIDENCE_GRADE_B),
    (NetworkEvidenceGrade::C, policy::NETWORK_EVIDENCE_GRADE_C),
    (NetworkEvidenceGrade::D, policy::NETWORK_EVIDENCE_GRADE_D),
];

const POLICY_ACTION_RULES: &[(NetworkEvidencePolicyAction, PolicyAction)] = &[
    (NetworkEvidencePolicyAction::None, PolicyAction::Unknown),
    (
        NetworkEvidencePolicyAction::AskParent,
        PolicyAction::AskParent,
    ),
    (NetworkEvidencePolicyAction::WarnChild, PolicyAction::Warn),
    (NetworkEvidencePolicyAction::Monitor, PolicyAction::Unknown),
    (NetworkEvidencePolicyAction::Limit, PolicyAction::TimeLimit),
    (NetworkEvidencePolicyAction::Block, PolicyAction::Block),
];

const POLICY_ACTION_PROTOCOL_RULES: &[(NetworkEvidencePolicyAction, &str)] = &[
    (
        NetworkEvidencePolicyAction::AskParent,
        policy::ACTION_ASK_PARENT,
    ),
    (NetworkEvidencePolicyAction::WarnChild, policy::ACTION_WARN),
    (
        NetworkEvidencePolicyAction::Monitor,
        policy::NETWORK_POLICY_ACTION_MONITOR,
    ),
    (
        NetworkEvidencePolicyAction::Limit,
        policy::ACTION_TIME_LIMIT,
    ),
    (NetworkEvidencePolicyAction::Block, policy::ACTION_BLOCK),
    (
        NetworkEvidencePolicyAction::None,
        policy::NETWORK_POLICY_ACTION_NONE,
    ),
];

const POLICY_TO_NETWORK_ACTION_RULES: &[(PolicyAction, NetworkEvidencePolicyAction)] = &[
    (PolicyAction::Warn, NetworkEvidencePolicyAction::WarnChild),
    (PolicyAction::Block, NetworkEvidencePolicyAction::Block),
    (PolicyAction::TimeLimit, NetworkEvidencePolicyAction::Limit),
    (
        PolicyAction::AskParent,
        NetworkEvidencePolicyAction::AskParent,
    ),
];

const POLICY_MODE_PROTOCOL_RULES: &[(NetworkEvidencePolicyMode, &str)] = &[
    (
        NetworkEvidencePolicyMode::ObserveOnly,
        policy::NETWORK_POLICY_MAPPING_MODE_OBSERVE_ONLY,
    ),
    (
        NetworkEvidencePolicyMode::DryRun,
        policy::NETWORK_POLICY_MAPPING_MODE_DRY_RUN,
    ),
    (
        NetworkEvidencePolicyMode::ParentReview,
        policy::NETWORK_POLICY_MAPPING_MODE_PARENT_REVIEW,
    ),
];

const MAPPING_REASON_RULES: &[(NetworkEvidencePolicyMode, &str)] = &[
    (
        NetworkEvidencePolicyMode::ParentReview,
        policy::REASON_NETWORK_EVIDENCE_GRADE_PARENT_REVIEW,
    ),
    (
        NetworkEvidencePolicyMode::ObserveOnly,
        policy::REASON_NETWORK_EVIDENCE_GRADE_OBSERVE_ONLY,
    ),
    (
        NetworkEvidencePolicyMode::DryRun,
        policy::REASON_NETWORK_EVIDENCE_GRADE_PARENT_REVIEW,
    ),
];

pub(crate) fn grade_mapped_network_decision(
    row: &PolicyPreviewStoreRow,
    mut decision: PolicyDecision,
) -> (PolicyDecision, Option<PolicyPreviewNetworkEvidenceMapping>) {
    let Some(evidence_grade) = network_evidence_grade(row) else {
        return (decision, None);
    };
    let Some(requested_action) = network_policy_action(decision.action) else {
        return (decision, None);
    };
    let mapping = match network_policy_mapping(evidence_grade, requested_action, &decision) {
        Ok(mapping) => mapping,
        Err(_) => return parent_review_mapping(&mut decision, evidence_grade, requested_action),
    };
    let mapped_action = policy_action(mapping.mapped_action);
    if mapped_action != decision.action {
        decision.action = mapped_action;
        push_unique_reason(&mut decision, grade_mapping_reason(mapping.mode));
    }
    (decision, Some(preview_network_evidence_mapping(&mapping)))
}

fn parent_review_mapping(
    decision: &mut PolicyDecision,
    evidence_grade: NetworkEvidenceGrade,
    requested_action: NetworkEvidencePolicyAction,
) -> (PolicyDecision, Option<PolicyPreviewNetworkEvidenceMapping>) {
    decision.action = PolicyAction::AskParent;
    push_unique_reason(
        decision,
        policy::REASON_NETWORK_EVIDENCE_GRADE_PARENT_REVIEW.to_string(),
    );
    (
        decision.clone(),
        Some(PolicyPreviewNetworkEvidenceMapping {
            evidence_grade: network_evidence_grade_protocol(evidence_grade).to_string(),
            requested_action: network_policy_action_protocol(requested_action).to_string(),
            mapped_action: policy::ACTION_ASK_PARENT.to_string(),
            mode: policy::NETWORK_POLICY_MAPPING_MODE_PARENT_REVIEW.to_string(),
            adapter_action_authorized: false,
            enforcement_command_authorized: false,
        }),
    )
}

fn network_evidence_grade(row: &PolicyPreviewStoreRow) -> Option<NetworkEvidenceGrade> {
    if row.kind != constants::activity_event_kind::DOMAIN_OBSERVED {
        return None;
    }
    let capability_status_available = field_equals(
        row,
        constants::field::CAPABILITY_STATUS,
        constants::activity_capture::CAPABILITY_STATUS_AVAILABLE,
    );
    let domain_observed = field_equals(
        row,
        constants::field::DOMAIN_ATTRIBUTION_STATUS,
        constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_DOMAIN_OBSERVED,
    );
    let process_attributed = field_equals(
        row,
        constants::field::PROCESS_ATTRIBUTION_STATUS,
        constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_ATTRIBUTED,
    );
    Some(
        EVIDENCE_GRADE_TABLE[((capability_status_available as usize) << 2)
            | ((domain_observed as usize) << 1)
            | (process_attributed as usize)],
    )
}

fn field_equals(row: &PolicyPreviewStoreRow, key: &str, expected: &str) -> bool {
    string_field(&row.fields, key).as_deref() == Some(expected)
}

fn network_policy_mapping(
    evidence_grade: NetworkEvidenceGrade,
    requested_action: NetworkEvidencePolicyAction,
    decision: &PolicyDecision,
) -> Result<NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingError> {
    let parent_rule_ref = decision.rule_ids.first().cloned().unwrap_or_default();
    let evidence_refs = decision
        .evidence_references
        .iter()
        .map(|reference| reference.evidence_reference_id.clone())
        .collect::<Vec<_>>();
    map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
        policy_decision_ref: decision.decision_id.clone(),
        parent_rule_ref,
        evidence_refs,
        local_ai_result_ref: decision.local_ai_result_id.clone(),
        evidence_grade,
        requested_action,
        adapter_capability_proof_ref: None,
    })
}

fn preview_network_evidence_mapping(
    mapping: &NetworkEvidencePolicyMapping,
) -> PolicyPreviewNetworkEvidenceMapping {
    PolicyPreviewNetworkEvidenceMapping {
        evidence_grade: network_evidence_grade_protocol(mapping.evidence_grade).to_string(),
        requested_action: network_policy_action_protocol(mapping.requested_action).to_string(),
        mapped_action: network_policy_action_protocol(mapping.mapped_action).to_string(),
        mode: network_policy_mode_protocol(mapping.mode).to_string(),
        adapter_action_authorized: mapping.adapter_action_authorized,
        enforcement_command_authorized: mapping.enforcement_command_authorized,
    }
}

fn network_evidence_grade_protocol(grade: NetworkEvidenceGrade) -> &'static str {
    EVIDENCE_GRADE_PROTOCOL_RULES
        .iter()
        .find_map(|(candidate, protocol)| (*candidate == grade).then_some(*protocol))
        .unwrap_or(policy::NETWORK_EVIDENCE_GRADE_D)
}

fn network_policy_action_protocol(action: NetworkEvidencePolicyAction) -> &'static str {
    POLICY_ACTION_PROTOCOL_RULES
        .iter()
        .find_map(|(candidate, protocol)| (*candidate == action).then_some(*protocol))
        .unwrap_or(policy::NETWORK_POLICY_ACTION_NONE)
}

fn network_policy_mode_protocol(mode: NetworkEvidencePolicyMode) -> &'static str {
    POLICY_MODE_PROTOCOL_RULES
        .iter()
        .find_map(|(candidate, protocol)| (*candidate == mode).then_some(*protocol))
        .unwrap_or(policy::NETWORK_POLICY_MAPPING_MODE_PARENT_REVIEW)
}

fn network_policy_action(action: PolicyAction) -> Option<NetworkEvidencePolicyAction> {
    POLICY_TO_NETWORK_ACTION_RULES
        .iter()
        .find_map(|(candidate, mapped)| (*candidate == action).then_some(*mapped))
}

fn policy_action(action: NetworkEvidencePolicyAction) -> PolicyAction {
    POLICY_ACTION_RULES
        .iter()
        .find_map(|(candidate, mapped)| (*candidate == action).then_some(*mapped))
        .unwrap_or(PolicyAction::Unknown)
}

fn grade_mapping_reason(mode: NetworkEvidencePolicyMode) -> String {
    MAPPING_REASON_RULES
        .iter()
        .find_map(|(candidate, reason)| (*candidate == mode).then_some(*reason))
        .unwrap_or(policy::REASON_NETWORK_EVIDENCE_GRADE_PARENT_REVIEW)
        .to_string()
}

fn push_unique_reason(decision: &mut PolicyDecision, reason_code: String) {
    if !decision
        .reason_codes
        .iter()
        .any(|existing| existing == &reason_code)
    {
        decision.reason_codes.push(reason_code);
    }
}
