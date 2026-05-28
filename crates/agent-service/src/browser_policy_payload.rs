use ocentra_parent_agent_protocol::{
    constants, policy_constants as policy, BrowserPolicyRejectionReason, BrowserPolicyUpdateKind,
    BrowserPolicyUpdateResponse, BrowserPolicyUpdateStatus, LogFieldValue, LogFields,
};

use crate::fields::fields_from_pairs;

pub fn browser_policy_scaffold_payload(
    request_id: String,
    kind: BrowserPolicyUpdateKind,
) -> LogFields {
    let response = BrowserPolicyUpdateResponse {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        request_id,
        kind,
        status: BrowserPolicyUpdateStatus::Rejected,
        policy: None,
        effective_policy: None,
        capability_registry: None,
        rejection_reason: Some(BrowserPolicyRejectionReason::ScaffoldUnavailable),
        audit_event_id: None,
        message: Some(constants::browser_policy::SCAFFOLD_UNAVAILABLE_MESSAGE.to_string()),
    };
    fields_from_pairs(vec![
        (
            constants::field::BROWSER_POLICY_RESPONSE,
            LogFieldValue::String(
                serde_json::to_string(&response).expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
        (
            constants::field::BROWSER_POLICY_UPDATE_KIND,
            LogFieldValue::String(response.kind.as_protocol_str().to_string()),
        ),
        (
            constants::field::BROWSER_POLICY_REJECTION_REASON,
            LogFieldValue::String(
                constants::browser_policy::REJECTION_SCAFFOLD_UNAVAILABLE.to_string(),
            ),
        ),
    ])
}
