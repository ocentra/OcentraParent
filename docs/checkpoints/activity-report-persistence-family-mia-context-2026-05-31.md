# Activity Report Persistence, Family Fan-Out, And MIA Context Checkpoint

Date: 2026-05-31

Branch: `codex/activity-report-persistence-family-mia-context`

## Scope

This checkpoint covers the non-visual Activity follow-up after PR #186:

- Activity request construction for family and per-device scopes.
- C-consumable Activity adapter operation metadata, including typed unavailable
  failure states and parse failure reasons.
- Saved report JSON metadata and historical report listing behavior.
- Family source fan-out states for reachable, offline, stale/unreachable, and
  error/unavailable child-device sources.
- Parent Assistant/MIA citation context from saved Activity reports, including
  stale, unreachable, unavailable, and offline source identifiers when present.

## Proof Boundary

The Portal remains an authoring/rendering consumer. Product data ownership stays
with the Rust service read models and saved local report store. C can render from
the typed operation manifest and adapter helpers without inventing commands,
payload fields, response kinds, unavailable states, or adapter failure reasons.

The Rust service remains the persistence/runtime owner for report generation,
save, history, source-state summaries, and Parent Assistant evidence context.

## Non-Claims

- No C-owned portal UI, vendor UI, temp scratchpad, `main.rs`, or `websocket.rs`
  integration path is changed here.
- No API AI, policy write, enforcement write, blocking, or child-safety decision
  is added.
- Physical household fan-out is still represented by typed source states until
  real paired devices are connected and routed.

## Evidence Files

- `packages/agent-protocol-domain/src/activity-surface-adapter.ts`
- `packages/agent-protocol-domain/src/activity-surface-adapter-manifest.ts`
- `packages/agent-protocol-domain/tests/activity-surface-adapter.test.ts`
- `crates/agent-service/src/activity_family_sources_tests.rs`
- `scripts/test/activity-parent-assistant-runtime-proof.mjs`
- `scripts/test/activity-mia-evidence-final-pass.mjs`
- `scripts/test/activity-mia-report-history-action-preview-proof.mjs`

## Validation Target

Expected proof harness:

```powershell
node scripts/test/activity-mia-report-history-action-preview-proof.mjs
```

Expected broader handoff gate before PR-ready:

```powershell
cmd /c npm run validate
```
