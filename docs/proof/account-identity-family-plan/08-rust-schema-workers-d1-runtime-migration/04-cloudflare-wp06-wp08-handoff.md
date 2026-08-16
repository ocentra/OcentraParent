# WP08 downstream Cloudflare handoff

Account WP08 supplies the Rust-owned contract and does not implement a worker.

- Cloudflare Control Plane WP06 consumes this contract and owns D1/DO/KV
  bindings, migrations, persistence, and storage proof.
- Cloudflare Control Plane WP08 starts only after WP06 and owns the Cloudflare
  test runner, integration, and test-pyramid proof.

PR #607's TypeScript adapter/D1-test-double branch remains historical evidence
only: it is neither rebased nor used as WP08 implementation or validation.

The current Rust input to that downstream ownership is
`family_identity::household_authority_handoff::HouseholdAuthorityHandoffDecision`.
It is schema-versioned and derived from canonical account/family records, so
Cloudflare WP06 can persist or map the explicit identifier/action/decision
envelope without re-deriving family scope from caller-supplied booleans. WP06
still owns its D1 binding, migration directory, persistence adapter, and
storage proof. WP08 still owns its module runner and integration proof.
