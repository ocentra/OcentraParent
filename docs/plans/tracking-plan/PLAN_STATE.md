# Tracking Plan State

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Tracking Plan State`
> Kind: current state and open gaps.
> Read when: Immediately after plan AGENTS.md.
> Proves: only the routed status below; not product readiness or workpack DONE.

<!-- /agent-capsule -->

## Scope

Tracking owns location evidence, device/permission/capability state,
geofence/expected-place/nearby-place semantics, child check-in, tracking policy
inputs, live/missing-device modes, tracking alerts, tracking read models, and
their no-claim boundaries. Adjacent crates retain generic eventing, policy,
notification, AI-provider, custody, platform, and portal-shell authority.

## Code-first Phase 1 audit (2026-08-15)

- Authoritative audit: [CODE_AUDIT.md](CODE_AUDIT.md).
- 43/43 graph-imported workpacks have reviewed code/test topology.
- 24 workpacks have no bounded source/test-writing gap; 19 retain concrete
  production-code or expected-test gaps.
- Phase 2 focused tests/Enforcer and Phase 3 proof regeneration were not run.
- Workpack checkboxes below remain document/proof state, not this code result.

## Production reachability pass (2026-08-16)

- Branch: `codex/tracking-plan-code-pass` from the consolidated app/game-plan
  audit baseline.
- No production slice was accepted. The highest-unblock WP37 path is not
  legally implementable in this lane because
  `TrackingRuntimeEventFlow::new` and the parent check-in flow construct an
  in-memory `EventBus`, while `ActivityStore::ingest_journal` consumes a
  separate `ActivityEvent` journal path. No shipped owner currently maps the
  tracking domain cascade into that journal, configures its durable key/path,
  replays it on startup, and projects it idempotently. New WP40 now explicitly
  owns that missing composition route; WP37 remains blocked behind WP40.
- WP34-WP36 are real typed/process-local Rust flows, not durable runtime
  completion. WP38 remains notification intent only; WP39 remains a read-only
  ActivityStore/portal surface that is not fed by the live tracking cascade.
- The per-workpack reachability and caller/effect boundary is recorded in
  [CODE_AUDIT.md](CODE_AUDIT.md). Historical checklist/proof state remains
  non-authoritative; tests, proof, platform execution, provider delivery, and
  runtime composition are deferred or manual-required as listed there.
- The 2026-08-29 Android source audit corrected WP08-WP10 ownership. Tracking
  owns the foreground/background/geofence/status semantics, but the shipped
  Android package, JNI bridge, service lifecycle, and platform-test roots are
  owned by Child Runtime Distribution WP05 after its WP10 trusted startup
  handoff. WP09 additionally depends on the reviewed durable ingress in WP40.
  No Tracking workpack may invent a parallel Android service or dead handoff.
- The 2026-08-29 iOS route audit found the same topology error in WP11-WP12:
  neutral Rust location/status/geofence models were being counted as iOS
  adapter implementation even though no Core Location producer, child-package
  handoff, or XCTest runtime coverage exists. Child Runtime Distribution WP06
  owns the canonical capability-only iOS app, Child WP10 owns trusted child
  startup/ingress, and Tracking WP40 owns trusted durable tracking ingress.
  WP11-WP12 must consume those reviewed owners and must not add a dead Swift
  JSON/file handoff or claim the neutral Rust models as platform code.

## Current owners

```text
crates/schema:
  canonical cross-family tracking event registry and generated bridge boundary.

agent-protocol:
  typed tracking identifiers, runtime/config events, read models, constants,
  retention commands, and event identity.

tracking-core:
  tracking decisions, validation, local models, retention transforms, and
  ActivityStore/SQLite query projection.

parent-runtime-core / child-runtime:
  parent config and child detection/check-in process-local event flows.

policy-control-core / child-policy-core:
  tracking policy compilation and child-local evaluation authority.

child-ai-core / child-notification-core:
  evidence-only AI validation and notification-intent conversion.

agent-core / agent-service:
  ActivityStore-backed tracking read model and service transport seams.

WP40 composition owner:
  the future child/service runtime owner for trusted tracking ingress, durable
  journal configuration, startup replay, and idempotent ActivityStore projection.
  This owner is routed but not implemented in the current checkout.

