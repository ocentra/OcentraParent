use ocentra_parent_agent_protocol::{
    constants, BrowserPolicyRejectionReason, BrowserPolicyUpdateResponse, LogFieldValue, LogFields,
};

use crate::fields::fields_from_pairs;

pub(crate) fn browser_policy_response_payload(response: &BrowserPolicyUpdateResponse) -> LogFields {
    let mut fields = fields_from_pairs(vec![
        (
            constants::field::BROWSER_POLICY_RESPONSE,
            LogFieldValue::String(
                serde_json::to_string(response).expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
        (
            constants::field::BROWSER_POLICY_UPDATE_KIND,
            LogFieldValue::String(response.kind.as_protocol_str().to_string()),
        ),
    ]);
    if let Some(reason) = response.rejection_reason {
        fields.insert(
            constants::field::BROWSER_POLICY_REJECTION_REASON.to_string(),
            LogFieldValue::String(rejection_reason_protocol_str(reason).to_string()),
        );
    }
    if let Some(effective_policy) = &response.effective_policy {
        fields.insert(
            constants::field::BROWSER_POLICY_EFFECTIVE_POLICY.to_string(),
            LogFieldValue::String(
                serde_json::to_string(effective_policy)
                    .expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        );
    }
    if let Some(capability_registry) = &response.capability_registry {
        fields.insert(
            constants::field::BROWSER_POLICY_CAPABILITY_REGISTRY.to_string(),
            LogFieldValue::String(
                serde_json::to_string(capability_registry)
                    .expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        );
    }
    fields
}

fn rejection_reason_protocol_str(reason: BrowserPolicyRejectionReason) -> &'static str {
    match reason {
        BrowserPolicyRejectionReason::InvalidRequest => {
            constants::browser_policy::REJECTION_INVALID_REQUEST
        }
        BrowserPolicyRejectionReason::StorageUnavailable => {
            constants::browser_policy::REJECTION_STORAGE_UNAVAILABLE
        }
        BrowserPolicyRejectionReason::StaleRevision => {
            constants::browser_policy::REJECTION_STALE_REVISION
        }
        BrowserPolicyRejectionReason::ScaffoldUnavailable => {
            constants::browser_policy::REJECTION_SCAFFOLD_UNAVAILABLE
        }
        BrowserPolicyRejectionReason::RevisionNotFound => {
            constants::browser_policy::REJECTION_REVISION_NOT_FOUND
        }
        BrowserPolicyRejectionReason::UnknownWritesTo => {
            constants::browser_policy::REJECTION_UNKNOWN_WRITES_TO
        }
        BrowserPolicyRejectionReason::UnknownField => {
            constants::browser_policy::REJECTION_UNKNOWN_FIELD
        }
        BrowserPolicyRejectionReason::InvalidEnumValue => {
            constants::browser_policy::REJECTION_INVALID_ENUM_VALUE
        }
        BrowserPolicyRejectionReason::MissingBudgetOrFallback => {
            constants::browser_policy::REJECTION_MISSING_BUDGET_OR_FALLBACK
        }
        BrowserPolicyRejectionReason::MissingManagedProofOrFallback => {
            constants::browser_policy::REJECTION_MISSING_MANAGED_PROOF_OR_FALLBACK
        }
        BrowserPolicyRejectionReason::CapabilityUnavailable => {
            constants::browser_policy::REJECTION_CAPABILITY_UNAVAILABLE
        }
    }
}
