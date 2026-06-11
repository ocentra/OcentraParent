# WP212 App/Game Android Child Runtime Local Receipt Proof

## Scope

Add Android child-app package-local receipt append and readback proof for the
app/game child runtime receipt path.

This moves the Android child package beyond store availability by proving that
it can write and read back a parent-safe internal receipt marker, while still
not claiming service ingestion, provider delivery, platform delivery channel
execution, adapter dispatch, or platform enforcement.

## Implementation

- Extended
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeTransportReceiptProof.java`
  with internal receipt append/readback proof states.
- Updated Android `MainActivity` to render local receipt append and readback
  states.
- Added
  `packages/parent-domain/src/app-game-android-child-runtime-local-receipt-proof.ts`.
- Added
  `packages/parent-domain/tests/app-game-android-child-runtime-local-receipt-proof.test.ts`.
- Added `scripts/test/app-game-android-child-runtime-local-receipt-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c node --check scripts/test/app-game-android-child-runtime-local-receipt-proof.mjs
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-child-runtime-local-receipt-proof
cmd /c node scripts/test/app-game-android-child-runtime-local-receipt-proof.mjs
```

## Proof

- `test-results/app-game-android-child-runtime-local-receipt-proof/proof.json`
- `output/app-game-plan-proof/212-app-game-android-child-runtime-local-receipt-proof/proof.json`
- `output/app-game-plan-proof/212-app-game-android-child-runtime-local-receipt-proof/00-source-snapshot.md`
- `output/app-game-plan-proof/212-app-game-android-child-runtime-local-receipt-proof/10-validation-commands.log`

## Boundaries

Proved:

- The Android child app package compiles with package-local internal receipt
  append and readback code.
- The Android activity renders parent-safe receipt append and readback states.
- Parent-domain accepts the proof only when package-local write/readback and UI
  evidence are present.

Not proved:

- Physical child runtime transport execution.
- Service receipt ingestion.
- Provider delivery execution.
- Platform delivery channel execution.
- Adapter dispatch or platform enforcement.
- Raw private source row custody.
