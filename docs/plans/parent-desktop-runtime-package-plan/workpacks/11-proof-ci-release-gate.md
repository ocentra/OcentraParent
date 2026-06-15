# Workpack 11 - Proof CI Release Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Workpack: `11-proof-ci-release-gate`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: close the route with proof pointers, CI evidence, and a PR-ready release gate.

## Must prove

- proof is stored in the designated external artifact path
- route docs, plan docs, and workpack docs stay aligned
- CI or local validation evidence is attached to the slice
- no PR-ready claim exists without a negative case and a teardown or rollback path

## Failure conditions

- proof is kept inside the plan folder
- route sync is skipped
- CI success is treated as a substitute for proof
