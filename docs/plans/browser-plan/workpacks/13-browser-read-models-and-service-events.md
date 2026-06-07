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
