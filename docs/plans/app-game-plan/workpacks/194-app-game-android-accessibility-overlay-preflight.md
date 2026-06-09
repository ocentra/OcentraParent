# WP194 App/Game Android Accessibility Overlay Preflight

## Scope

Turn physical Android Accessibility settings into a redacted app/game overlay
preflight for warning, block, request, and usage-context overlay actions.

This proves that Accessibility overlay actions have explicit readiness rows and
remain blocked before adapter dispatch until an enabled service, overlay
runtime proof, and child delivery proof exist.

## Implementation

- Added
  `packages/parent-domain/src/app-game-android-accessibility-overlay-preflight.ts`.
- Added focused tests for disabled-service and enabled-service-count states,
  plus rejection of raw service names, overlay execution, adapter dispatch, and
  platform enforcement overclaims.
- Added
  `scripts/test/app-game-android-accessibility-overlay-preflight-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-accessibility-overlay-preflight
cmd /c node scripts/test/app-game-android-accessibility-overlay-preflight-proof.mjs
```

## Proof

- `test-results/app-game-android-accessibility-overlay-preflight-proof/proof.json`
- `output/app-game-plan-proof/194-app-game-android-accessibility-overlay-preflight/proof.json`

## Boundaries

Proved:

- Android Accessibility overlay actions have explicit preflight rows.
- The physical Android target can provide redacted Accessibility settings
  evidence without storing service/component names.
- Overlay actions stay blocked before adapter dispatch until service enablement,
  overlay runtime, and child delivery proof exist.

Not proved:

- Ocentra Accessibility service implementation or enablement.
- Warning, blocking, request, or usage-context overlay execution.
- Adapter dispatch, platform enforcement, provider delivery, child-device
  delivery, raw Accessibility service/component names, or raw overlay content.
