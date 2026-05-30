# Activity/MIA Report History And Action Preview Proof - 2026-05-30

## Scope

This checkpoint covers the next non-visual Activity/MIA runtime proof slice after the
service-backed Activity adapter merge.

It strengthens:

- saved Activity report history metadata
- partial/degraded report storage state
- family/device source-state summary counts
- Parent Assistant/MIA action-preview evidence context from saved Activity reports
- child-agent contract boundaries that prevent direct enforcement or policy writes

## Touched Boundaries

- TypeScript Activity contracts expose `degraded` saved-report storage state and
  per-history-row `sourceStateSummary`.
- Rust protocol mirrors the Activity history source summary and degraded storage state.
- Rust report storage marks partially unreadable/unparseable history folders as
  `storageState=degraded` while still returning valid saved reports.
- Parent Assistant action-preview results now carry `evidenceContext`, so saved Activity
  report citations can accompany draft previews without applying changes.

## Product Truth

- Activity report history remains local parent storage, not Ocentra-hosted storage.
- Storage-unavailable and degraded states are explicit, typed, and renderable.
- Parent Assistant/MIA can cite saved Activity report context, source counts, and saved
  metadata.
- Action preview remains draft-only. It requires controller lease and child-agent contract
  execution before any policy or enforcement write.

## Non-Claims

- No C-owned portal UI, vendor UI, or temp scratchpad paths were edited.
- No API AI adapter was implemented.
- No policy write, enforcement write, child-safety decision, or child-agent runtime behavior
  is performed by this slice.
- Real multi-device fan-out remains represented by typed source states until real household
  device sources are connected.

## Proof Harness

Run:

```powershell
node scripts/test/activity-mia-report-history-action-preview-proof.mjs
```

Expected proof labels:

- `activity-mia-report-history.saved-metadata`
- `activity-mia-report-history.degraded-storage`
- `activity-mia-report-history.source-state-summary`
- `activity-mia-action-preview.saved-report-citations`
- `activity-mia-action-preview.child-contract-non-enforcement`
