#![forbid(unsafe_code)]

//! Issuer boundary for signed entitlement snapshots.
//!
//! The issuer owns the canonical projection-to-signing-bytes transition, but
//! never owns a private key in this crate. A platform/HSM/provider adapter in
//! a future owner composition supplies signatures through the typed port. The
//! projection accepted by
//! the issuer is opaque and can only be produced by the entitlement owner
//! after billing/account/device/revocation currentness and trusted clock/TTL
//! checks. The module is crate-internal until a concrete owner composer exists;
//! the shipped adapter is manual-required.

use crate::entitlement_snapshot::{
    SignedEntitlementSnapshot, UnsignedEntitlementSnapshotProjection,
};
use crate::entitlement_snapshot_values::EntitlementSignatureKeyId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntitlementSnapshotIssuerError {
    SigningUnavailable,
    InvalidSignature,
    InvalidProjection,
}

/// Opaque owner-produced issuance material.
///
/// No public constructor, serde implementation, or field accessor exists.
/// This type is intentionally uninhabitable by transport/caller code until a
/// billing/account/device owner mounts a real currentness composition inside
/// the entitlement owner.
pub struct TrustedEntitlementIssuanceProjection {
    projection: UnsignedEntitlementSnapshotProjection,
    issuer_key_id: EntitlementSignatureKeyId,
}

impl std::fmt::Debug for TrustedEntitlementIssuanceProjection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TrustedEntitlementIssuanceProjection")
            .field("authority", &"opaque")
            .finish()
    }
}

impl TrustedEntitlementIssuanceProjection {
    pub(crate) fn from_owner_currentness(
        projection: UnsignedEntitlementSnapshotProjection,
        issuer_key_id: EntitlementSignatureKeyId,
    ) -> Self {
        Self {
            projection,
            issuer_key_id,
        }
    }
}

/// External signing custody for the entitlement issuer.
///
/// Implementations must route to a durable, authenticated issuer or hardware
/// boundary.  The issuer invokes this port only with owner-selected key
/// material and canonical bytes; it never accepts a caller-selected wire
/// projection or clock.
pub trait EntitlementSnapshotSigningProvider: Send + Sync {
    fn sign(
        &self,
        issuer_key_id: &EntitlementSignatureKeyId,
        signing_bytes: &[u8],
    ) -> Result<Vec<u8>, EntitlementSnapshotIssuerError>;
}

#[derive(Debug, Default)]
pub struct ManualRequiredEntitlementSnapshotSigningProvider;

impl EntitlementSnapshotSigningProvider for ManualRequiredEntitlementSnapshotSigningProvider {
    fn sign(
        &self,
        _issuer_key_id: &EntitlementSignatureKeyId,
        _signing_bytes: &[u8],
    ) -> Result<Vec<u8>, EntitlementSnapshotIssuerError> {
        Err(EntitlementSnapshotIssuerError::SigningUnavailable)
    }
}

pub struct EntitlementSnapshotIssuer<P> {
    signing_provider: P,
}

impl<P> EntitlementSnapshotIssuer<P>
where
    P: EntitlementSnapshotSigningProvider,
{
    pub fn new(signing_provider: P) -> Self {
        Self { signing_provider }
    }

    pub fn issue(
        &self,
        trusted_projection: TrustedEntitlementIssuanceProjection,
    ) -> Result<SignedEntitlementSnapshot, EntitlementSnapshotIssuerError> {
        let mut snapshot = SignedEntitlementSnapshot::from_projection(
            trusted_projection.projection,
            trusted_projection.issuer_key_id,
        );
        snapshot
            .validate_shape_without_signature()
            .map_err(|_error| EntitlementSnapshotIssuerError::InvalidProjection)?;
        let signature = self
            .signing_provider
            .sign(&snapshot.signature_key_id, &snapshot.signing_bytes())?;
        if signature.len() != crate::entitlement_snapshot::ENTITLEMENT_SNAPSHOT_SIGNATURE_BYTES {
            return Err(EntitlementSnapshotIssuerError::InvalidSignature);
        }
        snapshot.signature = signature;
        Ok(snapshot)
    }
}
