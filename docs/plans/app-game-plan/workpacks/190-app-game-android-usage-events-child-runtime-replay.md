# WP190 App/Game Android UsageEvents Child Runtime Replay

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP190 App/Game Android UsageEvents Child Runtime Replay`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Attach the Android UsageEvents replay readiness row from WP188 to a child
runtime replay consumer boundary.

This proves a redacted-count consumer seam only. It does not prove Android
child-device delivery, raw UsageEvents row custody, Device Owner/Profile Owner
authority, hide/suspend, adapter dispatch, or platform enforcement.

## Implementation

- Added `packages/parent-domain/src/app-game-android-usage-events-child-runtime-replay.ts`.
- Added focused tests for redacted replay consumer attachment, unavailable
  replay fallback, and rejection of raw row, delivery, and drifted count
  overclaims.
- Added the combined platform runtime proof harness in
  `scripts/test/app-game-platform-runtime-readiness-batch.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-usage-events-child-runtime-replay app-game-linux-foreground-source-preflight
cmd /c node scripts/test/app-game-platform-runtime-readiness-batch.mjs
```

## Proof

- `test-results/app-game-platform-runtime-readiness-batch/proof.json`
- `output/app-game-plan-proof/190-191-platform-runtime-readiness-batch/proof.json`

## Boundaries

Proved:

- Redacted Android UsageEvents foreground counts can feed a child-runtime
  replay consumer boundary.
- The child runtime replay consumer gap is removed without claiming actual
  child-device delivery.

Not proved:

- Raw UsageEvents row storage or raw package/activity data.
- Android child-device delivery.
- Device Owner/Profile Owner authority.
- Hide/suspend/uninstall block, lock task, managed configuration, Play policy,
  adapter dispatch, platform enforcement, provider delivery, or broad blocking.
