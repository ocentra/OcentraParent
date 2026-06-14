# Workpack 06: Recovery Reset Re-Pair

Purpose: define encrypted recovery bundles, reset, revoke, and re-pair flows.

## Owns

- Encrypted recovery bundle shape.
- Household-bound restore semantics.
- Parent-authorized reset and re-pair.
- Revocation preservation during restore.

## Exit condition

- Recovery is not the same as account login.
- Wrong-household and wrong-key restores fail.
- Re-pair after reset is explicit and audited.

## Proof target

- `docs/proof/device-trust-bootstrap-plan/06-*`

## Negative cases

- Corrupted bundles fail closed.
- Partial restore cannot silently create a new trust root.
- Revoked trust cannot be resurrected by recovery.