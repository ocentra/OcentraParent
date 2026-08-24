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

1. Independently review source-only WP06 head `63d913b93`. Recheck immutable
   binding/lifecycle validation, corruption rejection, bounded cross-process
   serialization, crash-safe replacement/deletion recovery, link/reparse
   safety, and absence of service-side development/test identity. Repair any
   P0/P1/P2 source finding before canonical integration. Do not add launch,
   bridge, policy, Screen, or enforcement authority in WP06.
2. Canonical `f80b47c6a` is the accepted fail-closed WP07/WP09 source baseline.
   It removes unreachable launch state, env/dev profile authority, placeholder
   bridge polling, and the dead Browser-to-Screen runtime surface. Production
   managed-browser status is explicitly manual-required.
3. Do not recreate the deleted surface. After WP06 source acceptance, the WP07
   source packet requires a
   private owner-issued start/stop lifecycle, retained launch custody, pre/post
   I/O process/port/profile revalidation, confirmed teardown, and explicit
   restart/expiry state. WP09 follows WP07 and may bind targets only from that
   retained owner state; active tab remains `Unknown`.
4. After the repository-wide source wave, repair the exact stale Browser tests
   using old helper arities/private launch fields, then write the missing WP07
   and WP09 integration roots together. Include owner mismatch, process
   replacement, teardown, restart/expiry, malformed/oversized/timeout,
   disappearance/navigation, no-active-tab-claim, and manual-required Screen
   handoff.
5. WP06 is source-drafted and review-pending; WP07 and WP09 remain
   blocked/open. No test, proof, pre-commit, CI, PR_READY, runtime, or DONE
   claim follows from the pushed source packet.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [ ] Apply `workpacks/00-owner-boundary-proof-gate.md` to the selected workpack before source changes.
- [ ] Classify ambiguous/reference workpacks with `WORKPACK_FAMILIES.md` before opening source.
- [ ] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [ ] Record failure conditions, skipped checks, and evidence path in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.
