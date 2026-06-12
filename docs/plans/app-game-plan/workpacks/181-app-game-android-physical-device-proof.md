# WP181 - App/Game Android Physical Device Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP181 - App/Game Android Physical Device Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Add a typed app/game Android physical-device proof boundary that records real
ADB evidence from a physical Samsung Galaxy S9 while preserving the existing
normal-mode no-claim gates.

This workpack proves:

- the Android target is a physical device, not an emulator-only artifact;
- the target exposes Android build identity and API-level evidence;
- the package manager returns visible package rows, stored only as a count;
- `dumpsys usagestats` returns redacted UsageEvents samples and foreground
  activity event counts;
- device-policy state does not prove Device Owner or Profile Owner enrollment;
- app/game hide, suspend, broad blocking, adapter dispatch, and platform
  enforcement remain unclaimed until owner/profile-owner proof exists.

## Touched Areas

- `packages/parent-domain/src/app-game-android-physical-device-proof.ts`
- `packages/parent-domain/tests/app-game-android-physical-device-proof.test.ts`
- `scripts/test/app-game-android-physical-device-proof.mjs`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/README.md`

## Evidence

Proof root:

```text
output/app-game-plan-proof/181-app-game-android-physical-device-proof/
```

Runtime proof output:

```text
test-results/app-game-android-physical-device-proof/proof.json
```

The proof harness connects to the explicit physical-device target from the hub
mail:

```text
adb -s 192.168.2.45:5555
```

The generated proof redacts raw device serial/IP and package names. It stores
only parent-safe build identity, package count, usage-stats command visibility,
redacted UsageEvents sample counts, foreground activity event counts, and
owner/profile-owner proof state.

## Non-Claims

- No Android Device Owner support is claimed.
- No Android Profile Owner support is claimed.
- No `setApplicationHidden` or `setPackagesSuspended` adapter is claimed.
- No uninstall-block, lock-task, managed-configuration, Accessibility overlay,
  durable UsageEvents replay, or child-device delivery behavior is claimed.
- No raw package names, raw usage-event package/class names, raw activity rows,
  or raw device serials are written to the proof artifact.

## Validation

Focused validation for this workpack:

```powershell
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-android-physical-device-proof app-game-broad-blocking-proof-gates
cmd /c "node --check scripts/test/app-game-android-physical-device-proof.mjs && node scripts/test/app-game-android-physical-device-proof.mjs"
```
