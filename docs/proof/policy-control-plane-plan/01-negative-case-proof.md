# Policy Control Plane Negative Case Proof

This file records negative outcomes that are intentionally treated as open truth rather than silently promoted to green.

## Verified negative gates

- Missing proof files are treated as missing. Passing package or crate tests do not backfill absent workpack artifacts.
- `npm run test --workspace @ocentra-parent/portal -- tests/policy-preview-route-panel.test.ts tests/policy-preview-live-activity-state.test.ts` is not accepted as scoped proof because the workspace script expands to `vitest run tests` and pulled an unrelated LAN failure in `tests/live-activity-surface-adapter.test.ts`.
- `npm run lint:architecture -- --files packages/policy-domain crates/policy-control-core packages/agent-protocol-domain crates/agent-protocol apps/portal docs/plans/policy-control-plane-plan` is not accepted as green because `packages/agent-protocol-domain` still contains banned re-exports and unused disable warnings.
- Feature docs still mark parent authoring and assistant approval flows incomplete, so contract tests do not close WP02 or WP05.

## Consequence

The plan can record real contract coverage and still remain open. Negative proof here is the refusal to upgrade missing proof, failing architecture, or dependency-owned UI gaps into completion claims.

