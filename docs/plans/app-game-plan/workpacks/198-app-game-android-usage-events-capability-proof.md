# WP198 App/game Android UsageEvents capability proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP198 App/game Android UsageEvents capability proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Add an Android package-local bridge for app/game UsageEvents capability status
and bind it to a parent-domain proof contract.

This workpack keeps UsageStats as a settings-grant-required capability and
records only package-local readiness states, commands, events, proof refs, and
open gaps. It does not declare `PACKAGE_USAGE_STATS`, store raw UsageEvents
rows, store package names, or claim adapter dispatch, platform enforcement, or
child-device delivery.

## Non-Goals

- No UsageStats settings grant proof.
- No runtime UsageEvents collection from the Android package.
- No raw UsageEvents rows, package names, class names, or activity custody.
- No Device Owner/Profile Owner authority proof.
- No hide, suspend, uninstall-block, lock task, managed configuration, adapter
  dispatch, platform enforcement, provider delivery, or child delivery claim.

## Current source and test roots

- `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidUsageEventsCapabilityProof.java`
- `platforms/android/agent/app/src/main/java/ca/ocentra/child/agent/ChildAgentCompositionService.java`
- `platforms/android/agent/app/src/main/AndroidManifest.xml`
- `platforms/android/agent/app/src/test/java/ca/ocentra/parent/agent/AppGameAndroidUsageEventsCapabilityProofTest.java`

The historical `packages/parent-domain` and `scripts/test/app-game-*` owners
were removed during Rust-first convergence and must not be recreated. Canonical
`bb83a0aef` keeps the live manifest/service consumer-topology claim and tests
the exact redacted capability bundle, empty proof refs, gaps, and non-claims.

## Validation state

The focused Java behavior test is written but was not executed in the
code/test-source phase. No Android grant, package, physical-device, or retained
proof was produced.

## Remaining completion criteria

- Execute the focused Java test in the later validation wave.
- Retain real Android package/grant evidence without raw UsageEvents data.
- Keep raw storage, adapter dispatch, platform enforcement, provider delivery,
  and child-device delivery unclaimed.
