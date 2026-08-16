# App + Game Plan Next Actions

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `App + Game Plan Next Actions`
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
3. Read `workpacks/00-owner-boundary-proof-gate.md` to apply current app/game owner/import/proof rules.
4. If the workpack title is generated, long, or ambiguous, classify it with `WORKPACK_FAMILIES.md`.
5. Open that workpack and exact checklist rows only.
6. Do not claim `DONE` unless the assigned workpack's acceptance/proof rows are updated and validation is listed.

## Highest-open Phase 1 work by dependency impact

The 2026-08-15 code audit maps all 220 workpacks and leaves 33 source/test
writing gaps. Work from [CODE_AUDIT.md](CODE_AUDIT.md), in this order:

1. WP61-WP65: provider/preference owners and receipt-backed status. WP59's
   scheduler bridge and WP60's metadata-only audit-history bridge are
   implemented and focused-green at `4cf6a11c9` and `bae505ce8`; production
   history persistence, retry/quiet-hours workers, and provider delivery remain
   later runtime boundaries.
2. WP16, WP48, WP63, WP159: cohesive dashboard/source UI and hostile/large
   metadata tests.
3. WP188-WP202, WP204: Android replay/test closure and Linux foreground
   acquisition.
4. WP207 and WP211-WP222: Android child-runtime durability, receipt, delivery,
   notification, action, and focused test closure.
5. WP27 load/performance harnesses; WP102 implement-or-retire decision.

Do not schedule a no-code/proof packet as implementation work, and do not
recreate the removed legacy TypeScript owners named in old workpacks.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [ ] Apply `workpacks/00-owner-boundary-proof-gate.md` to the selected workpack before source changes.
- [ ] Classify ambiguous/generated workpacks with `WORKPACK_FAMILIES.md` before opening source.
- [ ] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [ ] Record failure conditions, skipped checks, and evidence path in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.
