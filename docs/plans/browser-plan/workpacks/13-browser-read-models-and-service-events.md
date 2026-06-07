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
Remaining read-model work requires portal adapter/UI consumption,
journal/SQLite inventory row proof, recent tab evidence reconciliation,
intervention row reconciliation, real service proof scripts, and UI snapshots
before product-facing status can claim more than the typed service event.
