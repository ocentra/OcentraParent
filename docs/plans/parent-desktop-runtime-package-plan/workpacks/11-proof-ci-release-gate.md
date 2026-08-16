# Workpack 11 - Proof CI Release Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Workpack: `11-proof-ci-release-gate`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: close the route with proof pointers, CI evidence, and a PR-ready release gate.

## Ownership boundary

```text
WP11 aggregates parent-client-runtime-distribution-plan proof roots only.
Adjacent plans own setup, child runtime, device trust, account, payment, policy, remote access, data custody, and portal UX behavior.
CI evidence supports proof but does not replace selected artifact proof.
```

## Must prove

- proof is stored in the designated external artifact path
- route docs, plan docs, and workpack docs stay aligned
- CI or local validation evidence is attached to the slice
- no PR-ready claim exists without a negative case and a teardown or rollback path

## Required release gate fields

The release-gate artifact must name, at minimum:

```text
release_gate_id
accepted_proof_roots
missing_proof_roots
carried_blockers
manual_required_gaps
artifact_claims_allowed
artifact_claims_blocked
signing_store_state
update_rollback_state
launch_smoke_state
setup_handoff_state
child_runtime_boundary_state
ci_validation_state
teardown_or_rollback_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Failure conditions

- proof is kept inside the plan folder
- route sync is skipped
- CI success is treated as a substitute for proof
- PR_READY is claimed while signing/store/update/rollback/manual-required gaps are hidden
- parent client release gate claims setup completion or child runtime readiness
