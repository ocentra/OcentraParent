# 13 Browser Read Models And Service Events

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `13 Browser Read Models And Service Events`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Where We Are

Browser managed status, browser evidence, policy, and intervention service
paths exist, but the full product read model needs reconciliation across
inventory, session, evidence, unmanaged fallback, and intervention.

2026-06-02 codex-d progress: the service now has a browser inventory read-model
helper, payload mapper, typed get command, and replayable reported event backed
by protocol constants. It derives typed rows from the existing managed-session
status contract and preserves honest custody, query visibility, exact URL,
active-tab, and unmanaged fallback boundaries. This does not yet expose a
portal UI command, SQLite inventory row store, or full
inventory/session/evidence/intervention reconciliation stream.

2026-08-16 browser-code-pass: the existing typed `ActivityBrowserReadModel`
service event is now consumed by the Rust parent bridge on the Activity and
Browser routes and projected through the existing portal live-activity field.
This is code-drafted, unvalidated, and tests/proof/checklist-deferred. It does
not add browser capture, active-tab focus authority, unmanaged exact-URL
authority, intervention delivery, or enforcement.

2026-08-16 browser-code-pass follow-up: the existing stored
`BrowserEvidenceReadModel` event is now consumed by the Rust parent bridge on
the Browser route and projected through the existing portal live-activity
fields. Its active-state, proof-source, custody, and query-visibility values
remain service-owned; no target-list evidence is promoted. This is
code-drafted, unvalidated, and tests/proof/checklist-deferred.

## Where We Want To Be

The Rust service emits typed, replayable browser status/read-model events that
portal, policy, and AI can consume without raw adapter access.

## Scope

- Inventory read model.
- Managed session status.
- Recent tab evidence read model.
- Active-state certainty.
- Stale/degraded bridge status.
- Unmanaged browser detections.
- Browser intervention rows.
- Capability and custody labels.

## Touched Paths

- `crates/agent-protocol/src/browser_read_model.rs`
- `crates/agent-protocol/src/constants/field.rs`
- `crates/agent-service/src/activity_api.rs`
- `crates/agent-service/src/browser_evidence_payload.rs`
- `crates/agent-service/src/browser_payload.rs`
- `crates/agent-service/src/browser_runtime*.rs`
- `packages/agent-protocol-domain/src/browser-policy-adapter.ts`
- `crates/parent-runtime-core/src/agent_service_client/snapshots_browser.rs`
- `crates/parent-runtime-core/src/agent_service_client/loaders.rs`
- `crates/parent-runtime-core/src/agent_service_client/types.rs`
- `crates/parent-runtime-core/src/parent_ui_bridge.rs`
- `crates/parent-runtime-core/src/parent_ui_bridge/route_snapshot.rs`
- `crates/parent-runtime-core/src/parent_ui_bridge/route_requirements.rs`
- `crates/parent-runtime-core/src/parent_ui_bridge/route_snapshot/dependencies.rs`
- `crates/parent-runtime-core/src/parent_ui_bridge/route_snapshot/dependencies/load.rs`
- `crates/parent-runtime-core/src/parent_ui_bridge/live_activity/snapshot.rs`
- `crates/parent-runtime-core/src/parent_ui_bridge/live_activity/snapshot/browser.rs`
- `crates/schema/src/parent_ui_bridge.rs`
- `apps/portal/generated/parent-ui-bridge.ts`

## Tests And Proof

- Rust protocol tests.
- Service event tests.
- TypeScript adapter tests.
- Real service proof script updates.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/13-browser-read-models-and-service-events/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service parity updated only after contracts exist; portal UI parity remains deferred because no visual surface changed.
- [ ] Raw evidence artifacts captured or marked N/A for this service-derived sub-slice: managed status fixtures, unmanaged process rows, and missing-browser status are mapped into read-model payloads and replayable service events; no journal, SQLite, policy, or action behavior changed.
- [ ] Tests/proof listed in this workpack are implemented for service read-model payload derivation and the replayable inventory read-model event; portal adapter tests and real service proof scripts remain manual-required.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed, so `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured for this sub-slice: unmanaged rows remain process-only and managed target-list rows do not claim active-tab support.
- [ ] Manual platform proof captured for real browser/OS claims; no new real OS/browser claim was made, so `09-manual-platform-proof.md` records the N/A boundary.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Read models expose status; they do not prove platform/browser behavior without
matching runtime evidence.

## Event Runtime Spine Addendum - 2026-06-07

`browser-event-runtime-spine-proof` adds a browser-specific consumer of the
reusable Rust `ocentra-eventing` crate in `agent-core`. It publishes an ordered
ref-only browser runtime chain for evidence, journal, AI request/result, policy
request/decision, intervention command/result, audit, and read-model projection.

Evidence:

- `crates/agent-core/src/browser_event_runtime.rs`
- `crates/agent-core/src/browser_event_runtime_phase.rs`
- `test-results/browser-event-runtime-spine-proof/proof.json`
- `output/browser-plan-proof/browser-event-runtime-spine/01-browser-event-runtime-spine-proof.md`
- `cmd /c node scripts/test/browser-event-runtime-spine-proof.mjs`

This does not change service WebSocket routing, portal UI, browser mutation, AI
execution, policy execution, or enforcement. Manual-required rows skip
intervention command/result phases while keeping audit and read-model projection
visible.

## Service Delivery Addendum - 2026-06-07

`browser-runtime-service-delivery-proof` adds the service-side adapter from
`BrowserEvidenceReadModel` rows into the reusable browser event-runtime chain.
It mirrors the existing network runtime delivery shape without adding a new
browser-private event bus.

Evidence:

- `crates/agent-service/src/browser_runtime_delivery.rs`
- `crates/agent-service/src/browser_runtime_delivery_tests.rs`
- `test-results/browser-runtime-service-delivery-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-service-delivery/01-browser-runtime-service-delivery-proof.md`
- `cmd /c node scripts/test/browser-runtime-service-delivery-proof.mjs`

The delivery proof keeps managed exact-URL rows evidence-only, keeps unavailable
rows manual-required, and proves no intervention command events are emitted from
read-model delivery alone. It does not change service WebSocket routing, portal
UI, AI execution, policy execution, browser mutation, or enforcement.
Remaining read-model work requires portal adapter/UI consumption,
journal/SQLite inventory row proof, recent tab evidence reconciliation,
intervention row reconciliation, real service proof scripts, and UI snapshots
before product-facing status can claim more than the typed service event.

## Event-Chain Stream Addendum - 2026-06-07

`browser-runtime-event-chain-stream-proof` exposes the browser runtime chain
through typed protocol command/event names and a service WebSocket route:
`agent.browser.runtime.event-chain.stream.get` ->
`agent.browser.runtime.event-chain.stream.reported`. The route reads the real
browser evidence read model from the activity store, streams rows through the
reusable browser event runtime, and returns protocol-facing camelCase event
payloads with event refs.

Evidence:

- `crates/agent-protocol/src/transport.rs`
- `packages/agent-protocol-domain/src/contracts.ts`
- `crates/agent-service/src/browser_runtime_stream_payload.rs`
- `crates/agent-service/src/browser_runtime_stream_events.rs`
- `crates/agent-service/src/browser_runtime_stream_tests.rs`
- `test-results/browser-runtime-event-chain-stream-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-event-chain-stream/01-browser-runtime-event-chain-stream-proof.md`
- `cmd /c node scripts/test/browser-runtime-event-chain-stream-proof.mjs`

The proof validates typed protocol parity, store-backed WebSocket delivery,
manual-required handling for unavailable rows, zero intervention command events,
and camelCase stream payloads. It does not claim portal UI consumption, AI
execution, policy execution, browser mutation, child intervention execution, or
enforcement.

## Portal Stream Consumer Addendum - 2026-06-07

`browser-runtime-portal-stream-consumer-proof` adds parent portal consumption of
the browser runtime stream without adding a new visual surface. The overview
command list requests `agent.browser.runtime.event-chain.stream.get`,
command-result routing accepts
`agent.browser.runtime.event-chain.stream.reported`, and
`PortalLiveActivityState` parses stream counts and entries so the portal can
consume the evented chain as state.

Evidence:

- `packages/portal-domain/src/commands.ts`
- `packages/portal-domain/src/command-results.ts`
- `apps/portal/src/live-activity-state.ts`
- `apps/portal/tests/live-activity-state.test.ts`
- `test-results/browser-runtime-portal-stream-consumer-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-portal-stream-consumer/01-browser-runtime-portal-stream-consumer-proof.md`
- `cmd /c node scripts/test/browser-runtime-portal-stream-consumer-proof.mjs`

The proof validates that manual-required rows remain visible to portal state and
intervention command events remain zero. It does not claim a new portal visual
surface, AI execution, policy execution, browser mutation, child intervention
execution, or enforcement.

## Event-Chain Ref Addendum - 2026-06-07

`browser-runtime-event-chain-ref-proof` tightens the browser runtime event-chain
audit path so each payload `previousPhaseRef` points at the previous published
browser event ref instead of a source/evidence/policy business ref. When
intervention phases are skipped for manual-required rows, the following audit and
read-model phases point to the last actually published event.

Evidence:

- `crates/agent-core/src/browser_event_runtime.rs`
- `crates/agent-core/src/browser_event_runtime_refs.rs`
- `crates/agent-core/src/browser_event_runtime_tests.rs`
- `scripts/test/browser-runtime-event-chain-ref-proof.mjs`
- `test-results/browser-runtime-event-chain-ref-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-event-chain-ref-proof/01-browser-runtime-event-chain-ref-proof.md`
- `cargo test -p ocentra-parent-agent-core browser_event_runtime --quiet`

This does not change event names, protocol payload fields, service WebSocket
routing, portal UI, AI execution, policy execution, browser mutation, child
intervention execution, or enforcement.

## Typed Stream Contract Addendum - 2026-06-07

`browser-runtime-typed-stream-contract-proof` adds a protocol-domain parser for
the service-backed browser runtime event-chain stream. The parser validates known
browser runtime event types, Rust-serialized phase names, event type/phase
consistency, stream counts, no AI-authority overclaim, and no hidden
intervention execution.

Evidence:

- `packages/agent-protocol-domain/src/browser-runtime-events.ts`
- `packages/agent-protocol-domain/tests/browser-runtime-events.test.ts`
- `scripts/test/browser-runtime-typed-stream-contract-proof.mjs`
- `test-results/browser-runtime-typed-stream-contract-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-typed-stream-contract/01-browser-runtime-typed-stream-contract-proof.md`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- browser-runtime-events.test.ts`
- `cmd /c npm run type-check --workspace @ocentra-parent/agent-protocol-domain`

