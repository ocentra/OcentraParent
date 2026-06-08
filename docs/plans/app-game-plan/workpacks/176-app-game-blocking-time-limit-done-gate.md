# WP176: App/Game Blocking Time-Limit Done Gate

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
