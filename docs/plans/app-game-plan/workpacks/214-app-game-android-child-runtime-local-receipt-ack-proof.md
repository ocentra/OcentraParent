# WP214 App/Game Android Child Runtime Local Receipt Ack Proof

## Scope

Add Android child-app package-local receipt acknowledgement proof for the
app/game child runtime receipt path.

This proves the Android child package can write/read a local receipt marker and
a local receipt-ack marker in its internal app files. It does not claim child
runtime transport execution, service receipt ingestion, provider delivery,
platform delivery channel execution, adapter dispatch, or platform enforcement.

## Implementation

- Extended
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeTransportReceiptProof.java`
  with local receipt-ack write/readback states.
- Updated Android `MainActivity` to render local receipt-ack and ack-readback
  states.
- Added
  `packages/parent-domain/src/app-game-android-child-runtime-local-receipt-ack-proof.ts`.
- Added
  `packages/parent-domain/tests/app-game-android-child-runtime-local-receipt-ack-proof.test.ts`.
- Added `scripts/test/app-game-android-child-runtime-local-receipt-ack-proof.mjs`.
- Added
  `scripts/test/app-game-android-child-runtime-local-receipt-ack-physical-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c node --check scripts/test/app-game-android-child-runtime-local-receipt-ack-proof.mjs
cmd /c node --check scripts/test/app-game-android-child-runtime-local-receipt-ack-physical-proof.mjs
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-child-runtime-local-receipt-ack-proof
cmd /c node scripts/test/app-game-android-child-runtime-local-receipt-ack-proof.mjs
cmd /c node scripts/test/app-game-android-child-runtime-local-receipt-ack-physical-proof.mjs
```

## Proof

- `test-results/app-game-android-child-runtime-local-receipt-ack-proof/proof.json`
- `test-results/app-game-android-child-runtime-local-receipt-ack-physical-proof/proof.json`
- `output/app-game-plan-proof/214-app-game-android-child-runtime-local-receipt-ack-proof/proof.json`
- `output/app-game-plan-proof/214-app-game-android-child-runtime-local-receipt-ack-proof/physical-proof.json`
- `output/app-game-plan-proof/214-app-game-android-child-runtime-local-receipt-ack-proof/00-source-snapshot.md`
- `output/app-game-plan-proof/214-app-game-android-child-runtime-local-receipt-ack-proof/01-physical-android-ack-snapshot.md`
- `output/app-game-plan-proof/214-app-game-android-child-runtime-local-receipt-ack-proof/10-validation-commands.log`
- `output/app-game-plan-proof/214-app-game-android-child-runtime-local-receipt-ack-proof/11-physical-validation-commands.log`

## Boundaries

Proved:

- The Android child app package compiles with package-local receipt and ack
  write/readback code.
- The Android activity renders parent-safe receipt ack and ack-readback states.
- Parent-domain accepts the proof only when package-local receipt and ack
  write/readback evidence is present.
- The physical Samsung Galaxy S9 proof target installs and launches the debug
  APK and exposes both internal marker files through debug `run-as`.

Not proved:

- Physical child runtime transport execution.
- Service receipt ingestion.
- Provider delivery execution.
- Platform delivery channel execution.
- Adapter dispatch or platform enforcement.
- Raw private source row custody.
