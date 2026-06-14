# Workpack 07: Rollout, Proof, and Route Gate

Purpose: close the loop on route sync, proof storage, and PR-ready readiness.

## Owns

- `PROOF_AND_TEST_INVENTORY.md`
- `PLAN_EXECUTION_BLUEPRINT.md`
- `TEST_PROOF_EXPECTATIONS.md`
- `PROOF_INDEX.md`
- `CHECKLIST_INDEX.md`
- route/index sync for this plan

## Must prove

- The workpack has a proof pointer outside the plan folder.
- Route docs match the workpack tree.
- Validation commands are recorded.
- No checklist item is being used as a proof artifact store.

## Proof path

- Use `docs/proof/payment-subscription-plan/wp07/` or the owning crate's local proof directory.

## Failure conditions

- The workpack fails if proof lives inside the plan folder.
- The workpack fails if the route index or workpack index is stale.
- The workpack fails if PR-ready is claimed without proof pointers.
