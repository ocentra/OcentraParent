#![forbid(unsafe_code)]

use ocentra_family_identity_core::{
    account_identity_authority::VerifiedAccountIdentityAuthority,
    device_trust_current_binding::CurrentChildDeviceTrustBinding,
};

use crate::entitlement_snapshot::SignedEntitlementSnapshot;
use crate::entitlement_snapshot_cache::SignedEntitlementRevocationUpdate;
use crate::entitlement_snapshot_values::EntitlementSnapshotFreshnessState;

use super::{
    EntitlementCurrentnessAuthority, EntitlementSnapshotVerificationFailure,
    ManualRequiredEntitlementCurrentnessAuthority,
};

impl EntitlementCurrentnessAuthority for ManualRequiredEntitlementCurrentnessAuthority {
    fn validate_revocation_generation(
        &self,
        _authority_generation: u64,
    ) -> Result<(), EntitlementSnapshotVerificationFailure> {
        Err(EntitlementSnapshotVerificationFailure::AuthorityUnavailable)
    }

    fn validate_current_identity(
        &self,
        _account_authority: &VerifiedAccountIdentityAuthority,
        _device_binding: &CurrentChildDeviceTrustBinding,
    ) -> Result<(), EntitlementSnapshotVerificationFailure> {
        Err(EntitlementSnapshotVerificationFailure::AuthorityUnavailable)
    }

    fn evaluate_snapshot_freshness(
        &self,
        _snapshot: &SignedEntitlementSnapshot,
        _revocation_update: &SignedEntitlementRevocationUpdate,
    ) -> Result<EntitlementSnapshotFreshnessState, EntitlementSnapshotVerificationFailure> {
        Err(EntitlementSnapshotVerificationFailure::AuthorityUnavailable)
    }
}
