# WP192 App/Game Android Authority Preflight

## Scope

Turn the physical Android policy-state proof into a machine-readable authority
preflight for package policy actions.

This proves that Android hide, suspend, uninstall-block, lock-task, and managed
configuration actions are explicitly blocked before adapter dispatch on the
current physical phone because Device Owner/Profile Owner proof is absent.

## Implementation

- Added `packages/parent-domain/src/app-game-android-authority-preflight.ts`.
- Added focused tests for the current not-enrolled physical device,
  `not-proved` owner-state handling, and rejection of raw data, dispatch, and
  enforcement overclaims.
- Added `scripts/test/app-game-android-authority-preflight-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-authority-preflight
cmd /c node scripts/test/app-game-android-authority-preflight-proof.mjs
```

## Proof

- `test-results/app-game-android-authority-preflight-proof/proof.json`
- `output/app-game-plan-proof/192-app-game-android-authority-preflight/proof.json`

## Boundaries

Proved:

- Android package policy actions have explicit authority preflight rows.
- The current physical Android target remains blocked before adapter dispatch
  because Device Owner/Profile Owner proof is absent.
- `not-proved` policy states do not count as owner proof.

Not proved:

- Device Owner/Profile Owner enrollment.
- Android hide, suspend, uninstall block, lock task, or managed configuration
  execution.
- Adapter dispatch, platform enforcement, provider delivery, child-device
  delivery, raw package names, or raw device serial custody.
