use serde::{Deserialize, Serialize};

use crate::dns::types::NetworkEvidenceGrade;
use crate::policy::{
    map_network_evidence_grade_to_policy, NetworkEvidencePolicyAction,
    NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingError,
};

/// The bounded metadata a network parser may hand to the policy mapper.
///
/// Parser output is evidence only. Exact URLs and decrypted payloads are not
/// valid network-parser claims and must be rejected before policy mapping.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkParserEvidence {
    pub evidence_ref: String,
    pub evidence_grade: NetworkEvidenceGrade,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkParserPolicyHandoffInput {
    pub parser_evidence: NetworkParserEvidence,
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub local_ai_result_ref: Option<String>,
    pub requested_action: NetworkEvidencePolicyAction,
    pub adapter_capability_proof_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkParserPolicyHandoffError {
    EmptyParserEvidenceRef,
    ExactUrlClaimRejected,
    DecryptedPayloadClaimRejected,
    PolicyMapping(NetworkEvidencePolicyMappingError),
}

pub fn map_network_parser_evidence_to_policy(
    input: NetworkParserPolicyHandoffInput,
) -> Result<NetworkEvidencePolicyMapping, NetworkParserPolicyHandoffError> {
    let NetworkParserPolicyHandoffInput {
        parser_evidence,
        policy_decision_ref,
        parent_rule_ref,
        local_ai_result_ref,
        requested_action,
        adapter_capability_proof_ref,
    } = input;

    if parser_evidence.evidence_ref.trim().is_empty() {
        return Err(NetworkParserPolicyHandoffError::EmptyParserEvidenceRef);
    }
    if parser_evidence.exact_url_available {
        return Err(NetworkParserPolicyHandoffError::ExactUrlClaimRejected);
    }
    if parser_evidence.decrypted_payload_available {
        return Err(NetworkParserPolicyHandoffError::DecryptedPayloadClaimRejected);
    }

    map_network_evidence_grade_to_policy(crate::policy::NetworkEvidencePolicyMappingInput {
        policy_decision_ref,
        parent_rule_ref,
        evidence_refs: vec![parser_evidence.evidence_ref],
        local_ai_result_ref,
        evidence_grade: parser_evidence.evidence_grade,
        requested_action,
        adapter_capability_proof_ref,
    })
    .map_err(NetworkParserPolicyHandoffError::PolicyMapping)
}
