# Workpack 01: Device Trust Source of Truth

Purpose: define trust ownership, trust states, bootstrap lifecycle, and cross-plan boundaries.

## Owns

- Trust vocabulary.
- Parent vs child authority.
- Bootstrap, revoke, reset, and re-pair transitions.
- Relationship to login, billing entitlement, remote access, and policy delivery.

## Exit condition

- The trust state machine is explicit and typed.
- Login is separate from trust.
- Child devices never own the trust root.
- Adjacent plan boundaries are written down in one place.

## Proof target

- `docs/proof/device-trust-bootstrap-plan/01-*`
- Route-sync note in the cross-plan gate workpack.

## Negative cases

- Copied binaries do not create trust.
- Login success does not imply device trust.
- Revoked trust cannot be revived by stale local state.