This does not claim portal UI, AI execution, policy execution, browser mutation,
child intervention execution, or enforcement.

## Portal Typed Stream Consumer Addendum - 2026-06-07

`browser-runtime-portal-typed-stream-consumer-proof` updates
`PortalLiveActivityState` to consume the shared protocol-domain typed parser for
the browser runtime event-chain stream. The portal no longer keeps a separate
loose JSON entry parser for this stream, and the focused portal tests reject
event type/phase drift, AI-authority overclaim, and stream count drift before
exposing the stream as state.

Evidence:

- `apps/portal/src/live-activity-state.ts`
- `apps/portal/tests/live-activity-state.test.ts`
- `scripts/test/browser-runtime-portal-typed-stream-consumer-proof.mjs`
- `test-results/browser-runtime-portal-typed-stream-consumer-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-portal-typed-stream-consumer/01-browser-runtime-portal-typed-stream-consumer-proof.md`
- `cmd /c node scripts/test/browser-runtime-portal-typed-stream-consumer-proof.mjs`

This claims portal state consumption only. It does not claim a new portal visual
surface, AI execution, policy execution, browser mutation, child intervention
execution, or enforcement.

## Context Stream Addendum - 2026-06-07

`browser-runtime-context-stream-proof` carries browser read-model context through
the reusable browser runtime event chain. Each event payload now includes
capability status, custody label, query visibility, and degraded reason so
subscribers can distinguish exact/live rows from manual-required, unavailable,
or degraded rows without reading a parallel source.

Evidence:

- `crates/agent-core/src/browser_event_runtime.rs`
- `crates/agent-service/src/browser_runtime_delivery.rs`
- `crates/agent-service/src/browser_runtime_stream_events.rs`
- `packages/agent-protocol-domain/src/browser-runtime-events.ts`
- `apps/portal/tests/live-activity-state.test.ts`
- `scripts/test/browser-runtime-context-stream-proof.mjs`
- `test-results/browser-runtime-context-stream-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-context-stream/01-browser-runtime-context-stream-proof.md`
- `cmd /c node scripts/test/browser-runtime-context-stream-proof.mjs`

The proof validates that unsupported exact URL context is rejected, unavailable
context needs a degraded reason, and portal state receives the same context from
the typed protocol parser. This does not create a new event bus, execute AI,
execute policy, mutate the browser, execute child intervention, or enforce.

## Dry-Run Action Handoff Addendum - 2026-06-07

`browser-runtime-dry-run-action-handoff-proof` extends the browser runtime
event-chain payload with policy preview id, parent action-intent id, `dryRun`,
and `adapterDispatchClaimed` fields. A dry-run policy/action handoff can now
publish evidence, journal, policy-evaluation, policy-decision, audit, and
read-model phases while the intervention command and result phases stay absent.

Evidence:

- `crates/agent-core/src/browser_event_runtime.rs`
- `crates/agent-service/src/browser_runtime_delivery.rs`
- `crates/agent-service/src/browser_runtime_stream_events.rs`
- `packages/agent-protocol-domain/src/browser-runtime-events.ts`
- `scripts/test/browser-runtime-dry-run-action-handoff-proof.mjs`
- `test-results/browser-runtime-dry-run-action-handoff-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-dry-run-action-handoff/01-browser-runtime-dry-run-action-handoff-proof.md`
- `cmd /c node scripts/test/browser-runtime-dry-run-action-handoff-proof.mjs`

The proof validates that dry-run policy/action rows can publish policy phases
without publishing intervention command/result phases, rejects forged dry-run
adapter dispatch or intervention refs in the protocol parser, and keeps
store-backed read-model stream rows non-dispatching. This does not create a new
event bus, publish portal business events, execute AI, execute final policy
actions, mutate the browser, execute child intervention, or enforce.

## Action-Intent Outbox Handoff Addendum - 2026-06-07

`browser-runtime-action-intent-outbox-handoff-proof` maps dry-run browser policy
decision events that carry `policyPreviewId` and `assistantActionIntentId` into
prepared local action-intent outbox candidates. The candidate preserves the
policy preview ref, action intent ref, source event ref, outbox ref, and handoff
ref so later browser subscribers can reason about pending work without reading a
parallel source.

Evidence:

- `crates/agent-core/src/browser_event_runtime.rs`
- `crates/agent-core/src/browser_event_runtime/topology.rs`
- `crates/agent-core/src/browser_event_runtime_tests.rs`
- `scripts/test/browser-runtime-action-intent-outbox-handoff-proof.mjs`
- `test-results/browser-runtime-action-intent-outbox-handoff-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-action-intent-outbox-handoff/01-browser-runtime-action-intent-outbox-handoff-proof.md`
- `cargo test -p ocentra-parent-agent-core browser_runtime_action_intent --quiet`
- `cargo test -p ocentra-parent-agent-core browser_runtime_chain_carries_dry_run --quiet`

The proof keeps `dispatchAttemptCount`, `adapterExecutionCount`,
`childInterventionExecutionCount`, and `enforcementExecutionCount` at zero. It
does not create a generic event bus, implement external transport, execute final
policy, mutate browser state, execute child intervention, or enforce.

## Action-Intent Status Bridge Addendum - 2026-06-07

`browser-runtime-action-intent-status-bridge-proof` adds a typed
protocol-domain subscriber projection over the existing browser runtime
event-chain stream. It derives pending action-intent candidates only from
dry-run policy decision events that carry a policy preview id and parent
action-intent id, preserving the stream event ref, source ref, evidence ref, and
observed-at timestamp.

Evidence:

- `packages/agent-protocol-domain/src/browser-runtime-events.ts`
- `packages/agent-protocol-domain/tests/browser-runtime-events.test.ts`
- `scripts/test/browser-runtime-action-intent-status-bridge-proof.mjs`
- `test-results/browser-runtime-action-intent-status-bridge-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-action-intent-status-bridge/01-browser-runtime-action-intent-status-bridge-proof.md`
- `cmd /c node scripts/test/browser-runtime-action-intent-status-bridge-proof.mjs`

The projection uses the existing browser runtime stream command/event instead
of adding a new command family. It keeps dispatch attempts, adapter execution,
child intervention execution, and enforcement at zero. It does not create a new
generic event bus, implement external transport, execute final policy, mutate
browser state, execute child intervention, or enforce.

## Runtime Stream Topology And Delivery Refresh - 2026-06-08

`browser-runtime-chain-topology-proof` now registers the local
`browser.runtime.stream.report.requested` request boundary in the reusable
`ocentra-eventing` topology manifest, with `browser-event-runtime-spine` as
publisher and `browser-runtime-stream-report` as subscriber/target.
`browser-runtime-delivery-decision-proof` also treats that route as
`local-in-process`, adding the stream-report route to the browser local-ready
delivery set while leaving external transport manual-required.

Evidence:

- `crates/agent-core/src/browser_event_runtime/topology.rs`
- `crates/agent-core/src/browser_event_runtime/delivery.rs`
- `crates/agent-core/src/browser_event_runtime_tests.rs`
- `scripts/test/browser-runtime-chain-topology-proof.mjs`
- `scripts/test/browser-runtime-delivery-decision-proof.mjs`
- `test-results/browser-runtime-chain-topology-proof/proof.json`
- `test-results/browser-runtime-delivery-decision-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-chain-topology/01-browser-runtime-chain-topology-proof.md`
- `output/browser-plan-proof/browser-runtime-delivery-decision/01-browser-runtime-delivery-decision-proof.md`

This refresh registers the service stream request route only. It does not change
the portal WebSocket command, add external transport, dispatch adapters, mutate
browser state, execute child intervention, execute final policy, or enforce.

## Action-Intent Event Subscriber Addendum - 2026-06-07

`browser-runtime-action-intent-event-subscriber-proof` adds a named Rust
event-bus request/response subscriber for browser action-intent status. It
publishes `browser.action-intent.status.requested`, routes it to the
`browser-action-intent-status` subscriber, and completes a typed response using
the reusable `ocentra-eventing` request path.

Evidence:

- `crates/agent-core/src/browser_event_runtime/action_status.rs`
- `crates/agent-core/src/browser_event_runtime.rs`
- `crates/agent-core/src/browser_event_runtime_tests.rs`
- `crates/agent-protocol/src/constants/browser.rs`
- `scripts/test/browser-runtime-action-intent-event-subscriber-proof.mjs`
- `test-results/browser-runtime-action-intent-event-subscriber-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-action-intent-event-subscriber/01-browser-runtime-action-intent-event-subscriber-proof.md`
- `cargo test -p ocentra-parent-agent-core browser_runtime_action_intent_event_subscriber --quiet`

