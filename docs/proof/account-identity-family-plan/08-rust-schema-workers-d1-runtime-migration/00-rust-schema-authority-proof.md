# WP08 Rust schema authority proof

Scope: Account Identity Family WP08 only, recorded 2026-08-04 on
`codex/account-authority-wp08`.

Rust owns the encoded account/family edge contract:

- `crates/schema/src/family_references_ts.rs` emits the checked-in TypeScript
  edge artifact from the Rust-owned template.
- `crates/schema/tests/contract/family_references.rs` byte-compares that output
  with `packages/schema-domain/src/generated-family-references.ts` in
  `family_references_generated_typescript_matches_checked_in_file`.
- The generated file is an edge shape for parent actor/account, family, child
  profile, parent device, evidence, and action references; it does not define
  TypeScript authorization behavior.

Evidence: the full schema contract target passed (98 tests), and the targeted
generated-edge drift test passed (1 test).

No claim: this proves neither an auth provider nor a Cloudflare, D1, Durable
Object, KV, browser, or deployment runtime.
