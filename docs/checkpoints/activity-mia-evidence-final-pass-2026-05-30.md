# Activity/MIA Evidence Final Pass

Date: 2026-05-30
Branch: `codex/activity-mia-evidence-final-pass`
Focused proof command: `node scripts/test/activity-mia-evidence-final-pass.mjs`

## Scope

This checkpoint records the non-visual Worker A final pass for Activity report
persistence, family/device Activity behavior, the Activity service-adapter
handoff that C can consume later, and Parent Assistant/MIA evidence context from
Activity/report read models.

## Proof Boundary

- Activity reports are generated through the Rust service, carry draft metadata,
  and can be saved as local JSON metadata without Ocentra-hosted activity
  storage.
- `saveActivityReport` and `listHistoricalReports` are exercised through typed
  command/event contracts and the service-backed runtime proof.
- Storage failure remains an explicit `storage-unavailable` state instead of a
  silent success.
- Family reports carry reachable, offline, and error source states. Remote
  device-scoped requests degrade to typed offline reports when the device source
  is unavailable.
- The TypeScript Activity adapter creates command payloads and parses report,
  history, and tab read-model events for the future C-owned visual surface.
- Adapter failures now carry a typed `unavailable` state and a reason so the UI
  can render a disabled/unavailable card without inventing state.
- Parent Assistant/MIA cites saved Activity report evidence, ready section
  counts, offline/unavailable source counts, and action-preview child-agent
  contract boundaries.

## Non-Claims

- This branch does not edit C-owned portal Activity UI, vendor portal,
  temp-scratchpad, parent-assistant API integration, `main.rs`, or websocket
  integration paths.
- This branch does not claim physical household multi-device fan-out. It keeps
  unavailable/offline/error source states explicit until real devices are wired.
- Parent Assistant/MIA remains citation-bound and does not write policy, apply
  enforcement, or make child-safety decisions directly.

## Evidence

- `packages/activity-domain/src/activity-surface.ts`
- `packages/agent-protocol-domain/src/activity-surface-adapter.ts`
- `packages/agent-protocol-domain/tests/activity-surface-adapter.test.ts`
- `crates/agent-service/src/activity_surface_report_store.rs`
- `crates/agent-service/src/parent_assistant_evidence_context.rs`
- `scripts/test/activity-parent-assistant-runtime-proof.mjs`
- `scripts/test/activity-surface-main-backed-adapter-proof.mjs`
- `scripts/test/activity-mia-evidence-final-pass.mjs`

Generated proof evidence is written to:

```text
test-results/activity-mia-evidence-final-pass/proof.json
```
