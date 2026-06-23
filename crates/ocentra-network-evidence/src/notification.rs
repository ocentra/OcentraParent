use serde::{Deserialize, Serialize};

use crate::{
    NetworkEvidenceGrade, NetworkEvidencePolicyAction, NetworkEvidencePolicyMapping,
    NetworkEvidencePolicyMode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkParentNotificationSeverity {
    Info,
    Review,
    Warning,
    Urgent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkParentNotificationDeliveryState {
    CandidateOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkParentNotificationCandidateInput {
    pub notification_candidate_ref: String,
    pub mapping: NetworkEvidencePolicyMapping,
    pub provider_delivery_available: bool,
    pub sensitive_payload_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkParentNotificationCandidate {
    pub notification_candidate_ref: String,
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub evidence_refs: Vec<String>,
    pub local_ai_result_ref: Option<String>,
    pub evidence_grade: NetworkEvidenceGrade,
    pub policy_mode: NetworkEvidencePolicyMode,
    pub policy_action: NetworkEvidencePolicyAction,
    pub severity: NetworkParentNotificationSeverity,
    pub delivery_state: NetworkParentNotificationDeliveryState,
    pub provider_delivery_authorized: bool,
    pub sensitive_payload_available: bool,
    pub adapter_action_authorized: bool,
    pub enforcement_command_authorized: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkParentNotificationCandidateError {
    EmptyNotificationCandidateRef,
    EmptyPolicyDecisionRef,
    EmptyParentRuleRef,
    EmptyEvidenceRef,
    EmptyLocalAiResultRef,
    ProviderDeliveryClaimRejected,
    SensitivePayloadRejected,
    AdapterAuthorityRejected,
    EnforcementCommandRejected,
}

pub fn map_network_parent_notification_candidate(
    input: &NetworkParentNotificationCandidateInput,
) -> Result<NetworkParentNotificationCandidate, NetworkParentNotificationCandidateError> {
    if input.provider_delivery_available {
        return Err(NetworkParentNotificationCandidateError::ProviderDeliveryClaimRejected);
    }
    if input.sensitive_payload_available {
        return Err(NetworkParentNotificationCandidateError::SensitivePayloadRejected);
    }
    if input.mapping.adapter_action_authorized {
        return Err(NetworkParentNotificationCandidateError::AdapterAuthorityRejected);
    }
    if input.mapping.enforcement_command_authorized {
        return Err(NetworkParentNotificationCandidateError::EnforcementCommandRejected);
    }

    let notification_candidate_ref = normalize_ref(&input.notification_candidate_ref)
        .ok_or(NetworkParentNotificationCandidateError::EmptyNotificationCandidateRef)?;
    let policy_decision_ref = normalize_ref(&input.mapping.policy_decision_ref)
        .ok_or(NetworkParentNotificationCandidateError::EmptyPolicyDecisionRef)?;
    let parent_rule_ref = normalize_ref(&input.mapping.parent_rule_ref)
        .ok_or(NetworkParentNotificationCandidateError::EmptyParentRuleRef)?;
    let evidence_refs = normalized_evidence_refs(&input.mapping.evidence_refs)?;
    let local_ai_result_ref =
        normalized_optional_ref(input.mapping.local_ai_result_ref.as_deref())?;

    Ok(NetworkParentNotificationCandidate {
        notification_candidate_ref,
        policy_decision_ref,
        parent_rule_ref,
        evidence_refs,
        local_ai_result_ref,
        evidence_grade: input.mapping.evidence_grade,
        policy_mode: input.mapping.mode,
        policy_action: input.mapping.mapped_action,
        severity: severity_for(&input.mapping),
        delivery_state: NetworkParentNotificationDeliveryState::CandidateOnly,
        provider_delivery_authorized: false,
        sensitive_payload_available: false,
        adapter_action_authorized: false,
        enforcement_command_authorized: false,
    })
}

fn severity_for(mapping: &NetworkEvidencePolicyMapping) -> NetworkParentNotificationSeverity {
    match mapping.mode {
        NetworkEvidencePolicyMode::ObserveOnly => NetworkParentNotificationSeverity::Info,
        NetworkEvidencePolicyMode::ParentReview => NetworkParentNotificationSeverity::Review,
        NetworkEvidencePolicyMode::DryRun => match mapping.mapped_action {
            NetworkEvidencePolicyAction::Block | NetworkEvidencePolicyAction::Limit => {
                NetworkParentNotificationSeverity::Urgent
            }
            NetworkEvidencePolicyAction::WarnChild | NetworkEvidencePolicyAction::Monitor => {
                NetworkParentNotificationSeverity::Warning
            }
            NetworkEvidencePolicyAction::AskParent => NetworkParentNotificationSeverity::Review,
            NetworkEvidencePolicyAction::None => NetworkParentNotificationSeverity::Info,
        },
    }
}

fn normalized_evidence_refs(
    refs: &[String],
) -> Result<Vec<String>, NetworkParentNotificationCandidateError> {
    let mut normalized = Vec::new();
    for value in refs {
        let Some(ref_value) = normalize_ref(value) else {
            return Err(NetworkParentNotificationCandidateError::EmptyEvidenceRef);
        };
        if !normalized.contains(&ref_value) {
            normalized.push(ref_value);
        }
    }
    if normalized.is_empty() {
        return Err(NetworkParentNotificationCandidateError::EmptyEvidenceRef);
    }
    Ok(normalized)
}

fn normalized_optional_ref(
    value: Option<&str>,
) -> Result<Option<String>, NetworkParentNotificationCandidateError> {
    match value {
        Some(raw) => normalize_ref(raw)
            .map(Some)
            .ok_or(NetworkParentNotificationCandidateError::EmptyLocalAiResultRef),
        None => Ok(None),
    }
}

fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}
