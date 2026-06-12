# WP185 App/Game Android UsageEvents Foreground Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP185 App/Game Android UsageEvents Foreground Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Extend the physical Android proof from package/policy-state visibility into a
redacted UsageEvents foreground-evidence boundary.

This is still a visibility-only Android platform proof. It proves that a real
physical Android device exposes foreground activity event samples through
`dumpsys usagestats`, but it does not claim Device Owner/Profile Owner
authority, hide/suspend execution, broad blocking, child runtime delivery, or
provider delivery.

## Implementation

- Extended `packages/parent-domain/src/app-game-android-physical-device-proof.ts`
  with a redacted UsageEvents dump state, sample counts, foreground activity
  event counts, and explicit foreground-evidence observation.
- Updated `packages/parent-domain/src/app-game-platform-proof-status.ts` so the
  Android platform proof status row uses redacted UsageEvents samples as runtime
  visibility and no longer reports `android-usage-events-not-proved` when
  foreground activity samples are observed.
- Updated `scripts/test/app-game-android-physical-device-proof.mjs` to collect
  `adb shell dumpsys usagestats`, count event rows and foreground activity
  events, and compact command output to redacted counts only.

## Validation

Focused validation for this workpack:

```powershell
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-physical-device-proof app-game-platform-proof-status
node scripts/test/app-game-android-physical-device-proof.mjs
```

## Proof

- `test-results/app-game-android-physical-device-proof/proof.json`
- `output/app-game-plan-proof/181-app-game-android-physical-device-proof/proof.json`
- `output/app-game-plan-proof/181-app-game-android-physical-device-proof/09-manual-platform-proof.md`

## Boundaries

Proved:

- The physical Samsung Galaxy S9 Android 10 target is reachable through explicit
  Wi-Fi ADB.
- `dumpsys usagestats` returns redacted UsageEvents samples.
- The proof records foreground activity event counts without storing package
  names, class names, raw device serials, or raw app activity rows.
- Android platform proof status can distinguish foreground evidence observed
  from Android ownership/enforcement authority.

Not proved:

- Android Device Owner or Profile Owner authority.
- Durable child-device UsageEvents replay or local child runtime delivery.
- Accessibility overlay behavior.
- `setApplicationHidden`, `setPackagesSuspended`, uninstall blocking, lock task,
  managed configurations, or Play policy proof.
- Adapter dispatch, broad installed-app blocking, platform enforcement,
  provider delivery, raw private rows/targets, or private diagnostics.
