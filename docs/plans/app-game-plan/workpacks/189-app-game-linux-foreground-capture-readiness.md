# WP189 App/Game Linux Foreground Capture Readiness

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP189 App/Game Linux Foreground Capture Readiness`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

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
