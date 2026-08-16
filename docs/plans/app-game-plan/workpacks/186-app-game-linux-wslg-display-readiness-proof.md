# WP186 App/Game Linux WSLg Display Readiness Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP186 App/Game Linux WSLg Display Readiness Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Extend the Linux WSL runtime proof with parent-safe WSLg display readiness.

This proves the Windows host can observe WSLg display infrastructure for Linux
app/game runtime work: the WSLg runtime directory, X11 socket, and Wayland
socket. It does not claim active-window foreground capture, Linux broad
blocking, platform enforcement, rollback, audit, or adapter dispatch.

## Implementation

- Extended `packages/parent-domain/src/app-game-linux-wsl-runtime-proof.ts`
  with display state, X11 socket state, Wayland socket state, active-window
  probe state, and explicit foreground-capture non-claim.
- Updated `packages/parent-domain/src/app-game-platform-proof-status.ts` so
  Linux rows keep a `linux-foreground-capture-not-proved` gap until a real
  active-window foreground capture source is attached.
- Updated `scripts/test/app-game-linux-wsl-runtime-proof.mjs` to collect WSLg,
  X11, and Wayland socket evidence from WSL without storing raw process/package
  rows or host paths.

## Validation

Focused validation for this workpack:

```powershell
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-linux-wsl-runtime-proof app-game-platform-proof-status
cmd /c node --check scripts/test/app-game-linux-wsl-runtime-proof.mjs
node scripts/test/app-game-linux-wsl-runtime-proof.mjs
```

## Proof

- `test-results/app-game-linux-wsl-runtime-proof/proof.json`
- `output/app-game-plan-proof/182-app-game-linux-wsl-runtime-proof/proof.json`
- `output/app-game-plan-proof/182-app-game-linux-wsl-runtime-proof/09-manual-platform-proof.md`

## Boundaries

Proved:

- WSL2 Ubuntu runtime remains reachable from the Windows host.
- WSLg display infrastructure is present.
- X11 socket `/tmp/.X11-unix/X0` is present.
- Wayland socket `/mnt/wslg/runtime-dir/wayland-0` is present.
- The proof stores parent-safe states and counts only.

Not proved:

- Active Linux foreground-window capture.
- X11 or Wayland app/game title/process mapping.
- AppArmor, SELinux, package-manager, Flatpak, or Snap restriction behavior.
- Launch blocking, rollback, or audit behavior.
- Adapter dispatch, broad installed-app blocking, platform enforcement,
  provider delivery, child-device delivery, raw private rows/targets, or
  private diagnostics.
