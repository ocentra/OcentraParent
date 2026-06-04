# WP48 - Source Freshness Policy Consumption

## Scope

Cross-record the shared app/game WP48 source freshness policy-readiness proof
for the native app plan.

This workpack proves that native app policy readiness can require fresh,
evidence-backed inventory, runtime, and foreground source rows from the
activity-surface `sourceStatusRows` payload before policy compile is allowed.

It does not add portal UI, Rust/WebSocket runtime evaluation, adapter execution,
broad app blocking, platform support, or live classifier execution.

## Implementation

- Reuse the shared parent-domain source freshness policy-readiness contract.
- Map concrete native app source rows into inventory, runtime, and foreground
  policy requirement classes.
- Keep stale, missing, permission-limited, unavailable, adapter-error,
  manual-required, and not-claimed source rows as manual-required before policy
  compile.
- Keep launcher source freshness scoped to native game rows in the shared
  app/game plan.
- Preserve the no-adapter boundary by requiring `not-dispatched` output and
  rejecting direct adapter call requests.

## Proof

- `cmd /c npm run build:contracts`
- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-source-freshness-policy-consumption`
- `node scripts/test/app-game-source-freshness-policy-consumption-proof.mjs`
- `cmd /c npm run format:check`
- `cmd /c npm run lint:schema-boundaries`
- `git diff --check`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

Proof artifacts live in:

```text
output/app-plan-proof/48-source-freshness-policy-consumption
```

## No-Claim Boundaries

- Source freshness policy readiness consumes already-projected read-model rows
  only.
- The contract does not parse raw private executable paths, raw window titles,
  registry paths, or launcher payloads.
- Policy-ready means deterministic compile may proceed; it does not dispatch or
  execute an enforcement adapter.
- Runtime service evaluation, portal rendering, child UX, notifications,
  platform adapters, and broad app blocking remain gaps.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged because
primary owns central checklist edits during the merge wave. WP48 moves the
policy-consumption gap forward at the contract/proof layer, but product status
should not move until runtime service consumption, portal polish, adapter
execution, and platform proof are complete.
