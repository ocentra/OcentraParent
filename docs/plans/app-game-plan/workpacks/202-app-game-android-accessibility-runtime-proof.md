# WP202 App/Game Android Accessibility Runtime Proof

## Scope

Move Android app/game warning, block, request, and usage-context overlay support
from settings-only preflight into a real package-declared Accessibility runtime
boundary.

This proves that the Android debug package declares an Ocentra
AccessibilityService, exposes declaration/runtime/sample state through
MainActivity, and keeps overlay execution blocked until service enablement,
runtime event sample proof, child-device delivery, and platform enforcement
proof exist.

## Implementation

- Added
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidAccessibilityRuntimeService.java`.
- Added
  `platforms/android/agent/app/src/main/res/xml/app_game_accessibility_service.xml`.
- Updated `platforms/android/agent/app/src/main/AndroidManifest.xml` to declare
  the service with `android.permission.BIND_ACCESSIBILITY_SERVICE`.
- Updated `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java`
  to surface parent-safe Accessibility declaration, runtime, and event-sample
  states.
- Added
  `packages/parent-domain/src/app-game-android-accessibility-runtime-proof.ts`.
- Added
  `packages/parent-domain/tests/app-game-android-accessibility-runtime-proof.test.ts`.
- Added `scripts/test/app-game-android-accessibility-runtime-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-accessibility-runtime-proof
cmd /c node scripts/test/app-game-android-accessibility-runtime-proof.mjs
```

## Proof

- `test-results/app-game-android-accessibility-runtime-proof/proof.json`
- `output/app-game-plan-proof/202-app-game-android-accessibility-runtime-proof/proof.json`

## Boundaries

Proved:

- Android package declares an Ocentra AccessibilityService.
- The service is bound by the Android Accessibility service permission.
- The service config listens only for window state changes and does not request
  window-content retrieval.
- MainActivity exposes parent-safe Accessibility declaration/runtime/sample
  states after package install and launch.
- Parent-domain proof rejects raw event rows, raw service/component names, raw
  overlay content, adapter dispatch, platform enforcement, and child-device
  delivery claims.

Not proved:

- Accessibility service enablement if Android settings do not include the
  package-local service.
- Runtime event sample observation if the service is not bound or no event was
  observed.
- Warning, block, request, or usage-context overlay execution.
- Adapter dispatch, platform enforcement, provider delivery, child-device
  delivery, Device Owner/Profile Owner authority, or Play policy proof.
