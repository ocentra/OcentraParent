# WP188 App/Game Android UsageEvents Replay Readiness

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP188 App/Game Android UsageEvents Replay Readiness`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Turn the redacted foreground UsageEvents counts from WP185 into a parent-domain
runtime visibility replay readiness read model.

This is not an Android enforcement adapter. It is a durable, parent-safe replay
boundary over counts and proof refs only.

## Implementation

- Added `packages/parent-domain/src/app-game-android-usage-events-replay.ts`.
- Added focused tests for accepted replay readiness, unavailable replay state,
  and rejection of raw row/enforcement claim upgrades.
- Updated parent-domain platform proof status so an attached Android replay
  read model adds `android-usage-events-replay-ref` and removes
  `android-durable-usage-events-replay-not-proved`.
- Added `scripts/test/app-game-android-usage-events-replay-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-usage-events-replay app-game-platform-proof-status
cmd /c node scripts/test/app-game-android-usage-events-replay-proof.mjs
```

## Proof

- `test-results/app-game-android-usage-events-replay-proof/proof.json`
- `output/app-game-plan-proof/188-app-game-android-usage-events-replay/proof.json`

## Boundaries

Proved:

- Redacted Android UsageEvents foreground counts can drive a durable replay
  readiness row.
- Parent-domain platform proof status can carry the replay proof ref and remove
  the durable replay gap when the replay read model is attached.

Not proved:

- Raw UsageEvents row storage or raw package/activity data.
- Android child runtime replay consumer.
- Device Owner/Profile Owner authority.
- Hide/suspend/uninstall block, lock task, managed configuration, Play policy,
  adapter dispatch, platform enforcement, provider delivery, or child-device
  delivery.
