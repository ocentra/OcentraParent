<!-- agent-capsule -->

> Agent Capsule
> Doc: Product Documentation Flow
> Kind: agent flow documentation; read only when selected by root AGENTS or TASK_ROUTER.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Product Documentation Flow

Use this when a feature, expectation, roadmap, capability checklist, product
claim, gap, or status changes.

## Minimum product-doc path

1. Read `docs/FEATURE_ROUTE_INDEX.md` or `docs/feature-list.md` to identify the
   single owning feature doc.
2. Read only that `docs/features/*.md` file, plus a second feature doc only if
   the work clearly crosses a second feature boundary.
3. Read only expectation files linked by the feature doc and relevant to the
   files you will touch.
4. Read the relevant milestone section in `docs/product-roadmap.md` only when
   milestone scope/status/order/completion changes.
5. Read relevant rows in `docs/product-capability-checklist.md` before/after
   when the feature status, proof, or gap changes.
6. Read each touched module README before editing that module.
7. Roadmaps live in `docs/roadmaps/`. Read them to understand the timeline or milestone requirements. Roadmaps are not engineering Plans; do not create workpacks inside roadmaps. Find the owning Plan folder to execute the actual workpacks.

## Reporting

Every `DONE` or PR-ready report must name which feature doc and product
capability checklist row were updated, or explicitly say no product-doc update
was needed and why.
