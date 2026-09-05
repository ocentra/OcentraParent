use ocentra_parent_agent_protocol::BrowserPolicyValue;

use crate::browser_policy_runtime_support::BrowserPolicyPolicyId;

pub fn default_browser_policy_id_for_test() -> BrowserPolicyPolicyId {
    BrowserPolicyPolicyId(
        ocentra_parent_agent_protocol::constants::browser_policy::POLICY_ID.to_string(),
    )
}

pub fn default_browser_policy_for_test(policy_id: BrowserPolicyPolicyId) -> BrowserPolicyValue {
    crate::browser_policy_runtime_support::default_policy(policy_id)
}
