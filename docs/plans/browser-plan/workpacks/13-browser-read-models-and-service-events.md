# 13 Browser Read Models And Service Events

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
- `crates/agent-service/src/browser_payload.rs`
- `crates/agent-service/src/browser_evidence_payload.rs`
- `crates/agent-service/src/browser_runtime*.rs`
- `packages/agent-protocol-domain/src/browser-policy-adapter.ts`

## Tests And Proof

- Rust protocol tests.
- Service event tests.
- TypeScript adapter tests.
- Real service proof script updates.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [x] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [x] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [x] Hub lock covers this workpack and exact implementation/docs paths.
- [x] Existing source layout inspected; no parallel browser truth created.
- [x] Before-state source snapshot recorded in `output/browser-plan-proof/13-browser-read-models-and-service-events/00-source-snapshot.md`.
- [x] Contracts updated first where this workpack changes behavior.
- [x] Rust/service parity updated only after contracts exist; portal UI parity remains deferred because no visual surface changed.
- [x] Raw evidence artifacts captured or marked N/A for this service-derived sub-slice: managed status fixtures, unmanaged process rows, and missing-browser status are mapped into read-model payloads and replayable service events; no journal, SQLite, policy, or action behavior changed.
- [x] Tests/proof listed in this workpack are implemented for service read-model payload derivation and the replayable inventory read-model event; portal adapter tests and real service proof scripts remain manual-required.
- [x] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [x] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed, so `ui-not-applicable.md` records why.
- [x] Security/no-claim negative proof captured for this sub-slice: unmanaged rows remain process-only and managed target-list rows do not claim active-tab support.
- [x] Manual platform proof captured for real browser/OS claims; no new real OS/browser claim was made, so `09-manual-platform-proof.md` records the N/A boundary.
- [x] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [x] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [x] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

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
- `apps/portal/src/event-results.ts`
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
browser action-intent status subscriber, and the browser action-intent handoff
subscriber. The current runtime chain is `local-service` ready, the
action-intent status subscriber is `local-in-process` ready, the action-intent
handoff subscriber is `local-in-process` ready, and the external transport
route stays `manual-required` because the custody/auth/encryption/retention/
replay/delete/offset/dedupe/transport artifacts are not present.

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
