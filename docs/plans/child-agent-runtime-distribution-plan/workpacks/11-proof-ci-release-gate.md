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

## Execution truth

- WP11 closes as an aggregate proof-gate slice only when the child plan truth matches the current accepted and rejected packets.
- WP11 does not require the overall child plan to be release-ready; it requires the gate to say when release readiness is blocked and why.
- WP11 must keep WP06 and WP09 open until their canonical shared contracts move from hand-authored TypeScript in `schema-domain` to Rust-owned shared contract surfaces and that correction is proved.
- WP11 must keep WP02 and WP04 visibly open-blocked until platform lifecycle execution is proved on the required hosts.
- WP11 completion does not upgrade blocked or rejected workpacks into done. It proves that aggregate route/index/state/proof truth is aligned and that PR-ready release gating is not falsely claimed.
