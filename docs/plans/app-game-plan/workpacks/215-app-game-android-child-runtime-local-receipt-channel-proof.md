# WP215 App/Game Android Child Runtime Local Receipt Channel Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP215 App/Game Android Child Runtime Local Receipt Channel Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Add Android child-app package-local receipt channel proof for the app/game child
runtime receipt path.

This proves the Android child package can trigger a non-exported in-package
broadcast receiver from `MainActivity`, record a package-local receipt channel
marker, and keep the existing local receipt and local receipt-ack markers in
internal app files. It does not claim service receipt ingestion, provider
delivery, platform delivery outside the package, adapter dispatch, platform
enforcement, or raw private row custody.

## Implementation

- Added
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeReceiptReceiver.java`.
- Extended
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeTransportReceiptProof.java`
  with package-local receipt channel state and marker writes.
- Updated Android `MainActivity` to trigger the package-local receipt channel
  proof through an explicit extra and render the channel state.
- Added the non-exported receiver declaration in
  `platforms/android/agent/app/src/main/AndroidManifest.xml`.
- Added
  `packages/parent-domain/src/app-game-android-child-runtime-local-receipt-channel-proof.ts`.
- Added
  `packages/parent-domain/tests/app-game-android-child-runtime-local-receipt-channel-proof.test.ts`.
- Added
  `scripts/test/app-game-android-child-runtime-local-receipt-channel-proof.mjs`.
- Added
  `scripts/test/app-game-android-child-runtime-local-receipt-channel-physical-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c node --check scripts/test/app-game-android-child-runtime-local-receipt-channel-proof.mjs
cmd /c node --check scripts/test/app-game-android-child-runtime-local-receipt-channel-physical-proof.mjs
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-child-runtime-local-receipt-channel-proof
cmd /c node scripts/test/app-game-android-child-runtime-local-receipt-channel-proof.mjs
cmd /c node scripts/test/app-game-android-child-runtime-local-receipt-channel-physical-proof.mjs
```

## Proof

- `test-results/app-game-android-child-runtime-local-receipt-channel-proof/proof.json`
- `test-results/app-game-android-child-runtime-local-receipt-channel-physical-proof/proof.json`
- `output/app-game-plan-proof/215-app-game-android-child-runtime-local-receipt-channel-proof/proof.json`
- `output/app-game-plan-proof/215-app-game-android-child-runtime-local-receipt-channel-proof/physical-proof.json`
- `output/app-game-plan-proof/215-app-game-android-child-runtime-local-receipt-channel-proof/00-source-snapshot.md`
- `output/app-game-plan-proof/215-app-game-android-child-runtime-local-receipt-channel-proof/01-physical-android-channel-snapshot.md`
- `output/app-game-plan-proof/215-app-game-android-child-runtime-local-receipt-channel-proof/10-validation-commands.log`
- `output/app-game-plan-proof/215-app-game-android-child-runtime-local-receipt-channel-proof/11-physical-validation-commands.log`

## Boundaries

Proved:

- The Android child app package compiles with a non-exported package-local
  receipt channel receiver.
- The Android activity can trigger the in-package receipt channel proof without
  exposing an external broadcast receiver.
- Parent-domain accepts the proof only when receipt, ack, receiver, and
  activity-trigger evidence is present.
- The physical Samsung Galaxy S9 proof target installs and launches the debug
  APK and exposes the channel, receipt, and ack internal marker files through
  debug `run-as`.

Not proved:

- Service receipt ingestion.
- Provider delivery execution.
- Platform delivery channel execution outside the child package.
- Adapter dispatch or platform enforcement.
- Raw private source row custody.
