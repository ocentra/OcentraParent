# SOCIAL-18 Source Snapshot

SOCIAL-18 adds contract-only platform connector authorization boundary support
under `packages/parent-domain`.

- `packages/parent-domain/src/social-platform-connector-authorization-values.ts`
  defines connector providers, authorization states, proof states, custody
  states, scopes, reasons, proof refs, and boundaries.
- `packages/parent-domain/src/social-platform-connector-authorization.ts`
  defines connector authorization row, claim-boundary, and boundary schemas plus
  decode helper.
- `packages/parent-domain/tests/social-platform-connector-authorization.test.ts`
  verifies honest optional connector rows and negative overclaim rejection.
- `scripts/test/social-platform-connector-authorization-proof.mjs` captures
  real public Google/YouTube supervision, Meta Family Center, and TikTok Family
  Pairing pages, writes screenshot refs and redacted hashes, and parses those
  refs through the connector boundary.

The boundary reuses parent-domain family, child, actor, evidence-ref, and
timestamp primitives and does not add public package/barrel exports in this row.
