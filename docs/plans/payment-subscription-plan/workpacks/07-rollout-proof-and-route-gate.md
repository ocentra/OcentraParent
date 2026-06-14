# Workpack 07: Rollout, Proof, and Route Gate

## Goal

Close the loop on route sync, proof storage, and PR-ready readiness.

## First-touch surface

- `docs/proof/payment-subscription-plan/07-validation-command-log.md`
- `scripts/test/real-evidence-proof-checkpoint.mjs`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PLAN_EXECUTION_BLUEPRINT.md](../PLAN_EXECUTION_BLUEPRINT.md)
- [WORKPACK_INDEX.md](../WORKPACK_INDEX.md)
- [NEXT_ACTIONS.md](../NEXT_ACTIONS.md)
- [DOC_INDEX.md](../DOC_INDEX.md)
- [PROOF_AND_TEST_INVENTORY.md](../PROOF_AND_TEST_INVENTORY.md)
- [REQUIRED_TEST_ASSERTION_MATRIX.md](../REQUIRED_TEST_ASSERTION_MATRIX.md)
- [PLAN_EXECUTION_SCORECARD_REVIEW.md](../PLAN_EXECUTION_SCORECARD_REVIEW.md)

## Output files

- [PROOF_AND_TEST_INVENTORY.md](../PROOF_AND_TEST_INVENTORY.md)
- [PLAN_EXECUTION_BLUEPRINT.md](../PLAN_EXECUTION_BLUEPRINT.md)
- [WORKPACK_INDEX.md](../WORKPACK_INDEX.md)
- [DOC_INDEX.md](../DOC_INDEX.md)
- [NEXT_ACTIONS.md](../NEXT_ACTIONS.md)
- `docs/proof/payment-subscription-plan/wp07-rollout-proof-and-route-gate/`

## Acceptance

- The workpack has a proof pointer outside the plan folder.
- Route docs match the workpack tree.
- Validation commands are recorded.
- No checklist item is being used as a proof artifact store.
- Proof artifacts include at least one negative case and one rollback or teardown case.
- Proof artifacts map to the exact assertion IDs for the selected workpack.

## Proof IDs

- `payment-route.plan-sync`
- `payment-route.workpack-proof-manifest`
- `payment-route.validation-log`
- `payment-route.negative-gate`
- `payment-route.rollback-path`

## Validation

- Docs validation: `npm run format:check`; `npm run lint:schema-boundaries`
- Required proof families: `payment-route.plan-sync`, `payment-route.workpack-proof-manifest`, `payment-route.validation-log`, `payment-route.negative-gate`, `payment-route.rollback-path`
- Proof bundle: `docs/proof/payment-subscription-plan/07-route-sync-proof.md`, `docs/proof/payment-subscription-plan/07-proof-path-proof.md`, `docs/proof/payment-subscription-plan/07-validation-command-log.md`
- Validation log template: `07-validation-command-log.md` must record the command, exit status, proof pointer, rollback/teardown pointer, and any remaining gap.
- Rollback artifact template: `07-proof-path-proof.md` must name the teardown evidence path before route closure is claimed.

## Negative cases

- Reject proof that lives inside the plan folder.
- Reject stale route or workpack indexes.
- Reject PR-ready claims without proof pointers.
- Reject proof manifests that do not name commands run.

## Failure conditions

- Do not mark PR-ready without the linked proof path and validation command log.
- Do not let checklist entries masquerade as proof storage.
- Do not claim route closure when the plan index is stale.
