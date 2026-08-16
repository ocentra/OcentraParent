# WP08 account authority parity proof

The canonical behavioral owner is `crates/family-identity-core`:

- `family_identity.rs` owns household membership, account, profile, device, and
  session state literals.
- `household_authority.rs` owns household action authorization and parent
  step-up decisions.
- `setup_lifecycle.rs` owns invite and recovery authorization.
- `session_lifecycle.rs` owns session issuance and action authorization.

Focused evidence passed:

- `cargo test -p ocentra-family-identity-core household_authority` — 15 tests.
- `cargo test -p ocentra-family-identity-core --test unit setup_lifecycle` — 18 tests.
- `cargo test -p ocentra-family-identity-core --test unit session_lifecycle` — 13 tests.
- `device_registration_rejects_wrong_household_child_binding` — 1 contract test.

The parity claim is limited to the Rust authority model and its generated edge
shape. It is not a claim of persistent Cloudflare account authority.

## Record-derived handoff recovery (2026-08-05)

`family_identity::household_authority_handoff` now derives a versioned,
identifier-only `HouseholdAuthorityHandoffDecision` from canonical parent,
child-profile, and device-registration records before delegating to the existing
Rust authority evaluator. The handoff carries an evaluation correlation id,
canonical identifiers, the requested action, and the decision; it does not
carry profile display names, invite/recovery material, or session credentials.

This is a Rust contract for Cloudflare WP06 to consume when it later owns D1
persistence. It does not add a Cloudflare binding or persistence adapter.
