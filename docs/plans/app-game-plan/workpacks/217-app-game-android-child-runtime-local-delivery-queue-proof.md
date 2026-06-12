# WP217 App/Game Android Child Runtime Local Delivery Queue Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP217 App/Game Android Child Runtime Local Delivery Queue Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Add Android child-app package-local delivery queue and drain proof for the
app/game child runtime delivery path.

This proves the Android child package can record package-local delivery intake,
queue, and drain markers while preserving package-local receipt channel,
receipt, and receipt-ack custody. It does not claim service delivery or receipt
ingestion, provider delivery, platform delivery outside the package, adapter
dispatch, platform enforcement, or raw private row custody.

## Implementation

- Extended
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeDeliveryProof.java`
  with package-local delivery queue and drain states.
- Added
  `packages/parent-domain/src/app-game-android-child-runtime-local-delivery-queue-proof.ts`.
- Added
  `packages/parent-domain/tests/app-game-android-child-runtime-local-delivery-queue-proof.test.ts`.
- Added
  `scripts/test/app-game-android-child-runtime-local-delivery-queue-proof.mjs`.
- Added
  `scripts/test/app-game-android-child-runtime-local-delivery-queue-physical-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c node --check scripts/test/app-game-android-child-runtime-local-delivery-queue-proof.mjs
cmd /c node --check scripts/test/app-game-android-child-runtime-local-delivery-queue-physical-proof.mjs
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-child-runtime-local-delivery-queue-proof
cmd /c node scripts/test/app-game-android-child-runtime-local-delivery-queue-proof.mjs
cmd /c node scripts/test/app-game-android-child-runtime-local-delivery-queue-physical-proof.mjs
```

## Proof

- `test-results/app-game-android-child-runtime-local-delivery-queue-proof/proof.json`
- `test-results/app-game-android-child-runtime-local-delivery-queue-physical-proof/proof.json`
- `output/app-game-plan-proof/217-app-game-android-child-runtime-local-delivery-queue-proof/proof.json`
- `output/app-game-plan-proof/217-app-game-android-child-runtime-local-delivery-queue-proof/physical-proof.json`
- `output/app-game-plan-proof/217-app-game-android-child-runtime-local-delivery-queue-proof/00-source-snapshot.md`
- `output/app-game-plan-proof/217-app-game-android-child-runtime-local-delivery-queue-proof/01-physical-android-delivery-queue-snapshot.md`
- `output/app-game-plan-proof/217-app-game-android-child-runtime-local-delivery-queue-proof/10-validation-commands.log`
- `output/app-game-plan-proof/217-app-game-android-child-runtime-local-delivery-queue-proof/11-physical-validation-commands.log`

## Boundaries

Proved:

- The Android child app package compiles with package-local delivery queue and
  drain marker custody.
- The physical Samsung Galaxy S9 proof target installs and launches the debug
  APK and exposes delivery intake, delivery queue, delivery drain, channel,
  receipt, and ack internal marker files through debug `run-as`.
- Parent-domain accepts the proof only when queue and drain evidence stay
  package-local and external delivery claims stay false.

Not proved:

- Service delivery or receipt ingestion.
- Provider delivery execution.
- Platform delivery channel execution outside the child package.
- Adapter dispatch or platform enforcement.
- Raw private source row custody.
