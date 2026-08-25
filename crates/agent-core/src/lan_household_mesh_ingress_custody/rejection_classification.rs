use super::{LanHouseholdMeshIngressCustodyError, LanHouseholdMeshIngressRejectionReason};

pub(super) fn for_custody_failure(
    error: &LanHouseholdMeshIngressCustodyError,
) -> Option<LanHouseholdMeshIngressRejectionReason> {
    match error {
        LanHouseholdMeshIngressCustodyError::InvalidInput => {
            Some(LanHouseholdMeshIngressRejectionReason::InvalidInput)
        }
        LanHouseholdMeshIngressCustodyError::IdentityMismatch => {
            Some(LanHouseholdMeshIngressRejectionReason::IdentityMismatch)
        }
        LanHouseholdMeshIngressCustodyError::AuthorityStale => {
            Some(LanHouseholdMeshIngressRejectionReason::AuthorityStale)
        }
        LanHouseholdMeshIngressCustodyError::SequenceRegression => {
            Some(LanHouseholdMeshIngressRejectionReason::SequenceRegression)
        }
        LanHouseholdMeshIngressCustodyError::DuplicateReceipt { .. } => {
            Some(LanHouseholdMeshIngressRejectionReason::DuplicateReceipt)
        }
        LanHouseholdMeshIngressCustodyError::ReconciliationRequired { .. } => {
            Some(LanHouseholdMeshIngressRejectionReason::ReconciliationRequired)
        }
        _ => None,
    }
}
