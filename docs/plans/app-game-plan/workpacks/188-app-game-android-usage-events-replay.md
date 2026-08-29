# WP188 App/Game Android UsageEvents Replay Readiness

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP188 App/Game Android UsageEvents Replay Readiness`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Turn the redacted foreground UsageEvents counts from WP185 into a parent-domain
runtime visibility replay readiness read model.

This is not an Android enforcement adapter. It is a durable, parent-safe replay
boundary over counts and proof refs only.

## Implementation

The source phase adds the Android production boundary at:

- `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidUsageEventsRuntimePreflight.java`
- `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidUsageEventsReplayStore.java`
- `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java`
- `platforms/android/agent/app/src/main/AndroidManifest.xml`

The preflight checks the real child APK identity, reads only count-only
`UsageEvents` rows, and commits a generation/timestamped count snapshot to
app-private storage. The manifest declares the Android special permission
needed for the AppOps grant path. The manifest-declared child composition
service consumes and revalidates that durable snapshot. Canonical
`a45575cfa` adds real JVM durable replay coverage and physical-device
instrumentation for grant, currentness, duplicate/older rejection, and newer
acceptance. The remaining workpack blocker is the stated parent-runtime
visibility boundary: no typed Android-to-Rust/parent ingress consumes this
local status.

## Validation

The focused JVM and instrumentation tests are written but were not executed in
the code/test-source phase. No proof harness was run or retained.

## Proof

No proof artifact exists in this source phase. The graph remains planned and
the required tests/proof/checklist evidence remain open.

## Boundaries

Source packet semantics:

- Android UsageEvents are queried and persisted as counts only, with monotonic
  generation and observed-at metadata.
- Raw rows, package names, activity names, proof refs, delivery, and
  enforcement are not stored or claimed.

Not proved:

- Typed Android-to-Rust/parent replay projection and runtime proof.
- Test execution and retained proof artifacts.
- Raw UsageEvents row storage or raw package/activity data.
- Android child runtime replay consumer.
- Device Owner/Profile Owner authority.
- Hide/suspend/uninstall block, lock task, managed configuration, Play policy,
  adapter dispatch, platform enforcement, provider delivery, or child-device
  delivery.
