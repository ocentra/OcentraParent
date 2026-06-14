# Workpack 07 - Parent Client Signing Store Matrix

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Workpack: `07-parent-client-signing-store-matrix`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: prove the signing, notarization, and store claims per parent client artifact.

## Must prove

- each artifact has its own signing state
- each artifact has its own store or notarization state
- manual-required states remain visible
- the matrix does not collapse parent and child artifacts together

## Failure conditions

- signing is claimed without an artifact
- store/notarization status is implied from preview output
- mobile store claims are shared between parent and child
