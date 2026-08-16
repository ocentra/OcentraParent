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

- `output/device-trust-bootstrap-plan-proof/05-entitlement-device-license/`

## Current audit state

- No proof root currently exists on disk for this workpack.
- Billing entitlement contracts exist elsewhere in the repo, but a device-trust-bound entitlement runtime is still missing.
- Entitlement-core now validates the local account, household, trusted-device,
  package, and active-time bindings before consulting an authority verifier.
  Signature verification and revocation remain unavailable/manual-required;
  the default verifier cannot unlock a capability.

## Negative cases

- Wrong device cannot unlock the entitlement.
- Wrong household cannot unlock the entitlement.
- Expired or revoked snapshots fail closed or enter labeled grace only.
