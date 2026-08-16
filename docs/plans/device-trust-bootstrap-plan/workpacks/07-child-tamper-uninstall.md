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
- `crates/child-runtime/src/removal.rs` now owns a durable, locked tamper-evidence record and exposes a separate manual-required readiness state without allowing that evidence to revoke, reauthorize, or create a platform-removal obligation.
- `crates/child-runtime/src/removal.rs` also requires an identity-bound `RevokeChildDevice` authority for revocation; the retained reference is audit evidence only. `crates/child-runtime/src/service.rs` blocks command ingress while tamper evidence is unresolved and exposes the child executable composition entrypoint.
- Package-manager/device-owner removal, tamper-source attestation, parent transport, platform handoff, tests, and proof remain unimplemented or externally owned.

## Code-drafted / validation deferred

- Local tamper evidence is fail-closed to `manual-required`; it does not claim anti-root, uninstall prevention, or a trusted tamper verdict.
- Parent revocation remains the only durable trust transition and must use the existing verified household authority bound to the configured household, child profile, and target device.

## Negative cases

- Tampered or copied binaries do not stay trusted.
- A child device cannot revoke its own parent relationship.
- Revoked devices stop unlocking behavior.
