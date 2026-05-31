# Activity MIA Final Pass Service Adapter Consumption Checkpoint

Date: 2026-05-31

Branch: `codex/activity-mia-final-pass-service-adapter-consumption`

## Scope

This checkpoint covers the final non-visual Activity/MIA handoff slice from the
post-PR190 main base:

- Activity report generation, save, and historical report listing remain
  service-owned and schema-backed.
- Saved Activity report JSON metadata stays visible through
  `saveActivityReport` and `listHistoricalReports`.
- Storage-unavailable and degraded report-history states stay typed and
  renderable instead of being promoted to ready.
- Family and per-device Activity requests are built through typed adapter
  helpers.
- Family reports preserve reachable, offline, stale/unreachable, and
  error/unavailable source states.
- The adapter manifest explicitly names the command-builder and event-parser
  helpers C can consume after the visual Activity branch lands.
- Parent Assistant/MIA evidence context cites saved Activity reports, source
  counts, source identifiers, and non-enforcing action-preview boundaries.

## C Consumption Boundary

C-owned UI should import the Activity service adapter helpers and operation
manifest rather than inventing command strings, payload fields, event names,
parser routing, or unavailable states.

The manifest maps each operation to:

- the protocol command and success event
- the response payload field and response kind
- the command-builder helper
- the event-parser helper
- read-model kind when the response is a tab read model
- the Rust-service product-data owner
- typed unavailable state and adapter failure reasons

This keeps Vite/Portal on the render/authorship side. Product data, persistence,
source-state aggregation, and Parent Assistant evidence context remain owned by
the Rust service and shared contracts.

## Non-Claims

- No C-owned portal UI, vendor UI, temp scratchpad, `main.rs`, or `websocket.rs`
  integration path is changed here.
- No API AI implementation, policy write, enforcement write, child blocking, or
  child-safety decision is added.
- Physical household fan-out still requires real paired devices; until then,
  typed source states represent offline, stale, unreachable, unavailable, and
  error conditions.

## Evidence Files

- `packages/agent-protocol-domain/src/activity-surface-adapter.ts`
- `packages/agent-protocol-domain/src/activity-surface-adapter-manifest.ts`
- `packages/agent-protocol-domain/tests/activity-surface-adapter.test.ts`
- `crates/agent-service/src/activity_surface_report_store.rs`
- `crates/agent-service/src/activity_surface_report_store_tests.rs`
- `crates/agent-service/src/activity_family_sources_tests.rs`
- `crates/agent-service/src/parent_assistant_evidence_context.rs`
- `scripts/test/activity-parent-assistant-runtime-proof.mjs`
- `scripts/test/activity-mia-evidence-final-pass.mjs`
- `scripts/test/activity-mia-report-history-action-preview-proof.mjs`

## Validation Target

Focused proof harness:

```powershell
node scripts/test/activity-mia-report-history-action-preview-proof.mjs
```

Broader handoff gate before PR-ready:

```powershell
cmd /c npm run validate
```
