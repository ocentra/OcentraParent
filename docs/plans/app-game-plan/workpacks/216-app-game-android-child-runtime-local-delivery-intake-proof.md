# WP216 App/Game Android Child Runtime Local Delivery Intake Proof

> **Current status (2026-08-29): BLOCKED / NO-CHANGE.** Existing Java code
> validates only a static broadcast action and writes package-local markers.
> It has no authenticated target/intent, generation/currentness, replay,
> idempotency, or durable service consumer.

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP216 App/Game Android Child Runtime Local Delivery Intake Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Add Android child-app package-local delivery intake proof for the app/game child
runtime delivery path.

This proves the Android child package can trigger a non-exported in-package
delivery receiver from `MainActivity`, record a package-local delivery intake
marker, and tie that local intake to package-local receipt channel, receipt,
and receipt-ack markers. It does not claim service delivery or receipt
ingestion, provider delivery, platform delivery outside the package, adapter
dispatch, platform enforcement, or raw private row custody.

## Implementation

The current launched child service has no delivery ingress and legacy
`MainActivity` is undeclared. The legal source order is WP207 durable writer,
WP208 typed transport/receipt boundary, then WP209 authenticated service
ingestion. Tests before that seam would certify marker theater, not delivery.

- Added
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeDeliveryProof.java`.
- Added
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeDeliveryReceiver.java`.
- Updated Android `MainActivity` to trigger package-local delivery intake proof
  through an explicit extra and render delivery intake/readback state.
- Added the non-exported delivery receiver declaration in
  `platforms/android/agent/app/src/main/AndroidManifest.xml`.
- Added
  `packages/parent-domain/src/app-game-android-child-runtime-local-delivery-intake-proof.ts`.
- Added
  `packages/parent-domain/tests/app-game-android-child-runtime-local-delivery-intake-proof.test.ts`.
- Added
  `scripts/test/app-game-android-child-runtime-local-delivery-intake-proof.mjs`.
- Added
  `scripts/test/app-game-android-child-runtime-local-delivery-intake-physical-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c node --check scripts/test/app-game-android-child-runtime-local-delivery-intake-proof.mjs
cmd /c node --check scripts/test/app-game-android-child-runtime-local-delivery-intake-physical-proof.mjs
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-child-runtime-local-delivery-intake-proof
cmd /c node scripts/test/app-game-android-child-runtime-local-delivery-intake-proof.mjs
cmd /c node scripts/test/app-game-android-child-runtime-local-delivery-intake-physical-proof.mjs
```

## Proof

- `test-results/app-game-android-child-runtime-local-delivery-intake-proof/proof.json`
- `test-results/app-game-android-child-runtime-local-delivery-intake-physical-proof/proof.json`
- `output/app-game-plan-proof/216-app-game-android-child-runtime-local-delivery-intake-proof/proof.json`
- `output/app-game-plan-proof/216-app-game-android-child-runtime-local-delivery-intake-proof/physical-proof.json`
- `output/app-game-plan-proof/216-app-game-android-child-runtime-local-delivery-intake-proof/00-source-snapshot.md`
- `output/app-game-plan-proof/216-app-game-android-child-runtime-local-delivery-intake-proof/01-physical-android-delivery-snapshot.md`
- `output/app-game-plan-proof/216-app-game-android-child-runtime-local-delivery-intake-proof/10-validation-commands.log`
- `output/app-game-plan-proof/216-app-game-android-child-runtime-local-delivery-intake-proof/11-physical-validation-commands.log`

## Boundaries

Proved:

- The Android child app package compiles with a non-exported package-local
  delivery intake receiver.
- The Android activity can trigger the in-package delivery proof without
  exposing an external broadcast receiver.
- The delivery intake path records local delivery, receipt channel, receipt,
  and receipt-ack marker custody.
- The physical Samsung Galaxy S9 proof target installs and launches the debug
  APK and exposes the delivery, channel, receipt, and ack internal marker files
  through debug `run-as`.

Not proved:

- Service delivery or receipt ingestion.
- Provider delivery execution.
- Platform delivery channel execution outside the child package.
- Adapter dispatch or platform enforcement.
- Raw private source row custody.
