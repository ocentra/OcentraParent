# Workpack 07: Child Tamper Uninstall

Purpose: define child tamper, uninstall, and anti-tamper boundaries.

## Owns

- Tamper signals.
- Uninstall authorization boundaries.
- Safe degraded or revoked states.
- Parent-controlled recovery response.

## Exit condition

- Child devices cannot self-authorize removal of trust.
- Tamper produces a clear response or revocation path.
- No magic anti-root claims are made.

## Proof target

- `output/device-trust-bootstrap-plan-proof/07-child-tamper-uninstall/`

## Current audit state

- No proof root currently exists on disk for this workpack.
- Tamper status contracts exist elsewhere in the repo, but parent-controlled uninstall and tamper execution for this plan remain unimplemented.

## Negative cases

- Tampered or copied binaries do not stay trusted.
- A child device cannot revoke its own parent relationship.
- Revoked devices stop unlocking behavior.
