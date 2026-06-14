# Workpack 04: Entitlement Delivery Gates

Purpose: define the app-owned billing, referral, and entitlement ledgers plus the signed snapshot gate consumed by trusted devices.

## Owns

- `APP_OWNED_BILLING_LEDGER.md`
- `APP_OWNED_REFERRAL_LEDGER.md`
- `APP_OWNED_ENTITLEMENT_LEDGER.md`
- `SIGNED_ENTITLEMENT_SNAPSHOT_MODEL.md`
- PSP-011

## Must prove

- Ledger entries explain the current access state.
- Signed snapshots are derived from the ledger.
- Snapshot verification rejects stale or mismatched devices.
- Revocation and grace states propagate into the snapshot.

## Proof path

- Use `docs/proof/payment-subscription-plan/wp04/` or the owning crate's local proof directory.

## Failure conditions

- The workpack fails if a snapshot can outlive the ledger state that created it.
- The workpack fails if child data leaks into the snapshot.
- The workpack fails if a revoked entitlement stays active in the snapshot.
