use super::LanHouseholdMeshIngressRejectionReason;

const STORED_REASONS: [(&str, LanHouseholdMeshIngressRejectionReason); 7] = [
    (
        "authority-stale",
        LanHouseholdMeshIngressRejectionReason::AuthorityStale,
    ),
    ("expired", LanHouseholdMeshIngressRejectionReason::Expired),
    (
        "identity-mismatch",
        LanHouseholdMeshIngressRejectionReason::IdentityMismatch,
    ),
    (
        "invalid-input",
        LanHouseholdMeshIngressRejectionReason::InvalidInput,
    ),
    (
        "duplicate-receipt",
        LanHouseholdMeshIngressRejectionReason::DuplicateReceipt,
    ),
    (
        "reconciliation-required",
        LanHouseholdMeshIngressRejectionReason::ReconciliationRequired,
    ),
    (
        "sequence-regression",
        LanHouseholdMeshIngressRejectionReason::SequenceRegression,
    ),
];

impl LanHouseholdMeshIngressRejectionReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuthorityStale => "authority-stale",
            Self::Expired => "expired",
            Self::IdentityMismatch => "identity-mismatch",
            Self::InvalidInput => "invalid-input",
            Self::DuplicateReceipt => "duplicate-receipt",
            Self::ReconciliationRequired => "reconciliation-required",
            Self::SequenceRegression => "sequence-regression",
        }
    }

    pub(super) fn from_stored(value: &str) -> Option<Self> {
        STORED_REASONS
            .iter()
            .find_map(|(stored, reason)| (*stored == value).then_some(*reason))
    }
}