The subscriber returns one pending candidate for dry-run policy decision events
with policy preview and action-intent refs, and zero candidates for
manual-required rows. It does not execute adapter dispatch, final policy,
browser mutation, child intervention, or enforcement.

## Action-Intent Handoff Event Subscriber Addendum - 2026-06-07

`browser-runtime-action-intent-handoff-event-subscriber-proof` adds a named
Rust event-bus request/response subscriber for browser action-intent handoff
preparation. It publishes `browser.action-intent.handoff.requested`, routes it
to the `browser-action-intent-handoff` subscriber, and completes a typed
response using the reusable `ocentra-eventing` request path.

Evidence:

- `crates/agent-core/src/browser_event_runtime/action_handoff.rs`
- `crates/agent-core/src/browser_event_runtime/delivery.rs`
- `crates/agent-core/src/browser_event_runtime.rs`
- `crates/agent-core/src/browser_event_runtime_tests.rs`
- `crates/agent-protocol/src/constants/browser.rs`
- `scripts/test/browser-runtime-action-intent-handoff-event-subscriber-proof.mjs`
- `test-results/browser-runtime-action-intent-handoff-event-subscriber-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-action-intent-handoff-event-subscriber/01-browser-runtime-action-intent-handoff-event-subscriber-proof.md`
- `cargo test -p ocentra-parent-agent-core browser_runtime_action_intent_handoff --quiet`
- `cargo test -p ocentra-parent-agent-core browser_runtime_delivery_decision --quiet`

The subscriber returns one prepared local outbox/handoff candidate for dry-run
policy decision events with policy preview and action-intent refs, and zero
candidates for manual-required rows. The delivery decision proof now includes
the handoff subscriber as a third local-ready route. It does not execute adapter
dispatch, final policy, browser mutation, child intervention, or enforcement.

## Action-Intent Service Status Addendum - 2026-06-07

`browser-runtime-action-intent-service-status-proof` keeps the existing
service-backed browser runtime event-chain stream command and enriches its
payload with action-intent status counters from the named Rust event-bus
subscriber. Current store-backed browser evidence rows still project zero
pending action-intent candidates because the evidence read model does not carry
policy preview or parent action-intent refs yet; a dry-run action-intent input
projects one pending candidate through the same service payload.

Evidence:

- `crates/agent-service/src/browser_runtime_stream_payload.rs`
- `crates/agent-service/src/browser_runtime_stream_tests.rs`
- `crates/agent-protocol/src/constants/field.rs`
- `packages/agent-protocol-domain/src/defaults.ts`
- `scripts/test/browser-runtime-action-intent-service-status-proof.mjs`
- `test-results/browser-runtime-action-intent-service-status-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-action-intent-service-status/01-browser-runtime-action-intent-service-status-proof.md`
- `cargo test -p ocentra-parent-agent-service service_browser_runtime_action_intent_status --quiet`

The service payload keeps dispatch attempts, adapter execution, child
intervention execution, final policy execution, browser mutation, and
enforcement at zero. It does not add a new browser command family, execute
policy, mutate browser state, execute child intervention, or enforce.

## Action-Intent Durable Handoff Result Addendum - 2026-06-07

`browser-runtime-action-intent-durable-handoff-proof` carries the named
browser action-intent handoff subscriber result into a durable result/read-model
row. The row preserves the request event, policy preview id, parent
action-intent id, source event ref, local outbox ref, local handoff ref,
durable result ref, durable store ref, read-model ref, and support-status ref.
Duplicate request event ids are rejected before projection.

Evidence:

- `crates/agent-core/src/browser_event_runtime/action_handoff_durable.rs`
- `crates/agent-core/src/browser_event_runtime/action_handoff_durable_types.rs`
- `crates/agent-core/src/browser_event_runtime.rs`
- `crates/agent-core/src/browser_event_runtime_tests.rs`
- `crates/agent-protocol/src/constants/browser.rs`
- `scripts/test/browser-runtime-action-intent-durable-handoff-proof.mjs`
- `test-results/browser-runtime-action-intent-durable-handoff-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-action-intent-durable-handoff/01-browser-runtime-action-intent-durable-handoff-proof.md`
- `cargo test -p ocentra-parent-agent-core browser_runtime_action_intent_durable_handoff --quiet`

The durable proof does not add external transport, dispatch attempts, adapter
execution, browser mutation, child intervention execution, final policy
execution, or enforcement. It is a local durable/read-model handoff proof for
later runtime subscribers.

## Action-Intent Store-Backed Policy Preview Addendum - 2026-06-07

`browser-runtime-action-intent-store-backed-proof` closes the prior
store-backed action-intent projection gap for browser evidence rows that have a
matching stored policy preview row. The service-backed browser runtime stream
loads the existing browser evidence read model and the existing policy preview
read model from the `ActivityStore`; matching evidence refs enrich the browser
runtime input with policy preview, policy decision, stable browser
action-intent, dry-run, and policy-authority refs before the named
action-intent status subscriber reports one pending candidate.

Evidence:

- `crates/agent-protocol/src/constants/browser.rs`
- `crates/agent-service/src/browser_runtime_delivery.rs`
- `crates/agent-service/src/browser_runtime_stream_payload.rs`
- `crates/agent-service/src/browser_runtime_stream_api.rs`
- `crates/agent-service/src/browser_runtime_stream_tests.rs`
- `crates/agent-service/src/policy_preview_api.rs`
- `scripts/test/browser-runtime-action-intent-store-backed-proof.mjs`
- `test-results/browser-runtime-action-intent-store-backed-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-action-intent-store-backed/01-browser-runtime-action-intent-store-backed-proof.md`
- `cmd /c node scripts/test/browser-runtime-action-intent-store-backed-proof.mjs`

This supersedes the earlier store-backed zero-candidate limitation only for
rows with a matching stored policy preview evidence reference. Dispatch
attempts, adapter execution, browser mutation, child intervention execution,
final policy execution, and enforcement remain zero.

## Action-Intent Portal State Addendum - 2026-06-07

`browser-runtime-action-intent-portal-state-proof` verifies that the shared
TypeScript protocol parser and portal live-activity state consume the browser
runtime action-intent service counters from the existing event-chain stream
payload. Pending candidate counts are allowed, while dispatch attempts, adapter
execution, child intervention execution, and enforcement execution must stay
zero.

Evidence:

