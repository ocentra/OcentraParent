# WP199 App/game Android UsageEvents runtime preflight

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP199 App/game Android UsageEvents runtime preflight`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Add an Android package-local runtime preflight for app/game UsageEvents
readiness.

The Android package now checks UsageStats AppOps state and UsageStats service
visibility from package code, then exposes only readiness states through
MainActivity. Parent-domain models the preflight as blocked until runtime sample
proof exists. No raw UsageEvents rows, package names, adapter dispatch, platform
enforcement, or child-device delivery are claimed.

## Non-Goals

- No `PACKAGE_USAGE_STATS` manifest declaration.
- No automatic settings grant or runtime permission claim.
- No runtime UsageEvents sample collection claim.
- No raw UsageEvents rows, package names, class names, or activity custody.
- No Device Owner/Profile Owner, hide/suspend/uninstall-block, lock task,
  managed configuration, adapter dispatch, platform enforcement, provider
  delivery, or child-device delivery claim.

## Current source and test roots

- `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidUsageEventsRuntimePreflight.java`
- `platforms/android/agent/app/src/test/java/ca/ocentra/parent/agent/AppGameAndroidUsageEventsRuntimePreflightTest.java`

Canonical `3dde089a6` makes readiness require both granted UsageStats AppOps and
a visible `UsageStatsManager`. Null/malformed contexts, AppOps/service/query
failures, and interrupted iteration fail closed without raw data or authority
claims. The focused Robolectric test covers denied, unavailable, malformed,
redacted, and non-claim behavior. WP200 owns the count-specific positive sample
coverage. Historical `packages/parent-domain` and `scripts/test/app-game-*`
paths are retired and must not be recreated.

## Validation state

The focused Java behavior test is written but was not executed in this
code/test-source phase. No physical UsageStats grant/sample proof was produced.

## Remaining completion criteria

- Execute the focused preflight and WP200 count tests in the validation wave.
- Retain a real package/grant sample proof without raw UsageEvents custody.
- Keep adapter dispatch, enforcement, provider delivery, and child-device
  delivery unclaimed.
