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

- `output/device-trust-bootstrap-plan-proof/06-recovery-reset-re-pair/`

## Current audit state

- No proof root currently exists on disk for this workpack.
- Recovery authorization and handoff rules exist in `packages/family-domain`, but encrypted recovery bundle handling and re-pair runtime proof are still missing.

## Negative cases

- Corrupted bundles fail closed.
- Partial restore cannot silently create a new trust root.
- Revoked trust cannot be resurrected by recovery.
