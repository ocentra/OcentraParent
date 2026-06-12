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

1. [WP01 Foundation Contracts And Eventing](workpacks/01-foundation-contracts-and-eventing.md) - required before network runtime or policy claims.
2. [WP02 Passive Capture And Parsing](workpacks/02-passive-capture-and-parsing.md) - required before live-capture, parser, or metadata claims.
3. [WP03 Classification And Correlation](workpacks/03-classification-and-correlation.md) - required before category, tunnel, app/browser/screen correlation, or ambiguity claims.
4. [WP04 Cross Slice Cascade And Parent Surface](workpacks/04-cross-slice-cascade-and-parent-surface.md) - required before parent-visible network rows, AI queue, policy mapping, or notification candidates.
5. [WP05 Intervention Adapter Proof Gates](workpacks/05-intervention-adapter-proof-gates.md) - required before DNS/firewall/VPN/WFP/NetworkExtension/Linux intervention claims.
6. [WP06 Analyzer AI Audit And Risk Budget](workpacks/06-analyzer-ai-audit-and-risk-budget.md) - required before analyzer, AI narrative, or risk-budget claims.
7. [WP07 Performance Security Rollout](workpacks/07-performance-security-rollout.md) - required before production readiness or support claims.
8. [WP08 Control Catalog Reference Routing](workpacks/08-control-catalog-reference-routing.md) - use only to route exact large reference-doc lookups.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.
