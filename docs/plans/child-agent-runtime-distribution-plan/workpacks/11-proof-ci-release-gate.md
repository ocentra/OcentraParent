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

- Status is source missing; WP11 is not complete.
- Documentation and historical aggregate proof can describe blockers, but no executable source gate currently joins canonical package identity, trusted startup, authenticated ingress/health, platform lifecycle, removal callbacks, updater/handoff, signing/store/mobile authority, tests, proof, and CI into one fail-closed result.
- No release workflow consumes one authoritative WP11 result.
- WP11 remains last and depends on WP01-WP10 production source. Normal completion still requires every strict test/proof/checklist/review/CI/merge gate.

## Required production source outcome

- one executable aggregate gate consumes the reviewed machine-readable outputs of WP01-WP10;
- every missing, stale, manual-required, unsupported, failed, or unreviewed input keeps release blocked with an exact owner/workpack reason;
- the gate distinguishes source completeness, test status, proof, platform/manual evidence, CI, review, and release approval;
- release workflows consume the result without treating CI success as a substitute for workpack completion.

## Expected test-source gap

- negative fixtures for parent-labelled identities, missing trust, missing authenticated ingress/health, stale lifecycle state, missing removal callbacks, unused handoff/updater integration, unsigned/manual platform rows, missing test/proof inputs, and stale CI/review state;
- deterministic blocker ordering and exact workpack ownership;
- a positive fixture only after every required source/test/proof input is present;
- proof that one green package/platform job cannot close the whole child plan.
