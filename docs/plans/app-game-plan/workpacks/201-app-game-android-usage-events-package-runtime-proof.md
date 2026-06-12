# WP201 App/game Android UsageEvents package runtime proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP201 App/game Android UsageEvents package runtime proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Install and launch the Android debug package on the physical Samsung Galaxy S9
target, then record package-local UsageEvents runtime state from AppOps and the
package UI.

This workpack proves package installation and launch on the physical target and
captures whether the package reports UsageStats granted, settings-grant
required, or permission-check unavailable. It keeps UI/XML details redacted,
does not store raw device serials, package lists, UsageEvents rows, package
names, class names, or activity data, and does not claim dispatch or platform
enforcement.

## Non-Goals

- No automatic UsageStats settings grant.
- No claim of live UsageEvents samples unless the package UI reports
  `sample-observed`.
- No raw UI XML, package list, UsageEvents rows, package names, class names, or
  activity custody.
- No adapter dispatch, platform enforcement, provider delivery, child-device
  delivery, Device Owner/Profile Owner, or Play policy proof.

## Files

- `packages/parent-domain/src/app-game-android-usage-events-package-runtime-proof.ts`
- `packages/parent-domain/tests/app-game-android-usage-events-package-runtime-proof.test.ts`
- `scripts/test/app-game-android-usage-events-package-runtime-proof.mjs`

## Validation

- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-usage-events-package-runtime-proof`
- `cmd /c node --check scripts/test/app-game-android-usage-events-package-runtime-proof.mjs`
- `cmd /c node scripts/test/app-game-android-usage-events-package-runtime-proof.mjs`

## Done Criteria

- Physical Android target is explicitly selected with Wi-Fi ADB.
- Debug APK is installed and MainActivity is launched.
- AppOps and UI state are captured as parent-safe proof refs.
- Parent-domain rejects missing UI/AppOps evidence, raw data custody, dispatch,
  enforcement, and child delivery claims.
