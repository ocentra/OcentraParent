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
- 42/42 graph-imported workpacks have reviewed code/test topology.
- 24 workpacks have no bounded source/test-writing gap; 18 retain concrete
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
  replays it on startup, and projects it idempotently.
- WP34-WP36 are real typed/process-local Rust flows, not durable runtime
  completion. WP38 remains notification intent only; WP39 remains a read-only
  ActivityStore/portal surface that is not fed by the live tracking cascade.
- The per-workpack reachability and caller/effect boundary is recorded in
  [CODE_AUDIT.md](CODE_AUDIT.md). Historical checklist/proof state remains
  non-authoritative; tests, proof, platform execution, provider delivery, and
  runtime composition are deferred or manual-required as listed there.

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

## Open runtime gaps

- WP07: production retention/custody execution.
- WP08-WP13: real Android, iOS, and desktop adapters.
- WP20: concrete Places/POI provider.
- WP22: durable parent-defined place database.
- WP24: selected AI provider routing/execution.
- WP27/WP38: escalation plus durable notification delivery lifecycle.
- WP28/WP29: durable live/missing-device runtime ownership.
- WP30: complete parent/child product UI.
- WP33: tracked executable aggregate verifier.
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

- Graph-imported workpacks: 42.
- Executable/coordination workpacks: WP01-WP39.
- Imported reference packets: capability guide, schema proposal, and settings
  inventory.
- Source/test result: 24 bounded complete, 18 incomplete.

## Product-ready no-claim boundaries

Do not claim product-ready tracking, physical-device background behavior,
provider delivery/receipt, durable production workers, authority-enrolled hard
control, full child UI, durable journal replay, or end-to-end portal projection
until the selected workpack proves it from current source/tests and regenerated
proof.
