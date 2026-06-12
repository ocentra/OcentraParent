<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `AGENTS.md`
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after PLAN_INDEX or FEATURE_ROUTE_INDEX selects the plan.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Route changes require PLAN_STATE.md, ROUTE_INDEX.md, WORKPACK_INDEX.md, PLAN_INDEX.md, and FEATURE_ROUTE_INDEX.md to stay aligned.

<!-- /agent-capsule -->

# Data Custody Storage Plan Agent Route

Use this plan for data custody guarantees, encrypted storage, evidence retention, export, sync, deletion/tombstones, no-stolen-data boundaries, cloud/relay custody, and query/report source truth.

## Research Gate

This plan is still under research. Before implementation, DONE, or PR_READY, the assigned agent must inspect existing storage/eventing/report/query code and docs, map what already exists versus missing, research provider/encryption tradeoffs for the touched slice, and discuss unresolved custody/privacy choices with Sujan. Do not treat first-pass workpacks as final architecture.

## High-Density Execution Contract

Task: work only the assigned slice for this plan.
Context: [PLAN_STATE.md](PLAN_STATE.md) is current state; [WORKPACK_INDEX.md](WORKPACK_INDEX.md) chooses one workpack; [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md) defines required local proof.
Scope rule: one plan, one workpack, exact checklist/proof rows. Adjacent plans are closed until the selected workpack names a handoff.
Implementation rule: docs define expected outcome, boundary, shape, validation, and proof. They do not prescribe implementation code.
Proof rule: proof must include command/log evidence, negative cases, artifact paths, updated rows, and skipped-risk notes when applicable.
Failure condition: no DONE/PR_READY when expected proof is missing, only happy-path evidence exists, or this plan is used to claim adjacent implementation completion.

## Local Decision Tree

- If the assignment names a workpack, open only that workpack.
- If the assignment names a checklist row but no workpack, use [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md), then choose one workpack from [WORKPACK_INDEX.md](WORKPACK_INDEX.md).
- If the assignment changes product status, read [DOC_INDEX.md](DOC_INDEX.md), [PLAN_STATE.md](PLAN_STATE.md), and [PROOF_INDEX.md](PROOF_INDEX.md) only for named rows.
- If the assignment touches adjacent implementation ownership, open only the adjacent plan named by the selected workpack.
- If the assignment is DONE/PR_READY, read [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md), [PROOF_INDEX.md](PROOF_INDEX.md), [PLAN_HEALTH.md](PLAN_HEALTH.md), then [../../agent/PR_DONE_FLOW.md](../../agent/PR_DONE_FLOW.md).

## Required Read Order

1. [PLAN_STATE.md](PLAN_STATE.md)
2. [NEXT_ACTIONS.md](NEXT_ACTIONS.md)
3. [WORKPACK_INDEX.md](WORKPACK_INDEX.md)
4. One assigned workpack under workpacks/
5. [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md)
6. [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md) and [PROOF_INDEX.md](PROOF_INDEX.md) only for named rows/artifacts

## Product Sources

- Feature docs: evidence-store-query.md, reports-notifications-sync.md, remote-lan-mobile-platforms.md
- Expectation docs: data-custody.md, evidence-storage.md, sync-export.md, cloud.md
- Adjacent plans: eventing-plan, portal-ux-household-surfaces-plan, parent-desktop-runtime-package-plan
