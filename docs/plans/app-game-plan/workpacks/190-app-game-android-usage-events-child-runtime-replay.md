# WP190 App/Game Android UsageEvents Child Runtime Replay

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
