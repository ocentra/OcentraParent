<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-plan`
> Doc: `NEXT_ACTIONS.md`
> Kind: short resume/action list.
> Read when: When starting or resuming this plan after PLAN_STATE.md.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Update when current next product slice changes.

<!-- /agent-capsule -->

# Native Apps Plan Next Actions

Latest selected slice: WP01 contract-boundary code/test/proof validation is
merged through PR #640/#643; do not treat it as full workpack or plan closure.
The next slice must be selected from the graph and must preserve the app-only
owner boundary.

1. Select the smallest current workpack from [WORKPACK_INDEX.md](WORKPACK_INDEX.md).
2. Read `workpacks/00-owner-boundary-proof-gate.md` to apply current app-plan owner/import/proof rules.
3. If the selected workpack owner path is unclear, classify it with [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md).
4. Confirm product source docs in [DOC_INDEX.md](DOC_INDEX.md).
5. Select required test/proof intents from [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md).
6. Record any adjacent-plan handoff in the selected workpack before opening that adjacent plan.
7. Keep status open until proof artifacts exist and checklist/proof rows are updated.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [ ] Apply `workpacks/00-owner-boundary-proof-gate.md` to the selected workpack before source changes.
- [ ] Classify ambiguous/generated workpacks with `WORKPACK_FAMILIES.md` before opening source.
- [ ] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [ ] Record failure conditions, skipped checks, and evidence path in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.

## State

- Route is still in planning-to-research-to-proof lane; implementation status is not marked complete.
- Keep this file aligned with `PLAN_STATE.md`, `PLAN_HEALTH.md`, and proof artifacts before any DONE/PR_READY update.
