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

1. Select the smallest current workpack from [WORKPACK_INDEX.md](WORKPACK_INDEX.md).
2. Confirm product source docs in [DOC_INDEX.md](DOC_INDEX.md).
3. Select required proof intents from [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md).
4. Record any adjacent-plan handoff in the selected workpack before opening that adjacent plan.
5. Keep status open until proof artifacts exist and workpack/proof rows are updated.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [ ] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [ ] Record failure conditions, skipped checks, and evidence path in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.

## State

- State remains open until custody runtime, retention, export/import, provider sync, report/query, and parent-storage proof slices are closed with test artifacts.
- Keep this file and `PLAN_STATE.md` synchronized before any DONE/PR_READY claim.
