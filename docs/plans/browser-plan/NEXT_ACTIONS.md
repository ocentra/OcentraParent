# Browser Plan Next Actions

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `Browser Plan Next Actions`
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
3. Read `workpacks/00-owner-boundary-proof-gate.md` to apply current browser owner/import/proof rules.
4. If the workpack owner path is unclear, classify it with `WORKPACK_FAMILIES.md`.
5. Open that workpack and exact checklist rows only.
6. Do not claim `DONE` unless the assigned workpack's acceptance/proof rows are updated and validation is listed.

## Highest-open workpacks by route dependency

1. WP07/WP09 have completed a plan-local source reachability review. Do not
   claim readiness from the existing launcher/parser tests: the service drops
   `BrowserManagedLaunch` and later polls with unknown/process-placeholder
   custody, while the CDP-to-Screen path has no production caller.
2. The next implementation packet must retain the managed launch/process/
   session/private authority in the service-owned `AppState` route, poll from
   that authority, mint target authority only from a same-launch trusted page
   candidate, and invoke the existing Screen handoff. Preserve target-list
   `Unknown` active-tab state and explicit teardown/restart/expiry failures.
3. Before implementation, route exact source and test roots through the global
   graph/coordinator. Required test roots include retained-launch lifecycle,
   owner/custody mismatch, malformed/oversized/timeout bridge responses,
   target disappearance/navigation/process replacement, same-launch target
   authority, Screen handoff outcomes, and no-active-tab-claim regression.
4. This plan-local route intentionally does not edit `graph.json`, global
   indexes/matrices, source, tests, or proof. A later coordinator rebuild is
   required before the packet can be considered READY; all WP07/WP09 rows and
   proof remain open.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [ ] Apply `workpacks/00-owner-boundary-proof-gate.md` to the selected workpack before source changes.
- [ ] Classify ambiguous/reference workpacks with `WORKPACK_FAMILIES.md` before opening source.
- [ ] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [ ] Record failure conditions, skipped checks, and evidence path in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.
