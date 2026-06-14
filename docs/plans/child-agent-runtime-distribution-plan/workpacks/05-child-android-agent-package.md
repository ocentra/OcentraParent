# Workpack 05 - Child Android Agent Package

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `05-child-android-agent-package`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: define the child Android package, install proof, and device-owner gap proof boundary.

## Owns

- Android package and install state
- child-agent lifecycle on Android
- managed-profile or device-owner gap truth
- uninstall and removal behavior on Android

## Must prove

- the package installs as a child-agent artifact on Android
- platform authority limits are explicit for managed-profile or device-owner claims
- install and launch states are honest under the chosen Android mode
- manual-required states are recorded when the platform cannot prove a capability
- no device-owner claim is made without platform evidence

## Failure conditions

- device-owner or managed-profile support is claimed without proof
- install success is treated as full authority
- Android gaps are hidden behind generic mobile wording
- parent-client distribution claims leak into this slice