- `packages/agent-protocol-domain/src/browser-runtime-events.ts`
- `packages/agent-protocol-domain/tests/browser-runtime-events.test.ts`
- `apps/portal/tests/live-activity-state.test.ts`
- `scripts/test/browser-runtime-action-intent-portal-state-proof.mjs`
- `test-results/browser-runtime-action-intent-portal-state-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-action-intent-portal-state/01-browser-runtime-action-intent-portal-state-proof.md`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- browser-runtime-events.test.ts`
- `cmd /c npm run test --workspace @ocentra-parent/portal -- live-activity-state.test.ts`

This is protocol/parser and portal state proof only. It does not add a new
portal visual surface, execute policy, mutate browser state, execute child
intervention, or enforce.

## Action-Intent Service Handoff Addendum - 2026-06-07

`browser-runtime-action-intent-service-handoff-proof` extends the
service-backed browser runtime path so the service asks the named
`browser.action-intent.handoff.requested` subscriber and records prepared local
outbox/handoff refs in report state for store-backed dry-run policy preview
rows.

Evidence:

- `crates/agent-service/src/browser_runtime_stream_payload.rs`
- `crates/agent-service/src/browser_runtime_stream_tests.rs`
- `crates/agent-core/src/browser_event_runtime/action_handoff.rs`
- `scripts/test/browser-runtime-action-intent-service-handoff-proof.mjs`
- `test-results/browser-runtime-action-intent-service-handoff-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-action-intent-service-handoff/01-browser-runtime-action-intent-service-handoff-proof.md`
- `cargo test -p ocentra-parent-agent-service service_browser_runtime_action_intent_status --quiet`
- `cargo test -p ocentra-parent-agent-service service_browser_runtime_stream_projects_store_backed_policy_preview_candidate --quiet`

The public wire shape is intentionally unchanged in this slice because shared
protocol field constants/defaults are owned by another active lane. The service
still keeps dispatch attempts, adapter execution, browser mutation, child
intervention execution, final policy execution, and enforcement at zero.

## Action-Intent Topology Addendum - 2026-06-07

`browser-runtime-action-intent-topology-proof` registers the named browser
action-intent status request event with the reusable `ocentra-eventing`
contract registry and topology manifest. The manifest declares
`browser-event-runtime-spine` as publisher and `browser-action-intent-status`
as the subscriber/target for `browser.action-intent.status.requested`, with no
unready topology entries.

Evidence:

- `crates/agent-core/src/browser_event_runtime/action_status.rs`
- `crates/agent-core/src/browser_event_runtime_tests.rs`
- `scripts/test/browser-runtime-action-intent-topology-proof.mjs`
- `test-results/browser-runtime-action-intent-topology-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-action-intent-topology/01-browser-runtime-action-intent-topology-proof.md`
- `cargo test -p ocentra-parent-agent-core browser_runtime_action_intent_topology_covers_named_event_and_subscriber --quiet`

This is topology/registry proof only. It does not add another browser bus,
external transport, adapter dispatch, browser mutation, child intervention
execution, final policy execution, or enforcement.

## Runtime Chain Topology Addendum - 2026-06-07

`browser-runtime-chain-topology-proof` registers the existing ordered browser
runtime chain with the reusable `ocentra-eventing` contract registry and
topology manifest. The manifest covers the ten current browser runtime phases
from evidence observed through read-model projected, all published by
`browser-event-runtime-spine` and all subscribed by their named phase
subscribers/targets.

Evidence:

- `crates/agent-core/src/browser_event_runtime.rs`
- `crates/agent-core/src/browser_event_runtime_tests.rs`
- `scripts/test/browser-runtime-chain-topology-proof.mjs`
- `test-results/browser-runtime-chain-topology-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-chain-topology/01-browser-runtime-chain-topology-proof.md`
- `cargo test -p ocentra-parent-agent-core browser_runtime_chain_topology_covers_ordered_event_spine --quiet`

This is topology/registry proof for the existing local chain only. It does not
add external transport, adapter dispatch, browser mutation, child intervention
execution, final policy execution, or enforcement.

## Delivery Decision Addendum - 2026-06-07

`browser-runtime-delivery-decision-proof` applies the reusable
`ocentra-eventing` delivery decision API to the browser runtime chain, the
browser action-intent status subscriber, the browser action-intent handoff
subscriber, the browser social-provider receipt status subscriber, and the
browser social parent-notification delivery status subscriber. The current
runtime chain is `local-service` ready, the action-intent status subscriber is
`local-in-process` ready, the action-intent handoff subscriber is
`local-in-process` ready, the social-provider receipt status subscriber is
`local-in-process` ready, the social parent-notification delivery status
subscriber is `local-in-process` ready, and the external transport route stays
`manual-required` because the custody/auth/encryption/retention/replay/delete/
offset/dedupe/transport artifacts are not present.

Evidence:

- `crates/agent-core/src/browser_event_runtime/delivery.rs`
- `crates/agent-core/src/browser_event_runtime_tests.rs`
- `scripts/test/browser-runtime-delivery-decision-proof.mjs`
- `test-results/browser-runtime-delivery-decision-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-delivery-decision/01-browser-runtime-delivery-decision-proof.md`
- `cargo test -p ocentra-parent-agent-core browser_runtime_delivery_decision_keeps_current_routes_local_only --quiet`

This is delivery-decision proof only. It does not add external transport,
relay delivery, adapter dispatch, browser mutation, child intervention
execution, final policy execution, or enforcement.

## Social Parent Notification Delivery Decision Refresh - 2026-06-08

`browser-runtime-delivery-decision-proof` now carries the internal
`browser.social.parent-notification-delivery.status.requested` route into the
delivery decision report as a sixth local-ready browser route. The route is
local in-process from `browser-event-runtime-spine` to
`browser-social-parent-notification-delivery-status`, matching the
service-backed read-model request/subscriber boundary used by
`social-parent-notification-delivery-ui-proof`.

Evidence:

- `crates/agent-protocol/src/constants/browser.rs`
- `crates/agent-core/src/browser_event_runtime/delivery.rs`
- `crates/agent-core/src/browser_event_runtime_tests.rs`
- `scripts/test/browser-runtime-delivery-decision-proof.mjs`
- `test-results/browser-runtime-delivery-decision-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-delivery-decision/01-browser-runtime-delivery-decision-proof.md`

This is delivery-decision proof only. It does not add external transport,
relay delivery, adapter dispatch, browser mutation, parent notification UI
delivery, child intervention execution, final policy execution, or enforcement.

## Receipt Delivery Decision Refresh - 2026-06-08

The delivery decision proof now includes the existing named
`browser.social.provider-receipt.status.requested` route as a local-in-process
eventing route from the browser runtime spine to the
browser-social-provider-receipt-status subscriber. This aligns the receipt
status service/UI proof path with the shared Rust eventing route-decision model
without adding provider dispatch, provider receipt ingestion, webhook runtime,
parent notification delivery, report delivery, browser mutation, final policy
execution, child intervention execution, or enforcement.

## Stale Unsupported Runtime Addendum - 2026-06-07

`browser-runtime-stale-unsupported-proof` closes the runtime/read-model proof gap
for bridge-disconnected stale state and unsupported later-adapter rows. The
managed status helper now reports bridge disconnect as explicit `stale`
capability instead of conflating it with launch/connect-pending bridge-missing
state. Inventory/read-model tests prove stale bridge rows stay
manual-required, unsupported later-adapter rows stay unsupported/not-claimed, and
runtime delivery plus service stream tests keep both rows parent-visible with
zero exact URL rows, zero intervention command events, zero adapter execution,
zero child intervention execution, and zero enforcement execution.

Evidence:

- `crates/agent-service/src/browser_runtime_status.rs`
- `crates/agent-service/src/browser_runtime_tests.rs`
- `crates/agent-service/src/browser_inventory_read_model_tests.rs`
- `crates/agent-service/src/browser_runtime_delivery_tests.rs`
- `crates/agent-service/src/browser_runtime_stream_tests.rs`
- `packages/agent-protocol-domain/tests/browser-runtime-events.test.ts`
- `scripts/test/browser-runtime-stale-unsupported-proof.mjs`
- `test-results/browser-runtime-stale-unsupported-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-stale-unsupported/01-browser-runtime-stale-unsupported-proof.md`
- `cmd /c node scripts/test/browser-runtime-stale-unsupported-proof.mjs`

This is stale/unsupported runtime proof only. It does not claim real non-Windows
platform support, exact active-tab enforcement, host blocking, browser mutation,
child intervention execution, final policy execution, AI authority, or
enforcement.

## Action-Intent Durable Status Addendum - 2026-06-07

`browser-runtime-action-intent-durable-status-proof` carries the prepared
browser action-intent handoff status through the existing service-backed browser
runtime event-chain stream and portal live-activity parser. The stream exposes
prepared handoff candidate count, local outbox refs, and handoff refs while
keeping dispatch, adapter execution, browser mutation, child intervention
execution, final policy execution, and enforcement at zero/unclaimed.

Evidence:

- `crates/agent-protocol/src/constants/field.rs`
- `packages/agent-protocol-domain/src/defaults.ts`
- `crates/agent-service/src/browser_runtime_stream_payload.rs`
- `crates/agent-service/src/browser_runtime_stream_tests.rs`
- `packages/agent-protocol-domain/src/browser-runtime-events.ts`
- `packages/agent-protocol-domain/tests/browser-runtime-events.test.ts`
- `apps/portal/tests/live-activity-state.test.ts`
- `scripts/test/browser-runtime-action-intent-durable-status-proof.mjs`
- `test-results/browser-runtime-action-intent-durable-status-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-action-intent-durable-status/01-browser-runtime-action-intent-durable-status-proof.md`
- `cargo test -p ocentra-parent-agent-service service_browser_runtime_stream_projects_store_backed_policy_preview_candidate --quiet`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- browser-runtime-events.test.ts`
- `cmd /c npm run test --workspace @ocentra-parent/portal -- live-activity-state.test.ts`

This is stream/read-model status only. It does not execute final policy,
dispatch an adapter, mutate browser state, execute child intervention, or
enforce.

## Historical/Test-Only Parent-Child Action-Intent Handoff Addendum - 2026-06-08

This is a historical/test-only fixture description, not current production
runtime evidence. The fixture describes the intended browser action-intent
sequence with a named `browser-action-intent-handoff` child command kind. At
the current Eventing WP08 boundary,
`crates/agent-core/src/parent_child_event_runtime.rs` returns
`EventingError::NoSubscriber`; no registered consumer, durable child
acceptance, child transport handoff, or parent read-model projection exists.
The decoder and assertions below therefore remain fixture/test-only and do not
establish a shipped parent/child runtime.

Evidence:

- `crates/agent-protocol/src/child_agent_events.rs`
- `crates/agent-protocol/src/child_agent_event_tests.rs`
- `crates/agent-core/src/parent_child_event_runtime.rs`
- `crates/agent-core/src/parent_child_event_runtime_tests.rs`
- `scripts/test/browser-runtime-parent-child-action-intent-handoff-proof.mjs`
- `test-results/browser-runtime-parent-child-action-intent-handoff-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-parent-child-action-intent-handoff/01-browser-runtime-parent-child-action-intent-handoff-proof.md`
- `cargo test -p ocentra-parent-agent-protocol child_agent_contracts_serialize_browser_action_intent_handoff_kind --quiet`
- `cargo test -p ocentra-parent-agent-core browser_action_intent_handoff_uses_parent_child_event_sequence_without_execution --quiet`

This is historical typed parent/child fixture coverage only. It does not add a
registered local consumer, durable child acceptance, external broker or relay
delivery, adapter dispatch, browser mutation, child intervention execution,
final policy execution, unmanaged exact URL support, or enforcement.

## Historical/Test-Only Action-Intent Child Status Addendum - 2026-06-08

This is also historical/test-only fixture coverage. The former composition
described by `browser-runtime-action-intent-child-status-proof` expected durable
result/read-model refs, child receive/acceptance refs, and parent read-model
projection visibility, but the current parent-child runtime is fail-closed with
`EventingError::NoSubscriber`. It does not establish a registered consumer,
durable child acceptance, transport, or production parent read-model path.

Evidence:

- `crates/agent-core/src/browser_event_runtime/action_handoff_child_status.rs`
- `crates/agent-core/src/browser_event_runtime/action_handoff_child_status_types.rs`
- `crates/agent-core/src/browser_event_runtime_tests.rs`
- `scripts/test/browser-runtime-action-intent-child-status-proof.mjs`
- `test-results/browser-runtime-action-intent-child-status-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-action-intent-child-status/01-browser-runtime-action-intent-child-status-proof.md`
- `cargo test -p ocentra-parent-agent-core browser_runtime_action_intent_child_status_links_durable_handoff_to_child_acceptance --quiet`

