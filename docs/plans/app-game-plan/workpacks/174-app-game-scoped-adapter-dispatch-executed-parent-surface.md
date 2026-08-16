# WP174 App/Game Scoped Adapter Dispatch Executed Parent Surface

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP174 App/Game Scoped Adapter Dispatch Executed Parent Surface`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Carry the explicit WP173
`agent.activity.app-game.adapter-dispatch.executed` event into parent-visible
state and panel intent data.

This workpack is limited to showing the latest scoped Windows owned-process
app/game timer execute result after a parent/dev user manually runs the execute
command. The read-model command remains side-effect-free, and the parent
surface must keep broad installed-app blocking, non-scoped platform
enforcement, provider delivery, child-device delivery, raw private rows/targets,
and private diagnostics unclaimed.

## Implementation

- Parse the latest `agent.activity.app-game.adapter-dispatch.executed` event in
  portal live activity state.
- Keep the executed result separate from the side-effect-free dispatch result
  read model.
- Render parent-safe execute command/result/status/audit/readback details in the
  existing app/game adapter dispatch result panel intent.
- Preserve the existing read-model row rendering and no-claim boundaries.

## Validation

- `node scripts/test/app-game-scoped-adapter-dispatch-executed-parent-surface-proof.mjs`
- `cmd /c npm run test --workspace @ocentra-parent/portal -- live-activity-state`
- `cmd /c npm run test --workspace @ocentra-parent/portal-domain -- app-game-adapter-dispatch-result-panel`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- app-game-adapter-dispatch-result contracts`
- `cmd /c npm run build --workspace @ocentra-parent/portal`
- `git diff --check`
- `npm run lanes:guard`
- `npm run hub:guard`

## Non-Claims

- The execute command is not auto-run by overview polling.
- Broad installed-app blocking execution is not implemented.
- Platform enforcement outside the scoped Windows owned-process timer boundary
  remains unclaimed.
- Provider delivery, provider receipt ingestion, and child-device runtime
  delivery are not implemented by this workpack.
- Raw private source rows, raw target values, and private diagnostics remain out
  of parent-facing payloads.
