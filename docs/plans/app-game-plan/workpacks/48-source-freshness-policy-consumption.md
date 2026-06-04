# WP48 - Source Freshness Policy Consumption

## Scope

Consume the backend `sourceStatusRows` added in WP47 through a parent-domain
policy-readiness contract.

This workpack proves that native app and native game policy readiness can
require fresh, evidence-backed inventory, runtime, foreground, and launcher
source rows before policy compile is allowed.

It does not add portal UI, Rust/WebSocket runtime evaluation, adapter execution,
broad blocking, platform hard-control support, or live classifier execution.

## Implementation

- Add a parent-domain source freshness policy-readiness contract for app/game
  rows.
- Map concrete read-model source kinds such as `osInstalledRecord`,
  `processSnapshot`, `foregroundWindow`, and `launcherManifest` into policy
  requirement classes.
- Allow policy compile only when every required source class has fresh,
  evidence-backed, available read-model rows.
- Keep stale, missing, permission-limited, unavailable, adapter-error,
  manual-required, and not-claimed source rows as manual-required before policy
  compile.
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
output/app-game-plan-proof/48-source-freshness-policy-consumption
```

## No-Claim Boundaries

- Source freshness policy readiness consumes already-projected read-model rows
  only.
- The contract does not parse raw private executable paths, raw window titles,
  registry paths, or launcher payloads.
- Policy-ready means deterministic compile may proceed; it does not dispatch or
  execute an enforcement adapter.
- Launcher freshness remains a separate requirement for game rows and does not
  promote launcher evidence into proved child-game state.
- Runtime service evaluation, portal rendering, child UX, notifications,
  platform adapters, and broad blocking remain gaps.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged because
primary owns central checklist edits during the merge wave. WP48 moves the
policy-consumption gap forward at the contract/proof layer, but product status
should not move until runtime service consumption, portal polish, adapter
execution, and platform proof are complete.
