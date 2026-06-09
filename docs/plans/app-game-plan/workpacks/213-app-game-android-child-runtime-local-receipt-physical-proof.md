# WP213 App/Game Android Child Runtime Local Receipt Physical Proof

## Scope

Run the Android child runtime local receipt proof on the physical Samsung Galaxy
S9 target available over Wi-Fi ADB.

This verifies that the debug Android package launches on physical hardware,
attempts to capture the package-local receipt append/readback UI state, and
writes the parent-safe internal receipt marker.

## Implementation

- Added
  `scripts/test/app-game-android-child-runtime-local-receipt-physical-proof.mjs`.
- The proof runner uses explicit `adb -s 192.168.2.45:5555` targeting.
- The runner installs the Android debug package, launches `MainActivity`,
  attempts a UIAutomator dump, and reads the debug package internal receipt
  marker with `run-as`. UIAutomator can be unavailable on the proof phone; the
  hard physical proof is launch plus internal receipt readback.

## Validation

Focused validation for this workpack:

```powershell
cmd /c node --check scripts/test/app-game-android-child-runtime-local-receipt-physical-proof.mjs
cmd /c node scripts/test/app-game-android-child-runtime-local-receipt-physical-proof.mjs
```

## Proof

- `test-results/app-game-android-child-runtime-local-receipt-physical-proof/proof.json`
- `test-results/app-game-android-child-runtime-local-receipt-physical-proof/ui.xml`
- `output/app-game-plan-proof/213-app-game-android-child-runtime-local-receipt-physical-proof/proof.json`
- `output/app-game-plan-proof/213-app-game-android-child-runtime-local-receipt-physical-proof/00-physical-android-ui-snapshot.md`
- `output/app-game-plan-proof/213-app-game-android-child-runtime-local-receipt-physical-proof/10-validation-commands.log`

## Boundaries

Proved:

- The Android debug package installs and launches on the physical Samsung
  Galaxy S9 target.
- The debug package internal receipt marker exists and is readable through
  `run-as` for proof.

Not proved:

- Guaranteed physical UIAutomator visibility of the rendered receipt text.
- Service receipt ingestion.
- Provider delivery execution.
- Platform delivery channel execution.
- Adapter dispatch or platform enforcement.
- Raw private source row custody.
