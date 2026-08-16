# WP191 App/Game Linux Foreground Source Preflight

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP191 App/Game Linux Foreground Source Preflight`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

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
