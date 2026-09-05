# WP222 App/Game Android Child Runtime Local Notification Request Queue Proof

> **Current status (2026-08-29): BLOCKED / NO-CHANGE.** Current code overwrites
> fixed queue/drain marker files. It has no request payload/identity, ordering,
> retry, restart-safe state, idempotency, durable receipt, or launched service
> consumer.

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP222 App/Game Android Child Runtime Local Notification Request Queue Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Add Android child-app package-local request queue proof for the app/game
ask-parent path.

This proves the Android child package can turn the local notification request
action into package-local request queue, readback, and drain markers. It does
not claim service request ingestion, parent approval round trip, provider
delivery, platform delivery outside the package, adapter dispatch, broad
blocking, platform enforcement, or raw private source-row custody.

## Implementation

The legal source order is WP207 -> WP216 -> WP217 -> WP220 -> WP221 -> WP222.
Tests against the current fixed marker helper would not exercise a runtime
queue. The historical TypeScript/proof-runner paths below are retired.

- Added
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeNotificationRequestQueueProof.java`.
- Extended
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeNotificationActionReceiver.java`
  so the package-local request action records queue/readback/drain markers.
- Extended
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java`
  with request queue status rendering.
- Added
  `packages/parent-domain/src/app-game-android-child-runtime-local-notification-request-queue-proof.ts`.
- Added
  `packages/parent-domain/tests/app-game-android-child-runtime-local-notification-request-queue-proof.test.ts`.
- Added
  `scripts/test/app-game-android-child-runtime-local-notification-request-queue-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c node --check scripts/test/app-game-android-child-runtime-local-notification-request-queue-proof.mjs
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-child-runtime-local-notification-request-queue-proof
cmd /c node scripts/test/app-game-android-child-runtime-local-notification-request-queue-proof.mjs
```

## Proof

- `test-results/app-game-android-child-runtime-local-notification-request-queue-proof/proof.json`
- `output/app-game-plan-proof/222-app-game-android-child-runtime-local-notification-request-queue-proof/proof.json`
- `output/app-game-plan-proof/222-app-game-android-child-runtime-local-notification-request-queue-proof/00-source-snapshot.md`
- `output/app-game-plan-proof/222-app-game-android-child-runtime-local-notification-request-queue-proof/10-validation-commands.log`

## Boundaries

Proved:

- The Android child package records package-local request queue evidence after
  the notification action path.
- The Android child package records package-local request readback and drain
  markers through internal app storage.
- Parent-domain accepts only action, queue, readback, drain, and opaque proof
  refs while rejecting service ingestion, approval round trip, provider,
  external platform, dispatch, enforcement, and raw private source-row claims.

Not proved:

- Service request ingestion.
- Parent approval round trip.
- Provider notification delivery.
- Platform delivery outside the child package.
- Adapter dispatch, broad blocking, or platform enforcement.
- Raw private source row custody.
