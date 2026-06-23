# Workpack 07 - Parent Client Signing Store Matrix

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Workpack: `07-parent-client-signing-store-matrix`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: prove the signing, notarization, and store claims per parent client artifact.

## Ownership boundary

```text
parent-client-runtime-distribution-plan owns per-artifact signing/store matrix proof.
selected platform docs/tools provide source reference only when a platform artifact is selected.
child-agent-runtime-distribution-plan owns child package signing/store states.
setup-install-provisioning-plan owns setup/install readiness, not signing proof.
```

## Must prove

- each artifact has its own signing state
- each artifact has its own store or notarization state
- manual-required states remain visible
- the matrix does not collapse parent and child artifacts together

## Required proof fields

The selected proof must name, at minimum:

```text
artifact_kind
platform
artifact_path
artifact_hash_state
signing_authority
certificate_state
provisioning_profile_state
notarization_state
store_state
review_state
manual_required_state
child_artifact_boundary_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Failure conditions

- signing is claimed without an artifact
- store/notarization status is implied from preview output
- mobile store claims are shared between parent and child
- unsigned or side-loaded artifacts are upgraded into release readiness
- CI package success is used as signing/store proof without explicit artifact evidence
