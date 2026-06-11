# WP191 App/Game Linux Foreground Source Preflight

## Scope

Turn Linux WSLg display and socket readiness from WP189 into a foreground source
preflight row.

This proves the display/source preflight shape only. On this Windows/WSL host,
the active-window tool is still missing, so active foreground capture remains
open.

## Implementation

- Added `packages/parent-domain/src/app-game-linux-foreground-source-preflight.ts`.
- Added focused tests for the current WSLg display-ready/tool-missing host,
  hypothetical tool-available preflight readiness, and rejection of raw window
  title, foreground-capture, and enforcement overclaims.
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

- WSLg display and X11/Wayland socket readiness can feed a foreground source
  preflight boundary.
- The current host is blocked on active-window tool availability before
  foreground capture can be claimed.

Not proved:

- Active foreground capture.
- Raw active-window title custody.
- AppArmor, SELinux, package manager, Flatpak, Snap, rollback, audit, launch
  blocking, adapter dispatch, platform enforcement, provider delivery, or
  child-device delivery.
