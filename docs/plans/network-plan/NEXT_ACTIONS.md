# Network Plan Next Actions

<!-- agent-capsule -->

> Agent Capsule
> Plan: `network-plan`
> Doc: `Network Plan Next Actions`
> Kind: resume queue and highest-open work.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This file is the short resume list for the next worker. It is derived from open workpack/checklist status and does not replace the assigned workpack.

## How to use

1. Confirm the hub assignment and lane.
2. Pick only the assigned workpack from the list below.
3. Open that workpack and exact checklist rows only.
4. Do not claim `DONE` unless the assigned workpack's acceptance/proof rows are updated and validation is listed.

## Highest-open workpacks by route dependency

1. Plan-truth repair: keep `PLAN_STATE.md`, `source-index.md`, and `PROOF_INDEX.md` aligned with the real source roots and current proof state before claiming any row progress.
2. [WP01 Foundation Contracts And Eventing](workpacks/01-foundation-contracts-and-eventing.md) - fix canonical contract ownership drift and remove architecture-ban re-exports before treating the foundation as complete.
3. [WP08 Control Catalog Reference Routing](workpacks/08-control-catalog-reference-routing.md) - keep the 363-setting catalog honest as a generated reference surface, not a runtime-complete control surface.
4. [WP02 Passive Capture And Parsing](workpacks/02-passive-capture-and-parsing.md) - regenerate real proof only from the current Rust/TS surfaces, not from stale planned artifact paths.
5. [WP03 Classification And Correlation](workpacks/03-classification-and-correlation.md) - keep classifier/correlation claims tied to real evidence refs, typed tests, and negative cases.
6. [WP04 Cross Slice Cascade And Parent Surface](workpacks/04-cross-slice-cascade-and-parent-surface.md) - keep portal/service/runtime claims sourced from service-backed tests and real runtime-chain refs.
7. [WP05 Intervention Adapter Proof Gates](workpacks/05-intervention-adapter-proof-gates.md) - Windows, Android, and Linux proof are expected locally where relevant; macOS/iOS real proof remains external-platform constrained from this host.
8. [WP06 Analyzer AI Audit And Risk Budget](workpacks/06-analyzer-ai-audit-and-risk-budget.md) - keep AI/risk claims fixture-backed unless real runtime execution artifacts are attached.
9. [WP07 Performance Security Rollout](workpacks/07-performance-security-rollout.md) - treat monitoring, metrics, tracing, alerting, and rollout claims as open until focused evidence exists.

## Blocker routing

### Real dependency blockers

- Browser exact-URL or managed-browser authority rows owned by `browser-plan`.
- Cross-slice screen fallback rows owned by `screen-plan`.
- AI runtime/provider ownership rows owned by `ai-plan`.
- Reusable eventing, replay, delivery, or idempotency semantics owned by `eventing-plan`.
- LAN/family-hub delivery rows owned by `lan-plan`.
- Enforcement-authority rows owned by `v0-8-enforcement-control-plan`.

### External platform constraints

- Real macOS proof.
- Real iOS proof.

### Avoidable local execution gaps

- Only the `01-network-foundation-shim-cleanup` proof pack exists; broader plan proof docs and artifacts are still missing.
- WSL installed but not started.
- Docker Desktop binary present but engine unavailable.
- Android SDK/emulator available, but no current device attached and the remembered Samsung Wi-Fi endpoint may need reconnect/auth.

## Current slice contradiction

- The old `network-foundation-shim-cleanup` contradiction is now locally closed: dead parent-domain `network-flow` and `network-contracts` frontage files are retired, and the stale `@ocentra-parent/parent-domain` `./network-control-catalog` surface is already retired.
- The next network move must reopen a real `WP01` contract/eventing or plan-truth packet, not another parent-domain shim-cleanup pass.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [ ] Repair plan-level truth docs before updating any workpack progress rows.
- [ ] Remove source-index drift and architecture-ban re-export drift from the canonical network surface before calling WP01 truthful.
- [ ] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [ ] Record failure conditions, skipped checks, and evidence path in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.

## Latest checkpoint — 2026-08-09

- WP08 route-boundary test and durable validation manifest are current.
- Continue with the next assigned runtime workpack (WP01-WP07); do not use the
  control catalog or this route test as network implementation proof.
