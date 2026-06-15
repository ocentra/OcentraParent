# App-Owned Entitlement Ledger

Purpose: define the access history the product uses to decide whether a household can keep using premium features.

## Ledger entries

| Entry                      | Meaning                          | Required fields                                                       |
| -------------------------- | -------------------------------- | --------------------------------------------------------------------- |
| EntitlementGrant           | A seat or access unit was added. | `entitlementId`, `householdId`, `source`, `quantity`, `effectiveFrom` |
| EntitlementHold            | Access is temporarily held.      | `entitlementId`, `reason`, `holdUntil`, `source`                      |
| EntitlementRevoke          | Access is removed or reduced.    | `entitlementId`, `reason`, `effectiveTo`, `source`                    |
| EntitlementSnapshotRequest | A signed snapshot was requested. | `householdId`, `deviceBindingId`, `requestedAt`, `mode`               |

## Rules

- The entitlement ledger is derived from billing, referral, device-trust, and support decisions.
- The ledger must explain every active child-device seat.
- A grace seat is time-bound and must be visible in the ledger.
- Signed snapshots are derived from the ledger, not the other way around.

## Failure conditions

- Do not expose child telemetry in entitlement rows.
- Do not apply product access directly from provider state without ledger materialization.
- Do not keep a revoked entitlement active after the ledger says it ended.
