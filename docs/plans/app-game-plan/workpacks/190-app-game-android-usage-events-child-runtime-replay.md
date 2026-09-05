# WP190 App/Game Android UsageEvents Child Runtime Replay

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP190 App/Game Android UsageEvents Child Runtime Replay`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Attach the Android UsageEvents replay readiness row from WP188 to a child
runtime replay consumer boundary.

This proves a redacted-count consumer seam only. It does not prove Android
child-device delivery, raw UsageEvents row custody, Device Owner/Profile Owner
authority, hide/suspend, adapter dispatch, or platform enforcement.

## Implementation

The source phase adds:

- `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidUsageEventsChildRuntimeReplay.java`
- `platforms/android/agent/app/src/main/java/ca/ocentra/child/agent/ChildAgentCompositionService.java`
- `platforms/android/agent/app/src/main/java/ca/ocentra/child/agent/ChildAgentActivity.java`

The consumer accepts only an owner-produced count snapshot, validates current
timestamp/count bounds, rejects duplicate or older generations, rejects
corrupt or stale durable readback, and revalidates the committed record before
reporting `CONSUMED`. The manifest-declared child service invokes the consumer
from its bounded worker, publishes synchronized immutable status, and waits
for bounded worker shutdown before closing native composition. The launcher
activity now binds to that service and polls a redacted status projection, so
the production consumer is reachable from an existing app lifecycle; it does
not claim device delivery or enforcement. Canonical `a45575cfa` adds real JVM
restart/currentness/corruption tests and a physical-device instrumentation path
that grants UsageStats through Android AppOps, consumes a real owner-produced
snapshot, proves duplicate/older rejection, and accepts a newer generation.

## Validation

The real JVM and instrumentation test sources are written but were not
executed in this code/test-source phase. No proof harness was run or retained.

## Proof

No proof artifact exists in this source phase. The graph remains planned and
the required tests/proof/checklist evidence remain open.

## Boundaries

Source packet semantics:

- The actual manifest-declared child service consumes the owner-produced,
  count-only snapshot on a bounded worker and exposes durable readback.
- The service publishes no raw rows, package/activity data, proof refs,
  delivery receipts, or enforcement claims.

Not proved:

- Raw UsageEvents row storage or raw package/activity data.
- Test execution and retained proof artifacts.
- Android child-device delivery.
- Device Owner/Profile Owner authority.
- Hide/suspend/uninstall block, lock task, managed configuration, Play policy,
  adapter dispatch, platform enforcement, provider delivery, or broad blocking.
