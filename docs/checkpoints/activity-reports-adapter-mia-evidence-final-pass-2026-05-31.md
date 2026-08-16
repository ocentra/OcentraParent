<!-- agent-capsule -->

> Agent Capsule
> Doc: Activity Reports Adapter And MIA Evidence Final Pass
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Activity Reports Adapter And MIA Evidence Final Pass

Date: 2026-05-31
Branch: `codex/activity-reports-adapter-mia-evidence-final-pass`
Focused proof command: `node scripts/test/activity-mia-evidence-final-pass.mjs`

## Scope

This checkpoint records the non-visual Worker A refresh for Activity report
persistence, the service-adapter operation boundary C can consume later, and
Parent Assistant/MIA evidence context from saved Activity reports.

## Proof Boundary

- Activity reports remain generated, saved, and listed through typed
  Rust-service/read-model commands, not Vite-owned product data.
- The Activity adapter now exposes a typed operation manifest for the exact
  C-facing operations: daily, weekly, monthly, save, history, screen, app use,
  browser, games, and network.
- Every operation in that manifest marks Rust service/read-model ownership,
  C-owned UI consumption, family/device scope support, and `unavailable` as the
  disabled/degraded handoff state.
- Saved report history keeps local JSON metadata, parsed report documents,
  degraded storage state, and family/device source summaries explicit.
- Parent Assistant/MIA report evidence now includes saved metadata, ready
  section counts, offline/stale/unreachable/unavailable counts, and
  offline/stale/unreachable/unavailable source identifiers where available.

## Non-Claims

- This branch does not edit C-owned portal Activity UI, vendor portal,
  temp-scratchpad, parent-assistant API integration, `main.rs`, or `websocket`
  integration paths.
- This branch does not claim physical household multi-device fan-out. It keeps
  unavailable, stale, offline, unreachable, and error source states explicit
  until real devices are connected.
- Parent Assistant/MIA remains citation-bound and does not write policy, apply
  enforcement, authorize API AI, or make child-safety decisions directly.

## Evidence

- `packages/agent-protocol-domain/src/activity-surface-adapter.ts`
- `packages/agent-protocol-domain/src/activity-surface-adapter-manifest.ts`
- `packages/agent-protocol-domain/tests/activity-surface-adapter.test.ts`
- `crates/agent-service/src/activity_surface_report_store.rs`
- `crates/agent-service/src/parent_assistant_evidence_context.rs`
- `crates/agent-service/src/parent_assistant_runtime_tests.rs`
- `scripts/test/activity-surface-main-backed-adapter-proof.mjs`
- `scripts/test/activity-mia-evidence-final-pass.mjs`
- `scripts/test/activity-mia-report-history-action-preview-proof.mjs`

Generated proof evidence is written to:

```text
test-results/activity-mia-evidence-final-pass/proof.json
```
