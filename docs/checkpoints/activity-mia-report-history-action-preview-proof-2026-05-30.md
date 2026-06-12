<!-- agent-capsule -->

> Agent Capsule
> Doc: Activity/MIA Report History And Action Preview Proof - 2026-05-30
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

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
