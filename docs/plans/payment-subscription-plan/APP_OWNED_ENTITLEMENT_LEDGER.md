# App-Owned Entitlement Ledger

Purpose: define the access history the product uses to decide whether a household can keep using premium features.

Current Rust owner: `crates/entitlement-core/src/entitlement_access.rs` owns the signed entitlement snapshot derivation contract and the snapshot-to-device gate bridge. TypeScript may validate or present the resulting shape, but it is not the runtime owner.

## Ledger entries

| Entry | Meaning | Required fields |
| --- | --- | --- |
| `HouseholdEntitlement` | Household-level entitlement state. | `householdRef`, `planTier`, `status`, `effectiveFrom`, `effectiveTo` |
| `FeatureEntitlement` | Feature-level access state. | `householdRef`, `featureKey`, `status`, `source`, `effectiveFrom` |
| `DeviceSeatEntitlement` | Child-device seat state. | `householdRef`, `deviceSeatCount`, `source`, `effectiveFrom`, `effectiveTo` |
| `EffectiveEntitlementProjection` | Queryable current access state. | `householdRef`, `effectiveChildDeviceLimit`, `graceState`, `updatedAt` |
| `EntitlementSnapshot` | Signed derived artifact consumed by trusted devices. | `snapshotId`, `accountRef`, `householdRef`, `trustedDeviceRef`, `planTier`, `featureFlags`, `limits`, `baseChildDeviceLimit`, `activeReferralCredits`, `paidExtraChildDeviceSeats`, `effectiveChildDeviceLimit`, `issuedAt`, `expiresAt`, `graceUntil`, `livemode`, `revocationCursor`, `deviceTrustRequired`, `packageBuildRef`, `signatureKeyId`, `signature` |
| `EntitlementAuditEvent` | Audited entitlement change. | `auditEventId`, `householdRef`, `actorRef`, `reason`, `createdAt` |

## Rules

- The entitlement ledger is derived from billing, referral, device-trust, and support decisions.
- The ledger must explain every active child-device seat.
- A grace seat is time-bound and must be visible in the ledger.
- Signed snapshots are derived from the ledger, not the other way around.
- The effective child-device limit must equal base seats plus active referral credits plus paid extra child-device seats.
- Provider state may annotate issuance mode only; it does not replace ledger-owned plan, feature, or seat truth.

## Failure conditions

- Do not expose child telemetry in entitlement rows.
- Do not apply product access directly from provider state without ledger materialization.
- Do not keep a revoked entitlement active after the ledger says it ended.
