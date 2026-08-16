use ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateResponse;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;

use crate::{fields::fields_from_pairs, json_contract::serialize_json_string};

const BROWSER_POLICY_REJECTION_REASON_PROTOCOL_STRINGS: [&str; 11] = [
    constants::browser_policy::REJECTION_INVALID_REQUEST,
    constants::browser_policy::REJECTION_UNKNOWN_WRITES_TO,
    constants::browser_policy::REJECTION_UNKNOWN_FIELD,
    constants::browser_policy::REJECTION_INVALID_ENUM_VALUE,
    constants::browser_policy::REJECTION_MISSING_BUDGET_OR_FALLBACK,
    constants::browser_policy::REJECTION_MISSING_MANAGED_PROOF_OR_FALLBACK,
    constants::browser_policy::REJECTION_CAPABILITY_UNAVAILABLE,
    constants::browser_policy::REJECTION_STORAGE_UNAVAILABLE,
    constants::browser_policy::REJECTION_STALE_REVISION,
    constants::browser_policy::REJECTION_SCAFFOLD_UNAVAILABLE,
    constants::browser_policy::REJECTION_REVISION_NOT_FOUND,
];

pub(crate) fn browser_policy_response_payload(response: &BrowserPolicyUpdateResponse) -> LogFields {
    let mut fields = fields_from_pairs(vec![
        (
            constants::field::BROWSER_POLICY_RESPONSE,
            LogFieldValue::String(serialize_json_string(response).0),
        ),
        (
            constants::field::BROWSER_POLICY_UPDATE_KIND,
            LogFieldValue::String(response.kind.as_protocol_str().to_string()),
        ),
    ]);
    if let Some(reason) = response.rejection_reason {
        fields.insert(
            constants::field::BROWSER_POLICY_REJECTION_REASON.to_string(),
            LogFieldValue::String(
                BROWSER_POLICY_REJECTION_REASON_PROTOCOL_STRINGS[reason as usize].to_string(),
            ),
        );
    }
    if let Some(effective_policy) = &response.effective_policy {
        fields.insert(
            constants::field::BROWSER_POLICY_EFFECTIVE_POLICY.to_string(),
            LogFieldValue::String(serialize_json_string(effective_policy).0),
        );
    }
    if let Some(capability_registry) = &response.capability_registry {
        fields.insert(
            constants::field::BROWSER_POLICY_CAPABILITY_REGISTRY.to_string(),
            LogFieldValue::String(serialize_json_string(capability_registry).0),
        );
    }
    fields
}
