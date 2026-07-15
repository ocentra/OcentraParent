#![forbid(unsafe_code)]

use super::{
    PolicyContractApprovalRequest, PolicyContractOverrideGrant, PolicyContractValidationResult,
};

mod state_rules;
mod type_rules;

pub(crate) fn validate_policy_override_grant(
    grant: &PolicyContractOverrideGrant,
    approval: &PolicyContractApprovalRequest,
    evaluated_at: &str,
) -> PolicyContractValidationResult {
    type_rules::validate_policy_override_grant_type_rules(grant, approval)?;
    state_rules::validate_policy_override_grant_state_rules(grant, evaluated_at)
}
