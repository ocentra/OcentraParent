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

- `docs/proof/device-trust-bootstrap-plan/07-*`

## Negative cases

- Tampered or copied binaries do not stay trusted.
- A child device cannot revoke its own parent relationship.
- Revoked devices stop unlocking behavior.