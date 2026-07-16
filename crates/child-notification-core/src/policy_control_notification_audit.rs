use ocentra_policy_control_core::policy_delivery::PolicyDeliveryRecord;
use ocentra_policy_control_core::policy_request::{
    ChildPolicyRequest, PolicyRequestTimestamp, PolicyTemporaryOverride,
};
use ocentra_policy_control_core::policy_source::PolicyAuditReferenceId;

pub fn build_audit_reference_ids(
    request: &ChildPolicyRequest,
    temporary_override: Option<&PolicyTemporaryOverride>,
    delivery: Option<&PolicyDeliveryRecord>,
) -> Vec<PolicyAuditReferenceId> {
    let mut audit_reference_ids = request.audit_reference_ids.clone();

    temporary_override
        .into_iter()
        .for_each(|temporary_override| {
            extend_unique_audit_refs(
                &mut audit_reference_ids,
                &temporary_override.audit_reference_ids,
            );
        });
    delivery.into_iter().for_each(|delivery| {
        extend_unique_audit_refs(&mut audit_reference_ids, &delivery.audit_reference_ids);
    });

    audit_reference_ids
}

pub fn extend_unique_audit_refs(
    audit_reference_ids: &mut Vec<PolicyAuditReferenceId>,
    additional: &[PolicyAuditReferenceId],
) {
    for audit_reference_id in additional {
        if !audit_reference_ids.contains(audit_reference_id) {
            audit_reference_ids.push(audit_reference_id.clone());
        }
    }
}

pub fn recorded_at_for(request: &ChildPolicyRequest) -> PolicyRequestTimestamp {
    request
        .resolved_at
        .clone()
        .unwrap_or_else(|| request.requested_at.clone())
}
