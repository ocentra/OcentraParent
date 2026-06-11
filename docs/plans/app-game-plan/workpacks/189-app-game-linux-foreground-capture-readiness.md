# WP189 App/Game Linux Foreground Capture Readiness

## Scope

Turn WSLg display and X11/Wayland socket proof into a parent-domain foreground
capture readiness row.

This does not prove active foreground capture. It records that the display layer
is ready on this Windows/WSL host and that an active-window capture tool/source
is still missing.

## Implementation

- Added `packages/parent-domain/src/app-game-linux-foreground-capture-readiness.ts`.
- Added focused tests for WSLg display-ready/capture-tool-missing readiness,
  display-not-ready fallback, and rejection of raw window title or enforcement
  claim upgrades.
- Updated parent-domain platform proof status so an attached Linux foreground
  readiness row adds `linux-foreground-capture-readiness-ref` while keeping
  `linux-foreground-capture-not-proved` open.
- Added `scripts/test/app-game-linux-foreground-capture-readiness-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-linux-foreground-capture-readiness app-game-platform-proof-status
cmd /c node scripts/test/app-game-linux-foreground-capture-readiness-proof.mjs
```

## Proof

- `test-results/app-game-linux-foreground-capture-readiness-proof/proof.json`
- `output/app-game-plan-proof/189-app-game-linux-foreground-capture-readiness/proof.json`

## Boundaries

Proved:

- WSLg display and X11/Wayland socket proof can feed a parent-safe foreground
  capture readiness row.
- The Linux platform proof status row can carry a foreground readiness ref.

Not proved:

- Active foreground capture.
- Raw active-window title custody.
- AppArmor, SELinux, package manager, Flatpak, Snap, rollback, audit, launch
  blocking, adapter dispatch, platform enforcement, provider delivery, or
  child-device delivery.
