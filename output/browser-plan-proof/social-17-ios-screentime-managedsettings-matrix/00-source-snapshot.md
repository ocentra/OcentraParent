# SOCIAL-17 Source Snapshot

SOCIAL-17 adds contract-only iOS Screen Time/ManagedSettings social capability
matrix support under `packages/parent-domain`.

- `packages/parent-domain/src/social-ios-screen-time-capability-matrix-values.ts`
  defines iOS social capability surfaces, target kinds, capability states,
  proof states, policy scopes, reasons, proof refs, and boundaries.
- `packages/parent-domain/src/social-ios-screen-time-capability-matrix.ts`
  defines capability row, claim-boundary, and matrix schemas plus decode helper.
- `packages/parent-domain/tests/social-ios-screen-time-capability-matrix.test.ts`
  verifies honest entitlement-required/token/manual iOS social capability rows
  and negative overclaim rejection.
- `scripts/test/social-ios-screen-time-host-proof.mjs` records the real
  host/tooling boundary for iOS proof on the current worker host.

The matrix reuses parent-domain product capability vocabulary and does not add
public package/barrel exports in this row.
