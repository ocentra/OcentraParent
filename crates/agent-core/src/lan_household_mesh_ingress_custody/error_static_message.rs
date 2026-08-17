use super::LanHouseholdMeshIngressCustodyError;

pub(super) fn for_error(error: &LanHouseholdMeshIngressCustodyError) -> &'static str {
    match error {
        LanHouseholdMeshIngressCustodyError::StorageUnavailable => {
            "household mesh custody storage unavailable"
        }
        LanHouseholdMeshIngressCustodyError::SchemaRejected => {
            "household mesh custody schema rejected"
        }
        LanHouseholdMeshIngressCustodyError::IntegrityRejected => {
            "household mesh custody integrity rejected"
        }
        LanHouseholdMeshIngressCustodyError::InvalidInput => "household mesh custody input invalid",
        LanHouseholdMeshIngressCustodyError::IdentityMismatch => {
            "household mesh ingress identity mismatch"
        }
        LanHouseholdMeshIngressCustodyError::AuthorityStale => {
            "household mesh ingress authority is stale"
        }
        LanHouseholdMeshIngressCustodyError::SequenceRegression => {
            "household mesh ingress sequence did not advance"
        }
        LanHouseholdMeshIngressCustodyError::Rejected { .. }
        | LanHouseholdMeshIngressCustodyError::DuplicateReceipt { .. }
        | LanHouseholdMeshIngressCustodyError::ReconciliationRequired { .. } => {
            "household mesh custody error"
        }
    }
}
