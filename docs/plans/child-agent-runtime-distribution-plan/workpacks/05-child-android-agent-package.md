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

## Execution truth

Status: complete for the Android contract/proof boundary; real device/runtime/authority evidence remains manual-required.

## Checklist truth

- [x] Debug APK plus checksum prove the child-agent artifact state as `debug-apk-built`.
- [x] The chosen Android mode is explicit as `debug-apk-sideload`.
- [x] Install truth is explicit as `manual-install-proof-required`.
- [x] Launch truth is explicit as `manual-launch-proof-required`.
- [x] Removal truth is explicit as `manual-removal-proof-required`.
- [x] Device-owner authority truth is explicit as `manual-required`.
- [x] Managed-profile authority truth is explicit as `manual-required`.
- [x] Device-owner claims are rejected without enrollment evidence.
- [x] Managed-profile claims are rejected without enrollment evidence.
- [x] Package-local Android proof cannot claim LAN/WebSocket transport or enforcement parity.
- [x] The device gate requires separate install, launch, and removal artifacts before readiness can rise.
- [x] Real proof-root artifacts exist under `output/child-agent-runtime-distribution-plan-proof/05-child-android-agent-package/`.
