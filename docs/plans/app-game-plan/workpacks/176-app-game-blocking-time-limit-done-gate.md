# WP176: App/Game Blocking Time-Limit Done Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP176: App/Game Blocking Time-Limit Done Gate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Close the feature-local blocking/time-limit done gate without broadening the
enforcement claim:

- aggregate the scoped Windows owned-process app/game timer execution proof;
- aggregate the App/Game Sessions explicit parent action proof;
- aggregate broad-blocking manual-required/not-claimed gates;
- fail the proof if any broad/platform/provider/child/private claim flips true.

## Acceptance

- The scoped Windows owned-process app/game timer path remains the only
  execution-ready path.
- App/Game Sessions execution remains an explicit parent action, not overview
  polling or read-model refresh.
- Broad installed-app blocking, non-scoped platform enforcement, provider
  delivery, child-device delivery, raw private rows/targets, and private
  diagnostics remain unclaimed.
- The app/game feature doc can mark the blocking/time-limit done gate complete
  with the above no-claim boundaries still visible.

## Validation

Run:

```powershell
node scripts/test/app-game-blocking-time-limit-done-gate-proof.mjs
git diff --check
cmd /c npm run lanes:guard
cmd /c npm run hub:guard
```

Proof output:

- `test-results/app-game-blocking-time-limit-done-gate-proof/proof.json`
- `output/app-game-plan-proof/176-app-game-blocking-time-limit-done-gate/proof.json`

## Non-Goals

- No broad installed-app blocking execution.
- No non-scoped platform enforcement.
- No provider delivery or provider receipt ingestion.
- No child-device runtime delivery.
- No raw private source rows, raw target values, or private diagnostics.
- No central product checklist edit unless the lane holding that checklist is
  clear and primary/user asks for consolidation.
