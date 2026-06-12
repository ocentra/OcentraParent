# WP213 App/Game Android Child Runtime Local Receipt Physical Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP213 App/Game Android Child Runtime Local Receipt Physical Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

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
