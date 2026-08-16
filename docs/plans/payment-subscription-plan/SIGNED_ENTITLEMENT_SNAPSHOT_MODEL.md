# Signed EntitlementSnapshot Model

Purpose: define the derived artifact the product can trust on-device after the server has decided the entitlement state.

Current Rust owner: `crates/entitlement-core/src/entitlement_access.rs` owns the signed snapshot derivation model and the bridge that turns a signed snapshot plus verification inputs into the device-side entitlement gate context. TypeScript proof files may consume this model but must not become the source of truth.

## Snapshot fields

| Field | Meaning |
| --- | --- |
| `schemaVersion` | Snapshot schema version. |
| `snapshotId` | Unique snapshot identifier. |
| `accountRef` | Billing account reference. |
| `householdRef` | Household reference. |
| `trustedDeviceRef` | Trusted device binding reference. |
| `planTier` | Current plan tier. |
| `featureFlags` | Snapshot feature flags. |
| `limits` | Snapshot limit bundle. |
| `baseChildDeviceLimit` | Starter bundle child-device count. |
| `activeReferralCredits` | Active referral-derived child-device credits. |
| `paidExtraChildDeviceSeats` | Monthly paid add-on seats. |
| `effectiveChildDeviceLimit` | Derived total child-device limit. |
| `issuedAt` | Snapshot issuance time. |
| `expiresAt` | Snapshot expiration time. |
| `graceUntil` | Grace expiry time. |
| `livemode` | Live or test mode indicator. |
| `revocationCursor` | Cursor used to invalidate stale snapshots. |
| `deviceTrustRequired` | Whether sealed device trust is required. |
| `packageBuildRef` | Package or build channel reference. |
| `signatureKeyId` | Signing key identifier. |
| `signature` | Server signature over the payload. |

## Validation rules

- The app must verify the signature before trusting the snapshot.
- The snapshot payload must be derived from billing, referral, and entitlement ledgers before it is signed.
- The snapshot must match the household and device binding.
- Expired or revoked snapshots must be rejected.
- `deviceTrustRequired` must flow into the local gate so missing sealed device trust fails closed only where the model says it is required.
- The snapshot must not contain child names, child activity, screenshots, or provider secrets.

## Rejection rules

- Reject if signature invalid.
- Reject if wrong household.
- Reject if wrong device.
- Reject if expired beyond grace.
- Reject if revoked.
- Reject if local sealed device trust missing.
- Reject if package or build channel invalid.

## Failure conditions

- Do not treat the snapshot as the root of trust.
- Do not let a snapshot outlive the ledger state that generated it.
- Do not reuse a snapshot across households or devices.
- Do not let provider echoes or provider device-limit hints replace ledger-owned plan or effective-limit truth.
