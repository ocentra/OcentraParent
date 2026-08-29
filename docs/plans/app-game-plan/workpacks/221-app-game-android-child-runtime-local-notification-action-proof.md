# WP221 App/Game Android Child Runtime Local Notification Action Proof

> **Current status (2026-08-29): BLOCKED / NO-CHANGE.** The non-exported
> receiver accepts a static action and writes fixed marker/queue records; it
> has no owner-issued action identity, target, generation/currentness,
> replay/idempotency, or durable receipt. The live child runtime never produces
> or consumes the notification.

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP221 App/Game Android Child Runtime Local Notification Action Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Add Android child-app package-local notification action proof for the app/game
ask-parent path.

This proves the Android child package can attach a local request action to the
app/game notification, route that action to a non-exported in-package receiver,
and write a package-local request marker for debug readback. It does not claim
provider delivery, platform delivery outside the package, service request
ingestion, parent approval round trip, adapter dispatch, broad blocking,
platform enforcement, or raw private source-row custody.

## Implementation

Source order is WP207 -> WP216 -> WP217 -> WP220 -> WP221. The current helper's
direct trigger bypasses actual receiver execution, so a focused helper test
would certify static-marker behavior rather than a request action. Historical
TypeScript/proof-runner paths below are retired.

- Extended
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeLocalNotificationProof.java`
  with a local notification request action and proof state.
- Added
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidChildRuntimeNotificationActionReceiver.java`.
- Registered the non-exported receiver in
  `platforms/android/agent/app/src/main/AndroidManifest.xml`.
- Extended
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java`
  with local notification action proof trigger and status rendering.
- Added the child notification action label in
  `platforms/android/agent/app/src/main/res/values/strings.xml`.
- Added
  `packages/parent-domain/src/app-game-android-child-runtime-local-notification-action-proof.ts`.
- Added
  `packages/parent-domain/tests/app-game-android-child-runtime-local-notification-action-proof.test.ts`.
- Added
  `scripts/test/app-game-android-child-runtime-local-notification-action-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c node --check scripts/test/app-game-android-child-runtime-local-notification-action-proof.mjs
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-child-runtime-local-notification-action-proof
cmd /c node scripts/test/app-game-android-child-runtime-local-notification-action-proof.mjs
```

## Proof

- `test-results/app-game-android-child-runtime-local-notification-action-proof/proof.json`
- `output/app-game-plan-proof/221-app-game-android-child-runtime-local-notification-action-proof/proof.json`
- `output/app-game-plan-proof/221-app-game-android-child-runtime-local-notification-action-proof/00-source-snapshot.md`
- `output/app-game-plan-proof/221-app-game-android-child-runtime-local-notification-action-proof/10-validation-commands.log`

## Boundaries

Proved:

- The Android child package exposes a package-local ask-parent action on the
  app/game local notification.
- The Android child package records and reads back a package-local request
  action marker.
- Parent-domain accepts only channel, post, action, marker readback, and opaque
  proof refs while rejecting service ingestion, approval round trip, provider,
  external platform, dispatch, enforcement, and raw private source-row claims.

Not proved:

- Provider notification delivery.
- Platform delivery outside the child package.
- Service request ingestion.
- Parent approval round trip.
- Adapter dispatch, broad blocking, or platform enforcement.
- Raw private source row custody.
