# WP219 App/Game Android Accessibility Enablement Sample Boundary

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP219 App/Game Android Accessibility Enablement Sample Boundary`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Add the Android child-agent event-count surface and success contract for a
settings-enabled AccessibilityService, then run the physical target proof
honestly: emit the enabled-sample read model only if the service binds and
reports a count-only window-state event sample; otherwise emit a manual-required
blocker proof for the target.

This moves the Android Accessibility proof beyond package declaration and
waiting-for-enablement in code shape, without faking device support. It does
not claim warning/block/request overlays, overlay content capture, provider
delivery, child-device delivery outside the package, adapter dispatch, broad
blocking, platform enforcement, Device Owner/Profile Owner authority, or Play
policy proof.

## Implementation

- Extended
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidAccessibilityRuntimeService.java`
  with count-only event sample output.
- Extended
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java`
  so the child app activity renders `accessibility-event-sample-count`.
- Added
  `packages/parent-domain/src/app-game-android-accessibility-enabled-sample-proof.ts`.
- Added
  `packages/parent-domain/tests/app-game-android-accessibility-enabled-sample-proof.test.ts`.
- Added
  `scripts/test/app-game-android-accessibility-enabled-sample-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c node --check scripts/test/app-game-android-accessibility-enabled-sample-proof.mjs
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-accessibility-enabled-sample-proof
cmd /c node scripts/test/app-game-android-accessibility-enabled-sample-proof.mjs
```

## Proof

- `test-results/app-game-android-accessibility-enabled-sample-proof/proof.json`
- `output/app-game-plan-proof/219-app-game-android-accessibility-enabled-sample-proof/proof.json`
- `output/app-game-plan-proof/219-app-game-android-accessibility-enabled-sample-proof/00-source-snapshot.md`
- `output/app-game-plan-proof/219-app-game-android-accessibility-enabled-sample-proof/10-validation-commands.log`

## Boundaries

Proved when the physical target binds the service:

- The Android debug package installs and launches on the physical Samsung Galaxy
  S9 proof target.
- The package AccessibilityService can be enabled through Android secure
  settings on that proof target.
- The package reports bound service state and count-only window-state event
  observation after launch.
- Parent-domain accepts only settings enablement, activity-visible event count,
  and opaque proof refs while rejecting raw event rows and control claims.

Proved when the physical target refuses or reverts service enablement:

- The package still installs with the declared AccessibilityService.
- `MainActivity` exposes the parent-safe runtime state and count-only event
  field.
- The device remains manual-required for Accessibility enablement; no event
  sample, overlay, dispatch, delivery, or enforcement claim is emitted.

Not proved:

- Warning, block, request, usage-context, or overlay runtime execution.
- Device Owner or Profile Owner authority.
- Play policy or store distribution proof.
- Raw Accessibility event rows, service names, overlay content, raw UI XML, or
  raw device serial custody.
- Provider delivery, child-device delivery outside the package, adapter
  dispatch, broad blocking, or platform enforcement.
