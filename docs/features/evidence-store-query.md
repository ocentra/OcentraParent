<!-- agent-capsule -->

> Agent Capsule
> Doc: Evidence Store And Query
> Kind: feature documentation; read only when selected by FEATURE_ROUTE_INDEX, PLAN_INDEX, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Evidence Store And Query

## Parent Outcome

Parents can trust that activity summaries, reports, AI explanations, policy
decisions, and alerts are backed by real local evidence references, not UI
guesswork.

## Ocentra Requirement

Child activity evidence is local by default. NDJSON journal records are the
append-only truth. SQLite/read models are query surfaces. Reports and AI
answers cite evidence references.

## Roadmap And Expectations

- Roadmap: V0.2 evidence store, V4 reports, V5 parent product.
- Expectations: [evidence storage](../expectations/evidence-storage.md),
  [data custody](../expectations/data-custody.md),
  [real evidence proof](../expectations/real-evidence-proof.md).
- Modules: `packages/activity-domain`, `crates/agent-core`,
  `crates/agent-service`.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
app inventory, reports/digests, screenshots/live screen, and local-first
privacy.

Competitors show activity history, app usage, reports, and alerts. Ocentra must
show the same parent value, but with local custody and evidence citations.

## Current Ocentra State

- Local evidence and read-model direction exists.
- Browser/app/network/screen evidence paths are represented through contracts
  and service/read-model proof.
- Activity report persistence and family fanout proof exists for the
  backend/read-model boundary.
- Activity report history now loads saved JSON report documents through the
  Rust service store, filters by family/device scope and requested report
  window, and returns typed saved/degraded/storage-unavailable states.
- Activity report source records, saved JSON metadata, and history rows now
  carry typed custody/source labels and `rawChildEvidenceIncluded=false`.

## Current Gap

Retention, migration, corruption recovery, parent-owned export, and
cross-feature query coverage are not yet product-complete. Report history has
service-backed JSON storage/query proof but still needs product UI integration
and retention/export controls.

## Checklist

- [ ] Evidence ids and source ids for every observation.
- [ ] Journal write and replay path.
- [ ] SQLite/read-model query state.
- [x] Custody/source labels for Activity report source records, saved metadata,
      and history rows.
- [ ] Retention labels.
- [ ] Corruption/degraded/recovery state.
- [ ] Report/history query coverage.
- [ ] Parent-owned export/delete path.
- [ ] AI/report/policy citations back to evidence refs.

## Next AI Instructions

Never let portal UI or generated reports invent activity. If a parent-visible
summary exists, trace it to stored evidence, read-model query, source state, and
custody label.