child-agent-runtime-distribution WP05:
  shipped Android package, JNI bridge, service lifecycle, and platform-test
  boundary consumed by Tracking WP08-WP10 after reviewed implementation.

child-agent-runtime-distribution WP06 + WP10:
  canonical capability-only iOS child package plus the future trusted child
  startup/ingress boundary consumed by Tracking WP11-WP12. WP06 alone is not a
  Core Location runtime or transport.

portal-domain / apps/portal:
  presentation of Rust-owned/generated tracking snapshots.
```

`packages/tracking-domain` does not exist and is not an owner. The historical
`scripts/test/tracking-*.mjs` proof suite is also absent.

## Implemented foundations

- Typed tracking runtime/config event contracts and identifiers.
- Location validation and status/capability evaluation.
- Geofence, expected-place, acknowledgement, check-in, nearby-place, AI
  evidence, tracking policy, alert, and notification-intent logic with focused
  tests present.
- Parent config and child detection event cascades over the shared EventBus.
- SQLite/ActivityStore tracking read-model queries and portal presentation.

## WP30 bounded UI truthfulness closeout (2026-09-05)

- The real `PolicyTracking` route mounts the typed Rust tracking snapshot.
- Fixed proof fixtures remain confined to `ProofPanels`; the product route does
  not fabricate rows when the service boundary is unavailable.
- Missing, unavailable, and malformed snapshots fail closed in behavioral
  Portal tests, and the hosted desktop/mobile route proof passed without
  overlap or product overclaim.
- The retained bundle and exact validation results are recorded in
  [WP30_PARENT_CHILD_UI_UX_SURFACES_PROOF.md](../../proof/tracking-plan/WP30_PARENT_CHILD_UI_UX_SURFACES_PROOF.md).
- This closes only the bounded product-consumer truthfulness slice. Full live
  location, authenticated child delivery, retention mutation, notification
  delivery, policy action authority, and restart projection remain open.

## Open runtime gaps

- WP07: production retention/custody execution.
- WP08-WP10: real Android tracking adapters after Child WP10 -> Child WP05;
  WP09 also waits for WP40 durable ingress.
- WP11-WP12: real iOS Core Location adapters after Child WP06, Child WP10, and
  Tracking WP40.
- WP13: real desktop presence-hint adapters.
- WP20: concrete Places/POI provider.
- WP22: durable parent-defined place database.
- WP24: selected AI provider routing/execution.
- WP27/WP38: escalation plus durable notification delivery lifecycle.
- WP28/WP29: durable live/missing-device runtime ownership.
- WP30 follow-on: complete the broader live parent/child product UI after the
  upstream runtime, authority, delivery, and restart owners are available; the
  bounded product-route truthfulness slice is complete.
- WP33: tracked executable aggregate verifier.
- WP40: trusted tracking runtime ingress and journal composition owner.
- WP37: durable tracking journal replay and idempotent SQLite projection.
- WP39: end-to-end event-to-portal restart chain.

## Resume route

1. Read this file.
2. Read `NEXT_ACTIONS.md`.
3. Select one row in `WORKPACK_INDEX.md`.
4. Use [source-index.md](source-index.md) for current owners and
   [CODE_AUDIT.md](CODE_AUDIT.md) for the last reviewed source/test result.
5. Run focused tests/Enforcer before proof.

## Checklist summary

- Full checklist: [implementation-checklist.md](implementation-checklist.md)
  (not default context).
- Checkbox rows detected before this audit: 111 total, 79 checked, 32 open.
- Do not infer implementation completion from those values.

## Workpack summary

- Graph-imported workpacks: 43.
- Executable/coordination workpacks: WP01-WP40.
- Imported reference packets: capability guide, schema proposal, and settings
  inventory.
- Source/test result: 24 bounded complete, 19 incomplete.

## Product-ready no-claim boundaries

Do not claim product-ready tracking, physical-device background behavior,
provider delivery/receipt, durable production workers, authority-enrolled hard
control, full child UI, durable journal replay, or end-to-end portal projection
until the selected workpack proves it from current source/tests and regenerated
proof.
