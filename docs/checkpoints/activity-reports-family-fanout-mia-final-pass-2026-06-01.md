<!-- agent-capsule -->

> Agent Capsule
> Doc: Activity Reports Family Fan-Out MIA Final Pass
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Activity Reports Family Fan-Out MIA Final Pass

Date: 2026-06-01

Branch: `codex/activity-reports-family-fanout-mia-evidence-final-pass`

Focused proof command:

```powershell
node scripts/test/activity-reports-family-fanout-mia-final-pass.mjs
```

## Scope

This checkpoint records the non-visual Activity follow-up after PR #195:

- Activity family aggregation is now an explicit typed contract derived from
  service-owned Activity report and history documents.
- The aggregation model preserves ready, offline, stale, unavailable,
  unreachable, and error source identifiers for family fan-out rendering and
  Parent Assistant/MIA citation context.
- The model requires `dataOwner=rust-service-read-model` and
  `viteDataOwner=false`, keeping Portal/Vite as a renderer and consumer only.
- Storage-unavailable report history remains renderable as an unavailable
  family aggregation with zero sources instead of being promoted to ready.
- Existing report save/history, degraded storage, source-state summary, and
  MIA action-preview citation proof remain the upstream runtime evidence.

## Non-Claims

- No C-owned portal UI, vendor UI, temp scratchpad, parent-assistant API
  constants, `main.rs`, or `websocket.rs` integration path is changed here.
- This is not physical household multi-device fan-out proof. Real paired
  devices and route proofs are still required before upgrading source states to
  live household aggregation.
- Parent Assistant/MIA remains citation context only. This does not write
  policy, authorize API AI, apply enforcement, or make child-device decisions.

## Evidence Files

- `packages/activity-domain/src/activity-family-aggregation.ts`
- `packages/activity-domain/tests/activity-family-aggregation.test.ts`
- `packages/activity-domain/src/activity-surface.ts`
- `crates/agent-service/src/activity_surface_report_store.rs`
- `crates/agent-service/src/activity_family_sources.rs`
- `crates/agent-service/src/parent_assistant_evidence_context.rs`
- `scripts/test/activity-reports-family-fanout-mia-final-pass.mjs`
- `scripts/test/activity-mia-report-history-action-preview-proof.mjs`

Generated proof evidence is written to:

```text
test-results/activity-reports-family-fanout-mia-final-pass/proof.json
```
