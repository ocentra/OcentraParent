<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `NEXT_ACTIONS.md`
> Kind: short resume/action list.
> Read when: When starting or resuming this plan after PLAN_STATE.md.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Update when current next product slice changes.

<!-- /agent-capsule -->

# Data Custody Storage Plan Next Actions

1. Resolve the WP07 aggregate-proof publication model so fresh checkouts can
   audit reviewable proof artifacts, then re-accept each upstream workpack from
   current code and validation rather than historical `output/` references.
2. Retain the concrete child-service startup wiring for
   `ChildRuntimeTombstoneEventFlow::recover_pending`; add service-owned restart
   validation and publishable proof later. The source path is wired, but restart
   evidence and aggregate route acceptance remain open.
3. Confirm product source docs in [DOC_INDEX.md](DOC_INDEX.md) for the next selected workpack.
4. Select required proof intents from [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md).
5. Record any adjacent-plan handoff in the selected workpack before opening that adjacent plan.
6. Keep overall plan status open until the remaining workpacks and proof roots are updated; WP07 still carries the remaining open plan-state work.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [ ] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [ ] Record failure conditions, skipped checks, and evidence path in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.

## State

- WP04 retention/delete/tombstone is closed with green implementation, proof artifacts, and focused validation.
- WP01 custody source of truth is closed with a Rust-owned contract, generated TS edge, real schema-domain contract test, focused proof runner, and proof root under `output/data-custody-storage-plan-proof/01-custody-source-of-truth/`.
- WP02 encryption key custody is closed with green implementation, proof artifacts, and focused validation.
- WP05 export/import/backup/recovery is closed with green implementation, proof artifacts, and focused validation.
- WP06 report/query custody is closed with green implementation, proof artifacts, and focused validation.
- WP03 parent-owned cloud sync is closed with green Rust contract/runtime coverage, green schema-domain build/test/proof reruns, and a refreshed proof root under `output/data-custody-storage-plan-proof/03-parent-owned-cloud-sync/`.
- WP07 now has a focused Rust retention lifecycle proof and concrete child-runtime
  startup recovery, but aggregate route acceptance remains open because its
  historic ignored `output/` artifacts are absent from a clean checkout and
  restart validation/publication are not yet retained.
- WP08 parent storage settings/apply flow is closed with green Rust contract/runtime coverage, green schema-domain build/test/proof reruns, and a refreshed proof root under `output/data-custody-storage-plan-proof/08-parent-storage-settings-apply-flow/`.
- State remains open until provider sync, AI runtime custody, rollout refresh, and any carried blocked proof slices are closed with test artifacts or exact blocker resolution.
- Keep this file and `PLAN_STATE.md` synchronized before any DONE/PR_READY claim.