This is agent-core durable/status composition proof only. Nonzero public stream
fields for child acceptance refs remain a follow-up until a real child
transport/status read model exists. It does not add external broker or relay
delivery, adapter dispatch, browser mutation, child intervention execution,
final policy execution, unmanaged exact URL support, or enforcement.

## Action-Intent Child Status Public Stream Addendum - 2026-06-08

`browser-runtime-action-intent-child-status-public-stream-proof` exposes the
browser action-intent child-status boundary through the service-backed public
stream without promoting fixture-backed child acceptance refs into runtime
state. The stream now carries accepted child row count plus child command, child
accepted-event, and parent read-model ref arrays. The current service-backed
runtime reports zero accepted child rows and empty ref arrays, and the shared
protocol parser rejects mismatched child-status counts before portal state
accepts the stream.

Evidence:

- `crates/agent-protocol/src/constants/field.rs`
- `crates/agent-service/src/browser_runtime_stream_payload.rs`
- `crates/agent-service/src/browser_runtime_stream_tests.rs`
- `packages/agent-protocol-domain/src/defaults.ts`
- `packages/agent-protocol-domain/src/browser-runtime-events.ts`
- `packages/agent-protocol-domain/tests/browser-runtime-events.test.ts`
- `apps/portal/tests/live-activity-state.test.ts`
- `scripts/test/browser-runtime-action-intent-child-status-public-stream-proof.mjs`
- `test-results/browser-runtime-action-intent-child-status-public-stream-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-action-intent-child-status-public-stream/01-browser-runtime-action-intent-child-status-public-stream-proof.md`
- `cargo test -p ocentra-parent-agent-service service_browser_runtime_action_intent_status_projects_pending_candidate --quiet`
- `cargo test -p ocentra-parent-agent-service websocket_browser_runtime_stream_command_reports_store_backed_chain --quiet`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- browser-runtime-events.test.ts`
- `cmd /c npm run test --workspace @ocentra-parent/portal -- live-activity-state.test.ts`

This is no-observation stream status only. It does not add external child
transport, adapter dispatch, browser mutation, child intervention execution,
final policy execution, unmanaged exact URL support, or enforcement.

## No Fixture Service Exposure Addendum - 2026-06-08

`browser-runtime-no-fixture-service-exposure-proof` guards the child-status
boundary from becoming a fake runtime claim. The child-status composition stays
`#[cfg(test)]`, the service-backed browser runtime stream does not call the
fixture-backed proof, and the shared protocol parser plus portal state expose
only no-observation child-status fields until a real child transport/status
read model exists.

Evidence:

- `scripts/test/browser-runtime-no-fixture-service-exposure-proof.mjs`
- `test-results/browser-runtime-no-fixture-service-exposure-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-no-fixture-service-exposure/01-browser-runtime-no-fixture-service-exposure-proof.md`
- `cmd /c node scripts/test/browser-runtime-no-fixture-service-exposure-proof.mjs`

This is a no-overclaim guard only. It does not add nonzero public child-status
refs, external transport, adapter dispatch, browser mutation, child intervention
execution, final policy execution, unmanaged exact URL support, or enforcement.

## Event Name Parity Addendum - 2026-06-08

`browser-runtime-event-name-parity-proof` aligns the shared TypeScript browser
runtime stream parser with the Rust browser runtime event type constants. The
parser now accepts the same dotted AI, policy, intervention, audit, evidence,
and read-model event names emitted by `crates/agent-protocol` and
`agent-core`, and the focused contract test covers all ten browser runtime
phases so later AI/policy/intervention stream entries cannot drift from the
Rust event bus boundary.

Evidence:

- `packages/agent-protocol-domain/src/browser-runtime-events.ts`
- `packages/agent-protocol-domain/tests/browser-runtime-events.test.ts`
- `scripts/test/browser-runtime-event-name-parity-proof.mjs`
- `test-results/browser-runtime-event-name-parity-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-event-name-parity/01-browser-runtime-event-name-parity-proof.md`
- `cmd /c node scripts/test/browser-runtime-event-name-parity-proof.mjs`

This is protocol parser parity proof only. It does not add a generic event bus,
portal UI, external transport, adapter dispatch, browser mutation, child
intervention execution, final policy execution, unmanaged exact URL support, AI
execution, or enforcement.

## Social Provider Receipt Event Subscriber Addendum - 2026-06-08

`browser-runtime-social-provider-receipt-event-subscriber-proof` adds a named
Rust event-bus request/response subscriber for the social alert/report provider
receipt boundary. The browser runtime publishes
`browser.social.provider-receipt.status.requested`; the
`browser-social-provider-receipt-status` subscriber returns a typed status
response. Dry-run action-intent rows become provider-dispatch-required receipt
boundary rows, while manual-required rows stay manual-receipt-required.

Evidence:

- `crates/agent-protocol/src/constants/browser.rs`
- `crates/agent-core/src/browser_event_runtime/social_provider_receipt.rs`
- `crates/agent-core/src/browser_event_runtime.rs`
- `crates/agent-core/src/browser_event_runtime_tests/browser_event_runtime_social_provider_receipt_tests.rs`
- `scripts/test/browser-runtime-social-provider-receipt-event-subscriber-proof.mjs`
- `test-results/browser-runtime-social-provider-receipt-event-subscriber-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-social-provider-receipt-event-subscriber/01-browser-runtime-social-provider-receipt-event-subscriber-proof.md`
- `cargo test -p ocentra-parent-agent-core browser_runtime_social_provider_receipt --quiet`

This is named event/subscriber boundary proof only. It does not claim external
provider delivery, provider receipt ingestion runtime, provider webhook runtime,
provider credentials, parent notification UI delivery, report delivery
execution, final policy execution, connector/native runtime, browser mutation,
child intervention execution, unmanaged exact URL support, or enforcement.

## Social Provider Receipt Service Status Addendum - 2026-06-08

`browser-runtime-social-provider-receipt-service-status-proof` carries the named
social provider receipt status subscriber into the existing service-side browser
runtime stream report. The service now asks
`browser.social.provider-receipt.status.requested` for each browser runtime
input, records provider-dispatch-required receipt boundary rows for store-backed
dry-run policy preview evidence, records manual-receipt-required rows for
manual-required browser evidence, and preserves provider attempt/receipt proof
refs internally.

Evidence:

- `crates/agent-service/src/browser_runtime_stream_payload.rs`
- `crates/agent-service/src/browser_runtime_stream_tests/browser_runtime_social_provider_receipt_service_status_tests.rs`
- `scripts/test/browser-runtime-social-provider-receipt-service-status-proof.mjs`
- `test-results/browser-runtime-social-provider-receipt-service-status-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-social-provider-receipt-service-status/01-browser-runtime-social-provider-receipt-service-status-proof.md`
- `cargo test -p ocentra-parent-agent-service social_provider_receipt --quiet`

This is service-side status projection only. It intentionally does not add public
browser runtime stream fields while protocol field constants are owned by
another active lane. It does not claim provider delivery, provider receipt
ingestion runtime, webhook runtime, credentials, parent notification UI delivery,
report delivery execution, final policy execution, connector/native runtime,
browser mutation, child intervention execution, unmanaged exact URL support, or
enforcement.

## Social Provider Receipt Durable Addendum - 2026-06-08

`browser-runtime-social-provider-receipt-durable-proof` projects the named
`browser.social.provider-receipt.status.requested` subscriber result into a
durable receipt read-model row. The row preserves the request event,
correlation, parent action-intent, provider attempt, receipt proof, durable
result, durable store, read-model, support-status, source, and evidence refs,
and rejects duplicate request event ids before read-model projection.

Evidence:

- `crates/agent-core/src/browser_event_runtime/social_provider_receipt_durable.rs`
- `crates/agent-core/src/browser_event_runtime/social_provider_receipt_durable_types.rs`
- `crates/agent-core/src/browser_event_runtime_tests.rs`
- `scripts/test/browser-runtime-social-provider-receipt-durable-proof.mjs`
- `test-results/browser-runtime-social-provider-receipt-durable-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-social-provider-receipt-durable/01-browser-runtime-social-provider-receipt-durable-proof.md`
- `cargo test -p ocentra-parent-agent-core social_provider_receipt_durable --quiet`

This is durable/read-model proof only. It intentionally does not claim provider
delivery, provider receipt ingestion runtime, webhook runtime, credentials,
parent notification UI delivery, report delivery execution, final policy
execution, connector/native runtime, browser mutation, child intervention
execution, unmanaged exact URL support, or enforcement.

## Social Provider Receipt Service Durable Status Addendum - 2026-06-08

`browser-runtime-social-provider-receipt-service-durable-status-proof` carries
the durable social provider receipt result/read-model refs into the existing
service-side browser runtime report. Provider-dispatch-required receipt boundary
rows now preserve durable result, durable store, read-model, and support-status
refs in service report state; manual-required receipt rows keep durable rows and
durable refs empty.

Evidence:

- `crates/agent-service/src/browser_runtime_stream_payload.rs`
- `crates/agent-service/src/browser_runtime_stream_tests/browser_runtime_social_provider_receipt_service_status_tests.rs`
- `scripts/test/browser-runtime-social-provider-receipt-service-durable-status-proof.mjs`
- `test-results/browser-runtime-social-provider-receipt-service-durable-status-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-social-provider-receipt-service-durable-status/01-browser-runtime-social-provider-receipt-service-durable-status-proof.md`
- `cargo test -p ocentra-parent-agent-service social_provider_receipt --quiet`

