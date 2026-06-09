# WP182 - App/Game Linux WSL Runtime Proof

## Scope

Add a typed Linux WSL runtime proof boundary for the unified app/game control
plan. This workpack uses local WSL evidence from Windows to prove Linux runtime
facts without promoting broad Linux app/game blocking.

This workpack proves:

- WSL2 Ubuntu runtime is reachable from the Windows host;
- Linux kernel, architecture, distro id/version, package manager, process
  snapshot, and session state can be observed;
- WSLg display readiness plus X11 and Wayland socket states can be observed;
- Docker CLI visibility is recorded separately;
- package names, process names, and raw distro names are redacted;
- Linux adapter dispatch, broad blocking, and platform enforcement remain
  unclaimed until mechanism, distro, session, rollback, and audit proof are all
  attached.

## Touched Areas

- `packages/parent-domain/src/app-game-linux-wsl-runtime-proof.ts`
- `packages/parent-domain/tests/app-game-linux-wsl-runtime-proof.test.ts`
- `scripts/test/app-game-linux-wsl-runtime-proof.mjs`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/README.md`

## Evidence

Proof root:

```text
output/app-game-plan-proof/182-app-game-linux-wsl-runtime-proof/
```

Runtime proof output:

```text
test-results/app-game-linux-wsl-runtime-proof/proof.json
```

The proof harness uses:

```text
wsl.exe -d Ubuntu-22.04 -- sh -lc ...
```

The generated proof stores only parent-safe derived facts and counts. It does
not store package lists, process lists, raw distro names, or private host paths.
It also stores WSLg/X11/Wayland display readiness as parent-safe states only,
not raw desktop rows or foreground window titles.

## Non-Claims

- No Linux broad app/game blocking is claimed.
- No active-window foreground capture, AppArmor/SELinux, package-manager
  restriction, Flatpak/Snap restriction, launch-blocking adapter, rollback, or
  audit behavior is claimed.
- No Docker-backed enforcement is claimed when Docker CLI is unavailable.
- No adapter dispatch or platform enforcement is claimed.

## Validation

Focused validation for this workpack:

```powershell
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-linux-wsl-runtime-proof app-game-broad-blocking-proof-gates
cmd /c "node --check scripts/test/app-game-linux-wsl-runtime-proof.mjs && node scripts/test/app-game-linux-wsl-runtime-proof.mjs"
```
