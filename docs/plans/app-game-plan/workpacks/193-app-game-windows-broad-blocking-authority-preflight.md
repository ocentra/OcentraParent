# WP193 App/Game Windows Broad Blocking Authority Preflight

## Scope

Turn the existing Windows broad-blocking manual gates into a machine-readable
authority preflight for AppLocker/App Control launch-blocking work.

This proves that Windows host visibility is available for app/game broad
blocking review, while broad installed-app launch blocking remains blocked
before adapter dispatch until AppLocker/App Control enforce proof, system-app
allowlist proof, rollback proof, and audit custody proof exist.

## Implementation

- Added
  `packages/parent-domain/src/app-game-windows-broad-blocking-authority-preflight.ts`.
- Added focused tests for the current host-visible/policy-proof-missing state,
  source gate refs, required proof refs, and rejection of raw policy, broad
  blocking, dispatch, and fake proof upgrades.
- Added
  `scripts/test/app-game-windows-broad-blocking-authority-preflight-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-windows-broad-blocking-authority-preflight
cmd /c node scripts/test/app-game-windows-broad-blocking-authority-preflight-proof.mjs
```

## Proof

- `test-results/app-game-windows-broad-blocking-authority-preflight-proof/proof.json`
- `output/app-game-plan-proof/193-app-game-windows-broad-blocking-authority-preflight/proof.json`

## Boundaries

Proved:

- Windows broad launch blocking has explicit authority preflight rows over the
  existing AppLocker/App Control manual gates.
- Windows host probe visibility is attached through a parent-safe opaque ref.
- AppLocker/App Control enforce proof, system-app allowlist proof, rollback
  proof, and audit custody proof remain required before adapter dispatch.

Not proved:

- Windows AppLocker enforce policy application.
- Windows App Control policy application.
- Windows system-app allowlist execution.
- Windows broad installed-app launch blocking.
- Adapter dispatch, platform enforcement, provider delivery, child-device
  delivery, raw executable paths, or raw policy XML custody.