This is service-side durable status proof only. It intentionally does not add
public protocol or portal stream fields while the shared protocol defaults file
is owned by another active lane. It does not claim provider delivery, provider
receipt ingestion runtime, webhook runtime, credentials, parent notification UI
delivery, report delivery execution, final policy execution, connector/native
runtime, browser mutation, child intervention execution, unmanaged exact URL
support, or enforcement.

## Social Provider Receipt Service Public Fields Addendum - 2026-06-08

`browser-runtime-social-provider-receipt-service-public-fields-proof` adds Rust
protocol field constants and service payload fields for the existing social
provider receipt status path. The payload now exposes social provider receipt
boundary rows, provider-dispatch-required rows, manual-receipt-required rows,
provider attempt refs, receipt proof refs, durable rows, durable result refs,
durable store refs, read-model refs, and support-status refs. Manual-required
receipt rows publish zero durable rows and empty durable refs.

Evidence:

- `crates/agent-protocol/src/constants/field.rs`
- `crates/agent-service/src/browser_runtime_stream_payload.rs`
- `crates/agent-service/src/browser_runtime_stream_tests/browser_runtime_social_provider_receipt_service_status_tests.rs`
- `scripts/test/browser-runtime-social-provider-receipt-service-public-fields-proof.mjs`
- `test-results/browser-runtime-social-provider-receipt-service-public-fields-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-social-provider-receipt-service-public-fields/01-browser-runtime-social-provider-receipt-service-public-fields-proof.md`
- `cargo test -p ocentra-parent-agent-service social_provider_receipt --quiet`

This is Rust protocol/service payload exposure only. It intentionally does not
update TypeScript defaults, the shared TypeScript parser, or portal state while
the shared protocol defaults file is owned by another active lane. It does not
claim provider delivery, provider receipt ingestion runtime, webhook runtime,
credentials, parent notification UI delivery, report delivery execution, final
policy execution, connector/native runtime, browser mutation, child intervention
execution, unmanaged exact URL support, or enforcement.

## Social Provider Receipt Ingestion Readiness Addendum - 2026-06-08

`social-alert-report-provider-receipt-ingestion-readiness-proof` adds a
parent-domain readiness boundary after the social provider receipt boundary. It
projects provider-dispatch-required, manual-receipt-required, and
provider-unavailable receipt rows into ingestion-readiness rows that require
webhook contract, provider credential proof, and durable receipt store proof
before any provider receipt can be observed or ingested.

Evidence:

- `packages/parent-domain/src/social-alert-report-provider-receipt-ingestion-readiness.ts`
- `packages/parent-domain/tests/social-alert-report-provider-receipt-ingestion-readiness.test.ts`
- `scripts/test/social-alert-report-provider-receipt-ingestion-readiness-proof.mjs`
- `test-results/social-alert-report-provider-receipt-ingestion-readiness-proof/proof.json`
- `output/browser-plan-proof/social-alert-report-provider-receipt-ingestion-readiness-proof/01-social-alert-report-provider-receipt-ingestion-readiness-proof.md`
- `cmd /c node scripts/test/social-alert-report-provider-receipt-ingestion-readiness-proof.mjs`

This is readiness/status proof only. It does not claim provider delivery,
provider receipt ingestion runtime, provider webhook runtime, provider
credentials, observed provider receipts, cloud routing, parent notification UI
delivery, report delivery execution, final policy execution,
connector/native runtime, browser mutation, child intervention execution,
unmanaged exact URL support, or enforcement. Parent-domain package subpath
export remains deferred while another active lane owns
`packages/parent-domain/package.json`.

## Social Provider Receipt Stream Parser Addendum - 2026-06-08

## Action-Intent Child Status Service Stream Addendum - 2026-06-08

`browser-runtime-action-intent-child-status-public-stream-proof` now carries
the browser action-intent child-status boundary through the service-backed
parent-child event path. After preparing a dry-run handoff candidate, the
service asks the input-driven child-status request and publishes child command,
child accepted-event, and parent read-model refs in the browser runtime stream.
Normal/manual rows remain zero/empty, and the service does not call the
fixture-backed child-status proof.

Evidence:

- `crates/agent-core/src/browser_event_runtime/action_handoff_child_status.rs`
- `crates/agent-core/src/browser_event_runtime/action_handoff_child_status_types.rs`
- `crates/agent-service/src/browser_runtime_stream_payload.rs`
- `crates/agent-service/src/browser_runtime_stream_tests.rs`
- `scripts/test/browser-runtime-action-intent-child-status-public-stream-proof.mjs`
- `scripts/test/browser-runtime-no-fixture-service-exposure-proof.mjs`
- `test-results/browser-runtime-action-intent-child-status-public-stream-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-action-intent-child-status-public-stream/01-browser-runtime-action-intent-child-status-public-stream-proof.md`
- `cmd /c node scripts/test/browser-runtime-action-intent-child-status-public-stream-proof.mjs`

This is service stream status proof only. It does not claim adapter dispatch,
browser mutation, child intervention execution, final policy execution,
unmanaged exact URL support, or enforcement.

`browser-runtime-social-provider-receipt-stream-parser-proof` carries the
service-published social provider receipt fields into the shared TypeScript
protocol parser and a portal-domain status projection. The parser now reads
social provider receipt boundary rows, provider-dispatch-required rows,
manual-receipt-required rows, provider attempt refs, receipt proof refs,
durable result/store refs, read-model refs, and support-status refs from the
existing browser runtime stream payload. It rejects manual receipt rows that
try to carry durable/provider refs, rejects provider-dispatch-required rows
missing provider attempt or durable refs, and keeps all execution counters at
zero. Portal-domain exposes the parsed status as a parent-visible intent
without reading raw log fields.

Evidence:

- `packages/agent-protocol-domain/src/defaults.ts`
- `packages/agent-protocol-domain/src/browser-runtime-events.ts`
- `packages/agent-protocol-domain/tests/browser-runtime-events.test.ts`
- `packages/portal-domain/src/browser-social-provider-receipt-stream-status.ts`
- `packages/portal-domain/tests/browser-social-provider-receipt-stream-status.test.ts`
- `scripts/test/browser-runtime-social-provider-receipt-stream-parser-proof.mjs`
- `test-results/browser-runtime-social-provider-receipt-stream-parser-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-social-provider-receipt-stream-parser/01-browser-runtime-social-provider-receipt-stream-parser-proof.md`
- `cmd /c node scripts/test/browser-runtime-social-provider-receipt-stream-parser-proof.mjs`

This is public protocol/portal-domain status proof only. It does not claim
provider delivery, provider receipt ingestion runtime, provider webhook
runtime, provider credentials, observed provider receipts, cloud routing,
parent notification UI delivery, report delivery execution, final policy
execution, connector/native runtime, browser mutation, child intervention
execution, unmanaged exact URL support, or enforcement.

## Social Provider Receipt Ingestion Readiness Stream Status Addendum - 2026-06-08

`browser-runtime-social-provider-receipt-ingestion-readiness-status-proof`
projects the parsed social provider receipt stream into a portal-domain receipt
ingestion readiness status. Provider-dispatch-required rows become
ingestion-contract-required because webhook contract, provider credential proof,
durable receipt store proof, and observed provider receipt ingestion remain
unavailable. Manual receipt rows stay manual-required and carry no durable or
provider refs.

Evidence:

- `packages/portal-domain/src/browser-social-provider-receipt-ingestion-readiness-status.ts`
- `packages/portal-domain/tests/browser-social-provider-receipt-ingestion-readiness-status.test.ts`
- `scripts/test/browser-runtime-social-provider-receipt-ingestion-readiness-status-proof.mjs`
- `test-results/browser-runtime-social-provider-receipt-ingestion-readiness-status-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-social-provider-receipt-ingestion-readiness-status/01-browser-runtime-social-provider-receipt-ingestion-readiness-status-proof.md`
- `cmd /c node scripts/test/browser-runtime-social-provider-receipt-ingestion-readiness-status-proof.mjs`

This is portal-domain status projection proof only. It does not claim provider
delivery, provider receipt ingestion runtime, webhook runtime, credentials,
observed provider receipts, cloud routing, parent notification UI delivery,
report delivery execution, final policy execution, connector/native runtime,
browser mutation, child intervention execution, unmanaged exact URL support, or
enforcement. The parent-domain package subpath export remains deferred while
another active lane owns `packages/parent-domain/package.json`.

## Social Provider Receipt Live Activity State Addendum - 2026-06-08

`browser-runtime-social-provider-receipt-live-activity-state-proof` carries the
parsed social provider receipt stream status and receipt ingestion readiness
status into the portal live activity state. The app state derives both
parent-visible intents from the shared protocol-domain stream parser and the
portal-domain status projections, and rejects dishonest receipt rows before
either intent is populated.

Evidence:

- `apps/portal/src/live-activity-state.ts`
- `apps/portal/tests/live-activity-state.test.ts`
- `scripts/test/browser-runtime-social-provider-receipt-live-activity-state-proof.mjs`
- `test-results/browser-runtime-social-provider-receipt-live-activity-state-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-social-provider-receipt-live-activity-state/01-browser-runtime-social-provider-receipt-live-activity-state-proof.md`
- `cmd /c node scripts/test/browser-runtime-social-provider-receipt-live-activity-state-proof.mjs`

