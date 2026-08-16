# WP175 App/Game Scoped Adapter Dispatch Parent Action Surface

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP175 App/Game Scoped Adapter Dispatch Parent Action Surface`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Mount the existing app/game adapter dispatch preflight/result/executed state on
the App/Game Sessions product route and expose a parent-safe manual execute
button only when the service-backed dispatch-result read model contains the
single accepted scoped Windows owned-process app/game timer row.

This is a full-goal continuation slice on
`codex/app-game-control-product-completion`, not a micro PR branch.

## Implementation

- `packages/portal-domain` now exposes an execute action descriptor from the
  adapter dispatch result panel intent only when the read model has an accepted
  scoped command-result row.
- `apps/portal` now mounts `AppGameAdapterDispatchRoutePanel` on App/Game
  Sessions. The route renders side-effect-free preflight and result read-model
  refresh controls and a separate explicit execute control for
  `agent.activity.app-game.adapter-dispatch.execute`.
- `ParentPortalRoute` passes the parsed live preflight, dispatch-result, and
  latest executed-result state into that route panel.
- Focused tests prove the route attaches only to App/Game Sessions, sends the
  exact execute command/event path, renders parent-safe state, and keeps broad
  blocking, non-scoped platform enforcement, provider delivery, child delivery,
  raw private rows/targets, and private diagnostics unclaimed.

## Validation

Run:

```powershell
node scripts/test/app-game-scoped-adapter-dispatch-parent-action-surface-proof.mjs
```

The proof script runs:

- `cmd /c npm run build:contracts`
- `cmd /c npm run test --workspace @ocentra-parent/portal-domain -- app-game-adapter-dispatch-result-panel`
- `cmd /c npm run test --workspace @ocentra-parent/portal -- app-game-adapter-dispatch-route-panel`
- `cmd /c npm run build --workspace @ocentra-parent/portal`

Proof artifacts:

- `test-results/app-game-scoped-adapter-dispatch-parent-action-surface-proof/proof.json`
- `output/app-game-plan-proof/175-app-game-scoped-adapter-dispatch-parent-action-surface/proof.json`

## No-Claim Boundaries

- Overview polling remains side-effect-free.
- The execute command remains an explicit parent action.
- Broad installed-app blocking remains unclaimed.
- Platform enforcement outside the scoped Windows owned-process time-limit row
  remains unclaimed.
- Provider delivery and child-device delivery remain unclaimed.
- Raw private source rows, raw target values, and private diagnostics remain
  hidden from the parent surface.
