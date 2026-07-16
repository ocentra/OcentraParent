# Workpack 05 - Parent iOS Package

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Workpack: `05-parent-ios-package`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: prove the parent iOS package, provisioning state, and store/manual-required state.

## Must prove

- `release:package:parent-ios` is the package anchor
- device or simulator install proof is explicit
- provisioning and store state are explicit
- no child-runtime distribution claims appear in the iOS row

## Failure conditions

- scaffold-only output is treated as store-ready
- provisioning is hidden behind generic mobile language
- iOS proof is used to claim desktop or Android parity