This is app state projection proof only. It does not add a new visual surface,
parse raw receipt stream fields in the portal app, claim provider delivery,
provider receipt ingestion runtime, webhook runtime, credentials, observed
provider receipts, report delivery execution, final policy execution, browser
mutation, child intervention execution, unmanaged exact URL support, or
enforcement.

## Social Provider Receipt Route Status UI Addendum - 2026-06-08

`browser-runtime-social-provider-receipt-route-status-ui-proof` renders the
live-activity social provider receipt stream status and receipt ingestion
readiness status in the existing Browser route social alert/report panel. The
proof uses the real portal E2E harness, which starts the Rust agent service and
Vite portal, requests the service-backed Browser route, asserts the receipt
status cards, and captures desktop/mobile screenshots.

Evidence:

- `apps/portal/src/ParentPortalRoute.tsx`
- `apps/portal/src/SocialAlertReportRoutePanel.tsx`
- `apps/portal/e2e/social-alert-report-ui-proof.spec.ts`
- `scripts/test/browser-runtime-social-provider-receipt-route-status-ui-proof.mjs`
- `test-results/browser-runtime-social-provider-receipt-route-status-ui-proof/proof.json`
- `test-results/browser-runtime-social-provider-receipt-route-status-ui-proof/accessibility-summary.json`
- `output/browser-plan-proof/browser-runtime-social-provider-receipt-route-status-ui/06-ui-snapshots/social-alert-report-browser-route.png`
- `output/browser-plan-proof/browser-runtime-social-provider-receipt-route-status-ui/06-ui-snapshots/social-alert-report-browser-route-mobile.png`

This is parent-visible status rendering only. It does not claim provider
delivery, provider receipt ingestion runtime, webhook runtime, credentials,
observed provider receipts, report delivery execution, final policy execution,
browser mutation, child intervention execution, unmanaged exact URL support, or
enforcement.

## Social Report Writer Delivery From Receipt Ingestion Addendum - 2026-06-08

`social-report-writer-delivery-proof` now consumes social provider receipt
ingestion readiness rows through a parent-domain builder instead of relying only
on a static report-writer sample. Provider-dispatch, manual-receipt, and
provider-unavailable receipt-ingestion rows become report-writer manual-required
or unavailable rows, preserving webhook, credential, durable receipt, observed
provider receipt, and runtime delivery proof requirements before any report
artifact or receipt can be claimed.

Evidence:

- `packages/parent-domain/src/social-report-writer-delivery-proof.ts`
- `packages/parent-domain/tests/social-report-writer-delivery-proof.test.ts`
- `scripts/test/social-report-writer-delivery-proof.mjs`
- `test-results/social-report-writer-delivery-proof/proof.json`
- `output/browser-plan-proof/social-report-writer-delivery-proof/01-social-report-writer-delivery-proof.md`
- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- social-report-writer-delivery-proof.test.ts`
- `cmd /c node scripts/test/social-report-writer-delivery-proof.mjs`

This is parent-domain report-writer readiness proof only. It does not claim
external runtime report delivery, provider delivery, provider receipt ingestion
runtime, webhook runtime, credentials, observed provider receipts, parent
notification UI delivery, final policy execution, connector/native runtime,
browser mutation, child intervention execution, unmanaged exact URL support, or
enforcement. Protocol stream exposure remains sequenced behind the active
`packages/agent-protocol-domain/src/defaults.ts` lock.

## Social Parent Notification Delivery Readiness Addendum - 2026-06-08

`social-parent-notification-delivery-readiness-proof` consumes the social report
writer delivery proof and produces a parent-domain parent notification/report
delivery readiness boundary. Parent-owned report-writer rows become
parent-report-status-ready rows with parent-owned report artifact and receipt
refs, but no parent notification UI delivery ref. Receipt-ingestion-backed rows
remain manual-required or unavailable until webhook, credential, durable receipt,
observed provider receipt, and parent notification UI delivery proofs exist.

Evidence:

- `packages/parent-domain/src/social-parent-notification-delivery-readiness.ts`
- `packages/parent-domain/tests/social-parent-notification-delivery-readiness.test.ts`
- `scripts/test/social-parent-notification-delivery-readiness-proof.mjs`
- `test-results/social-parent-notification-delivery-readiness-proof/proof.json`
- `output/browser-plan-proof/social-parent-notification-delivery-readiness-proof/01-social-parent-notification-delivery-readiness-proof.md`
- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- social-parent-notification-delivery-readiness.test.ts`
- `cmd /c node scripts/test/social-parent-notification-delivery-readiness-proof.mjs`

This is parent-domain readiness/status proof only. It does not claim parent
notification UI delivery, external runtime report delivery, provider delivery,
provider receipt ingestion runtime, webhook runtime, credentials, observed
provider receipts, final policy execution, connector/native runtime, browser
mutation, child intervention execution, unmanaged exact URL support, or
enforcement.

## Service Stream Eventing Addendum - 2026-06-08

`browser-runtime-service-stream-eventing-proof` moves the service-side browser
runtime stream projection behind a named local eventing request/subscriber
boundary. The existing portal command stays
`agent.browser.runtime.event-chain.stream.get`, but the service now publishes
`browser.runtime.stream.report.requested` internally and completes the response
through the reusable `ocentra-eventing` request/response path before building
the same public event payload.

Evidence:

- `crates/agent-protocol/src/constants/browser.rs`
- `crates/agent-service/Cargo.toml`
- `crates/agent-service/src/browser_runtime_stream_api.rs`
- `crates/agent-service/src/browser_runtime_stream_request.rs`
- `crates/agent-service/src/browser_runtime_stream_payload.rs`
- `crates/agent-service/src/browser_runtime_stream_events.rs`
- `crates/agent-service/src/browser_runtime_stream_tests.rs`
- `scripts/test/browser-runtime-service-stream-eventing-proof.mjs`
- `test-results/browser-runtime-service-stream-eventing-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-service-stream-eventing/01-browser-runtime-service-stream-eventing-proof.md`

This is an internal service eventing boundary only. It does not change the
portal wire command/event names and does not claim adapter dispatch, browser
mutation, child intervention execution, final policy execution, or enforcement.

## Runtime Stream Topology, Delivery, And Route Status Addendum - 2026-06-08

`browser-runtime-chain-topology-proof` and
`browser-runtime-delivery-decision-proof` now register the service stream request
event `browser.runtime.stream.report.requested` in the reusable browser runtime
topology and delivery-decision reports. The route stays local in-process from the
browser runtime spine to the stream report subscriber; external transport remains
manual-required and the existing portal command remains unchanged.

`browser-runtime-social-provider-receipt-route-status-ui-proof` now also renders
the browser action-intent stream status in the real Browser route social
alert/report panel, next to the receipt stream and receipt ingestion readiness
cards. The proof uses the real Rust agent service and Vite portal, writes an
accessibility summary, and captures desktop/mobile screenshots. The
action-intent status projection lives in `portal-domain`, but the route imports
the focused source directly while C owns the shared barrel/package export files.

Evidence:

- `crates/agent-core/src/browser_event_runtime/topology.rs`
- `crates/agent-core/src/browser_event_runtime/delivery.rs`
- `crates/agent-core/src/browser_event_runtime_tests.rs`
- `packages/portal-domain/src/browser-action-intent-stream-status.ts`
- `packages/portal-domain/tests/browser-action-intent-stream-status.test.ts`
- `apps/portal/src/SocialAlertReportRoutePanel.tsx`
- `apps/portal/e2e/social-alert-report-ui-proof.spec.ts`
- `scripts/test/browser-runtime-chain-topology-proof.mjs`
- `scripts/test/browser-runtime-delivery-decision-proof.mjs`
- `scripts/test/browser-runtime-social-provider-receipt-route-status-ui-proof.mjs`
- `test-results/browser-runtime-chain-topology-proof/proof.json`
- `test-results/browser-runtime-delivery-decision-proof/proof.json`
- `test-results/browser-runtime-social-provider-receipt-route-status-ui-proof/proof.json`
- `test-results/browser-runtime-social-provider-receipt-route-status-ui-proof/accessibility-summary.json`
- `output/browser-plan-proof/browser-runtime-social-provider-receipt-route-status-ui/06-ui-snapshots/social-alert-report-browser-route.png`
- `output/browser-plan-proof/browser-runtime-social-provider-receipt-route-status-ui/06-ui-snapshots/social-alert-report-browser-route-mobile.png`

No-claim boundary: this proves local event topology, local delivery readiness,
and parent-visible stream status only. It does not claim action adapter dispatch,
provider delivery, provider receipt ingestion runtime, browser mutation, child
intervention execution, unmanaged exact URL support, final policy execution, or
enforcement.

## Social Parent Notification Delivery Service UI Addendum - 2026-06-08

`social-parent-notification-delivery-ui-proof` carries the parent-domain social
parent-notification/report delivery readiness boundary into a service-backed
agent protocol command/event and the existing Browser route social alert/report
panel. The route requests
`agent.browser.social-parent-notification-delivery.read-model.get`, parses the
reported readiness snapshot, and renders parent-report-ready, manual-required,
and unavailable rows next to the existing social alert/report, browser
action-intent, and social provider receipt status cards.

Evidence:

