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

Status: production code drafted / test-deferred.

The Android package now owns a `ca.ocentra.child.agent` identity, a child
activity/foreground-service entrypoint, and an app-private `child-runtime/`
composition directory. Existing parent-package Android capability adapters are
deliberately retained behind the child shell. The Android package does not
embed or invoke the Rust child-runtime crate because no JNI or native runtime
bridge exists in this platform owner. External transport, device authority,
install/runtime lifecycle, and store readiness remain open.

## Code-drafted boundary

- The package identity and launcher now target the child agent.
- `ChildAgentCompositionService` owns the Android lifecycle entrypoint.
- `ChildAgentComposition` owns the composition directory and reports typed
  `RUST_RUNTIME_MANUAL_REQUIRED` readiness until a JNI/native bridge exists.
- Existing parent-package capability/proof adapters remain explicitly declared
  as legacy platform components behind the child shell; their proof route
  remains deferred and does not define child runtime readiness.

Tests, validation, proof, device-owner/managed-profile evidence, and release
readiness are deferred to the later global validation phase.
