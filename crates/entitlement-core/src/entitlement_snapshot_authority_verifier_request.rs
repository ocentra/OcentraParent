#![forbid(unsafe_code)]

use chrono::{DateTime, Utc};
use ocentra_family_identity_core::{
    account_identity_authority::VerifiedAccountIdentityAuthority,
    device_trust_current_binding::CurrentChildDeviceTrustBinding,
};

use crate::entitlement_snapshot_values::{
    EntitlementAccountRef, EntitlementHouseholdRef, EntitlementPackageBuildRef,
    EntitlementSnapshotReleaseChannel, EntitlementTrustedDeviceRef,
};

use super::EntitlementSnapshotVerificationFailure;

/// Current account/household/device/package selectors are produced only from
/// the family-owned opaque account authority and installed-package provider.
/// This type has no serde or public field constructor.
pub(super) struct EntitlementSnapshotVerificationRequest {
    pub(crate) account_ref: EntitlementAccountRef,
    pub(crate) household_ref: EntitlementHouseholdRef,
    pub(crate) trusted_device_ref: EntitlementTrustedDeviceRef,
    pub(crate) package_build_ref: EntitlementPackageBuildRef,
    pub(crate) release_channel: EntitlementSnapshotReleaseChannel,
}

impl EntitlementSnapshotVerificationRequest {
    pub(super) fn from_current_account_authority(
        authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
        package_build_ref: EntitlementPackageBuildRef,
        release_channel: EntitlementSnapshotReleaseChannel,
    ) -> Result<Self, EntitlementSnapshotVerificationFailure> {
        let (
            account_state,
            membership_state,
            device_trust_state,
            session_freshness_state,
            pairing_state,
            install_state,
            lifecycle_state,
            revocation_state,
        ) = authority.report_query_custody_states();
        if account_state
            != ocentra_schema::account_identity_authority::AccountIdentityAccountState::Active
            || membership_state
                != ocentra_schema::account_identity_authority::AccountIdentityMembershipState::Active
            || device_trust_state
                != ocentra_schema::account_identity_authority::AccountIdentityDeviceTrustState::Trusted
            || session_freshness_state
                != ocentra_schema::account_identity_authority::AccountIdentitySessionFreshnessState::Fresh
            || pairing_state
                != ocentra_schema::account_identity_authority::AccountIdentityPairingState::Paired
            || install_state
                != ocentra_schema::account_identity_authority::AccountIdentityInstallState::Installed
            || lifecycle_state
                != ocentra_schema::account_identity_authority::AccountIdentityBindingLifecycleState::Active
            || revocation_state
                != ocentra_schema::account_identity_authority::AccountIdentityBindingRevocationState::Active
        {
            return Err(EntitlementSnapshotVerificationFailure::CurrentAuthorityUnavailable);
        }
        let session_expires_at = DateTime::parse_from_rfc3339(authority.session_expires_at())
            .map(|timestamp| timestamp.with_timezone(&Utc))
            .map_err(|_error| {
                EntitlementSnapshotVerificationFailure::CurrentAuthorityUnavailable
            })?;
        if Utc::now() >= session_expires_at
            || device_binding.authority_generation() == 0
            || device_binding.lifecycle_generation() == 0
            || device_binding.installation_binding_generation() == 0
        {
            return Err(EntitlementSnapshotVerificationFailure::CurrentAuthorityUnavailable);
        }

        Ok(Self {
            account_ref: EntitlementAccountRef::parse(authority.account_id().as_str()).map_err(
                |_error| EntitlementSnapshotVerificationFailure::CurrentAuthorityUnavailable,
            )?,
            household_ref: EntitlementHouseholdRef::parse(authority.household_id().as_str())
                .map_err(|_error| {
                    EntitlementSnapshotVerificationFailure::CurrentAuthorityUnavailable
                })?,
            trusted_device_ref: EntitlementTrustedDeviceRef::parse(
                authority.child_device_id().as_str(),
            )
            .map_err(|_error| {
                EntitlementSnapshotVerificationFailure::CurrentAuthorityUnavailable
            })?,
            package_build_ref,
            release_channel,
        })
    }
}
