# Workpack 11 - Proof CI Release Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `11-proof-ci-release-gate`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: close the child-agent route with proof pointers, CI evidence, and PR-ready release gating.

## Owns

- external proof storage path
- route sync with PLAN_INDEX and route-gate docs
- CI or local validation evidence for the selected slice
- negative-case and teardown proof requirements

## Must prove

- proof is stored in the selected `output/child-agent-runtime-distribution-plan-proof/<workpack-file-stem>/` root
- route docs, state docs, and workpack docs stay aligned
- validation logs are attached to the slice
- PR-ready or DONE is not claimed without a negative case and teardown or uninstall proof
- workpack status and box counts stay open when the matching proof root is empty or only records blockers

## Failure conditions

- proof is kept in the plan folder
- a workpack is marked `done` while its proof root is empty or stale
- route sync is skipped
- CI success is treated as a substitute for proof
- the release gate is claimed without a teardown or uninstall path
