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

## Current source and test prerequisites

- `platforms/android/agent/app/src/main/AndroidManifest.xml`
- `platforms/android/agent/app/src/main/java/ca/ocentra/child/agent/ChildAgentActivity.java`
- `platforms/android/agent/app/src/main/java/ca/ocentra/child/agent/ChildAgentCompositionService.java`
- WP190/WP200 focused JVM and instrumentation test roots.

Independent review at canonical `f2d485b20` found no missing owned production
or behavioral test source. Historical `packages/parent-domain` and
`scripts/test/app-game-*` paths were removed by Rust-first convergence and are
not current owners.

## Validation state

This is now a physical-proof-only workpack. No Samsung S9 Wi-Fi ADB install,
launch, AppOps, or redacted UI capture was run in the code/test-source phase.

## Done Criteria

- Physical Android target is explicitly selected with Wi-Fi ADB.
- Debug APK is installed and MainActivity is launched.
- AppOps and UI state are captured as parent-safe proof refs.
- Captured proof rejects missing UI/AppOps evidence, raw data custody, dispatch,
  enforcement, and child delivery claims.
