# WP12 Rollout Proof And PR Gate

Scope: reconcile eventing checklist rows, proof artifacts, plan state, feature routes, and PR/DONE reporting.

Source rows: `05-implementation-workpacks.md` main gates and merge-blocking failures.

Read next:

- `../CHECKLIST_INDEX.md`
- `../PROOF_INDEX.md`
- `../PLAN_HEALTH.md`
- `../TEST_PROOF_EXPECTATIONS.md`
- `../../agent/PR_DONE_FLOW.md`
- `../../agent/VALIDATION_FLOW.md`

Expected outcome:

- Every closed row names exact implementation artifact, test command, proof path, and remaining gap status.
- `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `PLAN_HEALTH.md`, feature docs, and product checklist are synchronized only for claims with proof.
- PR/DONE report names the selected workpack, source rows, checklist rows, proof artifacts, validation commands, skipped risks, and remaining gaps.
- Consumer-plan claims are not moved by eventing proof unless consumer proof exists.
- If any expected proof root is absent, the route stays open and the blocker is
  recorded instead of inferred closed from historical docs.

Expected tests/proof:

- `eventing.rollout.markdown-link-check`
- `eventing.rollout.stale-route-check`
- `eventing.rollout.checklist-proof-reconciliation`
- `eventing.rollout.consumer-claim-negative`
- `eventing.rollout.pr-done-report`

Expected proof artifacts:

- `output/eventing-plan-proof/12-rollout-proof-and-pr-gate/proof-summary.json`
- `output/eventing-plan-proof/12-rollout-proof-and-pr-gate/pr-done-report.md`
- `output/eventing-plan-proof/12-rollout-proof-and-pr-gate/command-logs/`

These paths are the required local route-proof bundle for WP12. They are absent
in this checkout, so WP12 remains open. WP10 is blocked on LAN WP26; WP11 is
implementation-ready/open; WP13 is code-complete but validation/proof-open.

Validation commands:

- `node scripts/test/eventing-rollout-proof.mjs` (currently absent)
- `git diff --check -- docs/proof/eventing-plan docs/plans/eventing-plan`

Failure conditions:

- Do not close a workpack from green compile alone.
- Do not report broad eventing DONE while any required proof tier is missing.
- Do not update product status from eventing-only proof when the product behavior belongs to a consumer plan.
