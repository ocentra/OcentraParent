# WP200 App/game Android UsageEvents count sample

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP200 App/game Android UsageEvents count sample`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Extend the Android package-local UsageEvents runtime preflight with count-only
sample support.

The Android package can query `UsageStatsManager.queryEvents` when UsageStats is
granted and reduce the result to total event count plus foreground event count.
It does not store package names, class names, raw UsageEvents rows, or raw
activity data. Parent-domain accepts only count summaries and keeps dispatch,
platform enforcement, provider delivery, and child-device delivery unclaimed.

## Non-Goals

- No physical-device settings grant or live device sample claim.
- No raw UsageEvents rows, package names, class names, or activity custody.
- No child-device delivery, provider delivery, adapter dispatch, platform
  enforcement, Device Owner/Profile Owner, or Play policy proof.
- No broad blocking or Android package action execution.

## Current source and test roots

- `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidUsageEventsRuntimePreflight.java`
- `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidUsageEventsReplayStore.java`
- `platforms/android/agent/app/src/test/java/ca/ocentra/parent/agent/AppGameAndroidUsageEventsReplayTest.java`

Canonical `f2d485b20` uses real Robolectric AppOps and UsageStats shadows to add
in-window/out-of-window events, reduce three events to two foreground events,
persist/read back the count snapshot, and prove the app-private store contains
exactly the six expected durable keys. No package/activity row is retained.
Historical `packages/parent-domain` and proof-runner paths are retired.

## Validation state

The real Java behavior test is written but was not executed in this
code/test-source phase. No physical grant/sample proof was produced.

## Remaining completion criteria

- Execute the focused Java test in the later validation wave.
- Retain a physical count-only UsageEvents sample without raw row custody.
- Keep dispatch, enforcement, provider delivery, and child-device delivery
  unclaimed.