- `packages/agent-protocol-domain/src/social-parent-notification-delivery-read-model.ts`
- `crates/agent-protocol/src/social_parent_notification_delivery_read_model.rs`
- `crates/agent-service/src/activity_api/social_parent_notification_delivery_read_model_payload.rs`
- `packages/portal-domain/src/social-parent-notification-delivery-panel.ts`
- `apps/portal/src/SocialAlertReportRoutePanel.tsx`
- `apps/portal/e2e/social-alert-report-ui-proof.spec.ts`
- `scripts/test/social-parent-notification-delivery-ui-proof.mjs`
- `test-results/social-parent-notification-delivery-ui-proof/proof.json`
- `test-results/social-parent-notification-delivery-ui-proof/accessibility-summary.json`
- `output/browser-plan-proof/social-parent-notification-delivery-ui-proof/06-ui-snapshots/social-alert-report-browser-route.png`
- `output/browser-plan-proof/social-parent-notification-delivery-ui-proof/06-ui-snapshots/social-alert-report-browser-route-mobile.png`

No-claim boundary: this is a local service read-model and parent-visible status
projection only. It does not claim parent notification UI delivery, external
runtime report delivery, provider delivery, provider receipt ingestion, final
policy execution, browser mutation, child intervention execution, unmanaged
exact URL support, or enforcement.

## Social Parent Notification Delivery Eventing Addendum - 2026-06-08

`social-parent-notification-delivery-ui-proof` now also proves that the
service-backed parent-notification/report delivery readiness projection is
behind a named local Rust eventing request/subscriber boundary. The existing
portal command remains
`agent.browser.social-parent-notification-delivery.read-model.get`, but the
Rust service publishes
`browser.social.parent-notification-delivery.status.requested` internally and
completes the readiness response through the reusable `ocentra-eventing`
request/response path before building the reported WebSocket event.

Evidence:

- `crates/agent-protocol/src/constants/browser.rs`
- `crates/agent-service/src/activity_api/social_parent_notification_delivery_read_model_payload.rs`
- `crates/agent-service/src/activity_api/social_parent_notification_delivery_read_model_payload_tests.rs`
- `scripts/test/social-parent-notification-delivery-ui-proof.mjs`
- `test-results/social-parent-notification-delivery-ui-proof/proof.json`
- `output/browser-plan-proof/social-parent-notification-delivery-ui-proof/01-social-parent-notification-delivery-ui-proof.md`

No-claim boundary: this is an internal local service eventing boundary only. It
does not change the public portal command/event names and does not claim parent
notification UI delivery, external runtime report delivery, provider delivery,
provider receipt ingestion, final policy execution, browser mutation, child
intervention execution, unmanaged exact URL support, or enforcement.

## Social Report Writer Delivery Event Handoff Addendum - 2026-06-08

`social-parent-notification-delivery-ui-proof` now also separates report-writer
delivery readiness from the parent-notification projection. The service exposes
a local `browser.social.report-writer-delivery.status.requested` request/
subscriber boundary, and the parent-notification subscriber asks that boundary
before deriving its readiness rows. Parent-notification rows preserve the
returned report-writer delivery row ids as `sourceReportWriterDeliveryRowRef`.

Evidence:

- `crates/agent-protocol/src/constants/browser.rs`
- `crates/agent-protocol/src/social_parent_notification_delivery_read_model.rs`
- `crates/agent-service/src/activity_api/social_parent_notification_delivery_read_model_payload.rs`
- `crates/agent-service/src/activity_api/social_parent_notification_delivery_read_model_payload_tests.rs`
- `scripts/test/social-parent-notification-delivery-ui-proof.mjs`
- `test-results/social-parent-notification-delivery-ui-proof/proof.json`
- `output/browser-plan-proof/social-parent-notification-delivery-ui-proof/01-social-parent-notification-delivery-ui-proof.md`

No-claim boundary: this is an internal local service eventing handoff only. It
does not change public portal command/event names and does not claim parent
notification UI delivery, external runtime report delivery, provider delivery,
provider receipt ingestion, final policy execution, browser mutation, child
intervention execution, unmanaged exact URL support, or enforcement.

## Social Report Writer Delivery Route Decision Addendum - 2026-06-08

`browser-runtime-delivery-decision-proof` now also registers the internal
`browser.social.report-writer-delivery.status.requested` and
`browser.social-alert-report.parent-surface.status.requested` requests as
local-in-process routes. The browser runtime delivery-decision report now proves
eight local-ready routes: runtime chain, action-intent status, action-intent
handoff, runtime stream report, social provider receipt status, social
report-writer delivery status, social parent-notification delivery status, and
social alert/report parent-surface status.

Evidence:

- `crates/agent-core/src/browser_event_runtime/delivery.rs`
- `crates/agent-core/src/browser_event_runtime_tests.rs`
- `scripts/test/browser-runtime-delivery-decision-proof.mjs`
- `test-results/browser-runtime-delivery-decision-proof/proof.json`
- `output/browser-plan-proof/browser-runtime-delivery-decision/01-browser-runtime-delivery-decision-proof.md`

No-claim boundary: this is local route-decision proof only. It does not change
public portal command/event names and does not claim external adapter dispatch,
external transport, browser mutation, child intervention execution, final policy
execution, parent notification UI delivery, provider delivery/receipt ingestion,
or enforcement.

## Social Parent Surface Preference Status Addendum - 2026-06-08

`social-alert-report-parent-surface-intent-proof` now combines the social
provider-status handoff with the social preference/quiet-hours status handoff
before projecting parent-visible manual/unavailable surface intent rows. This
matches the shared notification handoff shape used by app/game notifications:
provider status, delivery result state, parent preference state, quiet-hours
decision, notification status refs, preference status refs, audit refs, and
manual-proof refs all travel together for a future authenticated parent surface.

Evidence:

- `packages/parent-domain/src/social-alert-report-parent-surface-intent-proof.ts`
- `packages/parent-domain/tests/social-alert-report-parent-surface-intent-proof.test.ts`
- `scripts/test/social-alert-report-parent-surface-intent-proof.mjs`
- `test-results/social-alert-report-parent-surface-intent-proof/proof.json`
- `test-results/social-alert-report-parent-surface-intent-proof/parent-surface-intent-read-model.json`
- `output/browser-plan-proof/social-alert-report-parent-surface-intent-proof/01-social-alert-report-parent-surface-intent-proof.md`
- `cmd /c node scripts/test/social-alert-report-parent-surface-intent-proof.mjs`

No-claim boundary: this is parent-domain parent-surface status proof only. It
does not render parent notification, preference, frequency-control, or history
UI and does not claim provider delivery, receipt ingestion, child delivery,
quiet-hours timer runtime, report delivery execution, final policy execution,
connector/native runtime, browser mutation, unmanaged exact URL support, or
enforcement.

## Social Parent Surface Service Eventing UI Addendum - 2026-06-08

`social-alert-report-parent-surface-service-ui-proof` carries the social
alert/report parent-surface status projection through a service-backed agent
protocol command/event and the existing Browser route social alert/report
panel. The public route requests
`agent.browser.social-alert-report.parent-surface.read-model.get`, parses the
reported snapshot, and renders provider/preference-derived
manual-action-required plus unavailable-visible parent-surface rows with
desktop/mobile screenshots.

The Rust service projection is behind a named local eventing
request/subscriber boundary. The service publishes
`browser.social-alert-report.parent-surface.status.requested` internally and
completes the response through the reusable `ocentra-eventing`
request/response path before building the reported WebSocket event. The
subscriber first asks the local provider-status and preference-status handoff
subscribers, then projects the returned handoff refs into the parent-surface
read model; it no longer maintains a narrower static parent-surface row source.

Evidence:

- `packages/agent-protocol-domain/src/social-alert-report-parent-surface-read-model.ts`
- `crates/agent-protocol/src/social_alert_report_parent_surface_read_model.rs`
- `crates/agent-service/src/activity_api/social_alert_report_parent_surface_read_model_payload.rs`
- `packages/portal-domain/src/social-alert-report-parent-surface-panel.ts`
- `apps/portal/src/SocialAlertReportRoutePanel.tsx`
- `apps/portal/e2e/social-alert-report-ui-proof.spec.ts`
- `scripts/test/social-alert-report-parent-surface-service-ui-proof.mjs`
- `test-results/social-alert-report-parent-surface-service-ui-proof/proof.json`
- `test-results/social-alert-report-parent-surface-service-ui-proof/accessibility-summary.json`
- `output/browser-plan-proof/social-alert-report-parent-surface-service-ui-proof/06-ui-snapshots/social-alert-report-browser-route.png`
- `output/browser-plan-proof/social-alert-report-parent-surface-service-ui-proof/06-ui-snapshots/social-alert-report-browser-route-mobile.png`

No-claim boundary: this is local service eventing and parent-visible status
projection only. It does not claim parent notification UI delivery, preference
UI delivery, notification history UI, provider delivery, provider receipt
ingestion, provider credentials, cloud routing, child delivery, quiet-hours
timer runtime, retry-worker runtime, production durable outbox storage, adapter
dispatch, report delivery execution, final policy execution, connector/native
runtime, browser mutation, unmanaged exact URL support, or enforcement.

## Authenticated Service-Command Boundary Addendum — 2026-08-27

PR #709 withdrew the unauthenticated agent-service/WebSocket tests that
invoked crate-private dispatcher helpers directly. Those tests were not
evidence of the production boundary
(`run_agent_service` -> `ParentLocalBridgeAdmission` -> `/dev_ws` ->
authenticated handshake/revalidation -> `command_entry`).

The retained WP13 protocol, read-model, payload, eventing, and portal-state
roots prove direct projection and invariant behavior only. They do not prove
an authenticated `/dev_ws` service command, handshake, peer revalidation, or
`command_entry` dispatch. That service-command coverage is
**manual-required** until a real authenticated service integration test
exercises the production boundary. Do not upgrade WP13, SOCIAL-20, or
SOCIAL-22, or product release status, from the retained direct tests.
