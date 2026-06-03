# 11 Cross-Platform Authority Matrix

## Target State

Windows, macOS, Linux, Android, and iOS/iPadOS capabilities are represented by
authority tier, setup state, proof state, and parent-visible limitation.

## Scope

- Observe-only, user-approved-helper, accessibility-assisted, managed-profile,
  device-owner, MDM-enrolled, supervised-device, system-extension,
  root/admin-service, kiosk/single-app, manual-required, and not-claimed states.
- Platform action matrix for inventory, runtime, foreground, warn, ask,
  time-limit, terminate, hide/suspend/shield, block launch, and allowlist.

## Tests And Proof

- Observe-only cannot block.
- Manual-required cannot execute.
- Android hide/suspend requires Device Owner/Profile Owner proof.
- iOS shield requires FamilyControls/ManagedSettings proof.
- macOS hard block requires MDM/Endpoint/System Extension proof.
- Linux block names mechanism and distro/session proof.

## Done Signal

Every app/game action shows what platform authority exists and what proof is
needed to move up.

Use the standard checklist in [workpacks README](README.md).

## Completion Note - 2026-06-03

Completed as contract/test proof on `codex/app-game-authority-matrix`:

- Added `packages/parent-domain/src/app-game-control-platform-authority.ts`.
- Added `packages/parent-domain/src/app-game-control-platform-authority-rules.ts`.
- Added `packages/parent-domain/tests/app-game-control-platform-authority.test.ts`.
- Re-exported the matrix from the existing app/game authority entrypoint.
- Proof root:
  `output/app-game-plan-proof/11-cross-platform-authority-matrix/`.

The done signal is satisfied at the contract/proof-gate layer only. Runtime
platform adapters, enrollment, rollback execution, cleanup execution, portal
rows, and product status remain later work.
