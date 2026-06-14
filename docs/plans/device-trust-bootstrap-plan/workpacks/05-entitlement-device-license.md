# Workpack 05: Entitlement Device License

Purpose: define signed entitlement snapshots and device-bound license unlock.

## Owns

- Entitlement snapshot fields.
- Signature verification.
- Expiry, grace, and revocation handling.
- Device-bound unlock behavior.

## Exit condition

- Copied binaries or configs do not unlock product behavior.
- The entitlement snapshot is signed and device-bound.
- Revocation overrides stale cache.

## Proof target

- `docs/proof/device-trust-bootstrap-plan/05-*`

## Negative cases

- Wrong device cannot unlock the entitlement.
- Wrong household cannot unlock the entitlement.
- Expired or revoked snapshots fail closed or enter labeled grace only.