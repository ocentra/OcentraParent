use ocentra_parent_agent_protocol::enforcement_policy_dispatch::{
    EnforcementPolicyDispatchReadModel, EnforcementPolicyDispatchRejectionReason,
};

#[path = "enforcement_policy_dispatch_counts.rs"]
mod enforcement_policy_dispatch_counts;
#[path = "enforcement_policy_dispatch_identity.rs"]
mod enforcement_policy_dispatch_identity;
#[path = "enforcement_policy_dispatch_matrix.rs"]
mod enforcement_policy_dispatch_matrix;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnforcementPolicyDispatchValidation {
    pub dispatch_ready_count: usize,
    pub dry_run_only_count: usize,
    pub rejected_count: usize,
    pub manual_required_count: usize,
    pub report_only_count: usize,
    pub recovery_needed_count: usize,
}

pub fn validate_enforcement_policy_dispatch_read_model(
    read_model: &EnforcementPolicyDispatchReadModel,
) -> Result<EnforcementPolicyDispatchValidation, EnforcementPolicyDispatchRejectionReason> {
    let mut validation = EnforcementPolicyDispatchValidation {
        dispatch_ready_count: 0,
        dry_run_only_count: 0,
        rejected_count: 0,
        manual_required_count: 0,
        report_only_count: 0,
        recovery_needed_count: 0,
    };

    for entry in &read_model.entries {
        enforcement_policy_dispatch_identity::validate_entry_identity(entry)?;
        enforcement_policy_dispatch_matrix::validate_entry_matrix(entry)?;
        enforcement_policy_dispatch_counts::update_validation_counts(&mut validation, entry);
    }

    Ok(validation)
}
